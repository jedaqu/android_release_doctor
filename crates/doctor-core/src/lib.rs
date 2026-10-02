use std::{
    fmt,
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use zip::ZipArchive;

pub mod axml;
pub mod elf;
pub mod manifest_proto;
pub mod play;
pub mod project;
pub mod report;
pub mod signature_verify;
pub mod signing;
pub mod zip_alignment;
pub use axml::{ComponentInfo, ManifestInfo};
pub use elf::{inspect_shared_object, load_segments_are_16kb_aligned, ElfInspection};
pub use play::{evaluate_play_policy, PlayPlatform, PLAY_POLICY_VERSION};
pub use project::{parse_project, GradleSyntax, ProjectInfo};
pub use report::{ReportV1, ReportV1Context};
pub use signature_verify::{
    verify_apk_signatures, ApkSignatureVerification, CryptoSchemeInfo, CryptoVerificationState,
};
pub use signing::{inspect_apk_signing_block, ApkSigningInfo};
pub use zip_alignment::{is_16kb_aligned, NativeZipCompression, ZIP_ALIGNMENT_16KB};

pub const ENGINE_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Pass,
    Warning,
    Blocker,
    ManualReview,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Warning => "WARNING",
            Self::Blocker => "BLOCKER",
            Self::ManualReview => "MANUAL-REVIEW",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub rule_id: &'static str,
    pub severity: Severity,
    pub title: String,
    pub summary: String,
    pub remediation: String,
}

impl Finding {
    fn pass(rule_id: &'static str, title: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            rule_id,
            severity: Severity::Pass,
            title: title.into(),
            summary: summary.into(),
            remediation: String::new(),
        }
    }

    fn warning(
        rule_id: &'static str,
        title: impl Into<String>,
        summary: impl Into<String>,
        remediation: impl Into<String>,
    ) -> Self {
        Self {
            rule_id,
            severity: Severity::Warning,
            title: title.into(),
            summary: summary.into(),
            remediation: remediation.into(),
        }
    }

    fn blocker(
        rule_id: &'static str,
        title: impl Into<String>,
        summary: impl Into<String>,
        remediation: impl Into<String>,
    ) -> Self {
        Self {
            rule_id,
            severity: Severity::Blocker,
            title: title.into(),
            summary: summary.into(),
            remediation: remediation.into(),
        }
    }

    fn manual_review(
        rule_id: &'static str,
        title: impl Into<String>,
        summary: impl Into<String>,
        remediation: impl Into<String>,
    ) -> Self {
        Self {
            rule_id,
            severity: Severity::ManualReview,
            title: title.into(),
            summary: summary.into(),
            remediation: remediation.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactKind {
    Apk,
    Aab,
}

impl ArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Apk => "APK",
            Self::Aab => "AAB",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeLibraryInfo {
    pub path: String,
    pub abi: String,
    pub load_segment_alignments: Vec<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeZipEntryInfo {
    pub path: String,
    pub abi: String,
    pub compression: NativeZipCompression,
    pub data_offset: u64,
    pub alignment_16kb: Option<bool>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtifactInventory {
    pub entries: Vec<String>,
    pub manifest_path: Option<String>,
    pub dex_files: Vec<String>,
    pub native_abis: Vec<String>,
    pub native_libraries: Vec<NativeLibraryInfo>,
    pub native_zip_entries: Vec<NativeZipEntryInfo>,
    pub signature_files: Vec<String>,
    pub apk_signing: Option<ApkSigningInfo>,
    pub apk_signing_error: Option<String>,
    pub apk_signature_verification: Option<ApkSignatureVerification>,
    pub apk_signature_verification_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuditReport {
    pub artifact_path: PathBuf,
    pub artifact_kind: ArtifactKind,
    pub size_bytes: u64,
    pub inventory: ArtifactInventory,
    pub manifest: Option<ManifestInfo>,
    pub manifest_error: Option<String>,
    pub project: Option<ProjectInfo>,
    pub project_error: Option<String>,
    pub findings: Vec<Finding>,
}

impl AuditReport {
    pub fn counts(&self) -> (usize, usize, usize) {
        self.findings
            .iter()
            .fold((0, 0, 0), |(p, w, b), finding| match finding.severity {
                Severity::Pass => (p + 1, w, b),
                Severity::Warning => (p, w + 1, b),
                Severity::Blocker => (p, w, b + 1),
                Severity::ManualReview => (p, w, b),
            })
    }

    pub fn manual_review_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.severity == Severity::ManualReview)
            .count()
    }

    pub fn render_text(&self) -> String {
        let (passes, warnings, blockers) = self.counts();
        let manual_reviews = self.manual_review_count();
        let mut out = String::new();

        out.push_str(
            "ANDROID RELEASE REPORT
",
        );
        out.push_str(
            "=======================

",
        );
        out.push_str(&format!(
            "Artifact
  Type: {}
  Path: {}
  Size: {} bytes
  Engine: {}

",
            self.artifact_kind.as_str(),
            self.artifact_path.display(),
            self.size_bytes,
            ENGINE_VERSION
        ));

        if let Some(error) = &self.manifest_error {
            out.push_str(
                "Manifest
",
            );
            out.push_str(&format!(
                "  Parse error: {}

",
                error
            ));
        }

        if let Some(manifest) = &self.manifest {
            out.push_str(
                "Application
",
            );
            out.push_str(&format!(
                "  Package: {}
",
                manifest.package_name.as_deref().unwrap_or("<missing>")
            ));
            out.push_str(&format!(
                "  Version code: {}
",
                manifest
                    .version_code
                    .map_or_else(|| "<missing>".to_string(), |value| value.to_string())
            ));
            out.push_str(&format!(
                "  Version name: {}
",
                manifest.version_name.as_deref().unwrap_or("<missing>")
            ));
            out.push_str(&format!(
                "  minSdk: {}
",
                manifest
                    .min_sdk
                    .map_or_else(|| "<missing>".to_string(), |value| value.to_string())
            ));
            out.push_str(&format!(
                "  targetSdk: {}
",
                manifest
                    .target_sdk
                    .map_or_else(|| "<missing>".to_string(), |value| value.to_string())
            ));
            out.push_str(&format!(
                "  Debuggable: {}
",
                manifest
                    .debuggable
                    .map_or_else(|| "<missing>".to_string(), |value| value.to_string())
            ));
            out.push_str(&format!(
                "  Permissions: {}
",
                manifest.permissions.len()
            ));
            out.push_str(&format!(
                "  Components: {}

",
                manifest.components.len()
            ));
        }

        if let Some(error) = &self.project_error {
            out.push_str(
                "Project
",
            );
            out.push_str(&format!(
                "  Parse error: {}

",
                error
            ));
        }

        if let Some(project) = &self.project {
            out.push_str(
                "Project
",
            );
            out.push_str(&format!(
                "  Syntax: {}
",
                project.syntax.as_str()
            ));
            out.push_str(&format!(
                "  Build file: {}
",
                project.build_file.display()
            ));
            out.push_str(&format!(
                "  Module: {}
",
                project.module_name
            ));
            out.push_str(&format!(
                "  Namespace: {}
",
                project.namespace.as_deref().unwrap_or("<missing>")
            ));
            out.push_str(&format!(
                "  Application ID: {}
",
                project.application_id.as_deref().unwrap_or("<missing>")
            ));
            out.push_str(&format!(
                "  compileSdk: {}
",
                project
                    .compile_sdk
                    .map_or_else(|| "<missing>".to_string(), |value| value.to_string())
            ));
            out.push_str(&format!(
                "  minSdk: {}
",
                project
                    .min_sdk
                    .map_or_else(|| "<missing>".to_string(), |value| value.to_string())
            ));
            out.push_str(&format!(
                "  targetSdk: {}
",
                project
                    .target_sdk
                    .map_or_else(|| "<missing>".to_string(), |value| value.to_string())
            ));
            out.push_str(&format!(
                "  Version code: {}
",
                project
                    .version_code
                    .map_or_else(|| "<missing>".to_string(), |value| value.to_string())
            ));
            out.push_str(&format!(
                "  Version name: {}
",
                project.version_name.as_deref().unwrap_or("<missing>")
            ));
            out.push_str(&format!(
                "  Release debuggable: {}

",
                project
                    .release_debuggable
                    .map_or_else(|| "<missing>".to_string(), |value| value.to_string())
            ));
        }

        out.push_str(
            "Checks
",
        );
        for finding in &self.findings {
            out.push_str(&format!(
                "  {:<7} {:<24} [{}]
",
                finding.severity.as_str(),
                finding.title,
                finding.rule_id
            ));
            out.push_str(&format!(
                "      {}
",
                finding.summary
            ));
            if !finding.remediation.is_empty() {
                out.push_str(&format!(
                    "      Fix: {}
",
                    finding.remediation
                ));
            }
        }

        out.push_str(
            "
Summary
",
        );
        out.push_str(&format!(
            "  BLOCKERS {}
  WARNINGS {}
  MANUAL REVIEW {}
  PASSED {}
",
            blockers, warnings, manual_reviews, passes
        ));

        out
    }
}

#[derive(Debug)]
pub enum AuditError {
    Io(io::Error),
    InvalidArtifact(String),
    InvalidArchive(String),
}

impl fmt::Display for AuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::InvalidArtifact(message) => write!(f, "invalid Android artifact: {message}"),
            Self::InvalidArchive(message) => write!(f, "invalid ZIP archive: {message}"),
        }
    }
}

