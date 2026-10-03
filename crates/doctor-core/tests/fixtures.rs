use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use doctor_core::{
    audit_path, audit_path_with_play, audit_path_with_project, parse_project, ArtifactKind,
    PlayPlatform, Severity,
};
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
    assert_eq!(report.inventory.native_libraries.len(), 1);
    assert_eq!(
        report.inventory.native_libraries[0].path,
        "lib/arm64-v8a/libdemo.so"
    );
    assert!(report.inventory.native_libraries[0].error.is_some());
    assert_eq!(report.inventory.signature_files.len(), 2);
    assert!(report
        .findings
        .iter()
        .all(|finding| finding.severity != Severity::Blocker));
}

#[test]
fn audits_minimal_aab_fixture() {
    let path = std::env::temp_dir().join(format!(
        "android-release-doctor-proto-aab-{}.aab",
        std::process::id()
    ));

    {
        let file = File::create(&path).expect("temporary AAB should be created");
        let mut archive = ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

        archive
            .start_file("base/manifest/AndroidManifest.xml", options)
            .expect("proto manifest entry should be created");
        archive
            .write_all(&proto_aab_manifest())
            .expect("proto manifest bytes should be written");

        archive
            .start_file("base/dex/classes.dex", options)
            .expect("dex entry should be created");
        archive
            .write_all(b"dex-fixture")
            .expect("dex bytes should be written");

        archive.finish().expect("temporary AAB should be finalized");
    }

    let report = audit_path(&path).expect("proto AAB should remain auditable");
    assert_eq!(report.artifact_kind, ArtifactKind::Aab);
    assert_eq!(
        report.inventory.manifest_path.as_deref(),
        Some("base/manifest/AndroidManifest.xml")
    );
    assert!(report.manifest_error.is_none());

    let manifest = report.manifest.expect("proto AAB manifest should parse");
    assert_eq!(
        manifest.package_name.as_deref(),
        Some("com.example.protoaab")
    );
    assert_eq!(manifest.version_code, Some(7));
    assert_eq!(manifest.version_name.as_deref(), Some("1.2.3"));
    assert_eq!(manifest.min_sdk, Some(24));
    assert_eq!(manifest.target_sdk, Some(35));
    assert_eq!(manifest.debuggable, Some(false));
    assert_eq!(
        manifest.permissions,
        vec!["android.permission.INTERNET".to_string()]
    );
    assert_eq!(manifest.components.len(), 1);
    assert_eq!(manifest.components[0].kind, "activity");
    assert!(manifest.components[0].has_intent_filters);

    assert!(report
        .findings
        .iter()
        .find(|finding| finding.rule_id == "MANIFEST-002")
        .is_none());
    assert!(report
        .findings
        .iter()
        .all(|finding| finding.severity != Severity::Blocker));

    fs::remove_file(path).expect("temporary AAB should be removed");
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
    assert_eq!(
        project.application_id.as_deref(),
        Some("com.example.doctorfixture")
    );
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
    let project_dir = std::env::temp_dir().join(format!(
        "android-release-doctor-project-mismatch-{}",
        std::process::id()
    ));
    let build_file = project_dir.join("build.gradle");
    let source = fs::read_to_string(project_fixture("project-release", "build.gradle"))
        .expect("project fixture should be readable");
    let source = source.replace("targetSdk 35", "targetSdk 36");
    fs::create_dir_all(&project_dir).expect("temporary project directory should be created");
    fs::write(&build_file, source).expect("temporary project build file should be written");

    let report = audit_path_with_project(fixture("minimal-release.apk"), &project_dir)
        .expect("project/artifact audit should complete");

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "CROSSCHECK-002")
            .map(|finding| finding.severity),
        Some(Severity::Blocker)
    );

    fs::remove_dir_all(project_dir).expect("temporary project directory should be removed");
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

fn native_zip_fixture(
    name: &str,
    compression: CompressionMethod,
    alignment: Option<u16>,
) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "android-release-doctor-{name}-{}.apk",
        std::process::id()
    ));

    let file = File::create(&path).expect("temporary APK should be created");
    let mut archive = ZipWriter::new(file);

    let mut options = SimpleFileOptions::default().compression_method(compression);
    if let Some(alignment) = alignment {
        options = options.with_alignment(alignment);
    }

    archive
        .start_file("lib/arm64-v8a/libdemo.so", options)
        .expect("native library entry should be created");
    archive
        .write_all(b"not-an-elf")
        .expect("native library bytes should be written");
    archive.finish().expect("temporary APK should be finalized");

    path
}

