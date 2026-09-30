use std::fmt;

const PT_LOAD: u32 = 1;
const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const PAGE_ALIGN_16KB: u64 = 16 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfInspection {
    pub load_segment_alignments: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfError(String);

impl fmt::Display for ElfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ElfError {}

pub fn inspect_shared_object(bytes: &[u8]) -> Result<ElfInspection, ElfError> {
    if bytes.len() < 16 || bytes[0..4] != ELF_MAGIC {
        return Err(ElfError("missing ELF magic".to_string()));
    }

    let class = bytes[4];
    if bytes[5] != ELFDATA2LSB {
        return Err(ElfError(
            "big-endian ELF shared objects are not supported".to_string(),
        ));
    }

    let (phoff, phentsize, phnum, min_phentsize) = match class {
        ELFCLASS32 => (
            read_u32(bytes, 28)? as u64,
            read_u16(bytes, 42)? as u64,
            read_u16(bytes, 44)? as u64,
            32_u64,
        ),
        ELFCLASS64 => (
            read_u64(bytes, 32)?,
            read_u16(bytes, 54)? as u64,
            read_u16(bytes, 56)? as u64,
            56_u64,
        ),
        _ => {
            return Err(ElfError(format!(
                "unsupported ELF class value {class}"
            )))
        }
    };

    if phnum == 0 {
        return Err(ElfError("ELF contains no program headers".to_string()));
    }

    if phentsize < min_phentsize {
        return Err(ElfError(format!(
            "ELF program-header entry size {phentsize} is smaller than the expected minimum {min_phentsize}"
        )));
    }

    let table_size = phentsize
        .checked_mul(phnum)
        .ok_or_else(|| ElfError("ELF program-header table size overflowed".to_string()))?;
    let table_end = phoff
        .checked_add(table_size)
        .ok_or_else(|| ElfError("ELF program-header table end overflowed".to_string()))?;

    if table_end > bytes.len() as u64 {
        return Err(ElfError(
            "ELF program-header table extends past the shared object".to_string(),
        ));
    }

    let mut load_segment_alignments = Vec::new();

    for index in 0..phnum {
        let offset = phoff + index * phentsize;
        let p_type = read_u32(bytes, offset as usize)?;

        if p_type != PT_LOAD {
            continue;
        }

        let p_align_offset = match class {
            ELFCLASS32 => offset
                .checked_add(28)
                .ok_or_else(|| ElfError("ELF32 p_align offset overflowed".to_string()))?,
            ELFCLASS64 => offset
                .checked_add(48)
                .ok_or_else(|| ElfError("ELF64 p_align offset overflowed".to_string()))?,
            _ => unreachable!(),
        };

        let p_align = if class == ELFCLASS32 {
            read_u32(bytes, p_align_offset as usize)? as u64
        } else {
            read_u64(bytes, p_align_offset as usize)?
        };

        load_segment_alignments.push(p_align);
    }

    if load_segment_alignments.is_empty() {
        return Err(ElfError(
            "ELF contains no PT_LOAD program segments".to_string(),
        ));
    }

    Ok(ElfInspection {
        load_segment_alignments,
    })
}

pub fn load_segments_are_16kb_aligned(alignments: &[u64]) -> bool {
    !alignments.is_empty()
        && alignments
            .iter()
            .all(|alignment| *alignment >= PAGE_ALIGN_16KB)
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, ElfError> {
    let slice = bytes
        .get(offset..offset + 2)
        .ok_or_else(|| ElfError("truncated ELF field".to_string()))?;

    Ok(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, ElfError> {
    let slice = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| ElfError("truncated ELF field".to_string()))?;

    Ok(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, ElfError> {
    let slice = bytes
        .get(offset..offset + 8)
        .ok_or_else(|| ElfError("truncated ELF field".to_string()))?;

    Ok(u64::from_le_bytes([
        slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn elf64(alignment: u64) -> Vec<u8> {
        let mut bytes = vec![0_u8; 64 + 56];
        bytes[0..4].copy_from_slice(&ELF_MAGIC);
        bytes[4] = ELFCLASS64;
        bytes[5] = ELFDATA2LSB;
        bytes[32..40].copy_from_slice(&64_u64.to_le_bytes());
        bytes[54..56].copy_from_slice(&56_u16.to_le_bytes());
        bytes[56..58].copy_from_slice(&1_u16.to_le_bytes());

        let ph = 64;
        bytes[ph..ph + 4].copy_from_slice(&PT_LOAD.to_le_bytes());
        bytes[ph + 48..ph + 56].copy_from_slice(&alignment.to_le_bytes());
        bytes
    }

    fn elf32(alignment: u32) -> Vec<u8> {
        let mut bytes = vec![0_u8; 52 + 32];
        bytes[0..4].copy_from_slice(&ELF_MAGIC);
        bytes[4] = ELFCLASS32;
        bytes[5] = ELFDATA2LSB;
        bytes[28..32].copy_from_slice(&52_u32.to_le_bytes());
        bytes[42..44].copy_from_slice(&32_u16.to_le_bytes());
        bytes[44..46].copy_from_slice(&1_u16.to_le_bytes());

        let ph = 52;
        bytes[ph..ph + 4].copy_from_slice(&PT_LOAD.to_le_bytes());
        bytes[ph + 28..ph + 32].copy_from_slice(&alignment.to_le_bytes());
        bytes
    }

    #[test]
    fn parses_elf64_load_alignment() {
        let inspection = inspect_shared_object(&elf64(16 * 1024)).expect("valid ELF64");
        assert_eq!(inspection.load_segment_alignments, vec![16 * 1024]);
        assert!(load_segments_are_16kb_aligned(
            &inspection.load_segment_alignments
        ));
    }

    #[test]
    fn parses_elf32_load_alignment() {
        let inspection = inspect_shared_object(&elf32(8192)).expect("valid ELF32");
        assert_eq!(inspection.load_segment_alignments, vec![8192]);
        assert!(!load_segments_are_16kb_aligned(
            &inspection.load_segment_alignments
        ));
    }

    #[test]
    fn rejects_truncated_program_header_table() {
        let mut bytes = elf64(16 * 1024);
        bytes.truncate(64);

        let error = inspect_shared_object(&bytes).expect_err("truncated ELF should fail");
        assert!(
            error
                .to_string()
                .contains("program-header table extends past")
        );
    }

    #[test]
    fn rejects_elf_without_load_segments() {
        let mut bytes = elf64(16 * 1024);
        bytes[64..68].copy_from_slice(&2_u32.to_le_bytes());

        let error = inspect_shared_object(&bytes).expect_err("missing PT_LOAD should fail");
        assert!(
            error
                .to_string()
                .contains("no PT_LOAD program segments")
        );
    }
}