impl std::error::Error for AuditError {}

impl From<io::Error> for AuditError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

pub fn audit_path(path: impl AsRef<Path>) -> Result<AuditReport, AuditError> {
    let path = path.as_ref();
    let kind = artifact_kind(path).ok_or_else(|| {
        AuditError::InvalidArtifact("expected a file ending in .apk or .aab".to_string())
    })?;

    let mut file = File::open(path)?;
    let metadata = file.metadata()?;
    let mut magic = [0_u8; 4];
    let bytes_read = file.read(&mut magic)?;
    if bytes_read < 4 || magic != *b"PK" {
        return Err(AuditError::InvalidArtifact(
            "file does not start with a ZIP local-file header".to_string(),
        ));
    }

    let file = File::open(path)?;
    let mut archive =
        ZipArchive::new(file).map_err(|error| AuditError::InvalidArchive(error.to_string()))?;
    let mut inventory = inspect_archive(&mut archive, kind)?;

    if kind == ArtifactKind::Apk {
        let mut signing_file = File::open(path)?;
        match signing::inspect_apk_signing_block(&mut signing_file) {
            Ok(info) => inventory.apk_signing = info,
            Err(error) => inventory.apk_signing_error = Some(error.to_string()),
        }

        match signature_verify::verify_apk_signatures(path) {
            Ok(info) => inventory.apk_signature_verification = info,
            Err(error) => inventory.apk_signature_verification_error = Some(error.to_string()),
        }
    }

    let (manifest, manifest_error) = match inventory.manifest_path.as_deref() {
        Some(path) => {
            let mut entry = archive
                .by_name(path)
                .map_err(|error| AuditError::InvalidArchive(error.to_string()))?;
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes)?;

            let parsed = match kind {
                ArtifactKind::Apk => {
                    axml::parse_manifest(&bytes).map_err(|error| error.to_string())
                }
                ArtifactKind::Aab => {
                    manifest_proto::parse_manifest(&bytes).map_err(|error| error.to_string())
                }
            };

            match parsed {
                Ok(manifest) => (Some(manifest), None),
                Err(error) => (None, Some(error)),
            }
        }
        None => (None, None),
    };

    let findings = evaluate(
        kind,
        &inventory,
        manifest.as_ref(),
        manifest_error.as_deref(),
    );

    Ok(AuditReport {
        artifact_path: path.to_path_buf(),
        artifact_kind: kind,
        size_bytes: metadata.len(),
        inventory,
        manifest,
        manifest_error,
        project: None,
        project_error: None,
        findings,
    })
}

pub fn audit_path_with_play(
    artifact_path: impl AsRef<Path>,
    platform: PlayPlatform,
) -> Result<AuditReport, AuditError> {
    let mut report = audit_path(artifact_path)?;
    report.findings.extend(play::evaluate_play_policy(
        report.manifest.as_ref(),
        &report.inventory,
        platform,
    ));
    Ok(report)
}

pub fn audit_path_with_project_and_play(
    artifact_path: impl AsRef<Path>,
    project_path: impl AsRef<Path>,
    platform: PlayPlatform,
) -> Result<AuditReport, AuditError> {
    let mut report = audit_path_with_project(artifact_path, project_path)?;
    report.findings.extend(play::evaluate_play_policy(
        report.manifest.as_ref(),
        &report.inventory,
        platform,
    ));
    Ok(report)
}

