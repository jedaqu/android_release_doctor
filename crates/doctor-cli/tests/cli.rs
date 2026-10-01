use std::{fs, path::PathBuf, process::Command};

use doctor_core::ENGINE_VERSION;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_android-release-doctor")
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(name)
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(binary())
        .args(args)
        .output()
        .expect("CLI process should start")
}

fn output_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "android-release-doctor-cli-{name}-{}.txt",
        std::process::id()
    ))
}

#[test]
fn default_format_remains_text_on_stdout() {
    let artifact = fixture("minimal-release.apk");
    let output = run(&[artifact.to_str().unwrap()]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");

    assert!(stdout.contains("ANDROID RELEASE REPORT"));
    assert!(stdout.contains("MANUAL REVIEW"));
    assert!(stderr.is_empty());
}

#[test]
fn json_format_emits_report_v1_only_on_stdout() {
    let artifact = fixture("minimal-release.apk");
    let output = run(&[
        "--format",
        "json",
        "--play",
        "--play-platform",
        "automotive",
        artifact.to_str().unwrap(),
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");

    assert!(stdout.starts_with("{"));
    assert!(stdout.ends_with("\n"));
    assert!(stdout.contains("\"schema_version\":\"1.0\""));
    assert!(stdout.contains("\"play\":{\"platform\":\"automotive\""));
    assert!(!stdout.contains("ANDROID RELEASE REPORT"));
    assert!(stderr.is_empty());
}

#[test]
fn output_file_receives_report_and_stdout_stays_empty() {
    let artifact = fixture("minimal-release.apk");
    let path = output_path("json");
    let output = run(&[
        "--format",
        "json",
        "--output",
        path.to_str().unwrap(),
        artifact.to_str().unwrap(),
    ]);

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());

    let content = fs::read_to_string(&path).expect("report file should exist");
    assert!(content.starts_with("{"));
    assert!(content.contains("\"schema_version\":\"1.0\""));

    fs::remove_file(path).expect("temporary report file should be removable");
}

#[test]
fn help_is_standalone_and_succeeds() {
    let output = run(&["--help"]);

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Usage: android-release-doctor"));
    assert!(output.stderr.is_empty());
}

#[test]
fn version_is_standalone_and_succeeds() {
    let output = run(&["--version"]);
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");

    assert!(output.status.success());
    assert_eq!(stdout, format!("android-release-doctor {ENGINE_VERSION}\n"));
    assert!(output.stderr.is_empty());
}

#[test]
fn usage_error_is_exit_code_two_and_uses_stderr() {
    let output = run(&["--format", "yaml"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown output format"));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Usage: android-release-doctor"));
}

#[test]
fn blocker_result_keeps_exit_code_one() {
    let artifact = fixture("minimal-release.apk");
    let output = run(&[
        "--play",
        "--play-platform",
        "mobile",
        artifact.to_str().unwrap(),
    ]);

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("BLOCKERS"));
}
