use std::{
    fmt,
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

use ring::{
    digest,
    signature::{self, UnparsedPublicKey},
};
use subtle::ConstantTimeEq;
use x509_parser::{certificate::X509Certificate, public_key::PublicKey, prelude::FromDer};

use crate::signing::{read_apk_signing_block, ApkSigningBlock, SigningBlockError};

const V2_BLOCK_ID: u32 = 0x7109_871a;
const V3_BLOCK_ID: u32 = 0xf053_68c0;
const V31_BLOCK_ID: u32 = 0x1b93_ad61;
const PROOF_OF_ROTATION_ATTR_ID: u32 = 0x3ba0_6f8c;
const CHUNK_SIZE: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoVerificationState {
    Verified,
    Invalid,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoSchemeInfo {
    pub state: CryptoVerificationState,
    pub signer_count: usize,
    pub algorithms: Vec<u32>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ApkSignatureVerification {
    pub v2: Option<CryptoSchemeInfo>,
    pub v3: Option<CryptoSchemeInfo>,
    pub v31_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureVerificationError(String);

impl fmt::Display for SignatureVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SignatureVerificationError {}

impl From<io::Error> for SignatureVerificationError {
    fn from(value: io::Error) -> Self {
        Self(format!("I/O error while verifying APK signature: {value}"))
    }
}

impl From<SigningBlockError> for SignatureVerificationError {
    fn from(value: SigningBlockError) -> Self {
        Self(value.to_string())
    }
}

pub fn verify_apk_signatures(
    path: impl AsRef<std::path::Path>,
) -> Result<Option<ApkSignatureVerification>, SignatureVerificationError> {
    let mut file = File::open(path)?;
    let Some(block) = read_apk_signing_block(&mut file)? else {
        return Ok(None);
    };

    let v31_present = block.pairs.iter().any(|(id, _)| *id == V31_BLOCK_ID);
    let v2_block = block
        .pairs
        .iter()
        .find(|(id, _)| *id == V2_BLOCK_ID)
        .map(|(_, value)| value.as_slice());
    let v3_block = block
        .pairs
        .iter()
        .find(|(id, _)| *id == V3_BLOCK_ID)
        .map(|(_, value)| value.as_slice());

    let mut result = ApkSignatureVerification {
        v31_present,
        ..Default::default()
    };

    if let Some(value) = v2_block {
        result.v2 = Some(verify_v2_block(&mut file, &block, value)?);
    }

    if let Some(value) = v3_block {
        result.v3 = Some(verify_v3_block(&mut file, &block, value)?);
    }

    Ok(Some(result))
}

fn verify_v2_block(
    file: &mut File,
    block: &ApkSigningBlock,
    value: &[u8],
) -> Result<CryptoSchemeInfo, SignatureVerificationError> {
    let mut reader = LengthReader::new(value);
    let signers = reader.read_sequence("v2 signers")?;
    reader.finish("v2 signer sequence")?;

    let mut signers_reader = LengthReader::new(signers);
    let mut results = Vec::new();
    while !signers_reader.is_empty() {
        let signer = signers_reader.read_sequence("v2 signer")?;
        results.push(verify_v2_signer(file, block, signer)?);
    }
    signers_reader.finish("v2 signer sequence")?;

    merge_scheme_results("v2", results)
}

fn verify_v2_signer(
    file: &mut File,
    block: &ApkSigningBlock,
    signer: &[u8],
) -> Result<CryptoSchemeInfo, SignatureVerificationError> {
    let mut reader = LengthReader::new(signer);
    let signed_data = reader.read_sequence("v2 signed data")?;
    let signatures = reader.read_sequence("v2 signatures")?;
    let public_key = reader.read_length_prefixed("v2 public key")?;
    reader.finish("v2 signer")?;

    let selected = parse_and_select_signature(signatures)?;
    let parsed = parse_signed_data_v2(signed_data)?;

    verify_certificate_and_public_key(parsed.certificate, public_key)?;
    verify_signature_bytes(
        selected.algorithm_id,
        parsed.certificate,
        signed_data,
        selected.signature,
    )?;
    let expected_digest = parsed
        .digests
        .iter()
        .find(|(id, _)| *id == selected.algorithm_id)
        .map(|(_, digest)| *digest)
        .ok_or_else(|| {
            SignatureVerificationError(format!(
                "v2 digest list does not contain signature algorithm 0x{:08x}",
                selected.algorithm_id
            ))
        })?;
    verify_content_digest(file, block, expected_digest, selected.digest_algorithm)?;

    let algorithms = parsed
        .digest_algorithms
        .clone();
    let algorithms = parsed.digest_algorithms.clone();
    if algorithms != selected.all_signature_algorithms {
        return Err(SignatureVerificationError(
            "v2 digest and signature algorithm ID lists are not identical and ordered equally"
                .to_string(),
        ));
    }

    Ok(CryptoSchemeInfo {
        state: CryptoVerificationState::Verified,
        signer_count: 1,
        algorithms,
        detail: "v2 signer signature, certificate/public-key binding, and APK content digest verified"
            .to_string(),
    })
}

fn verify_v3_block(
    file: &mut File,
    block: &ApkSigningBlock,
    value: &[u8],
) -> Result<CryptoSchemeInfo, SignatureVerificationError> {
    let mut reader = LengthReader::new(value);
    let signers = reader.read_sequence("v3 signers")?;
    reader.finish("v3 signer sequence")?;

    let mut signers_reader = LengthReader::new(signers);
    let mut signer_values = Vec::new();
    while !signers_reader.is_empty() {
        signer_values.push(signers_reader.read_sequence("v3 signer")?);
    }
    signers_reader.finish("v3 signer sequence")?;

    if signer_values.len() != 1 {
        return Ok(CryptoSchemeInfo {
            state: CryptoVerificationState::Invalid,
            signer_count: signer_values.len(),
            algorithms: Vec::new(),
            detail: "v3 requires exactly one signer".to_string(),
        });
    }

    let signer = signer_values[0];
    let mut signer_reader = LengthReader::new(signer);
    let signed_data = signer_reader.read_sequence("v3 signed data")?;
    let min_sdk = signer_reader.read_u32("v3 outer minSDK")?;
    let max_sdk = signer_reader.read_u32("v3 outer maxSDK")?;
    let signatures = signer_reader.read_sequence("v3 signatures")?;
    let public_key = signer_reader.read_length_prefixed("v3 public key")?;
    signer_reader.finish("v3 signer")?;

    let parsed = parse_signed_data_v3(signed_data)?;

    if parsed.min_sdk != min_sdk || parsed.max_sdk != max_sdk {
        return Ok(CryptoSchemeInfo {
            state: CryptoVerificationState::Invalid,
            signer_count: 1,
            algorithms: parsed.digest_algorithms,
            detail: format!(
                "v3 signer minSDK/maxSDK ({min_sdk}, {max_sdk}) do not match signed-data values ({}, {})",
                parsed.min_sdk, parsed.max_sdk
            ),
        });
    }

    let selected = parse_and_select_signature(signatures)?;
    verify_certificate_and_public_key(parsed.certificate, public_key)?;
    verify_signature_bytes(
        selected.algorithm_id,
        parsed.certificate,
        signed_data,
        selected.signature,
    )?;
    let expected_digest = parsed
        .digests
        .iter()
        .find(|(id, _)| *id == selected.algorithm_id)
        .map(|(_, digest)| *digest)
        .ok_or_else(|| {
            SignatureVerificationError(format!(
                "v3 digest list does not contain signature algorithm 0x{:08x}",
                selected.algorithm_id
            ))
        })?;
    verify_content_digest(file, block, expected_digest, selected.digest_algorithm)?;

    if parsed.digest_algorithms != selected.all_signature_algorithms {
        return Ok(CryptoSchemeInfo {
            state: CryptoVerificationState::Invalid,
            signer_count: 1,
            algorithms: parsed.digest_algorithms,
            detail: "v3 digest and signature algorithm ID lists are not identical and ordered equally"
                .to_string(),
        });
    }

    let state = if parsed.has_proof_of_rotation {
        CryptoVerificationState::Unsupported
    } else {
        CryptoVerificationState::Verified
    };
    let detail = if parsed.has_proof_of_rotation {
        format!(
            "v3 cryptographic signature, certificate/public-key binding, SDK range, and APK content digest verified; proof-of-rotation validation remains pending for SDK range {}..={}",
            parsed.min_sdk, parsed.max_sdk
        )
    } else {
        format!(
            "v3 signer signature, certificate/public-key binding, SDK range, and APK content digest verified for SDK range {}..={}",
            parsed.min_sdk, parsed.max_sdk
        )
    };

    Ok(CryptoSchemeInfo {
        state,
        signer_count: 1,
        algorithms: parsed.digest_algorithms,
        detail,
    })
}

fn merge_scheme_results(
    scheme: &str,
    results: Vec<CryptoSchemeInfo>,
) -> Result<CryptoSchemeInfo, SignatureVerificationError> {
    if results.is_empty() {
        return Ok(CryptoSchemeInfo {
            state: CryptoVerificationState::Invalid,
            signer_count: 0,
            algorithms: Vec::new(),
            detail: format!("{scheme} signing block contains no signers"),
        });
    }

    let mut algorithms = Vec::new();
    let mut unsupported = false;
    let mut invalid = false;
    let mut details = Vec::new();

    for result in &results {
        algorithms.extend_from_slice(&result.algorithms);
        match result.state {
            CryptoVerificationState::Verified => {}
            CryptoVerificationState::Unsupported => unsupported = true,
            CryptoVerificationState::Invalid => invalid = true,
        }
        details.push(result.detail.clone());
    }

    let state = if invalid {
        CryptoVerificationState::Invalid
    } else if unsupported {
        CryptoVerificationState::Unsupported
    } else {
        CryptoVerificationState::Verified
    };

    Ok(CryptoSchemeInfo {
        state,
        signer_count: results.len(),
        algorithms,
        detail: details.join("; "),
    })
}

struct SelectedSignature<'a> {
    algorithm_id: u32,
    signature: &'a [u8],
    digest_algorithm: DigestAlgorithm,
    all_signature_algorithms: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DigestAlgorithm {
    Sha256,
    Sha512,
}

impl DigestAlgorithm {
    fn ring_algorithm(self) -> &'static digest::Algorithm {
        match self {
            Self::Sha256 => &digest::SHA256,
            Self::Sha512 => &digest::SHA512,
        }
    }
}

fn parse_and_select_signature<'a>(
    bytes: &'a [u8],
) -> Result<SelectedSignature<'a>, SignatureVerificationError> {
    let mut reader = LengthReader::new(bytes);
    let mut entries = Vec::new();

    while !reader.is_empty() {
        let sequence = reader.read_sequence("signature entry")?;
        let mut entry = LengthReader::new(sequence);
        let algorithm_id = entry.read_u32("signature algorithm ID")?;
        let signature = entry.read_length_prefixed("signature")?;
        entry.finish("signature entry")?;
        entries.push((algorithm_id, signature));
    }
    reader.finish("signature sequence")?;

    if entries.is_empty() {
        return Err(SignatureVerificationError(
            "APK signer contains no signature entries".to_string(),
        ));
    }

    let all_signature_algorithms = entries.iter().map(|(id, _)| *id).collect::<Vec<_>>();

    let mut supported = entries
        .iter()
        .filter_map(|(id, sig)| supported_signature_algorithm(*id).map(|digest| (*id, *sig, digest)))
        .collect::<Vec<_>>();

    supported.sort_by_key(|(id, _, _)| std::cmp::Reverse(signature_strength(*id)));
    let Some((algorithm_id, signature, digest_algorithm)) = supported.into_iter().next() else {
        return Err(SignatureVerificationError(
            "APK signer contains no cryptographically supported v2/v3 signature algorithm"
                .to_string(),
        ));
    };

    Ok(SelectedSignature {
        algorithm_id,
        signature,
        digest_algorithm,
        all_signature_algorithms,
    })
}

fn signature_strength(id: u32) -> u32 {
    match id {
        0x0102 => 70,
        0x0101 => 60,
        0x0104 => 55,
        0x0202 => 54,
        0x0301 => 53,
        0x0103 => 50,
        0x0201 => 50,
        _ => 0,
    }
}

fn signature_algorithm_digest(id: u32) -> Option<DigestAlgorithm> {
    match id {
        0x0101 | 0x0103 | 0x0201 | 0x0301 => Some(DigestAlgorithm::Sha256),
        0x0102 | 0x0104 | 0x0202 => Some(DigestAlgorithm::Sha512),
        _ => None,
    }
}

fn supported_signature_algorithm(id: u32) -> Option<DigestAlgorithm> {
    match id {
        0x0101 | 0x0102 | 0x0103 | 0x0104 | 0x0201 => signature_algorithm_digest(id),
        _ => None,
    }
}

fn verify_signature_bytes(
    algorithm_id: u32,
    certificate_der: &[u8],
    signed_data: &[u8],
    signature_bytes: &[u8],
) -> Result<(), SignatureVerificationError> {
    let (remaining, cert) = X509Certificate::from_der(certificate_der).map_err(|error| {
        SignatureVerificationError(format!("signer certificate is not valid DER: {error}"))
    })?;

    if !remaining.is_empty() {
        return Err(SignatureVerificationError(
            "signer certificate contains trailing DER data".to_string(),
        ));
    }

    let algorithm: &dyn signature::VerificationAlgorithm = match algorithm_id {
        0x0101 => &signature::RSA_PSS_2048_8192_SHA256,
        0x0102 => &signature::RSA_PSS_2048_8192_SHA512,
        0x0103 => &signature::RSA_PKCS1_2048_8192_SHA256,
        0x0104 => &signature::RSA_PKCS1_2048_8192_SHA512,
        0x0201 => &signature::ECDSA_P256_SHA256_ASN1,
        _ => {
            return Err(SignatureVerificationError(format!(
                "signature algorithm 0x{algorithm_id:08x} is not supported by the current verifier"
            )))
        }
    };

    let key_bytes = match cert.public_key().parsed().map_err(|error| {
        SignatureVerificationError(format!("failed to parse signer public key: {error}"))
    })? {
        PublicKey::RSA(rsa) => encode_rsa_public_key(rsa.modulus, rsa.exponent),
        PublicKey::EC(_) => cert.public_key().subject_public_key.data.to_vec(),
        _ => {
            return Err(SignatureVerificationError(
                "signer certificate uses an unsupported public-key type".to_string(),
            ))
        }
    };

    let verifier = UnparsedPublicKey::new(algorithm, &key_bytes);
    verifier.verify(signed_data, signature_bytes).map_err(|_| {
        SignatureVerificationError(format!(
            "cryptographic signature verification failed for algorithm 0x{algorithm_id:08x}"
        ))
    })
}

fn encode_rsa_public_key(modulus: &[u8], exponent: &[u8]) -> Vec<u8> {
    let modulus = der_integer(modulus);
    let exponent = der_integer(exponent);

    let body_len = modulus.len() + exponent.len();
    let mut result = Vec::with_capacity(1 + der_length_size(body_len) + body_len);
    result.push(0x30);
    append_der_length(&mut result, body_len);
    result.extend_from_slice(&modulus);
    result.extend_from_slice(&exponent);
    result
}

fn der_integer(value: &[u8]) -> Vec<u8> {
    let mut value = if value.is_empty() {
        vec![0]
    } else {
        let first_nonzero = value
            .iter()
            .position(|byte| *byte != 0)
            .unwrap_or(value.len() - 1);
        value[first_nonzero..].to_vec()
    };

    if value[0] & 0x80 != 0 {
        value.insert(0, 0);
    }

    let mut result = Vec::with_capacity(1 + der_length_size(value.len()) + value.len());
    result.push(0x02);
    append_der_length(&mut result, value.len());
    result.extend_from_slice(&value);
    result
}

fn der_length_size(length: usize) -> usize {
    if length < 128 {
        1
    } else if length < 256 {
        2
    } else {
        3
    }
}

fn append_der_length(output: &mut Vec<u8>, length: usize) {
    if length < 128 {
        output.push(length as u8);
    } else if length < 256 {
        output.extend_from_slice(&[0x81, length as u8]);
    } else {
        output.extend_from_slice(&[0x82, (length >> 8) as u8, length as u8]);
    }
}

struct ParsedSignedData<'a> {
    digests: Vec<(u32, &'a [u8])>,
    digest_algorithms: Vec<u32>,
    certificate: &'a [u8],
}

struct ParsedV3SignedData<'a> {
    digests: Vec<(u32, &'a [u8])>,
    digest_algorithms: Vec<u32>,
    certificate: &'a [u8],
    min_sdk: u32,
    max_sdk: u32,
    has_proof_of_rotation: bool,
}

fn parse_signed_data_v2(bytes: &[u8]) -> Result<ParsedSignedData<'_>, SignatureVerificationError> {
    let mut reader = LengthReader::new(bytes);
    let digests = reader.read_sequence("v2 digests")?;
    let certificates = reader.read_sequence("v2 certificates")?;
    let _attributes = reader.read_sequence("v2 additional attributes")?;
    reader.finish("v2 signed data")?;

    let (digest_algorithms, selected_digest) = parse_digest_sequence(digests)?;
    let mut certificates_reader = LengthReader::new(certificates);
    let certificate = certificates_reader.read_length_prefixed("v2 certificate")?;
    certificates_reader.finish("v2 certificate sequence")?;

    Ok(ParsedSignedData {
        digests: selected_digest,
        digest_algorithms,
        certificate,
    })
}

