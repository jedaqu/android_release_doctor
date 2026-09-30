use std::{
    fmt,
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use zip::ZipArchive;

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

        out.push_str("ANDROID RELEASE REPORT\n");
        out.push_str("=======================\n\n");
        out.push_str(&format!(
            "Artifact\n  Type: {}\n  Path: {}\n  Size: {} bytes\n  Engine: {}\n\n",
            self.artifact_kind.as_str(),
            self.artifact_path.display(),
            self.size_bytes,
            ENGINE_VERSION
        ));

        out.push_str("Checks\n");
        for finding in &self.findings {
            out.push_str(&format!(
                "  {:<7} {:<24} [{}]\n",
                finding.severity.as_str(),
                finding.title,
                finding.rule_id
            ));
            out.push_str(&format!("      {}\n", finding.summary));
            if !finding.remediation.is_empty() {
                out.push_str(&format!("      Fix: {}\n", finding.remediation));
            }
        }

        out.push_str("\nSummary\n");
        out.push_str(&format!(
            "  BLOCKERS {}\n  WARNINGS {}\n  PASSED {}\n",
            blockers, warnings, passes
        ));
        out.push_str("\nCurrent coverage\n");
        out.push_str(
            "  Structural artifact inspection only. Manifest fields, SDK levels, debug/release flags and cryptographic signature verification are not yet evaluated.\n",
        );

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
    if bytes_read < 4 || magic != *b"PK\x03\x04" {
        return Err(AuditError::InvalidArtifact(
            "file does not start with a ZIP local-file header".to_string(),
        ));
    }

    let file = File::open(path)?;
    let mut archive =
        ZipArchive::new(file).map_err(|error| AuditError::InvalidArchive(error.to_string()))?;
    let inventory = inspect_archive(&mut archive, kind)?;

    let findings = evaluate(kind, &inventory);

    Ok(AuditReport {
        artifact_path: path.to_path_buf(),
        artifact_kind: kind,
        size_bytes: metadata.len(),
        inventory,
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

fn evaluate(kind: ArtifactKind, inventory: &ArtifactInventory) -> Vec<Finding> {
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
        Some(path) => findings.push(Finding::pass(
            "MANIFEST-001",
            "Manifest present",
            format!("Found AndroidManifest.xml at {path}."),
        )),
        None => findings.push(Finding::blocker(
            "MANIFEST-001",
            "Manifest missing",
            "No AndroidManifest.xml was found in the expected location.",
            "Produce a complete APK/AAB and run the audit on the release artifact itself.",
        )),
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
        let findings = evaluate(ArtifactKind::Apk, &inventory);
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
        let findings = evaluate(ArtifactKind::Aab, &inventory);
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