#[test]
fn reports_uncompressed_native_zip_alignment_when_misaligned() {
    let path = native_zip_fixture("native-zip-misaligned", CompressionMethod::Stored, None);

    let report = audit_path(&path).expect("stored native fixture should parse");
    assert_eq!(report.inventory.native_zip_entries.len(), 1);

    let entry = &report.inventory.native_zip_entries[0];
    assert_eq!(entry.compression, doctor_core::NativeZipCompression::Stored);
    assert_eq!(entry.alignment_16kb, Some(false));
    assert!(entry.data_offset > 0);

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "NATIVE-003")
            .map(|finding| finding.severity),
        Some(Severity::Warning)
    );

    fs::remove_file(path).expect("temporary APK should be removed");
}

#[test]
fn reports_uncompressed_native_zip_alignment_when_aligned() {
    let path = native_zip_fixture(
        "native-zip-aligned",
        CompressionMethod::Stored,
        Some(doctor_core::ZIP_ALIGNMENT_16KB as u16),
    );

    let report = audit_path(&path).expect("aligned stored native fixture should parse");
    assert_eq!(report.inventory.native_zip_entries.len(), 1);

    let entry = &report.inventory.native_zip_entries[0];
    assert_eq!(entry.compression, doctor_core::NativeZipCompression::Stored);
    assert_eq!(entry.alignment_16kb, Some(true));
    assert_eq!(entry.data_offset, doctor_core::ZIP_ALIGNMENT_16KB);

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "NATIVE-003")
            .map(|finding| finding.severity),
        Some(Severity::Pass)
    );

    fs::remove_file(path).expect("temporary APK should be removed");
}

#[test]
fn compressed_native_library_does_not_require_zip_offset_alignment() {
    let path = native_zip_fixture("native-zip-compressed", CompressionMethod::Deflated, None);

    let report = audit_path(&path).expect("compressed native fixture should parse");
    assert_eq!(report.inventory.native_zip_entries.len(), 1);

    let entry = &report.inventory.native_zip_entries[0];
    assert_eq!(
        entry.compression,
        doctor_core::NativeZipCompression::Compressed
    );
    assert_eq!(entry.alignment_16kb, None);
    assert!(entry.error.is_none());

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "NATIVE-003")
            .map(|finding| finding.severity),
        Some(Severity::Pass)
    );

    fs::remove_file(path).expect("temporary APK should be removed");
}

fn native_zip_matrix_fixture(
    name: &str,
    kind: ArtifactKind,
    entries: &[(&str, CompressionMethod, Option<u16>)],
) -> PathBuf {
    let extension = match kind {
        ArtifactKind::Apk => "apk",
        ArtifactKind::Aab => "aab",
    };
    let prefix = match kind {
        ArtifactKind::Apk => "lib",
        ArtifactKind::Aab => "base/lib",
    };
    let path = std::env::temp_dir().join(format!(
        "android-release-doctor-{name}-{}.{}",
        std::process::id(),
        extension
    ));

    let file = File::create(&path).expect("temporary native ABI fixture should be created");
    let mut archive = ZipWriter::new(file);

    for (abi, compression, alignment) in entries {
        let mut options = SimpleFileOptions::default().compression_method(*compression);
        if let Some(alignment) = alignment {
            options = options.with_alignment(*alignment);
        }

        archive
            .start_file(format!("{prefix}/{abi}/libdemo-{abi}.so"), options)
            .expect("native ABI entry should be created");
        archive
            .write_all(b"not-an-elf")
            .expect("native ABI bytes should be written");
    }

    archive
        .finish()
        .expect("native ABI fixture should be finalized");

    path
}

fn finding_severity(report: &doctor_core::AuditReport, rule_id: &str) -> Severity {
    report
        .findings
        .iter()
        .find(|finding| finding.rule_id == rule_id)
        .map(|finding| finding.severity)
        .expect("expected finding should exist")
}