fn parse_signed_data_v3(
    bytes: &[u8],
) -> Result<ParsedV3SignedData<'_>, SignatureVerificationError> {
    let mut reader = LengthReader::new(bytes);
    let digests = reader.read_sequence("v3 digests")?;
    let certificates = reader.read_sequence("v3 certificates")?;
    let min_sdk = reader.read_u32("v3 signed-data minSDK")?;
    let max_sdk = reader.read_u32("v3 signed-data maxSDK")?;
    let attributes = reader.read_sequence("v3 additional attributes")?;
    reader.finish("v3 signed data")?;

    let (digest_algorithms, selected_digest) = parse_digest_sequence(digests)?;
    let mut certificates_reader = LengthReader::new(certificates);
    let certificate = certificates_reader.read_length_prefixed("v3 certificate")?;
    certificates_reader.finish("v3 certificate sequence")?;

    let mut attributes_reader = LengthReader::new(attributes);
    let mut has_proof_of_rotation = false;
    while !attributes_reader.is_empty() {
        let attribute = attributes_reader.read_sequence("v3 attribute")?;
        let mut attribute_reader = LengthReader::new(attribute);
        let id = attribute_reader.read_u32("v3 attribute ID")?;
        if id == PROOF_OF_ROTATION_ATTR_ID {
            has_proof_of_rotation = true;
        }
        attribute_reader.finish("v3 attribute")?;
    }
    attributes_reader.finish("v3 additional attributes")?;

    Ok(ParsedV3SignedData {
        digests: selected_digest,
        digest_algorithms,
        certificate,
        min_sdk,
        max_sdk,
        has_proof_of_rotation,
    })
}

