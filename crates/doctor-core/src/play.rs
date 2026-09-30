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

    findings.push(Finding::warning(
        "PLAY-002",
        "Data Safety declaration",
        "Google Play requires the Data Safety form for published apps, including apps that do not collect user data.",
        "Review and submit the Data Safety declaration in Play Console so it accurately matches the app and its third-party SDK behavior.",
    ));

    findings.push(Finding::warning(
        "PLAY-003",
        "Privacy policy",
        "A valid privacy policy is a Play Console requirement for apps covered by the applicable privacy/data requirements; the artifact alone cannot prove its availability or correctness.",
        "Verify that the privacy policy is active, applies to this app, and is linked wherever Play requires it.",
    ));

    findings.push(Finding::warning(
        "PLAY-004",
        "Play Console content declarations",
        "Play Console requires release metadata and declarations that are not contained in the APK/AAB, such as ads and app-content information.",
        "Review App content in Play Console, including the ads declaration, app access instructions when applicable, content rating, and audience declarations.",
    ));

    if inventory.native_libraries.is_empty() {
        findings.push(Finding::pass(
            "PLAY-005",
            "16 KB native payload check",
            "No packaged native .so libraries were detected, so there is no native ELF payload in this artifact for the 16 KB page-alignment check.",
        ));
    } else {
        let target_sdk = manifest.and_then(|value| value.target_sdk);
        let parse_errors = inventory
            .native_libraries
            .iter()
            .filter(|library| library.error.is_some())
            .collect::<Vec<_>>();
        let incompatible = inventory
            .native_libraries
            .iter()
            .filter(|library| {
                crate::load_segments_are_16kb_aligned(&library.load_segment_alignments)
                    .not()
            })
            .collect::<Vec<_>>();

        if !parse_errors.is_empty() {
            let paths = parse_errors
                .iter()
                .map(|library| library.path.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            findings.push(Finding::warning(
                "PLAY-005",
                "16 KB page-size compatibility unavailable",
                format!(
                    "Native ELF inspection could not be completed for: {paths}."
                ),
                "Verify every packaged native library with an ELF-aware tool and Google's 16 KB guidance before release.",
            ));
        } else if target_sdk.is_some_and(|value| value >= 35) && !incompatible.is_empty() {
            let details = incompatible
                .iter()
                .map(|library| {
                    format!(
                        "{} [{}] alignments={:?}",
                        library.path, library.abi, library.load_segment_alignments
                    )
                })
                .collect::<Vec<_>>()
                .join("; ");

            findings.push(Finding::blocker(
                "PLAY-005",
                "16 KB page-size compatibility",
                format!(
                    "The artifact targets API {} and contains native ELF load segments below the 16 KB alignment threshold: {details}.",
                    target_sdk.unwrap()
                ),
                "Rebuild or replace the affected native libraries with 16 KB ELF load-segment alignment before Play submission.",
            ));
        } else if !incompatible.is_empty() {
            let details = incompatible
                .iter()
                .map(|library| {
                    format!(
                        "{} [{}] alignments={:?}",
                        library.path, library.abi, library.load_segment_alignments
                    )
                })
                .collect::<Vec<_>>()
                .join("; ");

            findings.push(Finding::warning(
                "PLAY-005",
                "Native ELF alignment below 16 KB",
                format!(
                    "The artifact contains native ELF load segments below the 16 KB alignment threshold: {details}."
                ),
                "Review the native libraries against the applicable Android/Google Play 16 KB guidance before publication.",
            ));
        } else {
            findings.push(Finding::pass(
                "PLAY-005",
                "16 KB page-size compatibility",
                if target_sdk.is_some_and(|value| value >= 35) {
                    "All inspected native ELF PT_LOAD segments meet the 16 KB alignment threshold required for API 35+ targets."
                } else {
                    "All inspected native ELF PT_LOAD segments meet the 16 KB alignment threshold."
                },
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
    fn native_payload_with_api_35_plus_requires_manual_16kb_review() {
        let inventory = ArtifactInventory {
            native_abis: vec!["arm64-v8a".to_string()],
            ..Default::default()
        };

        let findings = evaluate_play_policy(Some(&manifest(35)), &inventory, PlayPlatform::Mobile);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "PLAY-005")
                .map(|finding| finding.severity),
            Some(Severity::Warning)
        );
    }
}