#[test]
fn native_zip_64_bit_stored_alignment_matrix() {
    for (abi, name) in [("arm64-v8a", "arm64"), ("x86_64", "x86_64")] {
        let bad = native_zip_matrix_fixture(
            &format!("native-zip-{name}-bad"),
            ArtifactKind::Apk,
            &[(abi, CompressionMethod::Stored, Some(4096))],
        );
        let bad_report = audit_path(&bad).expect("misaligned 64-bit APK should be auditable");
        assert_eq!(
            finding_severity(&bad_report, "NATIVE-003"),
            Severity::Warning,
            "misaligned {abi} native ZIP entry must warn"
        );
        fs::remove_file(bad).expect("bad native ABI fixture should be removed");

        let good = native_zip_matrix_fixture(
            &format!("native-zip-{name}-good"),
            ArtifactKind::Apk,
            &[(
                abi,
                CompressionMethod::Stored,
                Some(doctor_core::ZIP_ALIGNMENT_16KB as u16),
            )],
        );
        let good_report = audit_path(&good).expect("aligned 64-bit APK should be auditable");
        assert_eq!(
            finding_severity(&good_report, "NATIVE-003"),
            Severity::Pass,
            "aligned {abi} native ZIP entry must pass"
        );
        fs::remove_file(good).expect("good native ABI fixture should be removed");
    }
}

#[test]
fn native_zip_32_bit_alignment_is_not_a_16kb_requirement() {
    for (abi, name) in [("armeabi-v7a", "armeabi-v7a"), ("x86", "x86")] {
        let path = native_zip_matrix_fixture(
            &format!("native-zip-{name}-4k"),
            ArtifactKind::Apk,
            &[(abi, CompressionMethod::Stored, Some(4096))],
        );
        let report = audit_path(&path).expect("32-bit APK should be auditable");
        assert_eq!(
            finding_severity(&report, "NATIVE-003"),
            Severity::Pass,
            "4 KiB {abi} native ZIP entry must not trigger NATIVE-003"
        );
        fs::remove_file(path).expect("32-bit native ABI fixture should be removed");
    }
}

#[test]
fn native_zip_mixed_32_and_64_bit_matrix_uses_only_applicable_abis() {
    let path = native_zip_matrix_fixture(
        "native-zip-mixed-32-64",
        ArtifactKind::Apk,
        &[
            ("armeabi-v7a", CompressionMethod::Stored, Some(4096)),
            ("x86", CompressionMethod::Stored, Some(4096)),
            ("arm64-v8a", CompressionMethod::Stored, Some(4096)),
            (
                "x86_64",
                CompressionMethod::Stored,
                Some(doctor_core::ZIP_ALIGNMENT_16KB as u16),
            ),
        ],
    );

    let report = audit_path(&path).expect("mixed ABI APK should be auditable");
    assert_eq!(finding_severity(&report, "NATIVE-003"), Severity::Warning);
    let native_abis = report.inventory.native_abis;
    assert_eq!(
        native_abis,
        vec![
            "arm64-v8a".to_string(),
            "armeabi-v7a".to_string(),
            "x86".to_string(),
            "x86_64".to_string()
        ]
    );

    fs::remove_file(path).expect("mixed native ABI fixture should be removed");
}

#[test]
fn native_zip_compressed_64_bit_libraries_pass_without_offset_alignment() {
    let path = native_zip_matrix_fixture(
        "native-zip-compressed-64",
        ArtifactKind::Apk,
        &[
            ("arm64-v8a", CompressionMethod::Deflated, None),
            ("x86_64", CompressionMethod::Deflated, None),
        ],
    );

    let report = audit_path(&path).expect("compressed 64-bit APK should be auditable");
    assert_eq!(finding_severity(&report, "NATIVE-003"), Severity::Pass);

    fs::remove_file(path).expect("compressed native ABI fixture should be removed");
}

#[test]
fn native_zip_aab_64_bit_stored_entries_require_manual_review() {
    let path = native_zip_matrix_fixture(
        "native-zip-aab-64-stored",
        ArtifactKind::Aab,
        &[("arm64-v8a", CompressionMethod::Stored, None)],
    );

    let report = audit_path(&path).expect("64-bit AAB should be auditable");
    assert_eq!(
        finding_severity(&report, "NATIVE-003"),
        Severity::ManualReview
    );

    fs::remove_file(path).expect("AAB native ABI fixture should be removed");
}

