use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradleSyntax {
    Groovy,
    Kotlin,
}

impl GradleSyntax {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Groovy => "Groovy",
            Self::Kotlin => "Kotlin DSL",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectInfo {
    pub build_file: PathBuf,
    pub syntax: GradleSyntax,
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

#[derive(Debug)]
pub enum ProjectError {
    Io(std::io::Error),
    InvalidPath(String),
    BuildFileNotFound(PathBuf),
    AmbiguousBuildFiles(Vec<PathBuf>),
    Parse(String),
}

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::InvalidPath(message) => write!(f, "invalid project path: {message}"),
            Self::BuildFileNotFound(path) => {
                write!(
                    f,
                    "could not find an Android application build.gradle file under {}",
                    path.display()
                )
            }
            Self::AmbiguousBuildFiles(paths) => {
                write!(
                    f,
                    "multiple Android application build files were found: {}",
                    format_paths(paths)
                )
            }
            Self::Parse(message) => write!(f, "Gradle parse error: {message}"),
        }
    }
}

impl std::error::Error for ProjectError {}

impl From<std::io::Error> for ProjectError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub fn parse_project(path: impl AsRef<Path>) -> Result<ProjectInfo, ProjectError> {
    let input = path.as_ref();
    let build_file = discover_application_build_file(input)?;
    let source = fs::read_to_string(&build_file)?;

    let sanitized = strip_comments(&source);
    let syntax = match build_file.extension().and_then(|value| value.to_str()) {
        Some("kts") => GradleSyntax::Kotlin,
        _ => GradleSyntax::Groovy,
    };

    let android_block = find_named_block(&sanitized, "android")
        .ok_or_else(|| ProjectError::Parse("no android { ... } block was found".to_string()))?;

    let default_config = find_named_block(&android_block, "defaultConfig");
    let build_types = find_named_block(&android_block, "buildTypes");
    let release = build_types
        .as_deref()
        .and_then(|value| find_named_block(value, "release"));

    let namespace = extract_string_value(&android_block, "namespace");
    let compile_sdk = extract_integer_value(&android_block, "compileSdk");

    let application_id = default_config
        .as_deref()
        .and_then(|value| extract_string_value(value, "applicationId"));
    let min_sdk = default_config
        .as_deref()
        .and_then(|value| extract_integer_value(value, "minSdk"));
    let target_sdk = default_config
        .as_deref()
        .and_then(|value| extract_integer_value(value, "targetSdk"));
    let version_code = default_config
        .as_deref()
        .and_then(|value| extract_integer_value(value, "versionCode"));
    let version_name = default_config
        .as_deref()
        .and_then(|value| extract_string_value(value, "versionName"));

    let release_debuggable = release.as_deref().and_then(|value| {
        extract_bool_value(value, "isDebuggable")
            .or_else(|| extract_bool_value(value, "debuggable"))
    });

    Ok(ProjectInfo {
        build_file: build_file.clone(),
        syntax,
        module_name: build_file
            .parent()
            .and_then(|value| value.file_name())
            .and_then(|value| value.to_str())
            .unwrap_or("<root>")
            .to_string(),
        namespace,
        application_id,
        compile_sdk,
        min_sdk,
        target_sdk,
        version_code,
        version_name,
        release_debuggable,
    })
}

fn discover_application_build_file(input: &Path) -> Result<PathBuf, ProjectError> {
    if input.is_file() {
        if is_gradle_build_file(input) {
            return Ok(input.to_path_buf());
        }
        return Err(ProjectError::InvalidPath(format!(
            "{} is not build.gradle or build.gradle.kts",
            input.display()
        )));
    }

    if !input.is_dir() {
        return Err(ProjectError::InvalidPath(format!(
            "{} does not exist",
            input.display()
        )));
    }

    let version_catalog = read_version_catalog(input)?;

    for name in ["build.gradle", "build.gradle.kts"] {
        let candidate = input.join(name);
        if candidate.is_file() {
            let source = fs::read_to_string(&candidate)?;
            if looks_like_android_application(&source, version_catalog.as_deref()) {
                return Ok(candidate);
            }
        }
    }

    let mut candidates = Vec::new();
    for entry in fs::read_dir(input)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        for name in ["build.gradle", "build.gradle.kts"] {
            let candidate = entry.path().join(name);
            if !candidate.is_file() {
                continue;
            }
            let source = fs::read_to_string(&candidate)?;
            if looks_like_android_application(&source, version_catalog.as_deref()) {
                candidates.push(candidate);
            }
        }
    }

    candidates.sort();
    match candidates.len() {
        0 => Err(ProjectError::BuildFileNotFound(input.to_path_buf())),
        1 => Ok(candidates.remove(0)),
        _ => Err(ProjectError::AmbiguousBuildFiles(candidates)),
    }
}