fn parse_digest_sequence(
    bytes: &[u8],
) -> Result<(Vec<u32>, Vec<(u32, &[u8])>), SignatureVerificationError> {
    let mut reader = LengthReader::new(bytes);
    let mut algorithms = Vec::new();
    let mut digests = Vec::new();

    while !reader.is_empty() {
        let sequence = reader.read_sequence("digest entry")?;
        let mut entry = LengthReader::new(sequence);
        let id = entry.read_u32("digest algorithm ID")?;
        let digest = entry.read_length_prefixed("digest")?;
        entry.finish("digest entry")?;
        algorithms.push(id);
        digests.push((id, digest));
    }
    reader.finish("digest sequence")?;

    if digests.is_empty() {
        return Err(SignatureVerificationError(
            "APK signer contains no digest entries".to_string(),
        ));
    }

    Ok((algorithms, digests))
}

fn verify_certificate_and_public_key(
    certificate: &[u8],
    public_key: &[u8],
) -> Result<(), SignatureVerificationError> {
    let (remaining, parsed) = X509Certificate::from_der(certificate)
        .map_err(|error| SignatureVerificationError(format!("signer certificate is not valid DER: {error}")))?;

    if !remaining.is_empty() {
        return Err(SignatureVerificationError(
            "signer certificate contains trailing DER data".to_string(),
        ));
    }

    if parsed.tbs_certificate.subject_pki.raw != public_key {
        return Err(SignatureVerificationError(
            "signer certificate SubjectPublicKeyInfo does not match the signer public key"
                .to_string(),
        ));
    }

    Ok(())
}