pub fn audit_path_with_project(
    artifact_path: impl AsRef<Path>,
    project_path: impl AsRef<Path>,
) -> Result<AuditReport, AuditError> {
    let mut report = audit_path(artifact_path)?;

    match project::parse_project(project_path) {
        Ok(project) => {
            report.findings.extend(evaluate_project_crosscheck(
                report.manifest.as_ref(),
                &project,
            ));
            report.project = Some(project);
        }
        Err(error) => {
            report.project_error = Some(error.to_string());
            report.findings.push(Finding::blocker(
                "PROJECT-001",
                "Project configuration unavailable",
                format!("The Android application Gradle configuration could not be parsed: {error}"),
                "Point --project at the Android application module containing a supported build.gradle or build.gradle.kts file.",
            ));
        }
    }

    Ok(report)
}

fn artifact_kind(path: &Path) -> Option<ArtifactKind> {
    match path
        .extension()
        .and_then(|value| value.to_str())?
        .to_ascii_lowercase()
        .as_str()
    {
        "apk" => Some(ArtifactKind::Apk),
        "aab" => Some(ArtifactKind::Aab),
        _ => None,
    }
}

fn inspect_archive<R: Read + io::Seek>(
    archive: &mut ZipArchive<R>,
    kind: ArtifactKind,
) -> Result<ArtifactInventory, AuditError> {
    let mut inventory = ArtifactInventory::default();

    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| AuditError::InvalidArchive(error.to_string()))?;
        let name = entry.name().replace('\\', "/");
        inventory.entries.push(name.clone());

        if name == "AndroidManifest.xml"
            || (kind == ArtifactKind::Aab && name == "base/manifest/AndroidManifest.xml")
        {
            inventory.manifest_path = Some(name.clone());
        }

        if name.ends_with(".dex") {
            inventory.dex_files.push(name.clone());
        }

        if name.ends_with(".so") {
            let abi = native_abi_from_path(&name).unwrap_or("unknown").to_string();
            if abi != "unknown" && !inventory.native_abis.iter().any(|item| item == &abi) {
                inventory.native_abis.push(abi.clone());
            }

            let compression = zip_alignment::classify_compression(entry.compression());
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes)?;

            let data_offset = entry.data_start();
            let (alignment_16kb, alignment_error) = match kind {
                ArtifactKind::Apk if compression == NativeZipCompression::Stored => (
                    Some(zip_alignment::is_16kb_aligned(data_offset)),
                    None,
                ),
                ArtifactKind::Apk => (None, None),
                ArtifactKind::Aab if compression == NativeZipCompression::Stored => (
                    None,
                    Some(
                        "AAB entry offset does not establish the final APK ZIP alignment; verify the bundle alignment configuration and generated APK."
                            .to_string(),
                    ),
                ),
                ArtifactKind::Aab => (None, None),
            };

            inventory.native_zip_entries.push(NativeZipEntryInfo {
                path: name.clone(),
                abi: abi.clone(),
                compression,
                data_offset,
                alignment_16kb,
                error: alignment_error,
            });

            match elf::inspect_shared_object(&bytes) {
                Ok(inspection) => inventory.native_libraries.push(NativeLibraryInfo {
                    path: name.clone(),
                    abi,
                    load_segment_alignments: inspection.load_segment_alignments,
                    error: None,
                }),
                Err(error) => inventory.native_libraries.push(NativeLibraryInfo {
                    path: name.clone(),
                    abi,
                    load_segment_alignments: Vec::new(),
                    error: Some(error.to_string()),
                }),
            }
        }

        let upper = name.to_ascii_uppercase();
        if upper.starts_with("META-INF/")
            && (upper.ends_with(".RSA")
                || upper.ends_with(".DSA")
                || upper.ends_with(".EC")
                || upper.ends_with(".SF"))
        {
            inventory.signature_files.push(name);
        }
    }

    inventory.native_abis.sort();
    inventory
        .native_libraries
        .sort_by(|left, right| left.path.cmp(&right.path));
    inventory
        .native_zip_entries
        .sort_by(|left, right| left.path.cmp(&right.path));
    inventory.signature_files.sort();
    inventory.dex_files.sort();
    inventory.entries.sort();

    Ok(inventory)
}

fn native_abi_from_path(name: &str) -> Option<&str> {
    name.split('/')
        .collect::<Vec<_>>()
        .windows(2)
        .find_map(|window| (window[0] == "lib").then_some(window[1]))
}

