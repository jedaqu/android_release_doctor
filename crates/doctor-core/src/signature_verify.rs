use std::{
    fmt,
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

use p384::ecdsa::{
    signature::hazmat::PrehashVerifier, Signature as P384Signature,
    VerifyingKey as P384VerifyingKey,
};
use p521::ecdsa::{
    signature::hazmat::PrehashVerifier as P521PrehashVerifier, Signature as P521Signature,
    VerifyingKey as P521VerifyingKey,
};
use ring::{
    digest,
    signature::{self, UnparsedPublicKey},
};
use rsa::{
    pkcs1v15::Pkcs1v15Sign,
    pss::Pss,
    sha2::{Digest as RsaDigest, Sha256 as RsaSha256, Sha512 as RsaSha512},
    RsaPublicKey,
};
use sha2::{Digest, Sha256, Sha512};
use subtle::ConstantTimeEq;
use x509_parser::{
    certificate::X509Certificate,
    oid_registry::{OID_EC_P256, OID_NIST_EC_P384, OID_NIST_EC_P521},
    pem::parse_x509_pem,
    prelude::FromDer,
    public_key::PublicKey,
};

use crate::signing::{read_apk_signing_block, ApkSigningBlock, SigningBlockError};

const V2_BLOCK_ID: u32 = 0x7109_871a;
const V3_BLOCK_ID: u32 = 0xf053_68c0;
const V31_BLOCK_ID: u32 = 0x1b93_ad61;
const V32_BLOCK_ID: u32 = 0x70e1_c89f;
const PROOF_OF_ROTATION_ATTR_ID: u32 = 0x3ba0_6f8c;
const ROTATION_MIN_SDK_VERSION_ATTR_ID: u32 = 0x559f_8b02;
const ROTATION_ON_DEV_RELEASE_ATTR_ID: u32 = 0xc2a6_b3ba;
const PROOF_OF_ROTATION_FLAG_INSTALLED_DATA: u32 = 0x0000_0001;
const PROOF_OF_ROTATION_FLAG_SHARED_USER_ID: u32 = 0x0000_0002;
const PROOF_OF_ROTATION_FLAG_PERMISSION: u32 = 0x0000_0004;
const PROOF_OF_ROTATION_FLAG_ROLLBACK: u32 = 0x0000_0008;
const PROOF_OF_ROTATION_FLAG_AUTH: u32 = 0x0000_0010;
const PROOF_OF_ROTATION_KNOWN_FLAGS: u32 = PROOF_OF_ROTATION_FLAG_INSTALLED_DATA
    | PROOF_OF_ROTATION_FLAG_SHARED_USER_ID
    | PROOF_OF_ROTATION_FLAG_PERMISSION
    | PROOF_OF_ROTATION_FLAG_ROLLBACK
    | PROOF_OF_ROTATION_FLAG_AUTH;
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
    pub certificate_sha256: Vec<String>,
    pub sdk_ranges: Vec<(u32, u32)>,
    pub rotation_min_sdk: Option<u32>,
    pub rotation_targets_dev_release: bool,
    pub proof_of_rotation: Vec<ProofOfRotationInfo>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofOfRotationCapabilities {
    pub flags: u32,
    pub known_flags: u32,
    pub unknown_flags: u32,
    pub installed_data: bool,
    pub shared_user_id: bool,
    pub permission: bool,
    pub rollback: bool,
    pub auth: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofOfRotationInfo {
    pub state: CryptoVerificationState,
    pub level_count: usize,
    pub lineage_certificate_sha256: Vec<String>,
    pub capabilities: Vec<ProofOfRotationCapabilities>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ApkSignatureVerification {
    pub v2: Option<CryptoSchemeInfo>,
    pub v3: Option<CryptoSchemeInfo>,
    pub v31: Option<CryptoSchemeInfo>,
    pub v31_present: bool,
    pub v32_present: bool,
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
    let v32_present = block.pairs.iter().any(|(id, _)| *id == V32_BLOCK_ID);
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
    let v31_block = block
        .pairs
        .iter()
        .find(|(id, _)| *id == V31_BLOCK_ID)
        .map(|(_, value)| value.as_slice());

    let mut result = ApkSignatureVerification {
        v31_present,
        v32_present,
        ..Default::default()
    };

    if let Some(value) = v2_block {
        result.v2 = Some(match verify_v2_block(&mut file, &block, value) {
            Ok(info) => info,
            Err(error) => error_to_scheme_info(error),
        });
    }

    if let Some(value) = v3_block {
        result.v3 = Some(
            match verify_v3_block(&mut file, &block, value, V3_BLOCK_ID, "v3") {
                Ok(info) => info,
                Err(error) => error_to_scheme_info(error),
            },
        );
    }

    if let Some(value) = v31_block {
        result.v31 = Some(
            match verify_v3_block(&mut file, &block, value, V31_BLOCK_ID, "v3.1") {
                Ok(info) => info,
                Err(error) => error_to_scheme_info(error),
            },
        );
    }

    apply_v31_cross_block_semantics(&mut result);

    Ok(Some(result))
}

fn apply_v31_cross_block_semantics(result: &mut ApkSignatureVerification) {
    if !result.v31_present {
        return;
    }

    let Some(v31) = result.v31.as_mut() else {
        return;
    };

    let Some(v3) = result.v3.as_ref() else {
        v31.state = CryptoVerificationState::Invalid;
        v31.detail = "v3.1 signing block found without a v3.0 base block".to_string();
        return;
    };

    if v3.state != CryptoVerificationState::Verified
        || v31.state != CryptoVerificationState::Verified
    {
        return;
    }

    if v3.signer_count != v31.signer_count {
        v31.state = CryptoVerificationState::Invalid;
        v31.detail = format!(
            "v3/v3.1 signer count mismatch: v3={}, v3.1={}",
            v3.signer_count, v31.signer_count
        );
        return;
    }

    let Some(v31_min_sdk) = v31.sdk_ranges.iter().map(|(min, _)| *min).min() else {
        v31.state = CryptoVerificationState::Invalid;
        v31.detail = "v3.1 verification produced no targeted SDK range".to_string();
        return;
    };

    let Some(v3_max_sdk) = v3.sdk_ranges.iter().map(|(_, max)| *max).max() else {
        v31.state = CryptoVerificationState::Invalid;
        v31.detail = "v3 verification produced no base SDK range".to_string();
        return;
    };

    if v3_max_sdk > v31_min_sdk
        || (v3_max_sdk == v31_min_sdk && v31_min_sdk > 32 && !v31.rotation_targets_dev_release)
    {
        v31.state = CryptoVerificationState::Invalid;
        v31.detail = format!(
            "v3/v3.1 targeted SDK ranges overlap without an allowed development-release boundary: v3 maxSDK={v3_max_sdk}, v3.1 minSDK={v31_min_sdk}"
        );
        return;
    }

    if v3_max_sdk < v31_min_sdk && v3_max_sdk.saturating_add(1) != v31_min_sdk {
        v31.state = CryptoVerificationState::Invalid;
        v31.detail = format!(
            "v3/v3.1 targeted SDK ranges are not contiguous: v3 maxSDK={v3_max_sdk}, v3.1 minSDK={v31_min_sdk}"
        );
        return;
    }

    if v3.rotation_min_sdk != Some(v31_min_sdk) {
        v31.state = CryptoVerificationState::Invalid;
        v31.detail = format!(
            "v3 stripping-protection rotation-min-sdk mismatch: v3 attribute={:?}, v3.1 target={v31_min_sdk}",
            v3.rotation_min_sdk
        );
        return;
    }

    let v3_lineages = v3
        .proof_of_rotation
        .iter()
        .filter(|proof| proof.state == CryptoVerificationState::Verified)
        .collect::<Vec<_>>();
    let v31_lineages = v31
        .proof_of_rotation
        .iter()
        .filter(|proof| proof.state == CryptoVerificationState::Verified)
        .collect::<Vec<_>>();

    if v31_lineages.is_empty() {
        v31.state = CryptoVerificationState::Invalid;
        v31.detail =
            "v3.1 verification requires proof-of-rotation lineage evidence in the v3.1 signer"
                .to_string();
        return;
    }

    let lineage_consistent = if v3_lineages.is_empty() {
        v3.certificate_sha256.iter().all(|certificate| {
            v31_lineages.iter().any(|proof| {
                proof
                    .lineage_certificate_sha256
                    .iter()
                    .any(|lineage_certificate| lineage_certificate == certificate)
            })
        })
    } else {
        v3_lineages.iter().all(|v3_proof| {
            v31_lineages.iter().any(|v31_proof| {
                !v3_proof.lineage_certificate_sha256.is_empty()
                    && v3_proof.lineage_certificate_sha256.len()
                        <= v31_proof.lineage_certificate_sha256.len()
                    && v3_proof
                        .lineage_certificate_sha256
                        .iter()
                        .zip(v31_proof.lineage_certificate_sha256.iter())
                        .all(|(left, right)| left == right)
            })
        })
    };

    if !lineage_consistent {
        v31.state = CryptoVerificationState::Invalid;
        v31.detail = "v3/v3.1 proof-of-rotation lineages are inconsistent".to_string();
    }
}

fn error_to_scheme_info(error: SignatureVerificationError) -> CryptoSchemeInfo {
    error_to_scheme_info_with_evidence(error, 0, Vec::new(), Vec::new(), Vec::new())
}

fn error_to_scheme_info_with_evidence(
    error: SignatureVerificationError,
    signer_count: usize,
    algorithms: Vec<u32>,
    certificate_sha256: Vec<String>,
    sdk_ranges: Vec<(u32, u32)>,
) -> CryptoSchemeInfo {
    let detail = error.to_string();
    let unsupported = detail.strip_prefix("UNSUPPORTED: ").is_some();
    CryptoSchemeInfo {
        state: if unsupported {
            CryptoVerificationState::Unsupported
        } else {
            CryptoVerificationState::Invalid
        },
        signer_count,
        algorithms,
        certificate_sha256,
        sdk_ranges,
        rotation_min_sdk: None,
        rotation_targets_dev_release: false,
        proof_of_rotation: Vec::new(),
        detail: detail
            .strip_prefix("UNSUPPORTED: ")
            .unwrap_or(&detail)
            .to_string(),
    }
}

fn error_to_scheme_info_with_rotation(
    error: SignatureVerificationError,
    signer_count: usize,
    algorithms: Vec<u32>,
    certificate_sha256: Vec<String>,
    sdk_ranges: Vec<(u32, u32)>,
    proof_of_rotation: Vec<ProofOfRotationInfo>,
) -> CryptoSchemeInfo {
    let detail = error.to_string();
    let unsupported = detail.strip_prefix("UNSUPPORTED: ").is_some();
    CryptoSchemeInfo {
        state: if unsupported {
            CryptoVerificationState::Unsupported
        } else {
            CryptoVerificationState::Invalid
        },
        signer_count,
        algorithms,
        certificate_sha256,
        sdk_ranges,
        rotation_min_sdk: None,
        rotation_targets_dev_release: false,
        proof_of_rotation,
        detail: detail
            .strip_prefix("UNSUPPORTED: ")
            .unwrap_or(&detail)
            .to_string(),
    }
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
        match verify_v2_signer(file, block, signer) {
            Ok(info) => results.push(info),
            Err(error) => results.push(error_to_scheme_info_with_evidence(
                error,
                1,
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )),
        }
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
        certificate_sha256: vec![certificate_sha256(parsed.certificate)?],
        sdk_ranges: Vec::new(),
        rotation_min_sdk: None,
        rotation_targets_dev_release: false,
        proof_of_rotation: Vec::new(),
        detail:
            "v2 signer signature, certificate/public-key binding, and APK content digest verified"
                .to_string(),
    })
}

fn verify_v3_block(
    file: &mut File,
    block: &ApkSigningBlock,
    value: &[u8],
    scheme_block_id: u32,
    scheme_name: &str,
) -> Result<CryptoSchemeInfo, SignatureVerificationError> {
    let mut reader = LengthReader::new(value);
    let signers = reader.read_sequence(if scheme_block_id == V31_BLOCK_ID {
        "v3.1 signers"
    } else {
        "v3 signers"
    })?;
    reader.finish("v3 signer sequence")?;

    let mut signers_reader = LengthReader::new(signers);
    let mut signer_values = Vec::new();
    while !signers_reader.is_empty() {
        signer_values.push(signers_reader.read_sequence("v3 signer")?);
    }
    signers_reader.finish("v3 signer sequence")?;

    let mut results = Vec::with_capacity(signer_values.len());

    for signer in signer_values {
        let mut signer_reader = LengthReader::new(signer);
        let signed_data = match signer_reader.read_sequence("v3 signed data") {
            Ok(value) => value,
            Err(error) => {
                results.push(error_to_scheme_info_with_evidence(
                    error,
                    1,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ));
                continue;
            }
        };
        let min_sdk = match signer_reader.read_u32("v3 outer minSDK") {
            Ok(value) => value,
            Err(error) => {
                results.push(error_to_scheme_info_with_evidence(
                    error,
                    1,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ));
                continue;
            }
        };
        let max_sdk = match signer_reader.read_u32("v3 outer maxSDK") {
            Ok(value) => value,
            Err(error) => {
                results.push(error_to_scheme_info_with_evidence(
                    error,
                    1,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ));
                continue;
            }
        };
        let sdk_ranges = vec![(min_sdk, max_sdk)];
        let signatures = match signer_reader.read_sequence("v3 signatures") {
            Ok(value) => value,
            Err(error) => {
                results.push(error_to_scheme_info_with_evidence(
                    error,
                    1,
                    Vec::new(),
                    Vec::new(),
                    sdk_ranges.clone(),
                ));
                continue;
            }
        };
        let public_key = match signer_reader.read_length_prefixed("v3 public key") {
            Ok(value) => value,
            Err(error) => {
                results.push(error_to_scheme_info_with_evidence(
                    error,
                    1,
                    Vec::new(),
                    Vec::new(),
                    sdk_ranges.clone(),
                ));
                continue;
            }
        };
        if let Err(error) = signer_reader.finish("v3 signer") {
            results.push(error_to_scheme_info_with_evidence(
                error,
                1,
                Vec::new(),
                Vec::new(),
                sdk_ranges.clone(),
            ));
            continue;
        }

        let parsed = match parse_signed_data_v3(signed_data, scheme_block_id) {
            Ok(parsed) => parsed,
            Err(error) => {
                results.push(error_to_scheme_info_with_evidence(
                    error,
                    1,
                    Vec::new(),
                    Vec::new(),
                    sdk_ranges.clone(),
                ));
                continue;
            }
        };

        let algorithms = parsed.digest_algorithms.clone();
        let certificate_sha256 = match certificate_sha256(parsed.certificate) {
            Ok(fingerprint) => vec![fingerprint],
            Err(error) => {
                results.push(error_to_scheme_info_with_rotation(
                    error,
                    1,
                    algorithms,
                    Vec::new(),
                    sdk_ranges.clone(),
                    parsed.proof_of_rotation.clone().into_iter().collect(),
                ));
                continue;
            }
        };

        if parsed.min_sdk != min_sdk || parsed.max_sdk != max_sdk {
            results.push(CryptoSchemeInfo {
                state: CryptoVerificationState::Invalid,
                signer_count: 1,
                algorithms,
                certificate_sha256,
                sdk_ranges,
                rotation_min_sdk: None,
                rotation_targets_dev_release: false,
                proof_of_rotation: parsed.proof_of_rotation.clone().into_iter().collect(),
                detail: format!(
                    "v3 signer minSDK/maxSDK ({min_sdk}, {max_sdk}) do not match signed-data values ({}, {})",
                    parsed.min_sdk, parsed.max_sdk
                ),
            });
            continue;
        }

        let selected = match parse_and_select_signature(signatures) {
            Ok(selected) => selected,
            Err(error) => {
                results.push(error_to_scheme_info_with_rotation(
                    error,
                    1,
                    algorithms.clone(),
                    certificate_sha256.clone(),
                    sdk_ranges.clone(),
                    parsed.proof_of_rotation.clone().into_iter().collect(),
                ));
                continue;
            }
        };

        if let Err(error) = verify_certificate_and_public_key(parsed.certificate, public_key) {
            results.push(error_to_scheme_info_with_rotation(
                error,
                1,
                algorithms.clone(),
                certificate_sha256.clone(),
                sdk_ranges.clone(),
                parsed.proof_of_rotation.clone().into_iter().collect(),
            ));
            continue;
        }

        if let Err(error) = verify_signature_bytes(
            selected.algorithm_id,
            parsed.certificate,
            signed_data,
            selected.signature,
        ) {
            results.push(error_to_scheme_info_with_rotation(
                error,
                1,
                algorithms.clone(),
                certificate_sha256.clone(),
                sdk_ranges.clone(),
                parsed.proof_of_rotation.clone().into_iter().collect(),
            ));
            continue;
        }

        let expected_digest = match parsed
            .digests
            .iter()
            .find(|(id, _)| *id == selected.algorithm_id)
            .map(|(_, digest)| *digest)
        {
            Some(digest) => digest,
            None => {
                results.push(error_to_scheme_info_with_rotation(
                    SignatureVerificationError(format!(
                        "v3 digest list does not contain signature algorithm 0x{:08x}",
                        selected.algorithm_id
                    )),
                    1,
                    algorithms.clone(),
                    certificate_sha256.clone(),
                    sdk_ranges.clone(),
                    parsed.proof_of_rotation.clone().into_iter().collect(),
                ));
                continue;
            }
        };

        if let Err(error) =
            verify_content_digest(file, block, expected_digest, selected.digest_algorithm)
        {
            results.push(error_to_scheme_info_with_rotation(
                error,
                1,
                algorithms.clone(),
                certificate_sha256.clone(),
                sdk_ranges.clone(),
                parsed.proof_of_rotation.clone().into_iter().collect(),
            ));
            continue;
        }

        if parsed.digest_algorithms != selected.all_signature_algorithms {
            results.push(CryptoSchemeInfo {
                state: CryptoVerificationState::Invalid,
                signer_count: 1,
                algorithms,
                certificate_sha256,
                sdk_ranges,
                rotation_min_sdk: None,
                rotation_targets_dev_release: false,
                proof_of_rotation: parsed.proof_of_rotation.clone().into_iter().collect(),
                detail:
                    "v3 digest and signature algorithm ID lists are not identical and ordered equally"
                        .to_string(),
            });
            continue;
        }

        let mut state = parsed
            .proof_of_rotation
            .as_ref()
            .map(|proof| proof.state)
            .unwrap_or(CryptoVerificationState::Verified);

        if scheme_block_id == V31_BLOCK_ID && parsed.proof_of_rotation.is_none() {
            state = CryptoVerificationState::Invalid;
        }

        let detail = if scheme_block_id == V31_BLOCK_ID {
            if let Some(proof) = &parsed.proof_of_rotation {
                format!(
                    "v3.1 signer signature, certificate/public-key binding, SDK range, and APK content digest verified for rotation target SDK range {}..={}; {}",
                    parsed.min_sdk, parsed.max_sdk, proof.detail
                )
            } else {
                format!(
                    "v3.1 signer signature, certificate/public-key binding, SDK range, and APK content digest verified for rotation target SDK range {}..={}",
                    parsed.min_sdk, parsed.max_sdk
                )
            }
        } else if let Some(proof) = &parsed.proof_of_rotation {
            format!(
                "v3 signer signature, certificate/public-key binding, SDK range, and APK content digest verified for SDK range {}..={}; {}",
                parsed.min_sdk, parsed.max_sdk, proof.detail
            )
        } else {
            format!(
                "v3 signer signature, certificate/public-key binding, SDK range, and APK content digest verified for SDK range {}..={}",
                parsed.min_sdk, parsed.max_sdk
            )
        };

        results.push(CryptoSchemeInfo {
            state,
            signer_count: 1,
            algorithms,
            certificate_sha256,
            sdk_ranges,
            rotation_min_sdk: parsed.rotation_min_sdk,
            rotation_targets_dev_release: parsed.rotation_targets_dev_release,
            proof_of_rotation: parsed.proof_of_rotation.clone().into_iter().collect(),
            detail,
        });
    }

    merge_scheme_results(scheme_name, results)
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
            certificate_sha256: Vec::new(),
            sdk_ranges: Vec::new(),
            rotation_min_sdk: None,
            rotation_targets_dev_release: false,
            proof_of_rotation: Vec::new(),
            detail: format!("{scheme} signing block contains no signers"),
        });
    }

    let mut algorithms = Vec::new();
    let mut certificate_sha256 = Vec::new();
    let mut unsupported = false;
    let mut invalid = false;
    let mut details = Vec::new();
    let mut proof_of_rotation = Vec::new();
    let mut rotation_min_sdk = None;
    let mut rotation_targets_dev_release = false;
    let mut first_rotation_value_seen = false;
    let mut rotation_metadata_consistent = true;

    for result in &results {
        algorithms.extend_from_slice(&result.algorithms);
        certificate_sha256.extend(result.certificate_sha256.iter().cloned());
        match result.state {
            CryptoVerificationState::Verified => {}
            CryptoVerificationState::Unsupported => unsupported = true,
            CryptoVerificationState::Invalid => invalid = true,
        }
        proof_of_rotation.extend(result.proof_of_rotation.iter().cloned());
        details.push(result.detail.clone());

        if !first_rotation_value_seen {
            rotation_min_sdk = result.rotation_min_sdk;
            first_rotation_value_seen = true;
        } else if rotation_min_sdk != result.rotation_min_sdk {
            rotation_metadata_consistent = false;
        }
        rotation_targets_dev_release |= result.rotation_targets_dev_release;
    }

    if !rotation_metadata_consistent {
        invalid = true;
        details.push(format!(
            "{scheme} signers disagree on rotation-min-sdk metadata"
        ));
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
        certificate_sha256,
        sdk_ranges: results
            .iter()
            .flat_map(|result| result.sdk_ranges.iter().copied())
            .collect(),
        rotation_min_sdk,
        rotation_targets_dev_release,
        proof_of_rotation,
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
        .filter_map(|(id, sig)| {
            supported_signature_algorithm(*id).map(|digest| (*id, *sig, digest))
        })
        .collect::<Vec<_>>();

    supported.sort_by_key(|(id, _, _)| std::cmp::Reverse(signature_strength(*id)));
    let Some((algorithm_id, signature, digest_algorithm)) = supported.into_iter().next() else {
        return Err(SignatureVerificationError(
            "UNSUPPORTED: APK signer contains no cryptographically supported v2/v3 signature algorithm"
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
    match signature_algorithm_digest(id) {
        Some(DigestAlgorithm::Sha512) => 2,
        Some(DigestAlgorithm::Sha256) => 1,
        None => 0,
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
        0x0101 | 0x0102 | 0x0103 | 0x0104 | 0x0201 | 0x0202 => signature_algorithm_digest(id),
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

    let parsed_public_key = cert.public_key().parsed().map_err(|error| {
        SignatureVerificationError(format!("failed to parse signer public key: {error}"))
    })?;

    if algorithm_id == 0x0202 {
        let PublicKey::EC(_) = &parsed_public_key else {
            return Err(SignatureVerificationError(
                "ECDSA signature algorithm is paired with a non-EC signer public key".to_string(),
            ));
        };

        let curve_oid = cert
            .public_key()
            .algorithm
            .parameters
            .as_ref()
            .and_then(|value| value.as_oid().ok());

        if matches!(curve_oid.as_ref(), Some(oid) if oid == &OID_NIST_EC_P521) {
            return verify_p521_ecdsa_signature(
                0x0202,
                &cert,
                signed_data,
                signature_bytes,
                DigestAlgorithm::Sha512,
            );
        }

        if !matches!(curve_oid.as_ref(), Some(oid) if oid == &OID_NIST_EC_P384) {
            return Err(SignatureVerificationError(
                "UNSUPPORTED: ECDSA SHA-512 signer curve is neither P-384 nor P-521".to_string(),
            ));
        }

        let verifying_key = P384VerifyingKey::from_sec1_bytes(
            &cert.public_key().subject_public_key.data,
        )
        .map_err(|error| {
            SignatureVerificationError(format!(
                "signer P-384 public key is not a valid SEC1 point: {error}"
            ))
        })?;
        let signature = P384Signature::from_der(signature_bytes).map_err(|error| {
            SignatureVerificationError(format!("ECDSA SHA-512 signature is not valid DER: {error}"))
        })?;
        let prehash = Sha512::digest(signed_data);
        PrehashVerifier::<P384Signature>::verify_prehash(
            &verifying_key,
            prehash.as_ref(),
            &signature,
        )
        .map_err(|_| {
            SignatureVerificationError(
                "cryptographic signature verification failed for algorithm 0x00000202".to_string(),
            )
        })?;
        return Ok(());
    }

    if let PublicKey::RSA(rsa) = &parsed_public_key {
        let key_size = rsa.key_size();
        if key_size == 1024 || key_size == 16_384 {
            return verify_extended_rsa_signature(
                algorithm_id,
                key_size,
                rsa.modulus,
                rsa.exponent,
                signed_data,
                signature_bytes,
            );
        }
    }

    let algorithm: &dyn signature::VerificationAlgorithm = match algorithm_id {
        0x0101 => {
            let PublicKey::RSA(rsa) = &parsed_public_key else {
                return Err(SignatureVerificationError(
                    "RSA signature algorithm is paired with a non-RSA signer public key".to_string(),
                ));
            };
            if !ring_rsa_key_size_supported(rsa.key_size()) {
                return Err(SignatureVerificationError(format!(
                    "UNSUPPORTED: RSA signer key size {} bits is outside the current ring verifier range of 2048-8192 bits",
                    rsa.key_size()
                )));
            }
            &signature::RSA_PSS_2048_8192_SHA256
        }
        0x0102 => {
            let PublicKey::RSA(rsa) = &parsed_public_key else {
                return Err(SignatureVerificationError(
                    "RSA signature algorithm is paired with a non-RSA signer public key".to_string(),
                ));
            };
            if !ring_rsa_key_size_supported(rsa.key_size()) {
                return Err(SignatureVerificationError(format!(
                    "UNSUPPORTED: RSA signer key size {} bits is outside the current ring verifier range of 2048-8192 bits",
                    rsa.key_size()
                )));
            }
            &signature::RSA_PSS_2048_8192_SHA512
        }
        0x0103 => {
            let PublicKey::RSA(rsa) = &parsed_public_key else {
                return Err(SignatureVerificationError(
                    "RSA signature algorithm is paired with a non-RSA signer public key".to_string(),
                ));
            };
            if !ring_rsa_key_size_supported(rsa.key_size()) {
                return Err(SignatureVerificationError(format!(
                    "UNSUPPORTED: RSA signer key size {} bits is outside the current ring verifier range of 2048-8192 bits",
                    rsa.key_size()
                )));
            }
            &signature::RSA_PKCS1_2048_8192_SHA256
        }
        0x0104 => {
            let PublicKey::RSA(rsa) = &parsed_public_key else {
                return Err(SignatureVerificationError(
                    "RSA signature algorithm is paired with a non-RSA signer public key".to_string(),
                ));
            };
            if !ring_rsa_key_size_supported(rsa.key_size()) {
                return Err(SignatureVerificationError(format!(
                    "UNSUPPORTED: RSA signer key size {} bits is outside the current ring verifier range of 2048-8192 bits",
                    rsa.key_size()
                )));
            }
            &signature::RSA_PKCS1_2048_8192_SHA512
        }
        0x0201 => {
            if !matches!(parsed_public_key, PublicKey::EC(_)) {
                return Err(SignatureVerificationError(
                    "ECDSA signature algorithm is paired with a non-EC signer public key"
                        .to_string(),
                ));
            }
            let curve_oid = cert
                .public_key()
                .algorithm
                .parameters
                .as_ref()
                .and_then(|value| value.as_oid().ok());

            match curve_oid {
                Some(oid) if oid == OID_EC_P256 => &signature::ECDSA_P256_SHA256_ASN1,
                Some(oid) if oid == OID_NIST_EC_P384 => &signature::ECDSA_P384_SHA256_ASN1,
                Some(oid) if oid == OID_NIST_EC_P521 => {
                    return verify_p521_ecdsa_signature(
                        0x0201,
                        &cert,
                        signed_data,
                        signature_bytes,
                        DigestAlgorithm::Sha256,
                    );
                }
                Some(_) => {
                    return Err(SignatureVerificationError(
                        "UNSUPPORTED: ECDSA SHA-256 signer curve is not supported by the current verifier"
                            .to_string(),
                    ))
                }
                None => {
                    return Err(SignatureVerificationError(
                        "signer EC public key does not identify a supported named curve".to_string(),
                    ))
                }
            }
        }
        _ => {
            return Err(SignatureVerificationError(format!(
                "UNSUPPORTED: signature algorithm 0x{algorithm_id:08x} is not supported by the current verifier"
            )))
        }
    };

    let key_bytes = match &parsed_public_key {
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

fn verify_extended_rsa_signature(
    algorithm_id: u32,
    key_size: usize,
    modulus: &[u8],
    exponent: &[u8],
    signed_data: &[u8],
    signature_bytes: &[u8],
) -> Result<(), SignatureVerificationError> {
    if key_size == 1024 && algorithm_id == 0x0102 {
        return Err(SignatureVerificationError(
            "UNSUPPORTED: RSA 1024-bit key is too short for RSASSA-PSS SHA-512 with the required 64-byte salt"
                .to_string(),
        ));
    }

    let public_key = RsaPublicKey::new(
        rsa::BigUint::from_bytes_be(modulus),
        rsa::BigUint::from_bytes_be(exponent),
    )
    .map_err(|error| {
        SignatureVerificationError(format!(
            "RSA public key construction failed for {}-bit key: {error}",
            key_size
        ))
    })?;

    let result = match algorithm_id {
        0x0101 => {
            let digest = RsaSha256::digest(signed_data);
            public_key.verify(
                Pss::new_with_salt::<RsaSha256>(32),
                digest.as_ref(),
                signature_bytes,
            )
        }
        0x0102 => {
            let digest = RsaSha512::digest(signed_data);
            public_key.verify(
                Pss::new_with_salt::<RsaSha512>(64),
                digest.as_ref(),
                signature_bytes,
            )
        }
        0x0103 => {
            let digest = RsaSha256::digest(signed_data);
            public_key.verify(
                Pkcs1v15Sign::new::<RsaSha256>(),
                digest.as_ref(),
                signature_bytes,
            )
        }
        0x0104 => {
            let digest = RsaSha512::digest(signed_data);
            public_key.verify(
                Pkcs1v15Sign::new::<RsaSha512>(),
                digest.as_ref(),
                signature_bytes,
            )
        }
        _ => {
            return Err(SignatureVerificationError(format!(
                "UNSUPPORTED: signature algorithm 0x{algorithm_id:08x} is not supported by the extended RSA verifier"
            )))
        }
    };

    result.map_err(|_| {
        SignatureVerificationError(format!(
            "cryptographic signature verification failed for algorithm 0x{algorithm_id:08x} on RSA {key_size}-bit key"
        ))
    })
}

fn ring_rsa_key_size_supported(bits: usize) -> bool {
    (2048..=8192).contains(&bits)
}

fn verify_p521_ecdsa_signature(
    algorithm_id: u32,
    cert: &X509Certificate<'_>,
    signed_data: &[u8],
    signature_bytes: &[u8],
    digest_algorithm: DigestAlgorithm,
) -> Result<(), SignatureVerificationError> {
    let verifying_key = P521VerifyingKey::from_sec1_bytes(
        &cert.public_key().subject_public_key.data,
    )
    .map_err(|error| {
        SignatureVerificationError(format!(
            "signer P-521 public key is not a valid SEC1 point: {error}"
        ))
    })?;
    let signature = P521Signature::from_der(signature_bytes).map_err(|error| {
        SignatureVerificationError(format!("ECDSA P-521 signature is not valid DER: {error}"))
    })?;

    let prehash: Vec<u8> = match digest_algorithm {
        DigestAlgorithm::Sha256 => Sha256::digest(signed_data).to_vec(),
        DigestAlgorithm::Sha512 => Sha512::digest(signed_data).to_vec(),
    };

    P521PrehashVerifier::<P521Signature>::verify_prehash(
        &verifying_key,
        prehash.as_ref(),
        &signature,
    )
    .map_err(|_| {
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

type ParsedDigestSequence<'a> = (Vec<u32>, Vec<(u32, &'a [u8])>);

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
    rotation_min_sdk: Option<u32>,
    rotation_targets_dev_release: bool,
    proof_of_rotation: Option<ProofOfRotationInfo>,
}

fn parse_signed_data_v2(bytes: &[u8]) -> Result<ParsedSignedData<'_>, SignatureVerificationError> {
    let mut reader = LengthReader::new(bytes);
    let digests = reader.read_sequence("v2 digests")?;
    let certificates = reader.read_sequence("v2 certificates")?;
    let _attributes = reader.read_sequence("v2 additional attributes")?;

    if !reader.is_empty() {
        let fourth = reader.read_sequence("v2 optional fourth signed-data element")?;
        if !fourth.is_empty() {
            return Err(SignatureVerificationError(
                "v2 optional fourth signed-data element must be empty".to_string(),
            ));
        }
    }

    reader.finish("v2 signed data")?;

    let (digest_algorithms, selected_digest) = parse_digest_sequence(digests)?;
    let mut certificates_reader = LengthReader::new(certificates);
    let certificate = certificates_reader.read_length_prefixed("v2 certificate")?;
    while !certificates_reader.is_empty() {
        certificates_reader.read_length_prefixed("v2 certificate chain entry")?;
    }
    certificates_reader.finish("v2 certificate sequence")?;

    Ok(ParsedSignedData {
        digests: selected_digest,
        digest_algorithms,
        certificate,
    })
}

fn parse_signed_data_v3(
    bytes: &[u8],
    scheme_block_id: u32,
) -> Result<ParsedV3SignedData<'_>, SignatureVerificationError> {
    let mut reader = LengthReader::new(bytes);
    let digests = reader.read_sequence("v3 digests")?;
    let certificates = reader.read_sequence("v3 certificates")?;
    let min_sdk = reader.read_u32("v3 signed-data minSDK")?;
    let max_sdk = reader.read_u32("v3 signed-data maxSDK")?;
    if min_sdk > max_sdk {
        return Err(SignatureVerificationError(format!(
            "v3 signed-data minSDK {min_sdk} is greater than maxSDK {max_sdk}"
        )));
    }
    let attributes = reader.read_sequence("v3 additional attributes")?;
    reader.finish("v3 signed data")?;

    let (digest_algorithms, selected_digest) = parse_digest_sequence(digests)?;
    let mut certificates_reader = LengthReader::new(certificates);
    let certificate = certificates_reader.read_length_prefixed("v3 certificate")?;
    while !certificates_reader.is_empty() {
        certificates_reader.read_length_prefixed("v3 certificate chain entry")?;
    }
    certificates_reader.finish("v3 certificate sequence")?;

    let mut attributes_reader = LengthReader::new(attributes);
    let mut proof_of_rotation = None;
    let mut rotation_min_sdk = None;
    let mut rotation_targets_dev_release = false;
    let scheme_label = if scheme_block_id == V31_BLOCK_ID {
        "v3.1"
    } else {
        "v3"
    };

    while !attributes_reader.is_empty() {
        let attribute = attributes_reader.read_sequence("v3 attribute")?;
        let mut attribute_reader = LengthReader::new(attribute);
        let id = attribute_reader.read_u32("v3 attribute ID")?;
        let value = &attribute_reader.bytes[attribute_reader.cursor..];
        attribute_reader.cursor = attribute_reader.bytes.len();

        if id == PROOF_OF_ROTATION_ATTR_ID {
            if proof_of_rotation.is_some() {
                proof_of_rotation = Some(ProofOfRotationInfo {
                    state: CryptoVerificationState::Invalid,
                    level_count: 0,
                    lineage_certificate_sha256: Vec::new(),
                    capabilities: Vec::new(),
                    detail: "v3 signed data contains multiple proof-of-rotation attributes"
                        .to_string(),
                });
            } else {
                proof_of_rotation = Some(validate_proof_of_rotation(value, certificate));
            }
        } else if id == ROTATION_MIN_SDK_VERSION_ATTR_ID {
            if rotation_min_sdk.is_some() {
                return Err(SignatureVerificationError(format!(
                    "{scheme_label} signed data contains multiple rotation-min-sdk attributes"
                )));
            }
            if value.len() != 4 {
                return Err(SignatureVerificationError(format!(
                    "{scheme_label} rotation-min-sdk attribute must contain exactly 4 bytes"
                )));
            }
            rotation_min_sdk = Some(u32::from_le_bytes(
                value.try_into().expect("checked four-byte attribute"),
            ));
        } else if id == ROTATION_ON_DEV_RELEASE_ATTR_ID {
            if rotation_targets_dev_release {
                return Err(SignatureVerificationError(format!(
                    "{scheme_label} signed data contains multiple development-release rotation attributes"
                )));
            }
            if !value.is_empty() {
                return Err(SignatureVerificationError(format!(
                    "{scheme_label} development-release rotation attribute must be empty"
                )));
            }
            rotation_targets_dev_release = true;
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
        rotation_min_sdk,
        rotation_targets_dev_release,
        proof_of_rotation,
    })
}

fn decode_proof_of_rotation_capabilities(flags: u32) -> ProofOfRotationCapabilities {
    let known_flags = flags & PROOF_OF_ROTATION_KNOWN_FLAGS;
    ProofOfRotationCapabilities {
        flags,
        known_flags,
        unknown_flags: flags & !PROOF_OF_ROTATION_KNOWN_FLAGS,
        installed_data: known_flags & PROOF_OF_ROTATION_FLAG_INSTALLED_DATA != 0,
        shared_user_id: known_flags & PROOF_OF_ROTATION_FLAG_SHARED_USER_ID != 0,
        permission: known_flags & PROOF_OF_ROTATION_FLAG_PERMISSION != 0,
        rollback: known_flags & PROOF_OF_ROTATION_FLAG_ROLLBACK != 0,
        auth: known_flags & PROOF_OF_ROTATION_FLAG_AUTH != 0,
    }
}

fn validate_proof_of_rotation(bytes: &[u8], current_certificate: &[u8]) -> ProofOfRotationInfo {
    let mut reader = LengthReader::new(bytes);
    let version = match reader.read_u32("v3 proof-of-rotation version") {
        Ok(value) => value,
        Err(error) => {
            return ProofOfRotationInfo {
                state: CryptoVerificationState::Invalid,
                level_count: 0,
                lineage_certificate_sha256: Vec::new(),
                capabilities: Vec::new(),
                detail: format!("proof-of-rotation is malformed: {error}"),
            };
        }
    };
    if version != 1 {
        return ProofOfRotationInfo {
            state: CryptoVerificationState::Invalid,
            level_count: 0,
            lineage_certificate_sha256: Vec::new(),
            capabilities: Vec::new(),
            detail: format!("unsupported proof-of-rotation version {version}"),
        };
    }

    let mut certificates = Vec::<Vec<u8>>::new();
    let mut last_certificate: Option<Vec<u8>> = None;
    let mut last_signature_algorithm = 0_u32;
    let mut level_count = 0_usize;
    let mut lineage_certificate_sha256 = Vec::<String>::new();
    let mut capabilities = Vec::<ProofOfRotationCapabilities>::new();

    macro_rules! return_info {
        ($state:expr, $detail:expr) => {
            return ProofOfRotationInfo {
                state: $state,
                level_count,
                lineage_certificate_sha256: lineage_certificate_sha256.clone(),
                capabilities: capabilities.clone(),
                detail: $detail,
            };
        };
    }

    while !reader.is_empty() {
        level_count += 1;
        let node = match reader.read_sequence("v3 proof-of-rotation node") {
            Ok(value) => value,
            Err(error) => {
                return_info!(
                    CryptoVerificationState::Invalid,
                    format!("proof-of-rotation is malformed: {error}")
                );
            }
        };
        let mut node_reader = LengthReader::new(node);
        let signed_data = match node_reader.read_sequence("v3 proof-of-rotation signed data") {
            Ok(value) => value,
            Err(error) => {
                return_info!(
                    CryptoVerificationState::Invalid,
                    format!("proof-of-rotation is malformed: {error}")
                );
            }
        };
        let flags = match node_reader.read_u32("v3 proof-of-rotation flags") {
            Ok(value) => value,
            Err(error) => {
                return_info!(
                    CryptoVerificationState::Invalid,
                    format!("proof-of-rotation is malformed: {error}")
                );
            }
        };
        capabilities.push(decode_proof_of_rotation_capabilities(flags));

        let signature_algorithm =
            match node_reader.read_u32("v3 proof-of-rotation signature algorithm") {
                Ok(value) => value,
                Err(error) => {
                    return_info!(
                        CryptoVerificationState::Invalid,
                        format!("proof-of-rotation is malformed: {error}")
                    );
                }
            };
        let signature = match node_reader.read_length_prefixed("v3 proof-of-rotation signature") {
            Ok(value) => value,
            Err(error) => {
                return_info!(
                    CryptoVerificationState::Invalid,
                    format!("proof-of-rotation is malformed: {error}")
                );
            }
        };
        if let Err(error) = node_reader.finish("v3 proof-of-rotation node") {
            return_info!(
                CryptoVerificationState::Invalid,
                format!("proof-of-rotation is malformed: {error}")
            );
        }

        let mut signed_reader = LengthReader::new(signed_data);
        let certificate =
            match signed_reader.read_length_prefixed("v3 proof-of-rotation certificate") {
                Ok(value) => value,
                Err(error) => {
                    return_info!(
                        CryptoVerificationState::Invalid,
                        format!("proof-of-rotation is malformed: {error}")
                    );
                }
            };
        let parent_signature_algorithm =
            match signed_reader.read_u32("v3 proof-of-rotation parent signature algorithm") {
                Ok(value) => value,
                Err(error) => {
                    return_info!(
                        CryptoVerificationState::Invalid,
                        format!("proof-of-rotation is malformed: {error}")
                    );
                }
            };
        if let Err(error) = signed_reader.finish("v3 proof-of-rotation signed data") {
            return_info!(
                CryptoVerificationState::Invalid,
                format!("proof-of-rotation is malformed: {error}")
            );
        }

        match X509Certificate::from_der(certificate) {
            Ok(([], _)) => {}
            Ok(_) => {
                return_info!(
                    CryptoVerificationState::Invalid,
                    "proof-of-rotation certificate contains trailing DER data".to_string()
                );
            }
            Err(error) => {
                return_info!(
                    CryptoVerificationState::Invalid,
                    format!("proof-of-rotation certificate is not valid DER: {error}")
                );
            }
        }

        if certificates
            .iter()
            .any(|item| item.as_slice() == certificate)
        {
            return_info!(
                CryptoVerificationState::Invalid,
                format!("duplicate proof-of-rotation certificate at level {level_count}")
            );
        }
        lineage_certificate_sha256.push(certificate_sha256(certificate).unwrap_or_default());
        certificates.push(certificate.to_vec());

        if level_count == 1 {
            if parent_signature_algorithm != 0 || !signature.is_empty() {
                return_info!(
                    CryptoVerificationState::Invalid,
                    "first proof-of-rotation level must not contain a parent signature".to_string()
                );
            }
        } else {
            if parent_signature_algorithm != last_signature_algorithm {
                return_info!(
                    CryptoVerificationState::Invalid,
                    format!(
                        "proof-of-rotation parent signature algorithm 0x{parent_signature_algorithm:08x} does not match previous level algorithm 0x{last_signature_algorithm:08x}"
                    )
                );
            }

            if let Err(error) = verify_signature_bytes(
                last_signature_algorithm,
                last_certificate
                    .as_deref()
                    .expect("previous proof-of-rotation certificate exists"),
                signed_data,
                signature,
            ) {
                let state = if error.0.starts_with("UNSUPPORTED: ") {
                    CryptoVerificationState::Unsupported
                } else {
                    CryptoVerificationState::Invalid
                };
                return_info!(
                    state,
                    format!("proof-of-rotation validation failed: {}", error.0)
                );
            }
        }

        if !reader.is_empty() && supported_signature_algorithm(signature_algorithm).is_none() {
            return_info!(
                CryptoVerificationState::Unsupported,
                format!(
                    "proof-of-rotation uses unsupported lineage signing algorithm 0x{signature_algorithm:08x}"
                )
            );
        }

        last_signature_algorithm = signature_algorithm;
        last_certificate = Some(certificate.to_vec());
    }

    if level_count == 0 {
        return ProofOfRotationInfo {
            state: CryptoVerificationState::Invalid,
            level_count: 0,
            lineage_certificate_sha256,
            capabilities,
            detail: "proof-of-rotation contains no lineage levels".to_string(),
        };
    }
    if last_signature_algorithm != 0 {
        return ProofOfRotationInfo {
            state: CryptoVerificationState::Invalid,
            level_count,
            lineage_certificate_sha256: lineage_certificate_sha256.clone(),
            capabilities,
            detail: "final proof-of-rotation level must not specify a next-level signing algorithm"
                .to_string(),
        };
    }
    if last_certificate.as_deref() != Some(current_certificate) {
        return ProofOfRotationInfo {
            state: CryptoVerificationState::Invalid,
            level_count,
            lineage_certificate_sha256: lineage_certificate_sha256.clone(),
            capabilities,
            detail:
                "final proof-of-rotation certificate does not match the current v3 signer certificate"
                    .to_string(),
        };
    }

    ProofOfRotationInfo {
        state: CryptoVerificationState::Verified,
        level_count,
        lineage_certificate_sha256,
        capabilities,
        detail: format!(
            "proof-of-rotation lineage verified across {level_count} certificate level(s)"
        ),
    }
}

fn parse_digest_sequence(
    bytes: &[u8],
) -> Result<ParsedDigestSequence<'_>, SignatureVerificationError> {
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
    let (remaining, parsed) = X509Certificate::from_der(certificate).map_err(|error| {
        SignatureVerificationError(format!("signer certificate is not valid DER: {error}"))
    })?;

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

fn certificate_sha256(certificate: &[u8]) -> Result<String, SignatureVerificationError> {
    let digest = digest::digest(&digest::SHA256, certificate);
    Ok(digest
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>())
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
    eocd[eocd_offset_field..eocd_offset_field + 4].copy_from_slice(&signed_cd_offset.to_le_bytes());

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
            return Err(SignatureVerificationError(format!("{label} is truncated")));
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
    fn verifies_real_v2_signed_apk_fixture() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/crypto-v2-release.apk");

        let result = verify_apk_signatures(&path)
            .expect("v2 fixture should be structurally readable")
            .expect("v2 fixture should contain an APK signing block");

        let v2 = result.v2.expect("v2 scheme should be detected");
        assert_eq!(
            v2.state,
            CryptoVerificationState::Verified,
            "verification detail: {}",
            v2.detail
        );
        assert_eq!(v2.signer_count, 1);
        assert!(v2.algorithms.contains(&0x0201));
        assert_eq!(v2.certificate_sha256.len(), 1);
        assert_eq!(v2.certificate_sha256[0].len(), 64);
        assert!(!result.v31_present);
    }

    fn tampered_fixture(source_name: &str, label: &str) -> std::path::PathBuf {
        let source = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures")
            .join(source_name);
        let target = std::env::temp_dir().join(format!(
            "android-release-doctor-{label}-{}.apk",
            std::process::id()
        ));
        let mut bytes = std::fs::read(source).expect("source fixture should be readable");
        let index = 40;
        bytes[index] ^= 0x01;
        std::fs::write(&target, bytes).expect("tampered fixture should be writable");
        target
    }

    #[test]
    fn detects_tampered_apk_content_as_invalid_v2_signature() {
        let path = tampered_fixture("crypto-v2-release.apk", "v2-tampered");
        let result = verify_apk_signatures(&path)
            .expect("tampered v2 fixture should be structurally readable")
            .expect("tampered v2 fixture should contain an APK signing block");

        let v2 = result.v2.expect("v2 scheme should be detected");
        assert_eq!(v2.state, CryptoVerificationState::Invalid);
        assert!(
            v2.detail.contains("digest mismatch"),
            "verification detail: {}",
            v2.detail
        );
        std::fs::remove_file(path).expect("temporary tampered v2 fixture should be removed");
    }

    #[test]
    fn verifies_real_v3_signed_apk_fixture() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/crypto-v3-release.apk");

        let result = verify_apk_signatures(&path)
            .expect("v3 fixture should be structurally readable")
            .expect("v3 fixture should contain an APK signing block");

        let v3 = result.v3.expect("v3 scheme should be detected");
        assert_eq!(
            v3.state,
            CryptoVerificationState::Verified,
            "verification detail: {}",
            v3.detail
        );
        assert_eq!(v3.signer_count, 1);
        assert!(v3.algorithms.contains(&0x0201));
        assert_eq!(v3.certificate_sha256.len(), 1);
        assert_eq!(v3.certificate_sha256[0].len(), 64);
        assert_eq!(v3.sdk_ranges.len(), 1);
        assert!(v3.sdk_ranges[0].0 <= v3.sdk_ranges[0].1);
        assert!(v3.proof_of_rotation.is_empty());
        assert!(!result.v31_present);
    }

    fn v31_fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures")
            .join(name)
    }

    #[test]
    fn verifies_real_v31_rotation_fixture_and_cross_block_semantics() {
        let result = verify_apk_signatures(v31_fixture("crypto-v31-release.apk"))
            .expect("v3.1 fixture should be structurally readable")
            .expect("v3.1 fixture should contain an APK signing block");

        assert!(result.v31_present);
        let v3 = result.v3.expect("v3 base block should be detected");
        let v31 = result.v31.expect("v3.1 block should be detected");

        assert_eq!(v3.state, CryptoVerificationState::Verified, "{}", v3.detail);
        assert_eq!(
            v31.state,
            CryptoVerificationState::Verified,
            "{}",
            v31.detail
        );
        assert_eq!(v3.rotation_min_sdk, Some(32));
        assert_eq!(v31.sdk_ranges, vec![(32, i32::MAX as u32)]);
        assert!(!v31.proof_of_rotation.is_empty());
    }

    #[test]
    fn rejects_v31_without_v3_base_block() {
        let result = verify_apk_signatures(v31_fixture("crypto-v31-no-v3.apk"))
            .expect("v3.1 no-v3 fixture should be structurally readable")
            .expect("v3.1 no-v3 fixture should contain an APK signing block");

        assert!(result.v31_present);
        assert!(result.v3.is_none());
        let v31 = result.v31.expect("v3.1 block should be detected");
        assert_eq!(v31.state, CryptoVerificationState::Invalid);
        assert!(v31.detail.contains("without a v3.0 base block"));
    }

    #[test]
    fn rejects_v31_when_v3_rotation_protection_attribute_is_missing() {
        let result = verify_apk_signatures(v31_fixture("crypto-v31-no-v3-attr.apk"))
            .expect("v3.1 no-attribute fixture should be structurally readable")
            .expect("v3.1 no-attribute fixture should contain an APK signing block");

        let v31 = result.v31.expect("v3.1 block should be detected");
        assert_eq!(v31.state, CryptoVerificationState::Invalid);
        assert!(v31.detail.contains("rotation-min-sdk"));
    }

    #[test]
    fn rejects_v31_when_rotation_min_sdk_does_not_match_target() {
        let result = verify_apk_signatures(v31_fixture("crypto-v31-wrong-rotation-min-sdk.apk"))
            .expect("v3.1 mismatch fixture should be structurally readable")
            .expect("v3.1 mismatch fixture should contain an APK signing block");

        let v31 = result.v31.expect("v3.1 block should be detected");
        assert_eq!(v31.state, CryptoVerificationState::Invalid);
        assert!(v31.detail.contains("mismatch"));
    }

    #[test]
    fn rejects_v31_with_malformed_lineage_fixture() {
        let result = verify_apk_signatures(v31_fixture("crypto-v31-lineage-mismatch.apk"))
            .expect("v3.1 malformed-lineage fixture should be structurally readable")
            .expect("v3.1 malformed-lineage fixture should contain an APK signing block");

        let v31 = result.v31.expect("v3.1 block should be detected");
        assert_eq!(v31.state, CryptoVerificationState::Invalid);
        assert!(v31.detail.contains("validation failed"));
    }

    #[test]
    fn rejects_v31_when_lineage_prefix_is_inconsistent() {
        let mut result = ApkSignatureVerification {
            v3: Some(CryptoSchemeInfo {
                state: CryptoVerificationState::Verified,
                signer_count: 1,
                algorithms: vec![0x0101],
                certificate_sha256: vec!["old-cert".to_string()],
                sdk_ranges: vec![(28, 32)],
                rotation_min_sdk: Some(32),
                rotation_targets_dev_release: false,
                proof_of_rotation: vec![ProofOfRotationInfo {
                    state: CryptoVerificationState::Verified,
                    level_count: 1,
                    lineage_certificate_sha256: vec!["old-cert".to_string()],
                    capabilities: Vec::new(),
                    detail: "lineage verified".to_string(),
                }],
                detail: "v3 verified".to_string(),
            }),
            v31: Some(CryptoSchemeInfo {
                state: CryptoVerificationState::Verified,
                signer_count: 1,
                algorithms: vec![0x0101],
                certificate_sha256: vec!["new-cert".to_string()],
                sdk_ranges: vec![(32, u32::MAX)],
                rotation_min_sdk: Some(32),
                rotation_targets_dev_release: false,
                proof_of_rotation: vec![ProofOfRotationInfo {
                    state: CryptoVerificationState::Verified,
                    level_count: 2,
                    lineage_certificate_sha256: vec![
                        "wrong-cert".to_string(),
                        "new-cert".to_string(),
                    ],
                    capabilities: Vec::new(),
                    detail: "lineage verified".to_string(),
                }],
                detail: "v3.1 verified".to_string(),
            }),
            v31_present: true,
            ..Default::default()
        };

        apply_v31_cross_block_semantics(&mut result);

        let v31 = result.v31.expect("v3.1 evidence should remain present");
        assert_eq!(v31.state, CryptoVerificationState::Invalid);
        assert!(v31.detail.contains("lineages are inconsistent"));
    }

    #[test]
    fn detects_tampered_apk_content_as_invalid_v3_signature() {
        let path = tampered_fixture("crypto-v3-release.apk", "v3-tampered");
        let result = verify_apk_signatures(&path)
            .expect("tampered v3 fixture should be structurally readable")
            .expect("tampered v3 fixture should contain an APK signing block");

        let v3 = result.v3.expect("v3 scheme should be detected");
        assert_eq!(v3.state, CryptoVerificationState::Invalid);
        assert!(
            v3.detail.contains("digest mismatch") || v3.detail.contains("signature verification"),
            "verification detail: {}",
            v3.detail
        );
        std::fs::remove_file(path).expect("temporary tampered v3 fixture should be removed");
    }

    fn encode_sequence(payload: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        output.extend_from_slice(payload);
        output
    }

    fn encode_signature_entries(entries: &[(u32, &[u8])]) -> Vec<u8> {
        let mut payload = Vec::new();
        for (algorithm_id, signature) in entries {
            let mut entry = Vec::new();
            entry.extend_from_slice(&algorithm_id.to_le_bytes());
            entry.extend_from_slice(&(signature.len() as u32).to_le_bytes());
            entry.extend_from_slice(signature);
            payload.extend_from_slice(&encode_sequence(&entry));
        }
        payload
    }

    #[test]
    fn verifies_real_v2_ecdsa_sha512_p384_apk_fixture() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/m07-crypto-v2-ecdsa-sha512-p384.apk");

        let result = verify_apk_signatures(&path)
            .expect("v2 ECDSA/SHA-512 fixture should be structurally readable")
            .expect("v2 ECDSA/SHA-512 fixture should contain an APK signing block");

        let v2 = result.v2.expect("v2 scheme should be detected");
        assert_eq!(v2.state, CryptoVerificationState::Verified, "{}", v2.detail);
        assert_eq!(v2.algorithms, vec![0x0202]);
        assert_eq!(v2.signer_count, 1);
        assert_eq!(v2.certificate_sha256.len(), 1);
    }

    #[test]
    fn verifies_real_v3_ecdsa_sha512_p384_apk_fixture() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/m07-crypto-v3-ecdsa-sha512-p384.apk");

        let result = verify_apk_signatures(&path)
            .expect("v3 ECDSA/SHA-512 fixture should be structurally readable")
            .expect("v3 ECDSA/SHA-512 fixture should contain an APK signing block");

        let v3 = result.v3.expect("v3 scheme should be detected");
        assert_eq!(v3.state, CryptoVerificationState::Verified, "{}", v3.detail);
        assert_eq!(v3.algorithms, vec![0x0202]);
        assert_eq!(v3.signer_count, 1);
        assert_eq!(v3.sdk_ranges, vec![(24, 35)]);
        assert_eq!(v3.certificate_sha256.len(), 1);
    }

    #[test]
    fn detects_tampered_ecdsa_sha512_p384_v2_apk_as_invalid() {
        let path = tampered_fixture(
            "m07-crypto-v2-ecdsa-sha512-p384.apk",
            "v2-ecdsa-sha512-tampered",
        );
        let result = verify_apk_signatures(&path)
            .expect("tampered v2 ECDSA/SHA-512 fixture should remain readable")
            .expect("tampered v2 ECDSA/SHA-512 fixture should contain a signing block");

        let v2 = result.v2.expect("v2 scheme should be detected");
        assert_eq!(v2.state, CryptoVerificationState::Invalid);
        std::fs::remove_file(path).expect("temporary tampered fixture should be removed");
    }

    #[test]
    fn detects_tampered_ecdsa_sha512_p384_v3_apk_as_invalid() {
        let path = tampered_fixture(
            "m07-crypto-v3-ecdsa-sha512-p384.apk",
            "v3-ecdsa-sha512-tampered",
        );
        let result = verify_apk_signatures(&path)
            .expect("tampered v3 ECDSA/SHA-512 fixture should remain readable")
            .expect("tampered v3 ECDSA/SHA-512 fixture should contain a signing block");

        let v3 = result.v3.expect("v3 scheme should be detected");
        assert_eq!(v3.state, CryptoVerificationState::Invalid);
        std::fs::remove_file(path).expect("temporary tampered fixture should be removed");
    }

    #[test]
    fn accepts_additional_v2_certificate_chain_entries() {
        let digest_entry = {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&0x0201_u32.to_le_bytes());
            bytes.extend_from_slice(&32_u32.to_le_bytes());
            bytes.extend_from_slice(&[0_u8; 32]);
            bytes
        };
        let digests = encode_sequence(&digest_entry);
        let mut certificates = Vec::new();
        certificates.extend_from_slice(&encode_sequence(b"signer"));
        certificates.extend_from_slice(&encode_sequence(b"intermediate"));
        certificates.extend_from_slice(&encode_sequence(b"root"));

        let mut signed_data = Vec::new();
        signed_data.extend_from_slice(&encode_sequence(&digests));
        signed_data.extend_from_slice(&encode_sequence(&certificates));
        signed_data.extend_from_slice(&encode_sequence(&[]));

        let parsed = parse_signed_data_v2(&signed_data).expect("certificate chain should parse");
        assert_eq!(parsed.certificate, b"signer");
    }

    fn v2_signed_data_for_parser(extra: &[u8]) -> Vec<u8> {
        let mut digest_entry = Vec::new();
        digest_entry.extend_from_slice(&0x0201_u32.to_le_bytes());
        digest_entry.extend_from_slice(&32_u32.to_le_bytes());
        digest_entry.extend_from_slice(&[0_u8; 32]);

        let digests = encode_sequence(&encode_sequence(&digest_entry));
        let certificates = encode_sequence(&encode_sequence(b"signer"));
        let attributes = encode_sequence(&[]);

        let mut signed_data = Vec::new();
        signed_data.extend_from_slice(&digests);
        signed_data.extend_from_slice(&certificates);
        signed_data.extend_from_slice(&attributes);
        signed_data.extend_from_slice(extra);
        signed_data
    }

    #[test]
    fn accepts_v2_empty_fourth_signed_data_element() {
        let signed_data = v2_signed_data_for_parser(&encode_sequence(&[]));

        parse_signed_data_v2(&signed_data)
            .expect("an empty fourth v2 signed-data element should be accepted");
    }

    #[test]
    fn rejects_v2_truncated_fourth_signed_data_element() {
        let signed_data = v2_signed_data_for_parser(&[0, 0, 0]);

        let error = match parse_signed_data_v2(&signed_data) {
            Ok(_) => panic!("a truncated fourth element length must be rejected"),
            Err(error) => error,
        };
        assert!(error
            .to_string()
            .contains("v2 optional fourth signed-data element is truncated"));
    }

    #[test]
    fn rejects_v2_non_empty_fourth_signed_data_element() {
        let signed_data = v2_signed_data_for_parser(&encode_sequence(b"x"));

        let error = match parse_signed_data_v2(&signed_data) {
            Ok(_) => panic!("a non-empty fourth element must be rejected"),
            Err(error) => error,
        };
        assert!(error
            .to_string()
            .contains("v2 optional fourth signed-data element must be empty"));
    }

    #[test]
    fn rejects_v2_multiple_fourth_signed_data_elements() {
        let mut extra = encode_sequence(&[]);
        extra.extend_from_slice(&encode_sequence(&[]));
        let signed_data = v2_signed_data_for_parser(&extra);

        let error = match parse_signed_data_v2(&signed_data) {
            Ok(_) => panic!("multiple residual elements must be rejected"),
            Err(error) => error,
        };
        assert!(error
            .to_string()
            .contains("v2 signed data contains trailing bytes"));
    }

    #[test]
    fn accepts_additional_v3_certificate_chain_entries() {
        let digest_entry = {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&0x0201_u32.to_le_bytes());
            bytes.extend_from_slice(&32_u32.to_le_bytes());
            bytes.extend_from_slice(&[0_u8; 32]);
            bytes
        };
        let digests = encode_sequence(&digest_entry);
        let mut certificates = Vec::new();
        certificates.extend_from_slice(&encode_sequence(b"signer"));
        certificates.extend_from_slice(&encode_sequence(b"intermediate"));

        let mut signed_data = Vec::new();
        signed_data.extend_from_slice(&encode_sequence(&digests));
        signed_data.extend_from_slice(&encode_sequence(&certificates));
        signed_data.extend_from_slice(&1_u32.to_le_bytes());
        signed_data.extend_from_slice(&2_u32.to_le_bytes());
        signed_data.extend_from_slice(&encode_sequence(&[]));

        let parsed = parse_signed_data_v3(&signed_data, V3_BLOCK_ID)
            .expect("certificate chain should parse");
        assert_eq!(parsed.certificate, b"signer");
    }

    fn decode_hex_bytes(input: &str) -> Vec<u8> {
        assert_eq!(input.len() % 2, 0, "hex input must contain complete bytes");
        (0..input.len())
            .step_by(2)
            .map(|index| u8::from_str_radix(&input[index..index + 2], 16).expect("valid hex"))
            .collect()
    }

    #[test]
    fn selects_ecdsa_sha512_algorithm() {
        let signatures = encode_signature_entries(&[(0x0202, b"ecdsa-sha512")]);
        let selected =
            parse_and_select_signature(&signatures).expect("ECDSA SHA-512 should be selectable");
        assert_eq!(selected.algorithm_id, 0x0202);
        assert_eq!(selected.digest_algorithm, DigestAlgorithm::Sha512);
    }

    #[test]
    fn verifies_real_ecdsa_sha512_p384_signature() {
        const CERTIFICATE_HEX: &str = "3082018a30820110a003020102020101300a06082a8648ce3d0403023030312e302c06035504030c25416e64726f69642052656c6561736520446f63746f72204d302e3720503338342054657374301e170d3230303130313030303030305a170d3330303130313030303030305a3030312e302c06035504030c25416e64726f69642052656c6561736520446f63746f72204d302e37205033383420546573743076301006072a8648ce3d020106052b8104002203620004aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab73617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f300a06082a8648ce3d0403020368003065023100e3e95b7cbcdb11f7848ff4b0bfac569efbea08246f376cc87bd115c66cdb80045e676627ce5f9610cefe4d33cf3e0dfe023062ea2e6d0c514053881751194ccd3009265198faca21850ace1503328481ca7785d380a26b6914cfe04e506d5bbf4d6a";
        const SIGNATURE_HEX: &str = "3065023100c2f74059ab82a8d731b855e9a74745151d1360b6d73e909b828cdd5344ae328aec360003381e22963b6acf228cad382302307fce45b1d1991aeb7a54ccd32e0b4f093742cf2dfdce7c75fe0984754a156a750d05f9c3053783038f315a70a0b3ac13";
        let certificate = decode_hex_bytes(CERTIFICATE_HEX);
        let signature = decode_hex_bytes(SIGNATURE_HEX);
        let signed_data = b"M0.7 Block 1 deterministic ECDSA/SHA-512 verification test message";

        let (_, parsed_certificate) = X509Certificate::from_der(&certificate)
            .expect("deterministic P-384 test certificate should parse");
        let public_key = parsed_certificate.tbs_certificate.subject_pki.raw.to_vec();
        verify_certificate_and_public_key(&certificate, &public_key)
            .expect("certificate and signer public key must match");

        verify_signature_bytes(0x0202, &certificate, signed_data, &signature)
            .expect("ECDSA/SHA-512 P-384 signature should verify");
    }

    #[test]
    fn verifies_real_ecdsa_sha256_p521_signature() {
        const CERTIFICATE_HEX: &str = "308201dd3082013ea003020102020101300a06082a8648ce3d04030230343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d30333820502d3532312054657374301e170d3236303130313030303030305a170d3336303130313030303030305a30343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d30333820502d353231205465737430819b301006072a8648ce3d020106052b810400230381860004015c3a5ace9aef63096b062372efa35b1f15bef8a48fb235ced04bddc95e672b5af569f5dafb1ece9c792c3f4a0aaf2a11770180915f49a0e712e2cb40d2acd7d1200192f3a581cd6d73f875a95404967b6338e89faead021eb531181749be978ff8d9fb27295e86fe23c0250ec162b70c1616fc376ed041753a4a3c96b8ab2a5889af74300a06082a8648ce3d04030203818c00308188024201131fde50b50fe958b8487a8641f2ca35ff6f36a8c0fbbd9117bcceb590b8826af0bb7f64d2dbda0880e773effad433bce43a4b7a69a08027895fb7882bb418705b02420187710b9840f04982b17e4e928beb3faa963f0ccebc2a1e413c36f2d7ec89e6cf9e52a18058a21e9b9c04967c4bbdee626596983330cdb46d9c88691c8a4eea580d";
        const SIGNATURE_HEX: &str = "308188024201cc69ba15bf6fa014d34fa1baf48a00043d1074ab774d81aa50bc2394107d96058620db3ca7338c4dc652168e1a16429cbc1ec261fc454c3318fc51ce7688d37cc0024200f2cc1e308af867a798d6a8a73ebbbd75d0b9c9dacbe6b6007940acee375d220a88d02b31f83d9bd9faeac416c8e840f303d25f8ca396da3acc1ef9248d1d8c4708";
        let certificate = decode_hex_bytes(CERTIFICATE_HEX);
        let signature = decode_hex_bytes(SIGNATURE_HEX);
        let signed_data = b"Android Release Doctor ERR-038 P-521 deterministic test message";

        verify_signature_bytes(0x0201, &certificate, signed_data, &signature)
            .expect("ECDSA/SHA-256 P-521 signature should verify");
    }

    #[test]
    fn verifies_real_ecdsa_sha512_p521_signature() {
        const CERTIFICATE_HEX: &str = "308201dd3082013ea003020102020101300a06082a8648ce3d04030230343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d30333820502d3532312054657374301e170d3236303130313030303030305a170d3336303130313030303030305a30343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d30333820502d353231205465737430819b301006072a8648ce3d020106052b810400230381860004015c3a5ace9aef63096b062372efa35b1f15bef8a48fb235ced04bddc95e672b5af569f5dafb1ece9c792c3f4a0aaf2a11770180915f49a0e712e2cb40d2acd7d1200192f3a581cd6d73f875a95404967b6338e89faead021eb531181749be978ff8d9fb27295e86fe23c0250ec162b70c1616fc376ed041753a4a3c96b8ab2a5889af74300a06082a8648ce3d04030203818c00308188024201131fde50b50fe958b8487a8641f2ca35ff6f36a8c0fbbd9117bcceb590b8826af0bb7f64d2dbda0880e773effad433bce43a4b7a69a08027895fb7882bb418705b02420187710b9840f04982b17e4e928beb3faa963f0ccebc2a1e413c36f2d7ec89e6cf9e52a18058a21e9b9c04967c4bbdee626596983330cdb46d9c88691c8a4eea580d";
        const SIGNATURE_HEX: &str = "308188024200820e8b942ed533a751968195a64273bae0dffb747660fdfba18933d9fb64118d4c595ebffba69ff5397af4ef462084e7e2a18f63207f7e9917aa9273cb3528f9500242012aa53edf0a7c6b27500ea8c77fe8b7de02a352eb7b7c9c5f57e2fc5433da653ca0504112219208c113226b1ca6d0eab6b5cfa654494a07c255e13c811b34fdf991";
        let certificate = decode_hex_bytes(CERTIFICATE_HEX);
        let signature = decode_hex_bytes(SIGNATURE_HEX);
        let signed_data = b"Android Release Doctor ERR-038 P-521 deterministic test message";

        verify_signature_bytes(0x0202, &certificate, signed_data, &signature)
            .expect("ECDSA/SHA-512 P-521 signature should verify");
    }

    #[test]
    fn rejects_tampered_ecdsa_sha256_p521_signature() {
        const CERTIFICATE_HEX: &str = "308201dd3082013ea003020102020101300a06082a8648ce3d04030230343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d30333820502d3532312054657374301e170d3236303130313030303030305a170d3336303130313030303030305a30343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d30333820502d353231205465737430819b301006072a8648ce3d020106052b810400230381860004015c3a5ace9aef63096b062372efa35b1f15bef8a48fb235ced04bddc95e672b5af569f5dafb1ece9c792c3f4a0aaf2a11770180915f49a0e712e2cb40d2acd7d1200192f3a581cd6d73f875a95404967b6338e89faead021eb531181749be978ff8d9fb27295e86fe23c0250ec162b70c1616fc376ed041753a4a3c96b8ab2a5889af74300a06082a8648ce3d04030203818c00308188024201131fde50b50fe958b8487a8641f2ca35ff6f36a8c0fbbd9117bcceb590b8826af0bb7f64d2dbda0880e773effad433bce43a4b7a69a08027895fb7882bb418705b02420187710b9840f04982b17e4e928beb3faa963f0ccebc2a1e413c36f2d7ec89e6cf9e52a18058a21e9b9c04967c4bbdee626596983330cdb46d9c88691c8a4eea580d";
        const SIGNATURE_HEX: &str = "308188024201cc69ba15bf6fa014d34fa1baf48a00043d1074ab774d81aa50bc2394107d96058620db3ca7338c4dc652168e1a16429cbc1ec261fc454c3318fc51ce7688d37cc0024200f2cc1e308af867a798d6a8a73ebbbd75d0b9c9dacbe6b6007940acee375d220a88d02b31f83d9bd9faeac416c8e840f303d25f8ca396da3acc1ef9248d1d8c4708";
        let certificate = decode_hex_bytes(CERTIFICATE_HEX);
        let mut signature = decode_hex_bytes(SIGNATURE_HEX);
        *signature.last_mut().expect("signature must not be empty") ^= 0x01;
        let signed_data = b"Android Release Doctor ERR-038 P-521 deterministic test message";

        let result = verify_signature_bytes(0x0201, &certificate, signed_data, &signature);
        let error = result.expect_err("tampered ECDSA/SHA-256 P-521 signature must fail");
        assert!(error
            .to_string()
            .contains("cryptographic signature verification failed for algorithm 0x00000201"));
    }

    #[test]
    fn rejects_tampered_ecdsa_sha512_p384_signature() {
        const CERTIFICATE_HEX: &str = "3082018b30820110a003020102020101300a06082a8648ce3d0403023030312e302c06035504030c25416e64726f69642052656c6561736520446f63746f72204d302e3720503338342054657374301e170d3230303130313030303030305a170d3330303130313030303030305a3030312e302c06035504030c25416e64726f69642052656c6561736520446f63746f72204d302e37205033383420546573743076301006072a8648ce3d020106052b8104002203620004a14aad95673d51513a385309151ee57b66f8ef6d80a03ae54b268767b28cb37f72f272aa5fb5d11d7395157d985b5f33229d4134d1a63d2a1afa184a2d09e52b2d71527e66fb1427c13e6b1cb1978d474a7b7b735d792cdaa0996332db968ab4300a06082a8648ce3d04030203690030660231008fdd8b061b24a5c28de24cd4103801ee8a5e7d68e9af6b679c9d2dd5a3b22731340114ec2aed1db84ddb6e479f39ae480231008d962e5ab24b64535aaa9be84463730099c3c8d47dd2c1ced09de30e0529359e6de123bf80030b43d9023f0e35066507";
        const SIGNATURE_HEX: &str = "30650230352054fd8fbb9ff1fd661502ce0a1160b09f722682f86ac8677a646c94b57edddc29b85c55e1e59094e04f3736069801023100b8151f6b21f13c101600f7ffb62c7ba2199531657f01b3a03f625ca1fb7f179730f592eea892d241c52a856e8f4ef28c";
        let certificate = decode_hex_bytes(CERTIFICATE_HEX);
        let mut signature = decode_hex_bytes(SIGNATURE_HEX);
        let last = signature.len() - 1;
        signature[last] ^= 0x01;
        let signed_data = b"M0.7 Block 1 deterministic ECDSA/SHA-512 verification test message";

        let result = verify_signature_bytes(0x0202, &certificate, signed_data, &signature);
        let error = result.expect_err("tampered ECDSA/SHA-512 signature must fail");
        assert!(error
            .to_string()
            .contains("cryptographic signature verification failed for algorithm 0x00000202"));
    }

    #[test]
    fn reports_unsupported_curve_for_ecdsa_sha512() {
        const CERTIFICATE_HEX: &str = "3082018a30820110a003020102020101300a06082a8648ce3d0403023030312e302c06035504030c25416e64726f69642052656c6561736520446f63746f72204d302e3720503338342054657374301e170d3230303130313030303030305a170d3330303130313030303030305a3030312e302c06035504030c25416e64726f69642052656c6561736520446f63746f72204d302e37205033383420546573743076301006072a8648ce3d020106052b8104002203620004aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab73617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f300a06082a8648ce3d0403020368003065023100e3e95b7cbcdb11f7848ff4b0bfac569efbea08246f376cc87bd115c66cdb80045e676627ce5f9610cefe4d33cf3e0dfe023062ea2e6d0c514053881751194ccd3009265198faca21850ace1503328481ca7785d380a26b6914cfe04e506d5bbf4d6a";
        let mut certificate = decode_hex_bytes(CERTIFICATE_HEX);
        let curve_oid = [0x06_u8, 0x05, 0x2b, 0x81, 0x04, 0x00, 0x22];
        let position = certificate
            .windows(curve_oid.len())
            .position(|window| window == curve_oid)
            .expect("P-384 curve OID should be present in certificate");
        certificate[position + curve_oid.len() - 1] = 0x21;

        let result = verify_signature_bytes(
            0x0202,
            &certificate,
            b"M0.7 Block 1 deterministic ECDSA/SHA-512 verification test message",
            &[],
        );
        let error = result.expect_err("unsupported ECDSA/SHA-512 curve must remain unsupported");
        assert!(error
            .to_string()
            .starts_with("UNSUPPORTED: ECDSA SHA-512 signer curve is neither P-384 nor P-521"));
    }

    #[test]
    fn verifies_extended_rsa_1024_pkcs1_sha256() {
        const CERTIFICATE_HEX: &str = "30820244308201ada00302010202142b39807fe7042283f23640917f3b9431a5453a77300d06092a864886f70d01010b050030343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d3033382d42205253412d31303234301e170d3236313030323138343235305a170d3336303932393138343235305a30343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d3033382d42205253412d3130323430819f300d06092a864886f70d010101050003818d0030818902818100c52ef610592856a31dd080c4f23e503528b5a1e19fd0446d7f0573a787db87cc68f8bb11b63900155cb1b1fdc69602bfe8d658ac9cf690abbc1f74a31ddf08e3cc7efa21af38209b5433dd56276f04edd5bd9ce9e6885fe8a02b34321c573df893541c7ac985f0e44692b4cd4322399e1e925a89a6c1a143790eecefbad978790203010001a3533051301d0603551d0e041604146af530ad002de35edb847d63051db47369a66ac8301f0603551d230418301680146af530ad002de35edb847d63051db47369a66ac8300f0603551d130101ff040530030101ff300d06092a864886f70d01010b05000381810087c69112fe20d7d3d5831f829a72b1ba28246f9ff594911dfe13c2b88c21211b6f0474bb7342ff807ec695280afd7ad45f44b4c5088bd5bb3a163cff4453e9a5706f9d2da24d26c17d9bf9d749a629c9296f789e593d80402f34e8f112d4cef739a633d482254612e4e1a000b9bd88d70277f7f3ec88e6945006545846657c38";
        const SIGNATURE_HEX: &str = "19b15a238dddad1d4451b6860bf614bb5872e2e317b8f03dfddd8468bfdf74820749b21f5a9e817efa46405ed7ceb476e9e78190537f13ac74ca979b7c369ca14999b6410d51860bf209795821d967ad9106d95e8a789171a04f4932ca047751996e256c2fde520a2d4bcef8e47852c42c546137df3b7e5965c98e059cca106e";
        let certificate = decode_hex_bytes(CERTIFICATE_HEX);
        let signed_data = b"Android Release Doctor ERR-038-B RSA-1024 deterministic test message";
        let signature = decode_hex_bytes(SIGNATURE_HEX);

        let (_, cert) =
            X509Certificate::from_der(&certificate).expect("RSA-1024 certificate should parse");
        let rsa = match cert.public_key().parsed().expect("RSA public key should parse") {
            PublicKey::RSA(rsa) => rsa,
            _ => panic!("expected RSA public key"),
        };

        verify_extended_rsa_signature(
            0x0103,
            1024,
            rsa.modulus,
            rsa.exponent,
            signed_data,
            &signature,
        )
        .expect("RSA-1024 PKCS1/SHA-256 should verify");
    }

    #[test]
    fn verifies_extended_rsa_1024_pkcs1_sha512() {
        const CERTIFICATE_HEX: &str = "30820244308201ada00302010202142b39807fe7042283f23640917f3b9431a5453a77300d06092a864886f70d01010b050030343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d3033382d42205253412d31303234301e170d3236313030323138343235305a170d3336303932393138343235305a30343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d3033382d42205253412d3130323430819f300d06092a864886f70d010101050003818d0030818902818100c52ef610592856a31dd080c4f23e503528b5a1e19fd0446d7f0573a787db87cc68f8bb11b63900155cb1b1fdc69602bfe8d658ac9cf690abbc1f74a31ddf08e3cc7efa21af38209b5433dd56276f04edd5bd9ce9e6885fe8a02b34321c573df893541c7ac985f0e44692b4cd4322399e1e925a89a6c1a143790eecefbad978790203010001a3533051301d0603551d0e041604146af530ad002de35edb847d63051db47369a66ac8301f0603551d230418301680146af530ad002de35edb847d63051db47369a66ac8300f0603551d130101ff040530030101ff300d06092a864886f70d01010b05000381810087c69112fe20d7d3d5831f829a72b1ba28246f9ff594911dfe13c2b88c21211b6f0474bb7342ff807ec695280afd7ad45f44b4c5088bd5bb3a163cff4453e9a5706f9d2da24d26c17d9bf9d749a629c9296f789e593d80402f34e8f112d4cef739a633d482254612e4e1a000b9bd88d70277f7f3ec88e6945006545846657c38";
        const SIGNATURE_HEX: &str = "b34ec713c854fe3fef244b71f255e1e445bfcfdfe8691d6fdd034601a3bc5b598b5efea9b17727f5a6f8154b914ddae70add643a218f768c5efcb38cf3d9677567744b4d51ae7b84aa11b78f62b45b989d621d607dae58fdbb91c946238b518f58e06217fedd5afef55651881fb952ab42ba90ec6b9f6cd3ffcef0fd28cc4901";
        let certificate = decode_hex_bytes(CERTIFICATE_HEX);
        let signed_data = b"Android Release Doctor ERR-038-B RSA-1024 deterministic test message";
        let signature = decode_hex_bytes(SIGNATURE_HEX);

        let (_, cert) =
            X509Certificate::from_der(&certificate).expect("RSA-1024 certificate should parse");
        let rsa = match cert.public_key().parsed().expect("RSA public key should parse") {
            PublicKey::RSA(rsa) => rsa,
            _ => panic!("expected RSA public key"),
        };

        verify_extended_rsa_signature(
            0x0104,
            1024,
            rsa.modulus,
            rsa.exponent,
            signed_data,
            &signature,
        )
        .expect("RSA-1024 PKCS1/SHA-512 should verify");
    }

    #[test]
    fn verifies_extended_rsa_1024_pss_sha256() {
        const CERTIFICATE_HEX: &str = "30820244308201ada00302010202142b39807fe7042283f23640917f3b9431a5453a77300d06092a864886f70d01010b050030343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d3033382d42205253412d31303234301e170d3236313030323138343235305a170d3336303932393138343235305a30343132303006035504030c29416e64726f69642052656c6561736520446f63746f72204552522d3033382d42205253412d3130323430819f300d06092a864886f70d010101050003818d0030818902818100c52ef610592856a31dd080c4f23e503528b5a1e19fd0446d7f0573a787db87cc68f8bb11b63900155cb1b1fdc69602bfe8d658ac9cf690abbc1f74a31ddf08e3cc7efa21af38209b5433dd56276f04edd5bd9ce9e6885fe8a02b34321c573df893541c7ac985f0e44692b4cd4322399e1e925a89a6c1a143790eecefbad978790203010001a3533051301d0603551d0e041604146af530ad002de35edb847d63051db47369a66ac8301f0603551d230418301680146af530ad002de35edb847d63051db47369a66ac8300f0603551d130101ff040530030101ff300d06092a864886f70d01010b05000381810087c69112fe20d7d3d5831f829a72b1ba28246f9ff594911dfe13c2b88c21211b6f0474bb7342ff807ec695280afd7ad45f44b4c5088bd5bb3a163cff4453e9a5706f9d2da24d26c17d9bf9d749a629c9296f789e593d80402f34e8f112d4cef739a633d482254612e4e1a000b9bd88d70277f7f3ec88e6945006545846657c38";
        const SIGNATURE_HEX: &str = "40cfd7f64fd834368ac56736e3c5a7fc2f590e2ae8222eddd6c88add518c25b83544d6f94da74fe42e4355c8d86f821582a47f9dac6cbd038fe1e922afdcfd4a9ac511fc1fb4b03d88fbfa52ae3daefe7ae72925286d2ff66d1856c4ab72a859af24026e3fd0dea339f09df4db45b63969fe1cc33260068d54e5aca4c881078b";
        let certificate = decode_hex_bytes(CERTIFICATE_HEX);
        let signed_data = b"Android Release Doctor ERR-038-B RSA-1024 deterministic test message";
        let signature = decode_hex_bytes(SIGNATURE_HEX);

        let (_, cert) =
            X509Certificate::from_der(&certificate).expect("RSA-1024 certificate should parse");
        let rsa = match cert.public_key().parsed().expect("RSA public key should parse") {
            PublicKey::RSA(rsa) => rsa,
            _ => panic!("expected RSA public key"),
        };

        verify_extended_rsa_signature(
            0x0101,
            1024,
            rsa.modulus,
            rsa.exponent,
            signed_data,
            &signature,
        )
        .expect("RSA-1024 PSS/SHA-256 should verify");
    }

    #[test]
    fn rejects_extended_rsa_1024_pss_sha512_as_unsupported() {
        let modulus = vec![0xff; 128];
        let exponent = vec![0x01, 0x00, 0x01];

        let error = verify_extended_rsa_signature(
            0x0102,
            1024,
            &modulus,
            &exponent,
            b"Android Release Doctor ERR-038-B RSA-1024 deterministic test message",
            &[0; 128],
        )
        .expect_err("RSA-1024 PSS/SHA-512 must be unsupported");

        assert!(error
            .to_string()
            .starts_with("UNSUPPORTED: RSA 1024-bit key is too short"));
    }

    #[test]
    fn verifies_aosp_rsa_16384_certificate_self_signature_sha256() {
        const CERTIFICATE_PEM: &str = r#"-----BEGIN CERTIFICATE-----
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
-----END CERTIFICATE-----"#;
        let (_, pem) =
            parse_x509_pem(CERTIFICATE_PEM.as_bytes()).expect("AOSP RSA-16384 PEM should parse");
        let certificate = pem.contents.as_ref();
        let (_, cert) =
            X509Certificate::from_der(certificate).expect("AOSP RSA-16384 certificate should parse");

        let rsa = match cert.public_key().parsed().unwrap() {
            PublicKey::RSA(rsa) => rsa,
            _ => panic!("expected RSA public key"),
        };
        assert_eq!(rsa.key_size(), 16_384);

        let signed_data =
            b"Android Release Doctor ERR-038-B RSA-16384 deterministic test message";
        let vectors = [
            (
                0x0101_u32,
                "tests/fixtures/err-038-b/rsa-16384/sig-pss-sha256.bin",
            ),
            (
                0x0102_u32,
                "tests/fixtures/err-038-b/rsa-16384/sig-pss-sha512.bin",
            ),
            (
                0x0103_u32,
                "tests/fixtures/err-038-b/rsa-16384/sig-pkcs1-sha256.bin",
            ),
            (
                0x0104_u32,
                "tests/fixtures/err-038-b/rsa-16384/sig-pkcs1-sha512.bin",
            ),
        ];

        for (algorithm_id, fixture) in vectors {
            let signature =
                std::fs::read(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
                    format!("../../{fixture}"),
                ))
                .expect("RSA-16384 signature fixture should be readable");

            verify_extended_rsa_signature(
                algorithm_id,
                16_384,
                rsa.modulus,
                rsa.exponent,
                signed_data,
                &signature,
            )
            .unwrap_or_else(|error| {
                panic!(
                    "RSA-16384 algorithm 0x{algorithm_id:04x} should verify: {error}"
                )
            });
        }

        let mut tampered =
            std::fs::read(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
                "../../tests/fixtures/err-038-b/rsa-16384/sig-pss-sha256.bin",
            ))
            .expect("RSA-16384 signature fixture should be readable");
        *tampered.last_mut().expect("signature must not be empty") ^= 0x01;
        assert!(
            verify_extended_rsa_signature(
                0x0101,
                16_384,
                rsa.modulus,
                rsa.exponent,
                signed_data,
                &tampered,
            )
            .is_err(),
            "tampered RSA-16384 PSS/SHA-256 signature must fail"
        );
    }

    #[test]
    fn ring_rsa_key_size_boundary_is_explicit() {
        assert!(!ring_rsa_key_size_supported(1024));
        assert!(ring_rsa_key_size_supported(2048));
        assert!(ring_rsa_key_size_supported(8192));
        assert!(!ring_rsa_key_size_supported(16384));
    }

    #[test]
    fn signature_selection_prefers_sha512_content_digests() {
        let signatures = encode_signature_entries(&[
            (0x0103, b"sha256"),
            (0x0104, b"sha512"),
            (0x0201, b"sha256-ecdsa"),
        ]);

        let selected =
            parse_and_select_signature(&signatures).expect("signature records should parse");
        assert_eq!(selected.algorithm_id, 0x0104);
        assert_eq!(selected.digest_algorithm, DigestAlgorithm::Sha512);
    }

    #[test]
    fn signature_strength_matches_android_digest_preference() {
        assert_eq!(signature_strength(0x0102), signature_strength(0x0104));
        assert_eq!(signature_strength(0x0104), signature_strength(0x0202));
        assert!(signature_strength(0x0104) > signature_strength(0x0103));
    }

    #[test]
    fn rejects_v3_signed_data_with_reversed_sdk_range() {
        let digest_entry = {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&0x0201_u32.to_le_bytes());
            bytes.extend_from_slice(&32_u32.to_le_bytes());
            bytes.extend_from_slice(&[0_u8; 32]);
            bytes
        };
        let mut digests = Vec::new();
        digests.extend_from_slice(&encode_sequence(&digest_entry));

        let certificate = encode_sequence(&[0_u8]);
        let mut signed_data = Vec::new();
        signed_data.extend_from_slice(&encode_sequence(&digests));
        signed_data.extend_from_slice(&encode_sequence(&certificate));
        signed_data.extend_from_slice(&100_u32.to_le_bytes());
        signed_data.extend_from_slice(&1_u32.to_le_bytes());
        signed_data.extend_from_slice(&encode_sequence(&[]));

        let result = parse_signed_data_v3(&signed_data, V3_BLOCK_ID);
        assert!(result.is_err());
        let error = result
            .err()
            .expect("reversed SDK range should produce an error");
        assert!(error
            .to_string()
            .contains("minSDK 100 is greater than maxSDK 1"));
    }

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

    #[test]
    fn signer_error_evidence_preserves_v3_range() {
        let info = error_to_scheme_info_with_evidence(
            SignatureVerificationError("signature verification failed".to_string()),
            1,
            vec![0x0201],
            vec!["a".repeat(64)],
            vec![(28, 32)],
        );

        assert_eq!(info.state, CryptoVerificationState::Invalid);
        assert_eq!(info.signer_count, 1);
        assert_eq!(info.algorithms, vec![0x0201]);
        assert_eq!(info.certificate_sha256.len(), 1);
        assert_eq!(info.sdk_ranges, vec![(28, 32)]);
        assert!(info.detail.contains("signature verification failed"));
    }

    #[test]
    fn signer_error_evidence_preserves_proof_of_rotation() {
        let proof = ProofOfRotationInfo {
            state: CryptoVerificationState::Verified,
            level_count: 2,
            lineage_certificate_sha256: Vec::new(),
            capabilities: Vec::new(),
            detail: "proof-of-rotation lineage verified across 2 certificate level(s)".to_string(),
        };

        let info = error_to_scheme_info_with_rotation(
            SignatureVerificationError("later signer verification failed".to_string()),
            1,
            vec![0x0201],
            vec!["a".repeat(64)],
            vec![(28, 35)],
            vec![proof.clone()],
        );

        assert_eq!(info.state, CryptoVerificationState::Invalid);
        assert_eq!(info.signer_count, 1);
        assert_eq!(info.proof_of_rotation, vec![proof]);
    }

    fn proof_rotation_fixture() -> Vec<u8> {
        std::fs::read(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/fixtures/proof-rotation-valid.bin"),
        )
        .expect("proof-of-rotation fixture should be readable")
    }

    fn proof_rotation_certificates(bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut reader = LengthReader::new(bytes);
        assert_eq!(
            reader
                .read_u32("fixture version")
                .expect("fixture should contain a version"),
            1
        );
        let first_node = reader
            .read_sequence("fixture first node")
            .expect("fixture should contain first lineage node");
        let second_node = reader
            .read_sequence("fixture second node")
            .expect("fixture should contain second lineage node");
        reader
            .finish("fixture")
            .expect("fixture should have no trailing bytes");

        let mut first_reader = LengthReader::new(first_node);
        let first_signed_data = first_reader
            .read_sequence("fixture first signed data")
            .expect("first node should contain signed data");
        let mut first_signed_reader = LengthReader::new(first_signed_data);
        let first_certificate = first_signed_reader
            .read_length_prefixed("fixture first certificate")
            .expect("first node should contain a certificate")
            .to_vec();

        let mut second_reader = LengthReader::new(second_node);
        let second_signed_data = second_reader
            .read_sequence("fixture second signed data")
            .expect("second node should contain signed data");
        let mut second_signed_reader = LengthReader::new(second_signed_data);
        let second_certificate = second_signed_reader
            .read_length_prefixed("fixture second certificate")
            .expect("second node should contain a certificate")
            .to_vec();

        (first_certificate, second_certificate)
    }

    fn proof_rotation_with_flags(bytes: &[u8], level_index: usize, flags: u32) -> Vec<u8> {
        let mut output = bytes.to_vec();
        let mut reader = LengthReader::new(&output);
        reader
            .read_u32("fixture version")
            .expect("fixture should contain a version");

        for index in 0..=level_index {
            let node = reader
                .read_sequence("fixture proof-of-rotation node")
                .expect("fixture node should be well formed")
                .to_vec();
            if index == level_index {
                let mut node_reader = LengthReader::new(&node);
                let signed_data = node_reader
                    .read_sequence("fixture signed data")
                    .expect("fixture node should contain signed data");
                let flags_offset = 4 + signed_data.len();
                let node_start = reader.cursor - node.len();
                let absolute_offset = node_start + flags_offset;
                output[absolute_offset..absolute_offset + 4].copy_from_slice(&flags.to_le_bytes());
                break;
            }
        }

        output
    }

    #[test]
    fn proof_of_rotation_flags_are_decoded_as_independent_capabilities() {
        let proof = proof_rotation_fixture();
        let (_, current_certificate) = proof_rotation_certificates(&proof);
        let flags = PROOF_OF_ROTATION_FLAG_INSTALLED_DATA
            | PROOF_OF_ROTATION_FLAG_SHARED_USER_ID
            | PROOF_OF_ROTATION_FLAG_PERMISSION
            | PROOF_OF_ROTATION_FLAG_ROLLBACK
            | PROOF_OF_ROTATION_FLAG_AUTH;

        let proof = proof_rotation_with_flags(&proof, 0, flags);
        let result = validate_proof_of_rotation(&proof, &current_certificate);

        assert_eq!(result.state, CryptoVerificationState::Verified);
        assert_eq!(result.capabilities.len(), 2);
        let first = &result.capabilities[0];
        assert_eq!(first.flags, flags);
        assert_eq!(first.known_flags, flags);
        assert_eq!(first.unknown_flags, 0);
        assert!(first.installed_data);
        assert!(first.shared_user_id);
        assert!(first.permission);
        assert!(first.rollback);
        assert!(first.auth);
    }

    #[test]
    fn proof_of_rotation_unknown_flags_are_preserved_without_invalidating_lineage() {
        let proof = proof_rotation_fixture();
        let (_, current_certificate) = proof_rotation_certificates(&proof);
        let flags = PROOF_OF_ROTATION_FLAG_PERMISSION | 0x8000_0000;

        let proof = proof_rotation_with_flags(&proof, 1, flags);
        let result = validate_proof_of_rotation(&proof, &current_certificate);

        assert_eq!(result.state, CryptoVerificationState::Verified);
        assert_eq!(result.capabilities.len(), 2);
        let second = &result.capabilities[1];
        assert_eq!(second.flags, flags);
        assert_eq!(second.known_flags, PROOF_OF_ROTATION_FLAG_PERMISSION);
        assert_eq!(second.unknown_flags, 0x8000_0000);
        assert!(second.permission);
        assert!(!second.installed_data);
        assert!(!second.shared_user_id);
        assert!(!second.rollback);
        assert!(!second.auth);
    }

    #[test]
    fn proof_of_rotation_zero_flags_are_distinct_from_unknown_flags() {
        let proof = proof_rotation_fixture();
        let (_, current_certificate) = proof_rotation_certificates(&proof);

        let zero_flags = proof_rotation_with_flags(&proof, 0, 0);
        let zero_result = validate_proof_of_rotation(&zero_flags, &current_certificate);
        assert_eq!(zero_result.state, CryptoVerificationState::Verified);
        assert_eq!(zero_result.capabilities[0].known_flags, 0);
        assert_eq!(zero_result.capabilities[0].unknown_flags, 0);

        let unknown_flags = proof_rotation_with_flags(&proof, 0, 0x4000_0000);
        let unknown_result = validate_proof_of_rotation(&unknown_flags, &current_certificate);
        assert_eq!(unknown_result.state, CryptoVerificationState::Verified);
        assert_eq!(unknown_result.capabilities[0].known_flags, 0);
        assert_eq!(unknown_result.capabilities[0].unknown_flags, 0x4000_0000);
    }

    #[test]
    fn proof_of_rotation_capability_evidence_survives_lineage_signature_failure() {
        let proof = proof_rotation_fixture();
        let (_, current_certificate) = proof_rotation_certificates(&proof);
        let flags = PROOF_OF_ROTATION_FLAG_ROLLBACK | 0x2000_0000;
        let mut proof = proof_rotation_with_flags(&proof, 1, flags);
        *proof.last_mut().expect("fixture must not be empty") ^= 1;

        let result = validate_proof_of_rotation(&proof, &current_certificate);

        assert_eq!(result.state, CryptoVerificationState::Invalid);
        assert_eq!(result.capabilities.len(), 2);
        assert_eq!(result.capabilities[1].flags, flags);
        assert_eq!(result.capabilities[1].unknown_flags, 0x2000_0000);
    }

    #[test]
    fn verifies_valid_proof_of_rotation_fixture() {
        let proof = proof_rotation_fixture();
        let (_, current_certificate) = proof_rotation_certificates(&proof);

        let result = validate_proof_of_rotation(&proof, &current_certificate);

        assert_eq!(result.state, CryptoVerificationState::Verified);
        assert_eq!(result.level_count, 2);
        assert!(result.detail.contains("lineage verified"));
    }

    #[test]
    fn parses_proof_of_rotation_attribute_as_structured_evidence() {
        let proof = proof_rotation_fixture();
        let (_, current_certificate) = proof_rotation_certificates(&proof);

        let mut digest_entry = Vec::new();
        digest_entry.extend_from_slice(&0x0201_u32.to_le_bytes());
        digest_entry.extend_from_slice(&32_u32.to_le_bytes());
        digest_entry.extend_from_slice(&[0_u8; 32]);

        let digests = encode_sequence(&digest_entry);
        let mut certificates = Vec::new();
        certificates.extend_from_slice(&encode_sequence(&current_certificate));

        let mut attribute = Vec::new();
        attribute.extend_from_slice(&PROOF_OF_ROTATION_ATTR_ID.to_le_bytes());
        attribute.extend_from_slice(&proof);
        let attributes = encode_sequence(&encode_sequence(&attribute));

        let mut signed_data = Vec::new();
        signed_data.extend_from_slice(&encode_sequence(&digests));
        signed_data.extend_from_slice(&encode_sequence(&certificates));
        signed_data.extend_from_slice(&28_u32.to_le_bytes());
        signed_data.extend_from_slice(&35_u32.to_le_bytes());
        signed_data.extend_from_slice(&attributes);

        let parsed =
            parse_signed_data_v3(&signed_data, V3_BLOCK_ID).expect("v3 signed data should parse");
        let proof_info = parsed
            .proof_of_rotation
            .expect("proof-of-rotation evidence should be present");

        assert_eq!(proof_info.state, CryptoVerificationState::Verified);
        assert_eq!(proof_info.level_count, 2);
    }

    #[test]
    fn invalid_proof_of_rotation_signature_cannot_verify() {
        let mut proof = proof_rotation_fixture();
        let (_, current_certificate) = proof_rotation_certificates(&proof);
        *proof.last_mut().expect("fixture must not be empty") ^= 1;

        let result = validate_proof_of_rotation(&proof, &current_certificate);

        assert_eq!(result.state, CryptoVerificationState::Invalid);
        assert!(result.detail.contains("validation failed"));
    }

    #[test]
    fn malformed_proof_of_rotation_cannot_verify() {
        let proof = proof_rotation_fixture();
        let (_, current_certificate) = proof_rotation_certificates(&proof);

        let result = validate_proof_of_rotation(&proof[..proof.len() - 1], &current_certificate);

        assert_eq!(result.state, CryptoVerificationState::Invalid);
        assert!(result.detail.contains("malformed"));
    }

    #[test]
    fn proof_of_rotation_final_certificate_must_match_current_signer() {
        let proof = proof_rotation_fixture();
        let (first_certificate, _) = proof_rotation_certificates(&proof);

        let result = validate_proof_of_rotation(&proof, &first_certificate);

        assert_eq!(result.state, CryptoVerificationState::Invalid);
        assert!(result
            .detail
            .contains("final proof-of-rotation certificate does not match"));
    }

    #[test]
    fn merges_verified_and_failed_v3_signers_without_losing_evidence() {
        let result = merge_scheme_results(
            "v3",
            vec![
                CryptoSchemeInfo {
                    state: CryptoVerificationState::Verified,
                    signer_count: 1,
                    algorithms: vec![0x0201],
                    certificate_sha256: vec!["a".repeat(64)],
                    sdk_ranges: vec![(28, 32)],
                    rotation_min_sdk: None,
                    rotation_targets_dev_release: false,
                    proof_of_rotation: vec![ProofOfRotationInfo {
                        state: CryptoVerificationState::Verified,
                        level_count: 2,
                        lineage_certificate_sha256: Vec::new(),
                        capabilities: Vec::new(),
                        detail: "proof-of-rotation lineage verified across 2 certificate level(s)"
                            .to_string(),
                    }],
                    detail: "targeted signer A".to_string(),
                },
                CryptoSchemeInfo {
                    state: CryptoVerificationState::Invalid,
                    signer_count: 1,
                    algorithms: vec![0x0201],
                    certificate_sha256: vec!["b".repeat(64)],
                    sdk_ranges: vec![(33, 36)],
                    rotation_min_sdk: None,
                    rotation_targets_dev_release: false,
                    proof_of_rotation: Vec::new(),
                    detail: "targeted signer B failed verification".to_string(),
                },
            ],
        )
        .expect("mixed v3 signer results should merge");

        assert_eq!(result.state, CryptoVerificationState::Invalid);
        assert_eq!(result.signer_count, 2);
        assert_eq!(result.sdk_ranges, vec![(28, 32), (33, 36)]);
        assert_eq!(result.certificate_sha256.len(), 2);
        assert_eq!(result.proof_of_rotation.len(), 1);
        assert_eq!(result.proof_of_rotation[0].level_count, 2);
        assert!(result
            .detail
            .contains("targeted signer B failed verification"));
    }

    #[test]
    fn merges_multiple_v3_targeted_signers_and_retains_sdk_ranges() {
        let result = merge_scheme_results(
            "v3",
            vec![
                CryptoSchemeInfo {
                    state: CryptoVerificationState::Verified,
                    signer_count: 1,
                    algorithms: vec![0x0201],
                    certificate_sha256: vec!["a".repeat(64)],
                    sdk_ranges: vec![(28, 32)],
                    rotation_min_sdk: None,
                    rotation_targets_dev_release: false,
                    proof_of_rotation: Vec::new(),
                    detail: "targeted signer A".to_string(),
                },
                CryptoSchemeInfo {
                    state: CryptoVerificationState::Verified,
                    signer_count: 1,
                    algorithms: vec![0x0201],
                    certificate_sha256: vec!["b".repeat(64)],
                    sdk_ranges: vec![(33, 36)],
                    rotation_min_sdk: None,
                    rotation_targets_dev_release: false,
                    proof_of_rotation: Vec::new(),
                    detail: "targeted signer B".to_string(),
                },
            ],
        )
        .expect("multiple v3 signer results should merge");

        assert_eq!(result.state, CryptoVerificationState::Verified);
        assert_eq!(result.signer_count, 2);
        assert_eq!(result.sdk_ranges, vec![(28, 32), (33, 36)]);
        assert_eq!(result.certificate_sha256.len(), 2);
    }
}