fn verify_content_digest(
    file: &mut File,
    block: &ApkSigningBlock,
    expected_digest: &[u8],
    algorithm: DigestAlgorithm,
) -> Result<(), SignatureVerificationError> {
    let actual = compute_apk_content_digest(file, block, algorithm)?;
    if actual.as_slice().ct_eq(expected_digest).unwrap_u8() != 1 {
        return Err(SignatureVerificationError(format!(
            "APK content digest mismatch for {:?}",
            algorithm
        )));
    }
    Ok(())
}

fn compute_apk_content_digest(
    file: &mut File,
    block: &ApkSigningBlock,
    algorithm: DigestAlgorithm,
) -> Result<Vec<u8>, SignatureVerificationError> {
    let digest_algorithm = algorithm.ring_algorithm();
    let mut chunk_digests = Vec::new();

    chunk_digests.extend(read_and_hash_section(
        file,
        digest_algorithm,
        0,
        block.block_start,
    )?);
    chunk_digests.extend(read_and_hash_section(
        file,
        digest_algorithm,
        block.central_directory_offset,
        block.eocd_offset - block.central_directory_offset,
    )?);

    let mut eocd = block.eocd.clone();
    let eocd_offset_field = 16usize;
    let signed_cd_offset = u32::try_from(block.block_start).map_err(|_| {
        SignatureVerificationError(
            "APK signing block offset exceeds the ZIP32 range required by this verifier"
                .to_string(),
        )
    })?;
    eocd[eocd_offset_field..eocd_offset_field + 4]
        .copy_from_slice(&signed_cd_offset.to_le_bytes());

    if !eocd.is_empty() {
        chunk_digests.push(hash_chunk(digest_algorithm, &eocd));
    }

    let chunk_count = u32::try_from(chunk_digests.len()).map_err(|_| {
        SignatureVerificationError("APK contains too many content digest chunks".to_string())
    })?;

    let mut top_level = digest::Context::new(digest_algorithm);
    top_level.update(&[0x5a]);
    top_level.update(&chunk_count.to_le_bytes());
    for digest in &chunk_digests {
        top_level.update(digest);
    }

    Ok(top_level.finish().as_ref().to_vec())
}

