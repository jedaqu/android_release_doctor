use std::path::PathBuf;

use doctor_core::{audit_path, ArtifactKind, Severity};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(name)
}

#[test]
fn audits_minimal_apk_fixture() {
    let report = audit_path(fixture("minimal-release.apk")).expect("APK fixture should parse");
    assert_eq!(report.artifact_kind, ArtifactKind::Apk);
    assert_eq!(
        report.inventory.manifest_path.as_deref(),
        Some("AndroidManifest.xml")
    );
    assert_eq!(report.inventory.dex_files, vec!["classes.dex"]);
    assert_eq!(report.inventory.native_abis, vec!["arm64-v8a"]);
    assert_eq!(report.inventory.signature_files.len(), 2);
    assert!(report
        .findings
        .iter()
        .all(|finding| finding.severity != Severity::Blocker));
}

#[test]
fn audits_minimal_aab_fixture() {
    let report = audit_path(fixture("minimal-release.aab")).expect("AAB fixture should parse");
    assert_eq!(report.artifact_kind, ArtifactKind::Aab);
    assert_eq!(
        report.inventory.manifest_path.as_deref(),
        Some("base/manifest/AndroidManifest.xml")
    );
    assert_eq!(report.inventory.dex_files, vec!["base/dex/classes.dex"]);
    assert_eq!(
        report.inventory.native_abis,
        vec!["arm64-v8a", "armeabi-v7a"]
    );
    assert_eq!(report.inventory.signature_files.len(), 2);
    assert!(report
        .findings
        .iter()
        .all(|finding| finding.severity != Severity::Blocker));
}