fn evaluate(
    kind: ArtifactKind,
    inventory: &ArtifactInventory,
    manifest: Option<&ManifestInfo>,
    manifest_error: Option<&str>,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    findings.push(Finding::pass(
        "ARTIFACT-001",
        "Artifact type",
        format!("Recognized {} release artifact.", kind.as_str()),
    ));
    findings.push(Finding::pass(
        "ARTIFACT-002",
        "ZIP structure",
        format!("Archive contains {} entries.", inventory.entries.len()),
    ));

    match inventory.manifest_path.as_deref() {
        None => findings.push(Finding::blocker(
            "MANIFEST-001",
            "Manifest missing",
            "No AndroidManifest.xml was found in the expected location.",
            "Produce a complete APK/AAB and run the audit on the release artifact itself.",
        )),
        Some(path) => {
            findings.push(Finding::pass(
                "MANIFEST-001",
                "Manifest present",
                format!("Found AndroidManifest.xml at {path}."),
            ));

            match (manifest, manifest_error) {
                (None, Some(error)) => findings.push(Finding::blocker(
                    "MANIFEST-002",
                    "Manifest unreadable",
                    format!("The manifest entry exists but could not be parsed: {error}"),
                    "Verify that the artifact contains a valid compiled AndroidManifest.xml.",
                )),
                (None, None) => findings.push(Finding::blocker(
                    "MANIFEST-002",
                    "Manifest unavailable",
                    "The manifest entry was found but no parsed manifest data is available.",
                    "Verify that the artifact contains a readable compiled AndroidManifest.xml.",
                )),
                (Some(_), Some(error)) => findings.push(Finding::blocker(
                    "MANIFEST-002",
                    "Manifest state inconsistent",
                    format!(
                        "The manifest parsed successfully but a parser error was also reported: {error}"
                    ),
                    "Report this condition as an Android Release Doctor defect.",
                )),
                (Some(manifest), None) => {
                    if let Some(package_name) = &manifest.package_name {
                        findings.push(Finding::pass(
                            "MANIFEST-003",
                            "Application ID",
                            format!("Package name is {package_name}."),
                        ));
                    } else {
                        findings.push(Finding::blocker(
                            "MANIFEST-003",
                            "Application ID missing",
                            "The manifest does not expose a package name.",
                            "Ensure the final manifest contains the application's package identity.",
                        ));
                    }

                    match (manifest.min_sdk, manifest.target_sdk) {
                        (Some(min_sdk), Some(target_sdk)) => findings.push(Finding::pass(
                            "SDK-001",
                            "SDK levels",
                            format!(
                                "Manifest declares minSdk {min_sdk} and targetSdk {target_sdk}."
                            ),
                        )),
                        _ => findings.push(Finding::warning(
                            "SDK-001",
                            "SDK levels incomplete",
                            "The manifest does not contain both minSdkVersion and targetSdkVersion as integer values.",
                            "Build the release with explicit Android SDK levels so the artifact can be audited precisely.",
                        )),
                    }

                    if manifest.debuggable == Some(true) {
                        findings.push(Finding::blocker(
                            "BUILD-001",
                            "Debuggable release",
                            "The final manifest sets android:debuggable=true.",
                            "Build a non-debuggable release artifact before publication.",
                        ));
                    } else if manifest.debuggable == Some(false) {
                        findings.push(Finding::pass(
                            "BUILD-001",
                            "Debuggable flag",
                            "The final manifest explicitly disables android:debuggable.",
                        ));
                    } else {
                        findings.push(Finding::pass(
                            "BUILD-001",
                            "Debuggable flag",
                            "The final manifest does not declare android:debuggable, so its effective value is false.",
                        ));
                    }

                    match manifest.version_code {
                        Some(version_code) => findings.push(Finding::pass(
                            "VERSION-001",
                            "Version code",
                            format!("Artifact versionCode is {version_code}."),
                        )),
                        None => findings.push(Finding::warning(
                            "VERSION-001",
                            "Version code missing",
                            "The final manifest did not expose an integer versionCode.",
                            "Ensure the release manifest contains an integer android:versionCode.",
                        )),
                    }

                    match manifest.version_name.as_deref() {
                        Some(version_name) => findings.push(Finding::pass(
                            "VERSION-002",
                            "Version name",
                            format!("Artifact versionName is {version_name}."),
                        )),
                        None => findings.push(Finding::warning(
                            "VERSION-002",
                            "Version name missing",
                            "The final manifest did not expose an android:versionName string.",
                            "Ensure the release manifest contains a user-visible version name.",
                        )),
                    }

                    findings.push(Finding::pass(
                        "PERMISSION-001",
                        "Permissions inventory",
                        format!(
                            "Detected {} declared permission(s). No permission-risk classification is applied yet.",
                            manifest.permissions.len()
                        ),
                    ));

                    for component in &manifest.components {
                        let requires_explicit_export = matches!(
                            component.kind.as_str(),
                            "activity" | "activity-alias" | "service" | "receiver"
                        );

                        if !requires_explicit_export || !component.has_intent_filters {
                            continue;
                        }

                        if component.exported.is_none() {
                            findings.push(Finding::blocker(
                                "COMPONENT-001",
                                "Exported component incomplete",
                                format!(
                                    "{} {} has an intent-filter but no explicit android:exported value.",
                                    component.kind, component.name
                                ),
                                "Set android:exported explicitly for components that contain intent filters.",
                            ));
                        } else {
                            findings.push(Finding::pass(
                                "COMPONENT-001",
                                "Exported component declared",
                                format!(
                                    "{} {} declares android:exported.",
                                    component.kind, component.name
                                ),
                            ));
                        }
                    }
                }
            }
        }
    }

    if inventory.dex_files.is_empty() {
        findings.push(Finding::warning(
            "DEX-001",
            "DEX payload",
            "No DEX file was detected.",
            "Verify that the submitted artifact contains the compiled application code.",
        ));
    } else {
        findings.push(Finding::pass(
            "DEX-001",
            "DEX payload",
            format!("Detected {} DEX file(s).", inventory.dex_files.len()),
        ));
    }

    if inventory.native_libraries.is_empty() {
        findings.push(Finding::pass(
            "NATIVE-001",
            "Native libraries",
            "No native .so libraries are present; ABI coverage is not applicable to this artifact.",
        ));
    } else if inventory.native_abis.is_empty() {
        findings.push(Finding::pass(
            "NATIVE-001",
            "Native libraries",
            format!(
                "Detected {} native .so library file(s), but no standard lib/<abi>/ path was available for ABI classification.",
                inventory.native_libraries.len()
            ),
        ));
    } else {
        findings.push(Finding::pass(
            "NATIVE-001",
            "Native libraries",
            format!(
                "Detected native libraries for ABI(s): {}.",
                inventory.native_abis.join(", ")
            ),
        ));
    }

    if inventory
        .native_libraries
        .iter()
        .any(|library| library.error.is_some())
    {
        let paths = inventory
            .native_libraries
            .iter()
            .filter(|library| library.error.is_some())
            .map(|library| library.path.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        findings.push(Finding::manual_review(
            "NATIVE-002",
            "16 KB ELF inspection incomplete",
            format!("Could not parse the native ELF program headers for: {paths}."),
            "Inspect those shared objects with an ELF-aware tool and verify 16 KB page-size compatibility before release.",
        ));
    } else if inventory.native_libraries.is_empty() {
        findings.push(Finding::pass(
            "NATIVE-002",
            "16 KB ELF inspection",
            "No native .so libraries are present, so ELF page alignment is not applicable.",
        ));
    } else {
        let incompatible = inventory
            .native_libraries
            .iter()
            .filter(|library| !load_segments_are_16kb_aligned(&library.load_segment_alignments))
            .collect::<Vec<_>>();

        if incompatible.is_empty() {
            findings.push(Finding::pass(
                "NATIVE-002",
                "16 KB ELF alignment",
                "All inspected native ELF PT_LOAD segments meet the 16 KB alignment threshold.",
            ));
        } else {
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
                "NATIVE-002",
                "16 KB ELF alignment",
                format!("One or more native libraries contain PT_LOAD alignment below 16 KB: {details}."),
                "Rebuild or replace the affected native libraries with 16 KB ELF load-segment alignment.",
            ));
        }
    }

    if inventory.native_zip_entries.is_empty() {
        findings.push(Finding::pass(
            "NATIVE-003",
            "16 KB ZIP packaging",
            "No native .so libraries are present, so native ZIP packaging alignment is not applicable.",
        ));
    } else {
        let packaging_errors = inventory
            .native_zip_entries
            .iter()
            .filter(|entry| entry.error.is_some())
            .collect::<Vec<_>>();
        let incompatible = inventory
            .native_zip_entries
            .iter()
            .filter(|entry| entry.alignment_16kb == Some(false))
            .collect::<Vec<_>>();
        let stored = inventory
            .native_zip_entries
            .iter()
            .filter(|entry| entry.compression == NativeZipCompression::Stored)
            .collect::<Vec<_>>();

        if !incompatible.is_empty() {
            let details = incompatible
                .iter()
                .map(|entry| {
                    format!(
                        "{} [{}] offset={}",
                        entry.path, entry.abi, entry.data_offset
                    )
                })
                .collect::<Vec<_>>()
                .join("; ");
            findings.push(Finding::warning(
                "NATIVE-003",
                "16 KB ZIP packaging alignment",
                format!(
                    "One or more uncompressed native libraries are not aligned to a 16 KB ZIP data boundary: {details}."
                ),
                "Repackage the affected uncompressed native libraries with 16 KB ZIP alignment, or use compressed native libraries.",
            ));
        } else if !packaging_errors.is_empty() {
            let details = packaging_errors
                .iter()
                .map(|entry| {
                    format!(
                        "{} [{}]: {}",
                        entry.path,
                        entry.abi,
                        entry
                            .error
                            .as_deref()
                            .unwrap_or("alignment could not be verified")
                    )
                })
                .collect::<Vec<_>>()
                .join("; ");
            findings.push(Finding::manual_review(
                "NATIVE-003",
                "16 KB ZIP packaging alignment unavailable",
                format!("Native ZIP packaging alignment could not be proven: {details}"),
                "For APKs, verify uncompressed native libraries with zipalign. For AABs, verify the bundle's ZIP alignment configuration and the generated APK before release.",
            ));
        } else if stored.is_empty() {
            findings.push(Finding::pass(
                "NATIVE-003",
                "16 KB ZIP packaging",
                "All packaged native libraries are compressed, so uncompressed ZIP data alignment is not applicable.",
            ));
        } else {
            findings.push(Finding::pass(
                "NATIVE-003",
                "16 KB ZIP packaging alignment",
                "All inspected uncompressed native APK libraries have 16 KB-aligned ZIP data offsets.",
            ));
        }
    }

    if inventory.signature_files.is_empty() {
        let has_modern_apk_signing = matches!(kind, ArtifactKind::Apk)
            && inventory
                .apk_signing
                .as_ref()
                .is_some_and(|info| info.v2 || info.v3 || info.v31 || info.v32);

        if has_modern_apk_signing {
            findings.push(Finding::pass(
                "SIGNING-001",
                "Legacy v1 signature material",
                "No META-INF signature files were found; modern APK signing evidence is reported by SIGNING-002 and SIGNING-003.",
            ));
        } else {
            findings.push(Finding::warning(
                "SIGNING-001",
                "Legacy v1 signature material",
                "No META-INF signature files were found and no modern APK signing-block evidence is available.",
                "Use a signed release artifact and verify its signing scheme before publication.",
            ));
        }
    } else {
        findings.push(Finding::pass(
            "SIGNING-001",
            "Legacy v1 signature material",
            format!(
                "Detected {} META-INF signature metadata file(s).",
                inventory.signature_files.len()
            ),
        ));
    }

    match kind {
        ArtifactKind::Aab => findings.push(Finding::pass(
            "SIGNING-002",
            "APK signing block",
            "APK signing-block inspection is not applicable to an AAB.",
        )),
        ArtifactKind::Apk => match (&inventory.apk_signing, &inventory.apk_signing_error) {
            (_, Some(error)) => findings.push(Finding::manual_review(
                "SIGNING-002",
                "APK signing block unreadable",
                format!("The APK signing-block structure could not be validated: {error}"),
                "Verify the APK with apksigner and rerun the audit before publication.",
            )),
            (Some(info), None) if info.v2 || info.v3 || info.v31 || info.v32 => {
                let mut schemes = Vec::new();
                if info.v2 {
                    schemes.push("v2");
                }
                if info.v3 {
                    schemes.push("v3");
                }
                if info.v31 {
                    schemes.push("v3.1");
                }
                if info.v32 {
                    schemes.push("v3.2");
                }
                findings.push(Finding::pass(
                    "SIGNING-002",
                    "APK signing block",
                    format!(
                        "Detected APK signing-block scheme(s): {}.",
                        schemes.join(", ")
                    ),
                ));
            }
            (Some(_), None) => findings.push(Finding::warning(
                "SIGNING-002",
                "APK signing block",
                "An APK signing block is present, but no supported v2/v3/v3.1/v3.2 signing scheme block was detected.",
                "Verify the APK signing scheme with apksigner and publish only the intended signed release artifact.",
            )),
            (None, None) => findings.push(Finding::warning(
                "SIGNING-002",
                "APK signing block missing",
                "No APK signing block was detected. The artifact may be v1-only signed or unsigned.",
                "Verify the APK signing schemes with apksigner before publication.",
            )),
        },
    }

    match kind {
        ArtifactKind::Aab => findings.push(Finding::pass(
            "SIGNING-003",
            "APK cryptographic signature verification",
            "Cryptographic APK signature verification is not applicable to an AAB.",
        )),
        ArtifactKind::Apk => {
            if let Some(error) = &inventory.apk_signature_verification_error {
                findings.push(Finding::manual_review(
                    "SIGNING-003",
                    "APK signature verification unavailable",
                    format!("Cryptographic APK signature verification could not be completed: {error}"),
                    "Verify the APK with apksigner and review the signing scheme before publication.",
                ));
            } else if let Some(verification) = &inventory.apk_signature_verification {
                let schemes = [
                    verification.v2.as_ref(),
                    verification.v3.as_ref(),
                    verification.v31.as_ref(),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                if schemes.is_empty() {
                    findings.push(Finding::manual_review(
                        "SIGNING-003",
                        "APK cryptographic signature unavailable",
                        "An APK signing block was found, but no supported v2/v3/v3.1 cryptographic scheme was available for verification.",
                        "Verify the intended signing scheme with apksigner before publication.",
                    ));
                } else if schemes
                    .iter()
                    .any(|scheme| scheme.state == CryptoVerificationState::Invalid)
                {
                    let details = schemes
                        .iter()
                        .filter(|scheme| scheme.state == CryptoVerificationState::Invalid)
                        .map(|scheme| scheme.detail.as_str())
                        .collect::<Vec<_>>()
                        .join("; ");
                    findings.push(Finding::blocker(
                        "SIGNING-003",
                        "APK cryptographic signature invalid",
                        format!("At least one detected APK signing scheme failed cryptographic verification: {details}"),
                        "Re-sign the release APK with a valid supported v2/v3/v3.1 signing configuration and rerun the audit.",
                    ));
                } else {
                    let v31_requires_manual = verification.v31_present
                        && !matches!(
                            verification.v31.as_ref(),
                            Some(scheme) if scheme.state == CryptoVerificationState::Verified
                        );

                    if schemes
                        .iter()
                        .any(|scheme| scheme.state == CryptoVerificationState::Unsupported)
                        || v31_requires_manual
                        || verification.v32_present
                    {
                        let details = schemes
                            .iter()
                            .filter(|scheme| scheme.state == CryptoVerificationState::Unsupported)
                            .map(|scheme| scheme.detail.as_str())
                            .collect::<Vec<_>>()
                            .join("; ");
                        let mut suffix = String::new();
                        if v31_requires_manual {
                            suffix.push_str(
                                " A v3.1 signing block is present but did not produce a verified v3.1 scheme result.",
                            );
                        }
                        if verification.v32_present {
                            suffix.push_str(
                                " A v3.2 hybrid signing block is present and is not cryptographically verified in this block.",
                            );
                        }
                        findings.push(Finding::manual_review(
                            "SIGNING-003",
                            "APK cryptographic verification requires manual review",
                            format!(
                                "The current verifier confirmed the supported cryptographic parts, but complete verification is not available: {details}.{suffix}"
                            ),
                            "Verify the APK with apksigner and review the reported signing scheme, proof-of-rotation, and any unsupported v3.2 evidence before publication.",
                        ));
                    } else {
                        findings.push(Finding::pass(
                            "SIGNING-003",
                            "APK cryptographic signature verified",
                            "The supported v2/v3/v3.1 APK signer data, certificate/public-key binding, cryptographic signatures, and APK content digests were verified.",
                        ));
                    }
                }
            } else {
                findings.push(Finding::warning(
                    "SIGNING-003",
                    "APK cryptographic signature unavailable",
                    "No v2/v3/v3.1 cryptographic verification result is available for this APK.",
                    "Verify the release APK signing scheme with apksigner before publication.",
                ));
            }
        }
    }

    findings
}

