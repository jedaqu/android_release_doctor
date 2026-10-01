use serde::Serialize;

use crate::signature_verify::{ProofOfRotationCapabilities, ProofOfRotationInfo};

use crate::{
    ApkSignatureVerification, ApkSigningInfo, ArtifactInventory, AuditReport, ComponentInfo,
    CryptoSchemeInfo, CryptoVerificationState, Finding, GradleSyntax, ManifestInfo,
    NativeLibraryInfo, NativeZipEntryInfo, PlayPlatform, ProjectInfo, Severity, ENGINE_VERSION,
    PLAY_POLICY_VERSION,
};

const REPORT_SCHEMA_VERSION: &str = "1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReportV1Context {
    pub play: Option<PlayPlatform>,
}

impl ReportV1Context {
    pub fn without_play() -> Self {
        Self { play: None }
    }

    pub fn with_play(platform: PlayPlatform) -> Self {
        Self {
            play: Some(platform),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ReportV1 {
    pub schema_version: String,
    pub engine_version: String,
    pub artifact: ArtifactV1,
    pub application: Option<ApplicationV1>,
    pub manifest_error: Option<String>,
    pub inventory: InventoryV1,
    pub project: Option<ProjectV1>,
    pub project_error: Option<String>,
    pub play: Option<PlayV1>,
    pub findings: Vec<FindingV1>,
    pub summary: SummaryV1,
}

impl ReportV1 {
    pub fn from_audit_report(report: &AuditReport, context: ReportV1Context) -> Self {
        let (passed, warnings, blockers) = report.counts();

        Self {
            schema_version: REPORT_SCHEMA_VERSION.to_string(),
            engine_version: ENGINE_VERSION.to_string(),
            artifact: ArtifactV1 {
                kind: report.artifact_kind.as_str().to_string(),
                path: report.artifact_path.display().to_string(),
                size_bytes: report.size_bytes,
            },
            application: report.manifest.as_ref().map(application_from_manifest),
            manifest_error: report.manifest_error.clone(),
            inventory: inventory_from_domain(&report.inventory),
            project: report.project.as_ref().map(project_from_domain),
            project_error: report.project_error.clone(),
            play: context.play.map(|platform| PlayV1 {
                platform: platform.as_str().to_string(),
                policy_version: PLAY_POLICY_VERSION.to_string(),
            }),
            findings: report.findings.iter().map(finding_from_domain).collect(),
            summary: SummaryV1 {
                passed,
                warnings,
                blockers,
                manual_review: report.manual_review_count(),
            },
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ArtifactV1 {
    pub kind: String,
    pub path: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplicationV1 {
    pub package_name: Option<String>,
    pub version_code: Option<u32>,
    pub version_name: Option<String>,
    pub min_sdk: Option<u32>,
    pub target_sdk: Option<u32>,
    pub debuggable: Option<bool>,
    pub permissions: Vec<String>,
    pub components: Vec<ComponentV1>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComponentV1 {
    pub kind: String,
    pub name: String,
    pub exported: Option<bool>,
    pub has_intent_filters: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct InventoryV1 {
    pub entries: Vec<String>,
    pub manifest_path: Option<String>,
    pub dex_files: Vec<String>,
    pub native_abis: Vec<String>,
    pub native_libraries: Vec<NativeLibraryV1>,
    pub native_zip_entries: Vec<NativeZipEntryV1>,
    pub signature_files: Vec<String>,
    pub apk_signing: Option<ApkSigningV1>,
    pub apk_signing_error: Option<String>,
    pub apk_signature_verification: Option<ApkSignatureVerificationV1>,
    pub apk_signature_verification_error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NativeLibraryV1 {
    pub path: String,
    pub abi: String,
    pub load_segment_alignments: Vec<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NativeZipEntryV1 {
    pub path: String,
    pub abi: String,
    pub compression: String,
    pub data_offset: u64,
    pub alignment_16kb: Option<bool>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApkSigningV1 {
    pub v2: bool,
    pub v3: bool,
    pub v31: bool,
    pub v32: bool,
    pub block_size: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApkSignatureVerificationV1 {
    pub v2: Option<CryptoSchemeV1>,
    pub v3: Option<CryptoSchemeV1>,
    pub v31: Option<CryptoSchemeV1>,
    pub v31_present: bool,
    pub v32_present: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CryptoSchemeV1 {
    pub state: String,
    pub signer_count: usize,
    pub algorithms: Vec<u32>,
    pub certificate_sha256: Vec<String>,
    pub sdk_ranges: Vec<[u32; 2]>,
    pub rotation_min_sdk: Option<u32>,
    pub rotation_targets_dev_release: bool,
    pub proof_of_rotation: Vec<ProofOfRotationV1>,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProofOfRotationV1 {
    pub state: String,
    pub level_count: usize,
    pub lineage_certificate_sha256: Vec<String>,
    pub capabilities: Vec<ProofOfRotationCapabilitiesV1>,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProofOfRotationCapabilitiesV1 {
    pub flags: u32,
    pub known_flags: u32,
    pub unknown_flags: u32,
    pub installed_data: bool,
    pub shared_user_id: bool,
    pub permission: bool,
    pub rollback: bool,
    pub auth: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectV1 {
    pub build_file: String,
    pub syntax: String,
    pub module_name: String,
    pub namespace: Option<String>,
    pub application_id: Option<String>,
    pub compile_sdk: Option<u32>,
    pub min_sdk: Option<u32>,
    pub target_sdk: Option<u32>,
    pub version_code: Option<u32>,
    pub version_name: Option<String>,
    pub release_debuggable: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayV1 {
    pub platform: String,
    pub policy_version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FindingV1 {
    pub rule_id: String,
    pub severity: String,
    pub title: String,
    pub summary: String,
    pub remediation: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SummaryV1 {
    pub passed: usize,
    pub warnings: usize,
    pub blockers: usize,
    pub manual_review: usize,
}

fn application_from_manifest(manifest: &ManifestInfo) -> ApplicationV1 {
    ApplicationV1 {
        package_name: manifest.package_name.clone(),
        version_code: manifest.version_code,
        version_name: manifest.version_name.clone(),
        min_sdk: manifest.min_sdk,
        target_sdk: manifest.target_sdk,
        debuggable: manifest.debuggable,
        permissions: manifest.permissions.clone(),
        components: manifest
            .components
            .iter()
            .map(component_from_domain)
            .collect(),
    }
}

fn component_from_domain(component: &ComponentInfo) -> ComponentV1 {
    ComponentV1 {
        kind: component.kind.clone(),
        name: component.name.clone(),
        exported: component.exported,
        has_intent_filters: component.has_intent_filters,
    }
}

fn inventory_from_domain(inventory: &ArtifactInventory) -> InventoryV1 {
    InventoryV1 {
        entries: inventory.entries.clone(),
        manifest_path: inventory.manifest_path.clone(),
        dex_files: inventory.dex_files.clone(),
        native_abis: inventory.native_abis.clone(),
        native_libraries: inventory
            .native_libraries
            .iter()
            .map(native_library_from_domain)
            .collect(),
        native_zip_entries: inventory
            .native_zip_entries
            .iter()
            .map(native_zip_entry_from_domain)
            .collect(),
        signature_files: inventory.signature_files.clone(),
        apk_signing: inventory.apk_signing.as_ref().map(apk_signing_from_domain),
        apk_signing_error: inventory.apk_signing_error.clone(),
        apk_signature_verification: inventory
            .apk_signature_verification
            .as_ref()
            .map(apk_signature_verification_from_domain),
        apk_signature_verification_error: inventory.apk_signature_verification_error.clone(),
    }
}

fn native_library_from_domain(library: &NativeLibraryInfo) -> NativeLibraryV1 {
    NativeLibraryV1 {
        path: library.path.clone(),
        abi: library.abi.clone(),
        load_segment_alignments: library.load_segment_alignments.clone(),
        error: library.error.clone(),
    }
}

fn native_zip_entry_from_domain(entry: &NativeZipEntryInfo) -> NativeZipEntryV1 {
    NativeZipEntryV1 {
        path: entry.path.clone(),
        abi: entry.abi.clone(),
        compression: match entry.compression {
            crate::NativeZipCompression::Stored => "stored",
            crate::NativeZipCompression::Compressed => "compressed",
        }
        .to_string(),
        data_offset: entry.data_offset,
        alignment_16kb: entry.alignment_16kb,
        error: entry.error.clone(),
    }
}

fn apk_signing_from_domain(info: &ApkSigningInfo) -> ApkSigningV1 {
    ApkSigningV1 {
        v2: info.v2,
        v3: info.v3,
        v31: info.v31,
        v32: info.v32,
        block_size: info.block_size,
    }
}

fn apk_signature_verification_from_domain(
    verification: &ApkSignatureVerification,
) -> ApkSignatureVerificationV1 {
    ApkSignatureVerificationV1 {
        v2: verification.v2.as_ref().map(crypto_scheme_from_domain),
        v3: verification.v3.as_ref().map(crypto_scheme_from_domain),
        v31: verification.v31.as_ref().map(crypto_scheme_from_domain),
        v31_present: verification.v31_present,
        v32_present: verification.v32_present,
    }
}

fn crypto_scheme_from_domain(info: &CryptoSchemeInfo) -> CryptoSchemeV1 {
    CryptoSchemeV1 {
        state: crypto_state_as_str(info.state).to_string(),
        signer_count: info.signer_count,
        algorithms: info.algorithms.clone(),
        certificate_sha256: info.certificate_sha256.clone(),
        sdk_ranges: info
            .sdk_ranges
            .iter()
            .map(|(minimum, maximum)| [*minimum, *maximum])
            .collect(),
        rotation_min_sdk: info.rotation_min_sdk,
        rotation_targets_dev_release: info.rotation_targets_dev_release,
        proof_of_rotation: info
            .proof_of_rotation
            .iter()
            .map(proof_of_rotation_from_domain)
            .collect(),
        detail: info.detail.clone(),
    }
}

fn proof_of_rotation_from_domain(info: &ProofOfRotationInfo) -> ProofOfRotationV1 {
    ProofOfRotationV1 {
        state: crypto_state_as_str(info.state).to_string(),
        level_count: info.level_count,
        lineage_certificate_sha256: info.lineage_certificate_sha256.clone(),
        capabilities: info
            .capabilities
            .iter()
            .map(capabilities_from_domain)
            .collect(),
        detail: info.detail.clone(),
    }
}

fn capabilities_from_domain(info: &ProofOfRotationCapabilities) -> ProofOfRotationCapabilitiesV1 {
    ProofOfRotationCapabilitiesV1 {
        flags: info.flags,
        known_flags: info.known_flags,
        unknown_flags: info.unknown_flags,
        installed_data: info.installed_data,
        shared_user_id: info.shared_user_id,
        permission: info.permission,
        rollback: info.rollback,
        auth: info.auth,
    }
}

fn project_from_domain(project: &ProjectInfo) -> ProjectV1 {
    ProjectV1 {
        build_file: project.build_file.display().to_string(),
        syntax: match project.syntax {
            GradleSyntax::Groovy => "Groovy",
            GradleSyntax::Kotlin => "Kotlin DSL",
        }
        .to_string(),
        module_name: project.module_name.clone(),
        namespace: project.namespace.clone(),
        application_id: project.application_id.clone(),
        compile_sdk: project.compile_sdk,
        min_sdk: project.min_sdk,
        target_sdk: project.target_sdk,
        version_code: project.version_code,
        version_name: project.version_name.clone(),
        release_debuggable: project.release_debuggable,
    }
}

fn finding_from_domain(finding: &Finding) -> FindingV1 {
    FindingV1 {
        rule_id: finding.rule_id.to_string(),
        severity: match finding.severity {
            Severity::Pass => "PASS",
            Severity::Warning => "WARNING",
            Severity::Blocker => "BLOCKER",
            Severity::ManualReview => "MANUAL-REVIEW",
        }
        .to_string(),
        title: finding.title.clone(),
        summary: finding.summary.clone(),
        remediation: finding.remediation.clone(),
    }
}

fn crypto_state_as_str(state: CryptoVerificationState) -> &'static str {
    match state {
        CryptoVerificationState::Verified => "verified",
        CryptoVerificationState::Invalid => "invalid",
        CryptoVerificationState::Unsupported => "unsupported",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ArtifactKind;
    use std::path::PathBuf;

    fn sample_report() -> AuditReport {
        AuditReport {
            artifact_path: PathBuf::from("fixtures/release/app.apk"),
            artifact_kind: ArtifactKind::Apk,
            size_bytes: 123_456,
            inventory: ArtifactInventory {
                entries: vec![
                    "AndroidManifest.xml".to_string(),
                    "classes.dex".to_string(),
                    "lib/arm64-v8a/libapp.so".to_string(),
                ],
                manifest_path: Some("AndroidManifest.xml".to_string()),
                dex_files: vec!["classes.dex".to_string()],
                native_abis: vec!["arm64-v8a".to_string()],
                native_libraries: vec![NativeLibraryInfo {
                    path: "lib/arm64-v8a/libapp.so".to_string(),
                    abi: "arm64-v8a".to_string(),
                    load_segment_alignments: vec![16_384, 32_768],
                    error: None,
                }],
                native_zip_entries: vec![NativeZipEntryInfo {
                    path: "lib/arm64-v8a/libapp.so".to_string(),
                    abi: "arm64-v8a".to_string(),
                    compression: crate::NativeZipCompression::Stored,
                    data_offset: 32_768,
                    alignment_16kb: Some(true),
                    error: None,
                }],
                signature_files: vec!["META-INF/CERT.RSA".to_string()],
                apk_signing: Some(ApkSigningInfo {
                    v2: true,
                    v3: true,
                    v31: true,
                    v32: false,
                    block_size: 4096,
                }),
                apk_signing_error: None,
                apk_signature_verification: Some(ApkSignatureVerification {
                    v2: Some(CryptoSchemeInfo {
                        state: CryptoVerificationState::Verified,
                        signer_count: 1,
                        algorithms: vec![0x0101],
                        certificate_sha256: vec!["a".repeat(64)],
                        sdk_ranges: vec![(24, 32)],
                        rotation_min_sdk: None,
                        rotation_targets_dev_release: false,
                        proof_of_rotation: Vec::new(),
                        detail: "v2 verified".to_string(),
                    }),
                    v3: Some(CryptoSchemeInfo {
                        state: CryptoVerificationState::Verified,
                        signer_count: 1,
                        algorithms: vec![0x0103],
                        certificate_sha256: vec!["b".repeat(64)],
                        sdk_ranges: vec![(28, 35)],
                        rotation_min_sdk: Some(28),
                        rotation_targets_dev_release: false,
                        proof_of_rotation: vec![ProofOfRotationInfo {
                            state: CryptoVerificationState::Verified,
                            level_count: 1,
                            lineage_certificate_sha256: vec!["b".repeat(64)],
                            capabilities: vec![ProofOfRotationCapabilities {
                                flags: 0x80,
                                known_flags: 0x1F,
                                unknown_flags: 0x60,
                                installed_data: true,
                                shared_user_id: false,
                                permission: true,
                                rollback: false,
                                auth: false,
                            }],
                            detail: "rotation verified".to_string(),
                        }],
                        detail: "v3 verified".to_string(),
                    }),
                    v31: None,
                    v31_present: false,
                    v32_present: false,
                }),
                apk_signature_verification_error: None,
            },
            manifest: Some(ManifestInfo {
                package_name: Some("com.example.app".to_string()),
                version_code: Some(42),
                version_name: Some("4.2.0".to_string()),
                min_sdk: Some(24),
                target_sdk: Some(35),
                debuggable: Some(false),
                permissions: Vec::new(),
                components: vec![ComponentInfo {
                    kind: "activity".to_string(),
                    name: "com.example.app.MainActivity".to_string(),
                    exported: Some(false),
                    has_intent_filters: false,
                }],
            }),
            manifest_error: None,
            project: Some(ProjectInfo {
                build_file: PathBuf::from("app/build.gradle.kts"),
                syntax: GradleSyntax::Kotlin,
                module_name: "app".to_string(),
                namespace: Some("com.example.app".to_string()),
                application_id: Some("com.example.app".to_string()),
                compile_sdk: Some(36),
                min_sdk: Some(24),
                target_sdk: Some(35),
                version_code: Some(42),
                version_name: Some("4.2.0".to_string()),
                release_debuggable: Some(false),
            }),
            project_error: Some("example project warning".to_string()),
            findings: vec![
                Finding {
                    rule_id: "TEST-001",
                    severity: Severity::Pass,
                    title: "Passed check".to_string(),
                    summary: "Evidence passed.".to_string(),
                    remediation: String::new(),
                },
                Finding {
                    rule_id: "TEST-002",
                    severity: Severity::Warning,
                    title: "Warning check".to_string(),
                    summary: "Evidence is incomplete.".to_string(),
                    remediation: "Review the artifact.".to_string(),
                },
                Finding {
                    rule_id: "TEST-003",
                    severity: Severity::Blocker,
                    title: "Blocker check".to_string(),
                    summary: "Evidence failed.".to_string(),
                    remediation: "Fix the artifact.".to_string(),
                },
                Finding {
                    rule_id: "TEST-004",
                    severity: Severity::ManualReview,
                    title: "Manual check".to_string(),
                    summary: "Evidence needs external verification.".to_string(),
                    remediation: "Verify externally.".to_string(),
                },
            ],
        }
    }

    #[test]
    fn report_v1_maps_all_major_sections() {
        let report = sample_report();
        let dto =
            ReportV1::from_audit_report(&report, ReportV1Context::with_play(PlayPlatform::Mobile));
        let value: serde_json::Value = serde_json::from_str(&dto.to_json().unwrap()).unwrap();

        let object = value.as_object().expect("top-level JSON object");
        let expected = [
            "schema_version",
            "engine_version",
            "artifact",
            "application",
            "manifest_error",
            "inventory",
            "project",
            "project_error",
            "play",
            "findings",
            "summary",
        ];

        assert_eq!(object.len(), expected.len());
        for key in expected {
            assert!(object.contains_key(key), "missing top-level key {key}");
        }

        assert_eq!(value["schema_version"], "1.0");
        assert_eq!(value["artifact"]["kind"], "APK");
        assert_eq!(value["artifact"]["size_bytes"], 123_456);
        assert_eq!(value["application"]["debuggable"], false);
        assert_eq!(value["application"]["components"][0]["exported"], false);
        assert_eq!(
            value["inventory"]["native_zip_entries"][0]["compression"],
            "stored"
        );
        assert_eq!(
            value["inventory"]["apk_signature_verification"]["v3"]["state"],
            "verified"
        );
        assert_eq!(
            value["inventory"]["apk_signature_verification"]["v3"]["proof_of_rotation"][0]
                ["capabilities"][0]["unknown_flags"],
            0x60
        );
        assert_eq!(value["project"]["syntax"], "Kotlin DSL");
        assert_eq!(value["project_error"], "example project warning");
        assert_eq!(value["play"]["platform"], "mobile");
        assert_eq!(value["play"]["policy_version"], PLAY_POLICY_VERSION);
        assert_eq!(value["summary"]["passed"], 1);
        assert_eq!(value["summary"]["warnings"], 1);
        assert_eq!(value["summary"]["blockers"], 1);
        assert_eq!(value["summary"]["manual_review"], 1);
        assert_eq!(value["application"]["permissions"], serde_json::json!([]));
    }

    #[test]
    fn report_v1_preserves_null_empty_false_and_error_semantics() {
        let mut report = sample_report();
        report.manifest = None;
        report.manifest_error = Some("truncated Android binary XML".to_string());
        report.inventory = ArtifactInventory::default();
        report.project = None;
        report.project_error = Some("Gradle parse error".to_string());

        let dto = ReportV1::from_audit_report(&report, ReportV1Context::without_play());
        let value: serde_json::Value = serde_json::from_str(&dto.to_json().unwrap()).unwrap();

        assert!(value["application"].is_null());
        assert_eq!(value["manifest_error"], "truncated Android binary XML");
        assert!(value["inventory"]["manifest_path"].is_null());
        assert_eq!(value["inventory"]["entries"], serde_json::json!([]));
        assert!(value["inventory"]["apk_signing"].is_null());
        assert!(value["inventory"]["apk_signature_verification"].is_null());
        assert!(value["project"].is_null());
        assert_eq!(value["project_error"], "Gradle parse error");
        assert!(value["play"].is_null());
    }

    #[test]
    fn report_v1_preserves_crypto_states_and_presence_boundaries() {
        let mut report = sample_report();
        report.inventory.apk_signature_verification = Some(ApkSignatureVerification {
            v2: Some(CryptoSchemeInfo {
                state: CryptoVerificationState::Unsupported,
                signer_count: 2,
                algorithms: vec![0x0101, 0x0201],
                certificate_sha256: vec!["c".repeat(64)],
                sdk_ranges: vec![(33, 35)],
                rotation_min_sdk: Some(33),
                rotation_targets_dev_release: true,
                proof_of_rotation: Vec::new(),
                detail: "algorithm unsupported".to_string(),
            }),
            v3: None,
            v31: None,
            v31_present: true,
            v32_present: true,
        });

        let dto = ReportV1::from_audit_report(&report, ReportV1Context::without_play());
        let value: serde_json::Value = serde_json::from_str(&dto.to_json().unwrap()).unwrap();

        assert_eq!(
            value["inventory"]["apk_signature_verification"]["v2"]["state"],
            "unsupported"
        );
        assert_eq!(
            value["inventory"]["apk_signature_verification"]["v2"]["rotation_targets_dev_release"],
            true
        );
        assert!(value["inventory"]["apk_signature_verification"]["v3"].is_null());
        assert_eq!(
            value["inventory"]["apk_signature_verification"]["v31_present"],
            true
        );
        assert_eq!(
            value["inventory"]["apk_signature_verification"]["v32_present"],
            true
        );
    }

    #[test]
    fn report_v1_json_is_deterministic_for_the_same_report_and_context() {
        let report = sample_report();
        let context = ReportV1Context::with_play(PlayPlatform::Mobile);
        let first = ReportV1::from_audit_report(&report, context)
            .to_json()
            .unwrap();
        let second = ReportV1::from_audit_report(&report, context)
            .to_json()
            .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn report_v1_pretty_json_is_valid_json() {
        let report = sample_report();
        let dto = ReportV1::from_audit_report(&report, ReportV1Context::without_play());

        let pretty = dto.to_json_pretty().unwrap();
        let value: serde_json::Value = serde_json::from_str(&pretty).unwrap();

        assert_eq!(value["schema_version"], "1.0");
    }
}