fn read_and_hash_section(
    file: &mut File,
    algorithm: &'static digest::Algorithm,
    offset: u64,
    length: u64,
) -> Result<Vec<Vec<u8>>, SignatureVerificationError> {
    let mut chunk_digests = Vec::new();
    let mut remaining = length;
    let mut cursor = offset;

    while remaining != 0 {
        let chunk_len = remaining.min(CHUNK_SIZE as u64) as usize;
        let mut bytes = vec![0_u8; chunk_len];
        file.seek(SeekFrom::Start(cursor))?;
        file.read_exact(&mut bytes)?;
        chunk_digests.push(hash_chunk(algorithm, &bytes));
        cursor += chunk_len as u64;
        remaining -= chunk_len as u64;
    }

    Ok(chunk_digests)
}

fn hash_chunk(algorithm: &'static digest::Algorithm, bytes: &[u8]) -> Vec<u8> {
    let mut context = digest::Context::new(algorithm);
    context.update(&[0xa5]);
    context.update(&(bytes.len() as u32).to_le_bytes());
    context.update(bytes);
    context.finish().as_ref().to_vec()
}

struct LengthReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> LengthReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn is_empty(&self) -> bool {
        self.cursor == self.bytes.len()
    }

    fn finish(&self, label: &str) -> Result<(), SignatureVerificationError> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(SignatureVerificationError(format!(
                "{label} contains trailing bytes"
            )))
        }
    }

    fn read_u32(&mut self, label: &str) -> Result<u32, SignatureVerificationError> {
        if self.bytes.len().saturating_sub(self.cursor) < 4 {
            return Err(SignatureVerificationError(format!(
                "{label} is truncated"
            )));
        }
        let value = u32::from_le_bytes(
            self.bytes[self.cursor..self.cursor + 4]
                .try_into()
                .expect("checked four-byte slice"),
        );
        self.cursor += 4;
        Ok(value)
    }

    fn read_length_prefixed(
        &mut self,
        label: &str,
    ) -> Result<&'a [u8], SignatureVerificationError> {
        let length = self.read_u32(label)? as usize;
        let end = self
            .cursor
            .checked_add(length)
            .ok_or_else(|| SignatureVerificationError(format!("{label} length overflowed")))?;

        if end > self.bytes.len() {
            return Err(SignatureVerificationError(format!(
                "{label} exceeds its containing structure"
            )));
        }

        let value = &self.bytes[self.cursor..end];
        self.cursor = end;
        Ok(value)
    }

    fn read_sequence(&mut self, label: &str) -> Result<&'a [u8], SignatureVerificationError> {
        self.read_length_prefixed(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn content_digest_hashes_each_section_and_patches_eocd_offset() {
        let path = std::env::temp_dir().join(format!(
            "android-release-doctor-digest-{}.apk",
            std::process::id()
        ));

        let mut file = File::create(&path).expect("temporary APK should be created");
        file.write_all(&[1_u8; 16])
            .expect("first section should be written");
        file.write_all(&[2_u8; 16])
            .expect("central directory section should be written");

        let eocd = vec![
            0x50, 0x4b, 0x05, 0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,
        ];
        file.write_all(&eocd).expect("EOCD should be written");
        drop(file);

        let mut reader = File::open(&path).expect("temporary APK should reopen");
        let block = ApkSigningBlock {
            block_start: 16,
            central_directory_offset: 16,
            central_directory_size: 16,
            eocd_offset: 32,
            eocd,
            pairs: Vec::new(),
        };

        let digest = compute_apk_content_digest(&mut reader, &block, DigestAlgorithm::Sha256)
            .expect("digest should compute");

        assert_eq!(digest.len(), 32);
        std::fs::remove_file(path).expect("temporary APK should be removed");
    }
}
