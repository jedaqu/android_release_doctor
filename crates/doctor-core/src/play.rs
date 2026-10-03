use crate::{ArtifactInventory, Finding, ManifestInfo};

pub const PLAY_POLICY_VERSION: &str = "2026-08-31";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayPlatform {
    Mobile,
    Wear,
    Automotive,
    Tv,
    Xr,
}

impl PlayPlatform {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "mobile" => Some(Self::Mobile),
            "wear" | "wear-os" => Some(Self::Wear),
            "automotive" | "android-automotive" => Some(Self::Automotive),
            "tv" | "android-tv" => Some(Self::Tv),
            "xr" | "android-xr" => Some(Self::Xr),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mobile => "mobile",
            Self::Wear => "wear",
            Self::Automotive => "automotive",
            Self::Tv => "tv",
            Self::Xr => "xr",
        }
    }

    pub fn required_target_sdk(self) -> u32 {
        match self {
            Self::Mobile => 36,
            Self::Wear | Self::Automotive => 35,
            Self::Tv | Self::Xr => 34,
        }
    }
}

fn is_16kb_64_bit_abi(abi: &str) -> bool {
    matches!(abi, "arm64-v8a" | "x86_64")
}

pub fn evaluate_play_policy(
    manifest: Option<&ManifestInfo>,
    inventory: &ArtifactInventory,
    platform: PlayPlatform,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    findings.push(Finding::pass(
        "PLAY-000",
        "Google Play policy profile",
        format!(
            "Using the Google Play submission policy profile {} for platform {}.",
            PLAY_POLICY_VERSION,
            platform.as_str()
        ),
    ));

    match manifest.and_then(|value| value.target_sdk) {
        Some(target_sdk) if target_sdk >= platform.required_target_sdk() => {
            findings.push(Finding::pass(
                "PLAY-001",
                "Target API level",
                format!(
                    "Artifact targetSdk {target_sdk} meets the {} requirement of API {}.",
                    platform.as_str(),
                    platform.required_target_sdk()
                ),
            ));
        }
        Some(target_sdk) => findings.push(Finding::blocker(
            "PLAY-001",
            "Target API level too low",
            format!(
                "Artifact targetSdk {target_sdk} is below the {} submission requirement of API {}.",
                platform.as_str(),
                platform.required_target_sdk()
            ),
            format!(
                "Raise targetSdk to at least API {} and rebuild the release artifact.",
                platform.required_target_sdk()
            ),
        )),
        None => findings.push(Finding::blocker(
            "PLAY-001",
            "Target API level unavailable",
            "The final artifact does not expose a comparable integer targetSdk value.",
            "Build a release artifact with a verifiable targetSdkVersion before Play submission.",
        )),
    }

    findings.push(Finding::manual_review(
        "PLAY-002",
        "Data Safety declaration",
        "Google Play requires the Data Safety form for published apps, including apps that do not collect user data.",
        "Review and submit the Data Safety declaration in Play Console so it accurately matches the app and its third-party SDK behavior.",
    ));

    findings.push(Finding::manual_review(
        "PLAY-003",
        "Privacy policy",
        "A valid privacy policy is a Play Console requirement for apps covered by the applicable privacy/data requirements; the artifact alone cannot prove its availability or correctness.",
        "Verify that the privacy policy is active, applies to this app, and is linked wherever Play requires it.",
    ));

    findings.push(Finding::manual_review(
        "PLAY-004",
        "Play Console content declarations",
        "Play Console requires release metadata and declarations that are not contained in the APK/AAB, such as ads and app-content information.",
        "Review App content in Play Console, including the ads declaration, app access instructions when applicable, content rating, and audience declarations.",
    ));

    if inventory.native_libraries.is_empty() {
        findings.push(Finding::pass(
            "PLAY-005",
            "16 KB native payload check",
            "No packaged native .so libraries were detected, so there is no native payload requiring ELF or ZIP 16 KB alignment verification.",
        ));
    } else if !inventory
        .native_libraries
        .iter()
        .any(|library| is_16kb_64_bit_abi(&library.abi))
    {
        findings.push(Finding::pass(
            "PLAY-005",
            "16 KB native payload check",
            "Native libraries are present only for 32-bit ABIs; the Google Play 16 KB compatibility requirement evaluated here applies to 64-bit device ABIs arm64-v8a and x86_64.",
        ));
    } else {
        let target_sdk = manifest.and_then(|value| value.target_sdk);
        let applicable_native_libraries = inventory
            .native_libraries
            .iter()
            .filter(|library| is_16kb_64_bit_abi(&library.abi))
            .collect::<Vec<_>>();
        let applicable_native_zip_entries = inventory
            .native_zip_entries
            .iter()
            .filter(|entry| is_16kb_64_bit_abi(&entry.abi))
            .collect::<Vec<_>>();
        let parse_errors = applicable_native_libraries
            .iter()
            .filter(|library| library.error.is_some())
            .collect::<Vec<_>>();
        let incompatible_elf = applicable_native_libraries
            .iter()
            .filter(|library| {
                !crate::load_segments_are_16kb_aligned(&library.load_segment_alignments)
            })
            .collect::<Vec<_>>();
        let zip_errors = applicable_native_zip_entries
            .iter()
            .filter(|entry| entry.error.is_some())
            .collect::<Vec<_>>();
        let incompatible_zip = applicable_native_zip_entries
            .iter()
            .filter(|entry| entry.alignment_16kb == Some(false))
            .collect::<Vec<_>>();

        if !parse_errors.is_empty() || !zip_errors.is_empty() {
            let mut details = Vec::new();

            if !parse_errors.is_empty() {
                details.push(format!(
                    "ELF inspection unavailable for: {}",
                    parse_errors
                        .iter()
                        .map(|library| library.path.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }

            if !zip_errors.is_empty() {
                details.push(format!(
                    "ZIP packaging verification unavailable for: {}",
                    zip_errors
                        .iter()
                        .map(|entry| {
                            format!(
                                "{} [{}]: {}",
                                entry.path,
                                entry.abi,
                                entry
                                    .error
                                    .as_deref()
                                    .unwrap_or("unknown ZIP alignment state")
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("; ")
                ));
            }

            findings.push(Finding::manual_review(
                "PLAY-005",
                "16 KB page-size compatibility unavailable",
                details.join(" "),
                "Verify every packaged native library with ELF-aware tooling and, for uncompressed APK libraries, zipalign. For AABs, verify the bundle ZIP alignment configuration and the APK generated from the bundle.",
            ));
        } else if let Some(target_sdk) = target_sdk.filter(|value| *value >= 35) {
            if !incompatible_elf.is_empty() || !incompatible_zip.is_empty() {
                let mut details = Vec::new();

                if !incompatible_elf.is_empty() {
                    details.push(format!(
                        "ELF alignment below 16 KB: {}",
                        incompatible_elf
                            .iter()
                            .map(|library| {
                                format!(
                                    "{} [{}] alignments={:?}",
                                    library.path, library.abi, library.load_segment_alignments
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("; ")
                    ));
                }

                if !incompatible_zip.is_empty() {
                    details.push(format!(
                        "ZIP data offset below 16 KB alignment: {}",
                        incompatible_zip
                            .iter()
                            .map(|entry| {
                                format!(
                                    "{} [{}] offset={:?}",
                                    entry.path, entry.abi, entry.data_offset
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("; ")
                    ));
                }

                findings.push(Finding::blocker(
                    "PLAY-005",
                    "16 KB page-size compatibility",
                    format!(
                        "The artifact targets API {target_sdk} and contains native payload packaging that fails the 16 KB alignment checks: {}.",
                        details.join(" ")
                    ),
                    "Rebuild or replace the affected native libraries and repackage the artifact so both ELF load segments and required ZIP data offsets meet the 16 KB requirements before Play submission.",
                ));
            } else {
                findings.push(Finding::pass(
                    "PLAY-005",
                    "16 KB page-size compatibility",
                    "All inspected native ELF PT_LOAD segments meet the 16 KB threshold and all verifiable uncompressed APK native libraries have 16 KB-aligned ZIP data offsets.",
                ));
            }
        } else if !incompatible_elf.is_empty() || !incompatible_zip.is_empty() {
            let mut details = Vec::new();

            if !incompatible_elf.is_empty() {
                details.push(format!(
                    "ELF alignment below 16 KB: {}",
                    incompatible_elf
                        .iter()
                        .map(|library| {
                            format!(
                                "{} [{}] alignments={:?}",
                                library.path, library.abi, library.load_segment_alignments
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("; ")
                ));
            }

            if !incompatible_zip.is_empty() {
                details.push(format!(
                    "ZIP data offset below 16 KB alignment: {}",
                    incompatible_zip
                        .iter()
                        .map(|entry| {
                            format!(
                                "{} [{}] offset={:?}",
                                entry.path, entry.abi, entry.data_offset
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("; ")
                ));
            }

            findings.push(Finding::warning(
                "PLAY-005",
                "Native payload alignment below 16 KB",
                format!(
                    "The artifact contains native payload alignment below the 16 KB threshold: {}.",
                    details.join(" ")
                ),
                "Review the native libraries against the applicable Android/Google Play 16 KB guidance before publication.",
            ));
        } else {
            findings.push(Finding::pass(
                "PLAY-005",
                "16 KB page-size compatibility",
                "All inspected native ELF PT_LOAD segments meet the 16 KB threshold and all verifiable native ZIP packaging alignment checks pass.",
            ));
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ArtifactInventory, ManifestInfo, Severity};

    fn manifest(target_sdk: u32) -> ManifestInfo {
        ManifestInfo {
            target_sdk: Some(target_sdk),
            ..Default::default()
        }
    }

    #[test]
    fn platform_target_requirements_match_current_play_matrix() {
        assert_eq!(PlayPlatform::Mobile.required_target_sdk(), 36);
        assert_eq!(PlayPlatform::Wear.required_target_sdk(), 35);
        assert_eq!(PlayPlatform::Automotive.required_target_sdk(), 35);
        assert_eq!(PlayPlatform::Tv.required_target_sdk(), 34);
        assert_eq!(PlayPlatform::Xr.required_target_sdk(), 34);
    }

    #[test]
    fn mobile_api_36_passes() {
        let findings = evaluate_play_policy(
            Some(&manifest(36)),
            &ArtifactInventory::default(),
            PlayPlatform::Mobile,
        );

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "PLAY-001")
                .map(|finding| finding.severity),
            Some(Severity::Pass)
        );
    }

    #[test]
    fn mobile_api_35_is_a_blocker() {
        let findings = evaluate_play_policy(
            Some(&manifest(35)),
            &ArtifactInventory::default(),
            PlayPlatform::Mobile,
        );

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "PLAY-001")
                .map(|finding| finding.severity),
            Some(Severity::Blocker)
        );
    }

    #[test]
    fn play_console_external_requirements_are_manual_review() {
        let findings = evaluate_play_policy(
            Some(&manifest(36)),
            &ArtifactInventory::default(),
            PlayPlatform::Mobile,
        );

        for rule_id in ["PLAY-002", "PLAY-003", "PLAY-004"] {
            assert_eq!(
                findings
                    .iter()
                    .find(|finding| finding.rule_id == rule_id)
                    .map(|finding| finding.severity),
                Some(Severity::ManualReview),
                "{rule_id} should require manual review"
            );
        }
    }

    #[test]
    fn misaligned_32_bit_native_payload_does_not_trigger_play_005() {
        let inventory = ArtifactInventory {
            native_abis: vec!["armeabi-v7a".to_string(), "x86".to_string()],
            native_libraries: vec![
                crate::NativeLibraryInfo {
                    path: "lib/armeabi-v7a/libbad.so".to_string(),
                    abi: "armeabi-v7a".to_string(),
                    load_segment_alignments: vec![4096],
                    error: None,
                },
                crate::NativeLibraryInfo {
                    path: "lib/x86/libbad.so".to_string(),
                    abi: "x86".to_string(),
                    load_segment_alignments: vec![4096],
                    error: None,
                },
            ],
            native_zip_entries: vec![
                crate::NativeZipEntryInfo {
                    path: "lib/armeabi-v7a/libbad.so".to_string(),
                    abi: "armeabi-v7a".to_string(),
                    compression: crate::NativeZipCompression::Stored,
                    data_offset: 4096,
                    alignment_16kb: Some(false),
                    error: None,
                },
                crate::NativeZipEntryInfo {
                    path: "lib/x86/libbad.so".to_string(),
                    abi: "x86".to_string(),
                    compression: crate::NativeZipCompression::Stored,
                    data_offset: 4096,
                    alignment_16kb: Some(false),
                    error: None,
                },
            ],
            ..Default::default()
        };

        let findings =
            evaluate_play_policy(Some(&manifest(36)), &inventory, PlayPlatform::Mobile);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "PLAY-005")
                .map(|finding| finding.severity),
            Some(Severity::Pass)
        );
    }

    #[test]
    fn native_payload_with_api_35_plus_and_bad_alignment_is_a_blocker() {
        let inventory = ArtifactInventory {
            native_abis: vec!["arm64-v8a".to_string()],
            native_libraries: vec![crate::NativeLibraryInfo {
                path: "lib/arm64-v8a/libbad.so".to_string(),
                abi: "arm64-v8a".to_string(),
                load_segment_alignments: vec![4096, 16384],
                error: None,
            }],
            ..Default::default()
        };

        let findings = evaluate_play_policy(Some(&manifest(35)), &inventory, PlayPlatform::Mobile);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "PLAY-005")
                .map(|finding| finding.severity),
            Some(Severity::Blocker)
        );
    }

    #[test]
    fn native_payload_with_api_35_plus_and_good_alignment_passes() {
        let inventory = ArtifactInventory {
            native_abis: vec!["arm64-v8a".to_string()],
            native_libraries: vec![crate::NativeLibraryInfo {
                path: "lib/arm64-v8a/libgood.so".to_string(),
                abi: "arm64-v8a".to_string(),
                load_segment_alignments: vec![16384, 32768],
                error: None,
            }],
            ..Default::default()
        };

        let findings = evaluate_play_policy(Some(&manifest(36)), &inventory, PlayPlatform::Mobile);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "PLAY-005")
                .map(|finding| finding.severity),
            Some(Severity::Pass)
        );
    }

    #[test]
    fn native_payload_with_misaligned_stored_zip_entry_is_a_blocker_for_api_35_plus() {
        let inventory = ArtifactInventory {
            native_abis: vec!["arm64-v8a".to_string()],
            native_libraries: vec![crate::NativeLibraryInfo {
                path: "lib/arm64-v8a/libbad.so".to_string(),
                abi: "arm64-v8a".to_string(),
                load_segment_alignments: vec![16384],
                error: None,
            }],
            native_zip_entries: vec![crate::NativeZipEntryInfo {
                path: "lib/arm64-v8a/libbad.so".to_string(),
                abi: "arm64-v8a".to_string(),
                compression: crate::NativeZipCompression::Stored,
                data_offset: 4096,
                alignment_16kb: Some(false),
                error: None,
            }],
            ..Default::default()
        };

        let findings = evaluate_play_policy(Some(&manifest(36)), &inventory, PlayPlatform::Mobile);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "PLAY-005")
                .map(|finding| finding.severity),
            Some(Severity::Blocker)
        );
    }

    #[test]
    fn aab_uncompressed_native_zip_alignment_stays_manual() {
        let inventory = ArtifactInventory {
            native_abis: vec!["arm64-v8a".to_string()],
            native_libraries: vec![crate::NativeLibraryInfo {
                path: "base/lib/arm64-v8a/libnative.so".to_string(),
                abi: "arm64-v8a".to_string(),
                load_segment_alignments: vec![16384],
                error: None,
            }],
            native_zip_entries: vec![crate::NativeZipEntryInfo {
                path: "base/lib/arm64-v8a/libnative.so".to_string(),
                abi: "arm64-v8a".to_string(),
                compression: crate::NativeZipCompression::Stored,
                data_offset: 16384,
                alignment_16kb: None,
                error: Some(
                    "AAB entry offset does not establish the final APK ZIP alignment".to_string(),
                ),
            }],
            ..Default::default()
        };

        let findings = evaluate_play_policy(Some(&manifest(36)), &inventory, PlayPlatform::Mobile);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "PLAY-005")
                .map(|finding| finding.severity),
            Some(Severity::ManualReview)
        );
    }

    #[test]
    fn native_payload_with_unreadable_elf_stays_manual() {
        let inventory = ArtifactInventory {
            native_abis: vec!["arm64-v8a".to_string()],
            native_libraries: vec![crate::NativeLibraryInfo {
                path: "lib/arm64-v8a/libunknown.so".to_string(),
                abi: "arm64-v8a".to_string(),
                load_segment_alignments: Vec::new(),
                error: Some("missing ELF magic".to_string()),
            }],
            ..Default::default()
        };

        let findings = evaluate_play_policy(Some(&manifest(36)), &inventory, PlayPlatform::Mobile);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "PLAY-005")
                .map(|finding| finding.severity),
            Some(Severity::ManualReview)
        );
    }
}