fn read_version_catalog(project_root: &Path) -> Result<Option<String>, ProjectError> {
    let path = project_root.join("gradle").join("libs.versions.toml");
    if !path.is_file() {
        return Ok(None);
    }

    Ok(Some(fs::read_to_string(path)?))
}

fn looks_like_android_application(
    source: &str,
    version_catalog: Option<&str>,
) -> bool {
    let sanitized = strip_comments(source);
    if sanitized.contains("com.android.application") {
        return true;
    }

    let plugins = match find_named_block(&sanitized, "plugins") {
        Some(block) => block,
        None => return false,
    };

    find_android_application_plugin_alias(&plugins, version_catalog)
}

fn find_android_application_plugin_alias(
    plugins: &str,
    version_catalog: Option<&str>,
) -> bool {
    let catalog = match version_catalog {
        Some(catalog) => catalog,
        None => return false,
    };

    let bytes = plugins.as_bytes();
    let mut index = 0;
    let mut quote = None;

    while index < bytes.len() {
        let current = bytes[index] as char;

        if let Some(active_quote) = quote {
            if current == '\\' {
                index += 2;
                continue;
            }
            if current == active_quote {
                quote = None;
            }
            index += 1;
            continue;
        }

        if current == '\'' || current == '"' {
            quote = Some(current);
            index += 1;
            continue;
        }

        if current == 'a'
            && plugins[index..].starts_with("alias")
            && is_word_boundary(bytes.get(index.wrapping_sub(1)).copied())
            && is_word_boundary(bytes.get(index + 5).copied())
        {
            let mut cursor = index + 5;
            cursor = skip_whitespace(plugins, cursor);
            if bytes.get(cursor) == Some(&b'(') {
                cursor = skip_whitespace(plugins, cursor + 1);
            }

            const PREFIX: &str = "libs.plugins.";
            if plugins[cursor..].starts_with(PREFIX) {
                let accessor_start = cursor + PREFIX.len();
                let mut accessor_end = accessor_start;
                while accessor_end < plugins.len() {
                    let byte = plugins.as_bytes()[accessor_end];
                    if byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'.' {
                        accessor_end += 1;
                    } else {
                        break;
                    }
                }

                if accessor_end > accessor_start
                    && !plugins[cursor..line_end(plugins, accessor_end)]
                        .contains("apply false")
                    && catalog_declares_plugin(
                        catalog,
                        &plugins[accessor_start..accessor_end],
                        "com.android.application",
                    )
                {
                    return true;
                }
            }

            index = cursor;
            continue;
        }

        index += 1;
    }

    false
}

fn catalog_declares_plugin(
    catalog: &str,
    accessor: &str,
    expected_plugin_id: &str,
) -> bool {
    let mut in_plugins = false;

    for raw_line in catalog.lines() {
        let line = strip_toml_comment(raw_line);
        let trimmed = line.trim();

        if trimmed.starts_with('[') {
            in_plugins = trimmed == "[plugins]";
            continue;
        }

        if !in_plugins || trimmed.is_empty() {
            continue;
        }

        let Some((raw_key, value)) = trimmed.split_once('=') else {
            continue;
        };

        let key = raw_key.trim().trim_matches('"');
        let key_matches = key == accessor
            || key.replace('-', ".") == accessor
            || key.replace('_', ".") == accessor;

        if key_matches
            && extract_string_value(value, "id").as_deref() == Some(expected_plugin_id)
        {
            return true;
        }
    }

    false
}