#[test]
fn native_zip_aab_32_bit_stored_entries_do_not_require_manual_review() {
    let path = native_zip_matrix_fixture(
        "native-zip-aab-32-stored",
        ArtifactKind::Aab,
        &[
            ("armeabi-v7a", CompressionMethod::Stored, None),
            ("x86", CompressionMethod::Stored, None),
        ],
    );

    let report = audit_path(&path).expect("32-bit AAB should be auditable");
    assert_eq!(finding_severity(&report, "NATIVE-003"), Severity::Pass);

    fs::remove_file(path).expect("32-bit AAB native ABI fixture should be removed");
}

#[test]
fn native_zip_aab_64_bit_compressed_entries_pass() {
    let path = native_zip_matrix_fixture(
        "native-zip-aab-64-compressed",
        ArtifactKind::Aab,
        &[
            ("arm64-v8a", CompressionMethod::Deflated, None),
            ("x86_64", CompressionMethod::Deflated, None),
        ],
    );

    let report = audit_path(&path).expect("compressed 64-bit AAB should be auditable");
    assert_eq!(finding_severity(&report, "NATIVE-003"), Severity::Pass);

    fs::remove_file(path).expect("compressed 64-bit AAB fixture should be removed");
}

#[test]
fn native_zip_aab_mixed_32_and_64_bit_keeps_manual_review_for_64_bit() {
    let path = native_zip_matrix_fixture(
        "native-zip-aab-mixed-32-64",
        ArtifactKind::Aab,
        &[
            ("armeabi-v7a", CompressionMethod::Stored, None),
            ("x86", CompressionMethod::Stored, None),
            ("arm64-v8a", CompressionMethod::Stored, None),
        ],
    );

    let report = audit_path(&path).expect("mixed ABI AAB should be auditable");
    assert_eq!(
        finding_severity(&report, "NATIVE-003"),
        Severity::ManualReview
    );

    fs::remove_file(path).expect("mixed AAB native ABI fixture should be removed");
}

fn decode_base64_fixture(encoded: &str) -> Vec<u8> {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut output = Vec::new();
    let mut accumulator = 0_u32;
    let mut bits = 0_u8;

    for byte in encoded.bytes() {
        if byte.is_ascii_whitespace() || byte == b'=' {
            continue;
        }

        let value = ALPHABET
            .iter()
            .position(|candidate| *candidate == byte)
            .expect("fixture base64 should contain only standard alphabet bytes")
            as u32;

        accumulator = (accumulator << 6) | value;
        bits += 6;

        if bits >= 8 {
            bits -= 8;
            output.push((accumulator >> bits) as u8);
            accumulator &= (1_u32 << bits) - 1;
        }
    }

    output
}

fn decoded_apk_fixture(source_name: &str, label: &str) -> PathBuf {
    let encoded =
        fs::read_to_string(fixture(source_name)).expect("base64 APK fixture should be readable");
    let bytes = decode_base64_fixture(&encoded);
    assert_eq!(
        bytes.len(),
        1588,
        "ERR-095 fixture byte length must remain stable"
    );

    let path = std::env::temp_dir().join(format!(
        "android-release-doctor-{label}-{}.apk",
        std::process::id()
    ));
    fs::write(&path, bytes).expect("decoded APK fixture should be writable");
    path
}

fn tampered_apk_fixture(source_name: &str, label: &str) -> PathBuf {
    let source = fixture(source_name);
    let path = std::env::temp_dir().join(format!(
        "android-release-doctor-{label}-{}.apk",
        std::process::id()
    ));
    let mut bytes = fs::read(source).expect("crypto fixture should be readable");
    bytes[40] ^= 0x01;
    fs::write(&path, bytes).expect("tampered crypto fixture should be writable");
    path
}

#[test]
fn valid_v2_fixture_produces_verified_signing_finding() {
    let report = audit_path(fixture("crypto-v2-release.apk"))
        .expect("valid v2 crypto fixture should remain auditable");

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "SIGNING-001")
            .map(|finding| finding.severity),
        Some(Severity::Pass)
    );
    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "SIGNING-003")
            .map(|finding| finding.severity),
        Some(Severity::Pass)
    );

    let verification = report
        .inventory
        .apk_signature_verification
        .as_ref()
        .expect("v2 verification result should be present");
    assert_eq!(
        verification.v2.as_ref().map(|scheme| scheme.state),
        Some(doctor_core::CryptoVerificationState::Verified)
    );
}