fn evaluate_project_crosscheck(
    manifest: Option<&ManifestInfo>,
    project: &ProjectInfo,
) -> Vec<Finding> {
    let mut findings = vec![Finding::pass(
        "PROJECT-001",
        "Project configuration",
        format!(
            "Loaded {} Android application Gradle configuration from {}.",
            project.syntax.as_str(),
            project.build_file.display()
        ),
    )];

    let Some(manifest) = manifest else {
        findings.push(Finding::warning(
            "CROSSCHECK-000",
            "Project/artifact comparison skipped",
            "The project configuration was parsed, but the final artifact manifest is unavailable.",
            "Resolve the artifact manifest findings before relying on project-vs-artifact comparison.",
        ));
        return findings;
    };

    match (&project.application_id, &manifest.package_name) {
        (Some(project_id), Some(artifact_id)) if project_id == artifact_id => findings.push(
            Finding::pass(
                "CROSSCHECK-001",
                "Application ID match",
                format!("Project applicationId and artifact package are both {artifact_id}."),
            ),
        ),
        (Some(project_id), Some(artifact_id)) => findings.push(Finding::blocker(
            "CROSSCHECK-001",
            "Application ID mismatch",
            format!(
                "Project defaultConfig declares applicationId {project_id}, but the artifact package is {artifact_id}."
            ),
            "Build and audit the artifact produced by the intended Android application module and release variant.",
        )),
        _ => findings.push(Finding::warning(
            "CROSSCHECK-001",
            "Application ID not comparable",
            "The project or artifact does not expose a comparable application identity.",
            "Ensure defaultConfig contains a literal applicationId and the artifact manifest contains its final package identity.",
        )),
    }

    compare_sdk(
        &mut findings,
        "CROSSCHECK-002",
        "targetSdk match",
        project.target_sdk,
        manifest.target_sdk,
        true,
        "Build the release from the intended project configuration and audit the resulting artifact.",
    );
    compare_sdk(
        &mut findings,
        "CROSSCHECK-003",
        "minSdk match",
        project.min_sdk,
        manifest.min_sdk,
        false,
        "Review variant or flavor configuration and rebuild the release artifact.",
    );

    match (project.version_code, manifest.version_code) {
        (Some(project_value), Some(artifact_value)) if project_value == artifact_value => {
            findings.push(Finding::pass(
                "CROSSCHECK-004",
                "Version code match",
                format!("Project versionCode and artifact versionCode are both {artifact_value}."),
            ));
        }
        (Some(project_value), Some(artifact_value)) => findings.push(Finding::warning(
            "CROSSCHECK-004",
            "Version code mismatch",
            format!(
                "Project defaultConfig declares versionCode {project_value}, but the artifact contains {artifact_value}."
            ),
            "Verify that no variant-specific or CI override changed the final release versionCode unexpectedly.",
        )),
        _ => findings.push(Finding::warning(
            "CROSSCHECK-004",
            "Version code not comparable",
            "The project or artifact does not expose a comparable versionCode.",
            "Use a literal defaultConfig versionCode when static project-vs-artifact comparison is required.",
        )),
    }

    match (&project.version_name, &manifest.version_name) {
        (Some(project_value), Some(artifact_value)) if project_value == artifact_value => {
            findings.push(Finding::pass(
                "CROSSCHECK-005",
                "Version name match",
                format!("Project versionName and artifact versionName are both {artifact_value}."),
            ));
        }
        (Some(project_value), Some(artifact_value)) => findings.push(Finding::warning(
            "CROSSCHECK-005",
            "Version name mismatch",
            format!(
                "Project defaultConfig declares versionName {project_value}, but the artifact contains {artifact_value}."
            ),
            "Verify that no variant-specific or CI override changed the final release versionName unexpectedly.",
        )),
        _ => findings.push(Finding::warning(
            "CROSSCHECK-005",
            "Version name not comparable",
            "The project or artifact does not expose a comparable versionName.",
            "Use a literal defaultConfig versionName when static project-vs-artifact comparison is required.",
        )),
    }

    match (project.release_debuggable, manifest.debuggable.unwrap_or(false)) {
        (Some(true), true) | (Some(false), false) => findings.push(Finding::pass(
            "CROSSCHECK-006",
            "Release debuggable match",
            "Project release configuration and the artifact agree on the effective android:debuggable value.",
        )),
        (Some(true), false) => findings.push(Finding::blocker(
            "CROSSCHECK-006",
            "Release debuggable mismatch",
            "The project release configuration enables debuggable, while the audited artifact is non-debuggable.",
            "Verify the release build type and ensure the project configuration being audited matches the artifact you intend to publish.",
        )),
        (Some(false), true) => findings.push(Finding::blocker(
            "CROSSCHECK-006",
            "Release debuggable mismatch",
            "The project release configuration disables debuggable, while the audited artifact is debuggable.",
            "Do not publish the artifact until the intended release configuration produces a non-debuggable artifact.",
        )),
        (None, _) => findings.push(Finding::warning(
            "CROSSCHECK-006",
            "Release debuggable not declared",
            "The parsed release build type does not contain an explicit debuggable/isDebuggable value.",
            "Use an explicit non-debuggable release configuration when static project-vs-artifact comparison is required.",
        )),
    }

    findings
}