fn strip_toml_comment(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut quote = None;
    let chars: Vec<char> = source.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        let current = chars[index];

        if let Some(active_quote) = quote {
            out.push(current);
            if current == '\\' && index + 1 < chars.len() {
                index += 1;
                out.push(chars[index]);
            } else if current == active_quote {
                quote = None;
            }
            index += 1;
            continue;
        }

        if current == '\'' || current == '"' {
            quote = Some(current);
            out.push(current);
            index += 1;
            continue;
        }

        if current == '#' {
            break;
        }

        out.push(current);
        index += 1;
    }

    out
}

fn line_end(source: &str, offset: usize) -> usize {
    source[offset..]
        .find('\\n')
        .map(|relative| offset + relative)
        .unwrap_or(source.len())
}

fn is_word_boundary(byte: Option<u8>) -> bool {
    byte.is_none() || !byte.unwrap().is_ascii_alphanumeric() && byte != Some(b'_')
}

fn is_gradle_build_file(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|value| value.to_str()),
        Some("build.gradle" | "build.gradle.kts")
    )
}

fn strip_comments(source: &str) -> String {
    let chars: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    let mut index = 0;
    let mut quote = None;

    while index < chars.len() {
        let current = chars[index];

        if let Some(active_quote) = quote {
            out.push(current);
            if current == '\\' && index + 1 < chars.len() {
                index += 1;
                out.push(chars[index]);
            } else if current == active_quote {
                quote = None;
            }
            index += 1;
            continue;
        }

        if current == '\'' || current == '"' {
            quote = Some(current);
            out.push(current);
            index += 1;
            continue;
        }

        if current == '/' && index + 1 < chars.len() && chars[index + 1] == '/' {
            out.push(' ');
            out.push(' ');
            index += 2;
            while index < chars.len() && chars[index] != '\n' {
                out.push(' ');
                index += 1;
            }
            continue;
        }

        if current == '/' && index + 1 < chars.len() && chars[index + 1] == '*' {
            out.push(' ');
            out.push(' ');
            index += 2;
            while index + 1 < chars.len() && !(chars[index] == '*' && chars[index + 1] == '/') {
                out.push(if chars[index] == '\n' { '\n' } else { ' ' });
                index += 1;
            }
            if index + 1 < chars.len() {
                out.push(' ');
                out.push(' ');
                index += 2;
            }
            continue;
        }

        out.push(current);
        index += 1;
    }

    out
}

fn find_named_block(source: &str, name: &str) -> Option<String> {
    let mut offset = 0;
    while let Some(position) = find_word(source, name, offset) {
        let after_name = position + name.len();
        let whitespace_end = skip_whitespace(source, after_name);
        if source.as_bytes().get(whitespace_end) == Some(&b'{') {
            let close = find_matching_brace(source, whitespace_end)?;
            return Some(source[whitespace_end + 1..close].to_string());
        }
        offset = after_name;
    }

    None
}

fn find_word(source: &str, word: &str, start: usize) -> Option<usize> {
    let mut offset = start;
    while let Some(relative) = source[offset..].find(word) {
        let position = offset + relative;
        let before_ok = position == 0
            || !source.as_bytes()[position - 1].is_ascii_alphanumeric()
                && source.as_bytes()[position - 1] != b'_';
        let after = position + word.len();
        let after_ok = after >= source.len()
            || !source.as_bytes()[after].is_ascii_alphanumeric()
                && source.as_bytes()[after] != b'_';

        if before_ok && after_ok {
            return Some(position);
        }

        offset = after;
    }

    None
}

fn find_matching_brace(source: &str, open: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut depth = 0_usize;
    let mut quote = None;
    let mut index = open;

    while index < bytes.len() {
        let current = bytes[index] as char;

        if let Some(active_quote) = quote {
            if current == '\\' {
                index += 2;
                continue;
            }
            if current == active_quote {
                quote = None;
            }
            index += 1;
            continue;
        }

        if current == '\'' || current == '"' {
            quote = Some(current);
            index += 1;
            continue;
        }

        if current == '{' {
            depth += 1;
        } else if current == '}' {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(index);
            }
        }

        index += 1;
    }

    None
}