#[test]
fn err_095_empty_fourth_v2_fixture_produces_verified_signing_finding() {
    let path = decoded_apk_fixture(
        "crypto-v2-empty-element-release.apk.b64",
        "v2-empty-element",
    );
    let report = audit_path(&path).expect("ERR-095 compatibility fixture should remain auditable");

    let verification = report
        .inventory
        .apk_signature_verification
        .as_ref()
        .expect("ERR-095 v2 verification result should be present");
    let v2 = verification
        .v2
        .as_ref()
        .expect("ERR-095 v2 verification result should be present");

    assert_eq!(
        v2.state,
        doctor_core::CryptoVerificationState::Verified,
        "verification detail: {}",
        v2.detail
    );
    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "SIGNING-003")
            .map(|finding| finding.severity),
        Some(Severity::Pass)
    );

    fs::remove_file(path).expect("decoded ERR-095 fixture should be removed");
}

#[test]
fn tampered_v2_fixture_produces_signature_blocker() {
    let path = tampered_apk_fixture("crypto-v2-release.apk", "v2-integration-tampered");
    let report = audit_path(&path).expect("tampered v2 crypto fixture should remain auditable");

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "SIGNING-003")
            .map(|finding| finding.severity),
        Some(Severity::Blocker)
    );

    let verification = report
        .inventory
        .apk_signature_verification
        .as_ref()
        .expect("tampered v2 verification result should be present");
    assert_eq!(
        verification.v2.as_ref().map(|scheme| scheme.state),
        Some(doctor_core::CryptoVerificationState::Invalid)
    );

    fs::remove_file(path).expect("temporary tampered v2 fixture should be removed");
}

#[test]
fn valid_v3_fixture_produces_verified_signing_finding() {
    let report = audit_path(fixture("crypto-v3-release.apk"))
        .expect("valid v3 crypto fixture should remain auditable");

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "SIGNING-001")
            .map(|finding| finding.severity),
        Some(Severity::Pass)
    );
    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "SIGNING-003")
            .map(|finding| finding.severity),
        Some(Severity::Pass)
    );

    let verification = report
        .inventory
        .apk_signature_verification
        .as_ref()
        .expect("v3 verification result should be present");
    assert_eq!(
        verification.v3.as_ref().map(|scheme| scheme.state),
        Some(doctor_core::CryptoVerificationState::Verified)
    );
}

#[test]
fn tampered_v3_fixture_produces_signature_blocker() {
    let path = tampered_apk_fixture("crypto-v3-release.apk", "v3-integration-tampered");
    let report = audit_path(&path).expect("tampered v3 crypto fixture should remain auditable");

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "SIGNING-003")
            .map(|finding| finding.severity),
        Some(Severity::Blocker)
    );

    let verification = report
        .inventory
        .apk_signature_verification
        .as_ref()
        .expect("tampered v3 verification result should be present");
    assert_eq!(
        verification.v3.as_ref().map(|scheme| scheme.state),
        Some(doctor_core::CryptoVerificationState::Invalid)
    );

    fs::remove_file(path).expect("temporary tampered v3 fixture should be removed");
}

#[test]
fn signing_block_absence_is_reported_without_a_blocker() {
    let report =
        audit_path(fixture("minimal-release.apk")).expect("fixture should remain auditable");

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "SIGNING-001")
            .map(|finding| finding.severity),
        Some(Severity::Pass)
    );
    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "SIGNING-002")
            .map(|finding| finding.severity),
        Some(Severity::Warning)
    );
    assert!(report.inventory.apk_signing.is_none());
    assert!(report.inventory.apk_signing_error.is_none());
}

#[test]
fn play_mobile_profile_flags_existing_fixture_target_api() {
    let report = audit_path_with_play(fixture("minimal-release.apk"), PlayPlatform::Mobile)
        .expect("Play readiness audit should complete");

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "PLAY-001")
            .map(|finding| finding.severity),
        Some(Severity::Blocker)
    );

    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "PLAY-002")
            .map(|finding| finding.severity),
        Some(Severity::ManualReview)
    );
    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "PLAY-003")
            .map(|finding| finding.severity),
        Some(Severity::ManualReview)
    );
    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "PLAY-004")
            .map(|finding| finding.severity),
        Some(Severity::ManualReview)
    );
    assert_eq!(
        report
            .findings
            .iter()
            .find(|finding| finding.rule_id == "PLAY-005")
            .map(|finding| finding.severity),
        Some(Severity::ManualReview)
    );
}

