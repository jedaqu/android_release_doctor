use zip::CompressionMethod;

pub const ZIP_ALIGNMENT_16KB: u64 = 16 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeZipCompression {
    Stored,
    Compressed,
}

impl NativeZipCompression {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stored => "stored",
            Self::Compressed => "compressed",
        }
    }
}

pub fn classify_compression(method: CompressionMethod) -> NativeZipCompression {
    match method {
        CompressionMethod::Stored => NativeZipCompression::Stored,
        _ => NativeZipCompression::Compressed,
    }
}

pub fn is_16kb_aligned(data_offset: u64) -> bool {
    data_offset % ZIP_ALIGNMENT_16KB == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stored_is_classified_as_uncompressed() {
        assert_eq!(
            classify_compression(CompressionMethod::Stored),
            NativeZipCompression::Stored
        );
    }

    #[test]
    fn non_stored_methods_are_classified_as_compressed() {
        assert_eq!(
            classify_compression(CompressionMethod::Deflated),
            NativeZipCompression::Compressed
        );
    }

    #[test]
    fn sixteen_kib_alignment_is_checked_against_data_offset() {
        assert!(is_16kb_aligned(0));
        assert!(is_16kb_aligned(16 * 1024));
        assert!(is_16kb_aligned(32 * 1024));
        assert!(!is_16kb_aligned(4 * 1024));
        assert!(!is_16kb_aligned(16 * 1024 + 1));
    }
}
