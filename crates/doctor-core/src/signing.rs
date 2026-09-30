use std::{
    fmt,
    io::{self, Read, Seek, SeekFrom},
};

const EOCD_SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x05, 0x06];
const APK_SIG_BLOCK_MAGIC: [u8; 16] = *b"APK Sig Block 42";
const V2_BLOCK_ID: u32 = 0x7109_871a;
const V3_BLOCK_ID: u32 = 0xf053_68c0;
const V31_BLOCK_ID: u32 = 0x1b93_ad61;
const V32_BLOCK_ID: u32 = 0x70e1_c89f;
const EOCD_LEN: u64 = 22;
const MAX_EOCD_COMMENT: u64 = 65_535;
const SIGNING_BLOCK_FOOTER_LEN: u64 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ApkSigningInfo {
    pub v2: bool,
    pub v3: bool,
    pub v31: bool,
    pub v32: bool,
    pub block_size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApkSigningBlock {
    pub block_start: u64,
    pub central_directory_offset: u64,
    pub central_directory_size: u64,
    pub eocd_offset: u64,
    pub eocd: Vec<u8>,
    pub pairs: Vec<(u32, Vec<u8>)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigningBlockError(String);

impl fmt::Display for SigningBlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SigningBlockError {}

pub fn read_apk_signing_block<R: Read + Seek>(
    reader: &mut R,
) -> Result<Option<ApkSigningBlock>, SigningBlockError> {
    let file_len = reader.seek(SeekFrom::End(0)).map_err(io_error)?;

    if file_len < EOCD_LEN {
        return Err(SigningBlockError(
            "APK is shorter than the ZIP end-of-central-directory record".to_string(),
        ));
    }

    let scan_len = file_len.min(EOCD_LEN + MAX_EOCD_COMMENT);
    let scan_start = file_len - scan_len;
    reader.seek(SeekFrom::Start(scan_start)).map_err(io_error)?;

    let mut tail = vec![0_u8; scan_len as usize];
    reader.read_exact(&mut tail).map_err(io_error)?;

    let eocd_relative = find_eocd(&tail).ok_or_else(|| {
        SigningBlockError("ZIP end-of-central-directory record was not found".to_string())
    })?;
    let eocd_offset = scan_start + eocd_relative as u64;

    let eocd_end = eocd_relative
        + EOCD_LEN as usize
        + u16::from_le_bytes(
            tail[eocd_relative + 20..eocd_relative + 22]
                .try_into()
                .expect("EOCD comment length is two bytes"),
        ) as usize;
    if eocd_end != tail.len() {
        return Err(SigningBlockError(
            "ZIP End of Central Directory is not the final record in the APK".to_string(),
        ));
    }

    let central_directory_size = u32::from_le_bytes(
        tail[eocd_relative + 12..eocd_relative + 16]
            .try_into()
            .unwrap(),
    );
    let central_directory_offset = u32::from_le_bytes(
        tail[eocd_relative + 16..eocd_relative + 20]
            .try_into()
            .unwrap(),
    );

    if central_directory_offset == u32::MAX || central_directory_size == u32::MAX {
        return Err(SigningBlockError(
            "ZIP64 central-directory fields are not supported for APK signing verification"
                .to_string(),
        ));
    }

    let central_directory_offset = central_directory_offset as u64;
    let central_directory_size = central_directory_size as u64;

    if central_directory_offset
        .checked_add(central_directory_size)
        .ok_or_else(|| SigningBlockError("ZIP central-directory bounds overflowed".to_string()))?
        != eocd_offset
    {
        return Err(SigningBlockError(
            "ZIP Central Directory is not immediately followed by the End of Central Directory"
                .to_string(),
        ));
    }

    if central_directory_offset < SIGNING_BLOCK_FOOTER_LEN {
        return Err(SigningBlockError(
            "APK central-directory offset is before the signing-block footer".to_string(),
        ));
    }

    reader
        .seek(SeekFrom::Start(
            central_directory_offset - SIGNING_BLOCK_FOOTER_LEN,
        ))
        .map_err(io_error)?;

    let mut footer = [0_u8; SIGNING_BLOCK_FOOTER_LEN as usize];
    reader.read_exact(&mut footer).map_err(io_error)?;

    if footer[8..] != APK_SIG_BLOCK_MAGIC {
        return Ok(None);
    }

    let block_size = u64::from_le_bytes(footer[0..8].try_into().unwrap());
    if block_size < SIGNING_BLOCK_FOOTER_LEN {
        return Err(SigningBlockError(
            "APK signing block declares an invalid size".to_string(),
        ));
    }

    let total_block_size = block_size
        .checked_add(8)
        .ok_or_else(|| SigningBlockError("APK signing block size overflowed".to_string()))?;

    if total_block_size > central_directory_offset {
        return Err(SigningBlockError(
            "APK signing block starts before the beginning of the file".to_string(),
        ));
    }

    let block_start = central_directory_offset - total_block_size;

    reader
        .seek(SeekFrom::Start(block_start))
        .map_err(io_error)?;

    let mut leading_size = [0_u8; 8];
    reader.read_exact(&mut leading_size).map_err(io_error)?;
    let leading_size = u64::from_le_bytes(leading_size);

    if leading_size != block_size {
        return Err(SigningBlockError(
            "APK signing block leading and trailing size fields differ".to_string(),
        ));
    }

    let pairs_start = block_start + 8;
    let pairs_end = central_directory_offset - SIGNING_BLOCK_FOOTER_LEN;
    let mut cursor = pairs_start;
    let mut pairs = Vec::new();

    while cursor < pairs_end {
        if pairs_end - cursor < 8 {
            return Err(SigningBlockError(
                "APK signing block contains a truncated ID-value pair length".to_string(),
            ));
        }

        reader.seek(SeekFrom::Start(cursor)).map_err(io_error)?;
        let mut pair_size_bytes = [0_u8; 8];
        reader.read_exact(&mut pair_size_bytes).map_err(io_error)?;
        let pair_size = u64::from_le_bytes(pair_size_bytes);

        if pair_size < 4 || pair_size > pairs_end - cursor - 8 {
            return Err(SigningBlockError(
                "APK signing block contains an invalid ID-value pair length".to_string(),
            ));
        }

        let mut id_bytes = [0_u8; 4];
        reader.read_exact(&mut id_bytes).map_err(io_error)?;
        let id = u32::from_le_bytes(id_bytes);

        let value_len = usize::try_from(pair_size - 4).map_err(|_| {
            SigningBlockError("APK signing-block pair value is too large".to_string())
        })?;
        let mut value = vec![0_u8; value_len];
        reader.read_exact(&mut value).map_err(io_error)?;
        pairs.push((id, value));

        cursor += 8 + pair_size;
    }

    if cursor != pairs_end {
        return Err(SigningBlockError(
            "APK signing block ID-value pairs do not consume the full block".to_string(),
        ));
    }

    let eocd_len = file_len
        .checked_sub(eocd_offset)
        .ok_or_else(|| SigningBlockError("EOCD offset is outside the APK".to_string()))?;
    let eocd_len = usize::try_from(eocd_len)
        .map_err(|_| SigningBlockError("EOCD record is too large".to_string()))?;
    reader
        .seek(SeekFrom::Start(eocd_offset))
        .map_err(io_error)?;
    let mut eocd = vec![0_u8; eocd_len];
    reader.read_exact(&mut eocd).map_err(io_error)?;

    Ok(Some(ApkSigningBlock {
        block_start,
        central_directory_offset,
        central_directory_size,
        eocd_offset,
        eocd,
        pairs,
    }))
}

pub fn inspect_apk_signing_block<R: Read + Seek>(
    reader: &mut R,
) -> Result<Option<ApkSigningInfo>, SigningBlockError> {
    let Some(block) = read_apk_signing_block(reader)? else {
        return Ok(None);
    };

    let mut info = ApkSigningInfo {
        block_size: block.central_directory_offset - block.block_start,
        ..Default::default()
    };

    for (id, _) in &block.pairs {
        match *id {
            V2_BLOCK_ID => info.v2 = true,
            V3_BLOCK_ID => info.v3 = true,
            V31_BLOCK_ID => info.v31 = true,
            V32_BLOCK_ID => info.v32 = true,
            _ => {}
        }
    }

    Ok(Some(info))
}

fn find_eocd(tail: &[u8]) -> Option<usize> {
    if tail.len() < EOCD_LEN as usize {
        return None;
    }

    for offset in (0..=tail.len() - EOCD_LEN as usize).rev() {
        if tail.get(offset..offset + 4)? != EOCD_SIGNATURE {
            continue;
        }

        let comment_len =
            u16::from_le_bytes(tail[offset + 20..offset + 22].try_into().ok()?) as usize;

        if offset + EOCD_LEN as usize + comment_len == tail.len() {
            return Some(offset);
        }
    }

    None
}

fn io_error(error: io::Error) -> SigningBlockError {
    SigningBlockError(format!(
        "I/O error while reading APK signing block: {error}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn synthetic_apk(
        with_v2: bool,
        with_v3: bool,
        with_v31: bool,
        with_v32: bool,
        malformed_size: bool,
    ) -> Vec<u8> {
        let mut pairs = Vec::new();

        for id in [
            with_v2.then_some(V2_BLOCK_ID),
            with_v3.then_some(V3_BLOCK_ID),
            with_v31.then_some(V31_BLOCK_ID),
            with_v32.then_some(V32_BLOCK_ID),
        ]
        .into_iter()
        .flatten()
        {
            let pair_size = 4_u64;
            pairs.extend_from_slice(&pair_size.to_le_bytes());
            pairs.extend_from_slice(&id.to_le_bytes());
        }

        let block_size = SIGNING_BLOCK_FOOTER_LEN + pairs.len() as u64;
        let stored_block_size = if malformed_size {
            block_size + 8
        } else {
            block_size
        };

        let mut apk = vec![0x41; 128];
        let central_directory_offset = apk.len() as u64 + 8 + block_size;

        apk.extend_from_slice(&stored_block_size.to_le_bytes());
        apk.extend_from_slice(&pairs);
        apk.extend_from_slice(&block_size.to_le_bytes());
        apk.extend_from_slice(&APK_SIG_BLOCK_MAGIC);

        let eocd_start = apk.len();
        apk.extend_from_slice(&EOCD_SIGNATURE);
        apk.extend_from_slice(&0_u16.to_le_bytes()); // disk number
        apk.extend_from_slice(&0_u16.to_le_bytes()); // central-directory disk
        apk.extend_from_slice(&1_u16.to_le_bytes()); // entries on disk
        apk.extend_from_slice(&1_u16.to_le_bytes()); // total entries
        apk.extend_from_slice(&0_u32.to_le_bytes()); // central-directory size
        apk.extend_from_slice(&(central_directory_offset as u32).to_le_bytes());
        apk.extend_from_slice(&0_u16.to_le_bytes()); // comment length

        assert_eq!(apk.len(), eocd_start + 22);
        apk
    }

    #[test]
    fn detects_v2_and_v3_signing_blocks() {
        let bytes = synthetic_apk(true, true, true, true, false);
        let mut cursor = Cursor::new(bytes);

        let info = inspect_apk_signing_block(&mut cursor)
            .expect("synthetic APK should parse")
            .expect("signing block should be present");

        assert!(info.v2);
        assert!(info.v3);
        assert!(info.v31);
        assert!(info.v32);
        assert_eq!(info.block_size, 8 + SIGNING_BLOCK_FOOTER_LEN + 12 * 4);
    }

    #[test]
    fn missing_signing_block_is_not_an_error() {
        let mut bytes = vec![0x41; 64];
        let central_directory_offset = bytes.len() as u32;
        bytes.extend_from_slice(&EOCD_SIGNATURE);
        bytes.extend_from_slice(&0_u16.to_le_bytes()); // disk number
        bytes.extend_from_slice(&0_u16.to_le_bytes()); // central-directory disk
        bytes.extend_from_slice(&0_u16.to_le_bytes()); // entries on disk
        bytes.extend_from_slice(&0_u16.to_le_bytes()); // total entries
        bytes.extend_from_slice(&0_u32.to_le_bytes()); // central-directory size
        bytes.extend_from_slice(&central_directory_offset.to_le_bytes());
        bytes.extend_from_slice(&0_u16.to_le_bytes()); // comment length

        let mut cursor = Cursor::new(bytes);
        assert_eq!(
            inspect_apk_signing_block(&mut cursor).expect("plain APK-like ZIP should parse"),
            None
        );
    }

    #[test]
    fn rejects_mismatched_signing_block_sizes() {
        let bytes = synthetic_apk(true, false, true, false, true);
        let mut cursor = Cursor::new(bytes);

        let error = inspect_apk_signing_block(&mut cursor)
            .expect_err("mismatched signing block sizes should fail");

        assert!(error
            .to_string()
            .contains("leading and trailing size fields differ"));
    }
}