fn proto_encode_varint(value: u64, output: &mut Vec<u8>) {
    let mut value = value;
    while value >= 0x80 {
        output.push((value as u8 & 0x7f) | 0x80);
        value >>= 7;
    }
    output.push(value as u8);
}

fn proto_encode_key(field: u32, wire_type: u8, output: &mut Vec<u8>) {
    proto_encode_varint((u64::from(field) << 3) | u64::from(wire_type), output);
}

fn proto_encode_bytes(field: u32, value: &[u8], output: &mut Vec<u8>) {
    proto_encode_key(field, 2, output);
    proto_encode_varint(value.len() as u64, output);
    output.extend_from_slice(value);
}

fn proto_encode_string(field: u32, value: &str, output: &mut Vec<u8>) {
    proto_encode_bytes(field, value.as_bytes(), output);
}

fn proto_compiled_primitive(value_type: u32, data: u32) -> Vec<u8> {
    let mut primitive = Vec::new();
    proto_encode_key(1, 0, &mut primitive);
    proto_encode_varint(u64::from(value_type), &mut primitive);
    proto_encode_key(2, 0, &mut primitive);
    proto_encode_varint(u64::from(data), &mut primitive);

    let mut item = Vec::new();
    proto_encode_bytes(7, &primitive, &mut item);
    item
}

fn proto_attribute(
    namespace: &str,
    name: &str,
    value: &str,
    primitive: Option<(u32, u32)>,
) -> Vec<u8> {
    let mut output = Vec::new();
    proto_encode_string(1, namespace, &mut output);
    proto_encode_string(2, name, &mut output);
    proto_encode_string(3, value, &mut output);
    if let Some((value_type, data)) = primitive {
        proto_encode_bytes(6, &proto_compiled_primitive(value_type, data), &mut output);
    }
    output
}

fn proto_element_node(name: &str, attributes: &[Vec<u8>], children: &[Vec<u8>]) -> Vec<u8> {
    let mut element = Vec::new();
    proto_encode_string(3, name, &mut element);
    for attribute in attributes {
        proto_encode_bytes(4, attribute, &mut element);
    }
    for child in children {
        proto_encode_bytes(5, child, &mut element);
    }

    let mut node = Vec::new();
    proto_encode_bytes(1, &element, &mut node);
    node
}

fn proto_aab_manifest() -> Vec<u8> {
    proto_element_node(
        "manifest",
        &[
            proto_attribute("", "package", "com.example.protoaab", None),
            proto_attribute(
                "http://schemas.android.com/apk/res/android",
                "versionCode",
                "7",
                Some((0x10, 7)),
            ),
            proto_attribute(
                "http://schemas.android.com/apk/res/android",
                "versionName",
                "1.2.3",
                None,
            ),
        ],
        &[
            proto_element_node(
                "uses-sdk",
                &[
                    proto_attribute(
                        "http://schemas.android.com/apk/res/android",
                        "minSdkVersion",
                        "24",
                        Some((0x10, 24)),
                    ),
                    proto_attribute(
                        "http://schemas.android.com/apk/res/android",
                        "targetSdkVersion",
                        "35",
                        Some((0x10, 35)),
                    ),
                ],
                &[],
            ),
            proto_element_node(
                "uses-permission",
                &[proto_attribute(
                    "http://schemas.android.com/apk/res/android",
                    "name",
                    "android.permission.INTERNET",
                    None,
                )],
                &[],
            ),
            proto_element_node(
                "application",
                &[proto_attribute(
                    "http://schemas.android.com/apk/res/android",
                    "debuggable",
                    "false",
                    Some((0x12, 0)),
                )],
                &[proto_element_node(
                    "activity",
                    &[
                        proto_attribute(
                            "http://schemas.android.com/apk/res/android",
                            "name",
                            "com.example.protoaab.MainActivity",
                            None,
                        ),
                        proto_attribute(
                            "http://schemas.android.com/apk/res/android",
                            "exported",
                            "true",
                            Some((0x12, 1)),
                        ),
                    ],
                    &[proto_element_node("intent-filter", &[], &[])],
                )],
            ),
        ],
    )
}
