use std::{
    fmt,
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use zip::ZipArchive;

pub mod axml;
pub use axml::{ComponentInfo, ManifestInfo};

pub const ENGINE_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Pass,
    Warning,
    Blocker,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Warning => "WARNING",
            Self::Blocker => "BLOCKER",
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtifactInventory {
    pub entries: Vec<String>,
    pub manifest_path: Option<String>,
    pub dex_files: Vec<String>,
    pub native_abis: Vec<String>,
    pub signature_files: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AuditReport {
    pub artifact_path: PathBuf,
    pub artifact_kind: ArtifactKind,
    pub size_bytes: u64,
    pub inventory: ArtifactInventory,
    pub manifest: Option<ManifestInfo>,
    pub manifest_error: Option<String>,
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
            })
    }

    pub fn render_text(&self) -> String {
        let (passes, warnings, blockers) = self.counts();
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
  PASSED {}
",
            blockers, warnings, passes
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
    let inventory = inspect_archive(&mut archive, kind)?;

    let (manifest, manifest_error) = match inventory.manifest_path.as_deref() {
        Some(path) => {
            let mut entry = archive
                .by_name(path)
                .map_err(|error| AuditError::InvalidArchive(error.to_string()))?;
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes)?;

            match axml::parse_manifest(&bytes) {
                Ok(manifest) => (Some(manifest), None),
                Err(error) => (None, Some(error.to_string())),
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
        findings,
    })
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
        let entry = archive
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

        let parts: Vec<&str> = name.split('/').collect();
        for window in parts.windows(2) {
            if window[0] == "lib" && name.ends_with(".so") {
                let abi = window[1];
                if !abi.is_empty() && !inventory.native_abis.iter().any(|item| item == abi) {
                    inventory.native_abis.push(abi.to_string());
                }
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
    inventory.signature_files.sort();
    inventory.dex_files.sort();
    inventory.entries.sort();

    Ok(inventory)
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

    if inventory.native_abis.is_empty() {
        findings.push(Finding::pass(
            "NATIVE-001",
            "Native libraries",
            "No native .so libraries are present; ABI coverage is not applicable to this artifact.",
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

    if inventory.signature_files.is_empty() {
        findings.push(Finding::warning(
            "SIGNING-001",
            "Signature material",
            "No META-INF signature files were found.",
            "Use a signed release artifact. Cryptographic signature validation will be added in a later rule set.",
        ));
    } else {
        findings.push(Finding::pass(
            "SIGNING-001",
            "Signature material",
            format!(
                "Detected {} signature metadata file(s).",
                inventory.signature_files.len()
            ),
        ));
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let findings = evaluate(
            ArtifactKind::Apk,
            &inventory,
            Some(&manifest),
            None,
        );

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

        let findings = evaluate(
            ArtifactKind::Apk,
            &inventory,
            Some(&manifest),
            None,
        );

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

        let findings = evaluate(
            ArtifactKind::Apk,
            &inventory,
            Some(&manifest),
            None,
        );

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

        let findings = evaluate(
            ArtifactKind::Apk,
            &inventory,
            Some(&manifest),
            None,
        );

        assert!(
            findings
                .iter()
                .find(|finding| finding.rule_id == "COMPONENT-001")
                .is_none()
        );
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
