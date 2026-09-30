use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use doctor_core::{audit_path, audit_path_with_project, parse_project, ArtifactKind, Severity};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(name)
}

fn project_fixture(module: &str, file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(module)
        .join(file)
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


#[test]
fn cross_checks_gradle_groovy_against_apk_artifact() {
    let report = audit_path_with_project(
        fixture("minimal-release.apk"),
        project_fixture("project-release", "build.gradle"),
    )
    .expect("project and artifact should be audited together");

    let project = report.project.expect("project should be parsed");
    assert_eq!(project.application_id.as_deref(), Some("com.example.doctorfixture"));
    assert_eq!(project.target_sdk, Some(35));
    assert_eq!(project.version_code, Some(7));
    assert_eq!(project.release_debuggable, Some(false));

    for rule_id in [
        "CROSSCHECK-001",
        "CROSSCHECK-002",
        "CROSSCHECK-003",
        "CROSSCHECK-004",
        "CROSSCHECK-005",
        "CROSSCHECK-006",
    ] {
        assert_eq!(
            report
                .findings
                .iter()
                .find(|finding| finding.rule_id == rule_id)
                .map(|finding| finding.severity),
            Some(Severity::Pass),
            "{rule_id} should pass for matching project and artifact fixtures"
        );
    }
}

#[test]
fn parses_kotlin_dsl_fixture() {
    let project = parse_project(project_fixture(
        "project-release-kotlin",
        "build.gradle.kts",
    ))
    .expect("Kotlin DSL fixture should parse");

    assert_eq!(project.syntax.as_str(), "Kotlin DSL");
    assert_eq!(
        project.application_id.as_deref(),
        Some("com.example.doctorfixture")
    );
    assert_eq!(project.compile_sdk, Some(36));
    assert_eq!(project.min_sdk, Some(24));
    assert_eq!(project.target_sdk, Some(35));
    assert_eq!(project.version_code, Some(7));
    assert_eq!(project.version_name.as_deref(), Some("1.2.3"));
    assert_eq!(project.release_debuggable, Some(false));
}

#[test]
fn target_sdk_mismatch_is_a_blocker() {
    let path = std::env::temp_dir().join(format!(
        "android-release-doctor-project-mismatch-{}-build.gradle",
        std::process::id()
    ));
    let source = fs::read_to_string(project_fixture("project-release", "build.gradle"))
        .expect("project fixture should be readable");
    let source = source.replace("targetSdk 35", "targetSdk 36");
    fs::write(&path, source).expect("temporary project build file should be written");

    let report = audit_path_with_project(fixture("minimal-release.apk"), &path)
        .expect("project/artifact audit should complete");

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "CROSSCHECK-002")
            .map(|finding| finding.severity),
        Some(Severity::Blocker)
    );

    fs::remove_file(path).expect("temporary project file should be removed");
}

#[test]
fn invalid_project_path_stays_inside_report() {
    let path = std::env::temp_dir().join(format!(
        "android-release-doctor-missing-project-{}",
        std::process::id()
    ));

    let report = audit_path_with_project(fixture("minimal-release.apk"), &path)
        .expect("missing project should stay inside the audit report");

    assert!(report.project.is_none());
    assert!(report.project_error.is_some());
    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "PROJECT-001")
            .map(|finding| finding.severity),
        Some(Severity::Blocker)
    );
}