fn extract_string_value(source: &str, key: &str) -> Option<String> {
    let mut offset = 0;
    while let Some(position) = find_word(source, key, offset) {
        let mut cursor = skip_whitespace(source, position + key.len());
        if source.as_bytes().get(cursor) == Some(&b'=') {
            cursor = skip_whitespace(source, cursor + 1);
        }

        if let Some(value) = read_quoted_string(source, cursor) {
            return Some(value);
        }

        offset = position + key.len();
    }

    None
}

fn extract_integer_value(source: &str, key: &str) -> Option<u32> {
    let mut offset = 0;
    while let Some(position) = find_word(source, key, offset) {
        let mut cursor = skip_whitespace(source, position + key.len());
        if source.as_bytes().get(cursor) == Some(&b'=') {
            cursor = skip_whitespace(source, cursor + 1);
        }

        let start = cursor;
        while cursor < source.len() && source.as_bytes()[cursor].is_ascii_digit() {
            cursor += 1;
        }

        if cursor > start {
            return source[start..cursor].parse().ok();
        }

        offset = position + key.len();
    }

    None
}

fn extract_bool_value(source: &str, key: &str) -> Option<bool> {
    let mut offset = 0;
    while let Some(position) = find_word(source, key, offset) {
        let mut cursor = skip_whitespace(source, position + key.len());
        if source.as_bytes().get(cursor) == Some(&b'=') {
            cursor = skip_whitespace(source, cursor + 1);
        }

        if source[cursor..].starts_with("true")
            && is_value_terminator(source.as_bytes().get(cursor + 4).copied())
        {
            return Some(true);
        }

        if source[cursor..].starts_with("false")
            && is_value_terminator(source.as_bytes().get(cursor + 5).copied())
        {
            return Some(false);
        }

        offset = position + key.len();
    }

    None
}

fn is_value_terminator(byte: Option<u8>) -> bool {
    byte.is_none() || !byte.unwrap().is_ascii_alphanumeric() && byte != Some(b'_')
}

fn read_quoted_string(source: &str, start: usize) -> Option<String> {
    let quote = *source.as_bytes().get(start)?;
    if quote != b'\'' && quote != b'"' {
        return None;
    }

    let mut index = start + 1;
    let mut value = String::new();
    while index < source.len() {
        let byte = source.as_bytes()[index];
        if byte == b'\\' && index + 1 < source.len() {
            value.push(source.as_bytes()[index + 1] as char);
            index += 2;
            continue;
        }
        if byte == quote {
            return Some(value);
        }
        value.push(byte as char);
        index += 1;
    }

    None
}

fn skip_whitespace(source: &str, mut offset: usize) -> usize {
    while offset < source.len() && source.as_bytes()[offset].is_ascii_whitespace() {
        offset += 1;
    }
    offset
}