fn compare_sdk(
    findings: &mut Vec<Finding>,
    rule_id: &'static str,
    title: &'static str,
    project_value: Option<u32>,
    artifact_value: Option<u32>,
    mismatch_is_blocker: bool,
    remediation: &'static str,
) {
    match (project_value, artifact_value) {
        (Some(project_value), Some(artifact_value)) if project_value == artifact_value => {
            findings.push(Finding::pass(
                rule_id,
                title,
                format!(
                    "Project and artifact both declare SDK level {artifact_value}."
                ),
            ));
        }
        (Some(project_value), Some(artifact_value)) => {
            let message = format!(
                "Project declares SDK level {project_value}, but the artifact contains {artifact_value}."
            );
            if mismatch_is_blocker {
                findings.push(Finding::blocker(
                    rule_id,
                    title,
                    message,
                    remediation,
                ));
            } else {
                findings.push(Finding::warning(
                    rule_id,
                    title,
                    message,
                    remediation,
                ));
            }
        }
        _ => findings.push(Finding::warning(
            rule_id,
            title,
            "The project or artifact does not expose a comparable SDK level.",
            "Use literal Gradle SDK declarations when static project-vs-artifact comparison is required.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual_review_is_first_class_and_does_not_count_as_blocker() {
        let report = AuditReport {
            artifact_path: PathBuf::from("app.apk"),
            artifact_kind: ArtifactKind::Apk,
            size_bytes: 0,
            inventory: ArtifactInventory::default(),
            manifest: None,
            manifest_error: None,
            project: None,
            project_error: None,
            findings: vec![Finding::manual_review(
                "TEST-MANUAL",
                "Manual verification",
                "Evidence is incomplete.",
                "Verify externally.",
            )],
        };

        assert_eq!(report.counts(), (0, 0, 0));
        assert_eq!(report.manual_review_count(), 1);
        assert!(report.render_text().contains("MANUAL-REVIEW"));
        assert!(report.render_text().contains("MANUAL REVIEW 1"));
    }

    #[test]
    fn unavailable_native_elf_evidence_requires_manual_review() {
        let inventory = ArtifactInventory {
            native_abis: vec!["arm64-v8a".to_string()],
            native_libraries: vec![NativeLibraryInfo {
                path: "lib/arm64-v8a/libunknown.so".to_string(),
                abi: "arm64-v8a".to_string(),
                load_segment_alignments: Vec::new(),
                error: Some("missing ELF magic".to_string()),
            }],
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Apk, &inventory, None, None);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "NATIVE-002")
                .map(|finding| finding.severity),
            Some(Severity::ManualReview)
        );
    }

    #[test]
    fn unavailable_native_zip_evidence_requires_manual_review() {
        let inventory = ArtifactInventory {
            native_zip_entries: vec![NativeZipEntryInfo {
                path: "base/lib/arm64-v8a/libnative.so".to_string(),
                abi: "arm64-v8a".to_string(),
                compression: NativeZipCompression::Stored,
                data_offset: ZIP_ALIGNMENT_16KB,
                alignment_16kb: None,
                error: Some("AAB offset is not final APK evidence".to_string()),
            }],
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Aab, &inventory, None, None);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "NATIVE-003")
                .map(|finding| finding.severity),
            Some(Severity::ManualReview)
        );
    }

    #[test]
    fn unavailable_apk_signing_block_requires_manual_review() {
        let inventory = ArtifactInventory {
            apk_signing_error: Some("truncated APK signing block".to_string()),
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Apk, &inventory, None, None);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "SIGNING-002")
                .map(|finding| finding.severity),
            Some(Severity::ManualReview)
        );
    }

    #[test]
    fn unavailable_apk_crypto_verification_requires_manual_review() {
        let inventory = ArtifactInventory {
            apk_signature_verification_error: Some(
                "cryptographic verification capability unavailable".to_string(),
            ),
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Apk, &inventory, None, None);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "SIGNING-003")
                .map(|finding| finding.severity),
            Some(Severity::ManualReview)
        );
    }

    fn test_crypto_scheme(state: CryptoVerificationState, detail: &str) -> CryptoSchemeInfo {
        CryptoSchemeInfo {
            state,
            signer_count: 1,
            algorithms: vec![0x0101],
            certificate_sha256: vec!["a".repeat(64)],
            sdk_ranges: vec![(28, 32)],
            rotation_min_sdk: Some(32),
            rotation_targets_dev_release: false,
            proof_of_rotation: Vec::new(),
            detail: detail.to_string(),
        }
    }

    #[test]
    fn verified_v31_is_accepted_by_signing_003() {
        let inventory = ArtifactInventory {
            apk_signature_verification: Some(ApkSignatureVerification {
                v31: Some(test_crypto_scheme(
                    CryptoVerificationState::Verified,
                    "v3.1 verified",
                )),
                v31_present: true,
                ..Default::default()
            }),
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Apk, &inventory, None, None);
        let finding = findings
            .iter()
            .find(|finding| finding.rule_id == "SIGNING-003")
            .expect("SIGNING-003 should be emitted");

        assert_eq!(finding.severity, Severity::Pass);
        assert!(finding.summary.contains("v3.1"));
    }

    #[test]
    fn invalid_v31_remains_a_signing_003_blocker() {
        let inventory = ArtifactInventory {
            apk_signature_verification: Some(ApkSignatureVerification {
                v31: Some(test_crypto_scheme(
                    CryptoVerificationState::Invalid,
                    "v3.1 verification failed",
                )),
                v31_present: true,
                ..Default::default()
            }),
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Apk, &inventory, None, None);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "SIGNING-003")
                .map(|finding| finding.severity),
            Some(Severity::Blocker)
        );
    }

    #[test]
    fn unsupported_v31_remains_manual_review() {
        let inventory = ArtifactInventory {
            apk_signature_verification: Some(ApkSignatureVerification {
                v31: Some(test_crypto_scheme(
                    CryptoVerificationState::Unsupported,
                    "unsupported v3.1 algorithm",
                )),
                v31_present: true,
                ..Default::default()
            }),
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Apk, &inventory, None, None);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "SIGNING-003")
                .map(|finding| finding.severity),
            Some(Severity::ManualReview)
        );
    }

    #[test]
    fn v32_presence_remains_manual_review_boundary() {
        let inventory = ArtifactInventory {
            apk_signature_verification: Some(ApkSignatureVerification {
                v2: Some(test_crypto_scheme(
                    CryptoVerificationState::Verified,
                    "v2 verified",
                )),
                v32_present: true,
                ..Default::default()
            }),
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Apk, &inventory, None, None);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "SIGNING-003")
                .map(|finding| finding.severity),
            Some(Severity::ManualReview)
        );
    }

    #[test]
    fn missing_manifest_is_a_blocker() {
        let inventory = ArtifactInventory {
            entries: vec!["classes.dex".to_string()],
            ..Default::default()
        };
        let findings = evaluate(ArtifactKind::Apk, &inventory, None, None);
        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "MANIFEST-001")
                .map(|finding| finding.severity),
            Some(Severity::Blocker)
        );
    }

    #[test]
    fn aab_inventory_accepts_base_manifest_and_multiple_abis() {
        let inventory = ArtifactInventory {
            manifest_path: Some("base/manifest/AndroidManifest.xml".to_string()),
            dex_files: vec!["base/dex/classes.dex".to_string()],
            native_abis: vec!["arm64-v8a".to_string(), "armeabi-v7a".to_string()],
            ..Default::default()
        };
        let findings = evaluate(ArtifactKind::Aab, &inventory, None, None);
        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "MANIFEST-001")
                .unwrap()
                .severity,
            Severity::Pass
        );
        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "NATIVE-001")
                .unwrap()
                .severity,
            Severity::Pass
        );
    }

    #[test]
    fn missing_debuggable_attribute_has_effective_false_value() {
        let inventory = ArtifactInventory {
            manifest_path: Some("AndroidManifest.xml".to_string()),
            ..Default::default()
        };
        let manifest = ManifestInfo {
            package_name: Some("com.example.app".to_string()),
            min_sdk: Some(24),
            target_sdk: Some(35),
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Apk, &inventory, Some(&manifest), None);

        let finding = findings
            .iter()
            .find(|finding| finding.rule_id == "BUILD-001")
            .expect("BUILD-001 should be present");

        assert_eq!(finding.severity, Severity::Pass);
        assert!(finding.summary.contains("effective value is false"));
    }

    #[test]
    fn debug_enabled_is_a_blocker() {
        let inventory = ArtifactInventory {
            manifest_path: Some("AndroidManifest.xml".to_string()),
            ..Default::default()
        };
        let manifest = ManifestInfo {
            package_name: Some("com.example.app".to_string()),
            debuggable: Some(true),
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Apk, &inventory, Some(&manifest), None);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "BUILD-001")
                .map(|finding| finding.severity),
            Some(Severity::Blocker)
        );
    }

    #[test]
    fn manifest_parse_error_is_reported_as_a_blocker() {
        let inventory = ArtifactInventory {
            manifest_path: Some("AndroidManifest.xml".to_string()),
            ..Default::default()
        };

        let findings = evaluate(
            ArtifactKind::Apk,
            &inventory,
            None,
            Some("truncated Android binary XML"),
        );

        let finding = findings
            .iter()
            .find(|finding| finding.rule_id == "MANIFEST-002")
            .expect("MANIFEST-002 should be present");

        assert_eq!(finding.severity, Severity::Blocker);
        assert!(finding.summary.contains("truncated Android binary XML"));
    }

    #[test]
    fn component_without_exported_is_a_blocker() {
        let inventory = ArtifactInventory {
            manifest_path: Some("AndroidManifest.xml".to_string()),
            ..Default::default()
        };
        let manifest = ManifestInfo {
            package_name: Some("com.example.app".to_string()),
            components: vec![ComponentInfo {
                kind: "activity".to_string(),
                name: "com.example.app.MainActivity".to_string(),
                exported: None,
                has_intent_filters: true,
            }],
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Apk, &inventory, Some(&manifest), None);

        assert_eq!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "COMPONENT-001")
                .map(|finding| finding.severity),
            Some(Severity::Blocker)
        );
    }

    #[test]
    fn provider_intent_filter_is_not_subject_to_android12_export_requirement() {
        let inventory = ArtifactInventory {
            manifest_path: Some("AndroidManifest.xml".to_string()),
            ..Default::default()
        };
        let manifest = ManifestInfo {
            package_name: Some("com.example.app".to_string()),
            components: vec![ComponentInfo {
                kind: "provider".to_string(),
                name: "com.example.app.Provider".to_string(),
                exported: None,
                has_intent_filters: true,
            }],
            ..Default::default()
        };

        let findings = evaluate(ArtifactKind::Apk, &inventory, Some(&manifest), None);

        assert!(findings
            .iter()
            .find(|finding| finding.rule_id == "COMPONENT-001")
            .is_none());
    }

    #[test]
    fn artifact_kind_is_extension_based() {
        assert_eq!(
            artifact_kind(Path::new("release.apk")),
            Some(ArtifactKind::Apk)
        );
        assert_eq!(
            artifact_kind(Path::new("release.AAB")),
            Some(ArtifactKind::Aab)
        );
        assert_eq!(artifact_kind(Path::new("release.zip")), None);
    }
}
