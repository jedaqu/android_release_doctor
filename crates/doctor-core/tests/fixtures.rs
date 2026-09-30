use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use doctor_core::{audit_path, ArtifactKind, Severity};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

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

    assert!(report.manifest_error.is_none());
    let manifest = report.manifest.expect("manifest should parse");
    assert_eq!(
        manifest.package_name.as_deref(),
        Some("com.example.doctorfixture")
    );
    assert_eq!(manifest.version_code, Some(7));
    assert_eq!(manifest.version_name.as_deref(), Some("1.2.3"));
    assert_eq!(manifest.min_sdk, Some(24));
    assert_eq!(manifest.target_sdk, Some(35));
    assert_eq!(manifest.debuggable, Some(false));
    assert_eq!(
        manifest.permissions,
        vec![
            "android.permission.INTERNET",
            "android.permission.POST_NOTIFICATIONS"
        ]
    );
    assert_eq!(manifest.components.len(), 1);
    assert_eq!(manifest.components[0].kind, "activity");
    assert_eq!(manifest.components[0].exported, Some(true));
    assert!(manifest.components[0].has_intent_filters);

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

    assert!(report.manifest_error.is_none());
    let manifest = report.manifest.expect("manifest should parse");
    assert_eq!(
        manifest.package_name.as_deref(),
        Some("com.example.doctorfixture")
    );
    assert_eq!(manifest.min_sdk, Some(24));
    assert_eq!(manifest.target_sdk, Some(35));
    assert_eq!(manifest.components.len(), 1);
    assert!(manifest.components[0].has_intent_filters);

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

#[test]
fn reports_manifest_parse_error_in_the_audit_report() {
    let path = std::env::temp_dir().join(format!(
        "android-release-doctor-malformed-{}.apk",
        std::process::id()
    ));

    {
        let file = File::create(&path).expect("temporary APK should be created");
        let mut archive = ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        archive
            .start_file("AndroidManifest.xml", options)
            .expect("manifest entry should be created");
        archive
            .write_all(&[0x03, 0x00, 0x08])
            .expect("manifest bytes should be written");
        archive.finish().expect("temporary APK should be finalized");
    }

    let report = audit_path(&path).expect("invalid manifest should stay inside the audit report");

    assert!(report.manifest.is_none());
    assert_eq!(
        report.manifest_error.as_deref(),
        Some("truncated Android binary XML")
    );
    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "MANIFEST-002")
            .map(|finding| finding.severity),
        Some(Severity::Blocker)
    );

    fs::remove_file(path).expect("temporary APK should be removed");
}