fn format_paths(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    const GROOVY: &str = r#"
plugins {
    id 'com.android.application'
}

android {
    namespace 'com.example.fixture'
    compileSdk 36

    defaultConfig {
        applicationId 'com.example.fixture'
        minSdk 24
        targetSdk 35
        versionCode 7
        versionName '1.2.3'
    }

    buildTypes {
        release {
            debuggable false
        }
    }
}
"#;

    const KOTLIN: &str = r#"
plugins {
    id("com.android.application")
}

android {
    namespace = "com.example.fixture"
    compileSdk = 36

    defaultConfig {
        applicationId = "com.example.fixture"
        minSdk = 24
        targetSdk = 35
        versionCode = 7
        versionName = "1.2.3"
    }

    buildTypes {
        release {
            isDebuggable = false
        }
    }
}
"#;

    #[test]
    fn parses_groovy_assignments() {
        let android = find_named_block(&strip_comments(GROOVY), "android").unwrap();
        let default_config = find_named_block(&android, "defaultConfig").unwrap();
        let release = find_named_block(
            &find_named_block(&android, "buildTypes").unwrap(),
            "release",
        )
        .unwrap();

        assert_eq!(
            extract_string_value(&default_config, "applicationId").as_deref(),
            Some("com.example.fixture")
        );
        assert_eq!(
            extract_integer_value(&default_config, "targetSdk"),
            Some(35)
        );
        assert_eq!(
            extract_string_value(&default_config, "versionName").as_deref(),
            Some("1.2.3")
        );
        assert_eq!(extract_bool_value(&release, "debuggable"), Some(false));
    }

    #[test]
    fn parses_kotlin_assignments() {
        let android = find_named_block(&strip_comments(KOTLIN), "android").unwrap();
        let default_config = find_named_block(&android, "defaultConfig").unwrap();

        assert_eq!(
            extract_string_value(&android, "namespace").as_deref(),
            Some("com.example.fixture")
        );
        assert_eq!(extract_integer_value(&android, "compileSdk"), Some(36));
        assert_eq!(
            extract_integer_value(&default_config, "versionCode"),
            Some(7)
        );
        assert_eq!(
            extract_bool_value(
                &find_named_block(
                    &find_named_block(&android, "buildTypes").unwrap(),
                    "release"
                )
                .unwrap(),
                "isDebuggable"
            ),
            Some(false)
        );
    }

    #[test]
    fn discovers_android_application_from_version_catalog_alias() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/project-release-version-catalog");

        let project = parse_project(root).expect("version-catalog project should be discovered");

        assert_eq!(project.module_name, "app");
        assert_eq!(project.syntax, GradleSyntax::Kotlin);
        assert_eq!(
            project.application_id.as_deref(),
            Some("com.example.catalogfixture")
        );
        assert_eq!(project.target_sdk, Some(35));
        assert_eq!(project.version_code, Some(8));
        assert_eq!(project.version_name.as_deref(), Some("2.0.0"));
        assert_eq!(project.release_debuggable, Some(false));
    }

    #[test]
    fn direct_application_plugin_id_remains_an_application_match() {
        assert!(looks_like_android_application(
            "plugins { id(\"com.android.application\") }",
            None
        ));
    }

    #[test]
    fn resolves_only_android_application_plugin_aliases() {
        let catalog = r#"
[versions]
agp = "8.9.1"

[plugins]
androidApplication = { id = "com.android.application", version.ref = "agp" }
androidLibrary = { id = "com.android.library", version.ref = "agp" }
"#;

        assert!(looks_like_android_application(
            "plugins { alias(libs.plugins.androidApplication) }",
            Some(catalog)
        ));
        assert!(!looks_like_android_application(
            "plugins { alias(libs.plugins.androidLibrary) }",
            Some(catalog)
        ));
    }

    #[test]
    fn ignores_apply_false_application_alias_during_discovery() {
        let catalog = r#"
[plugins]
androidApplication = { id = "com.android.application", version = "8.9.1" }
"#;

        assert!(!looks_like_android_application(
            "plugins { alias(libs.plugins.androidApplication) apply false }",
            Some(catalog)
        ));
    }

    #[test]
    fn ignores_application_alias_text_inside_plugin_strings() {
        let catalog = r#"
[plugins]
androidApplication = { id = "com.android.application", version = "8.9.1" }
"#;

        assert!(!looks_like_android_application(
            r#"plugins { id("example") println("alias(libs.plugins.androidApplication)") }"#,
            Some(catalog)
        ));
    }

    #[test]
    fn ignores_alias_without_a_version_catalog() {
        assert!(!looks_like_android_application(
            "plugins { alias(libs.plugins.androidApplication) }",
            None
        ));
    }

    #[test]
    fn ignores_aliases_in_non_plugin_blocks() {
        let catalog = r#"
[plugins]
androidApplication = { id = "com.android.application", version = "8.9.1" }
"#;

        assert!(!looks_like_android_application(
            r#"
            dependencies {
                println("alias(libs.plugins.androidApplication)")
            }
            "#,
            Some(catalog)
        ));
    }

    #[test]
    fn ignores_words_inside_comments() {
        let source = strip_comments(
            r#"
            // applicationId 'wrong'
            android {
                defaultConfig {
                    applicationId 'right'
                }
            }
            "#,
        );
        let default_config = find_named_block(
            &find_named_block(&source, "android").unwrap(),
            "defaultConfig",
        )
        .unwrap();

        assert_eq!(
            extract_string_value(&default_config, "applicationId").as_deref(),
            Some("right")
        );
    }
}
