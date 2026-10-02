# AUDIT — ERR-105 Pre-Audit: Stale v0.1.1 Package Names in README

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Baseline main: `f75ae4ee8fc1ab954b3e2441ec27b877d899968d`
Branch: `fix/err-105-readme-stale-v0.1.1-package-names-2026-10-02`

## Observed defect

The current README describes the corrected release target as `v0.1.2`, but the three package filenames in the supported-download table still contain `v0.1.1`.

Affected lines are the Linux, Windows, and macOS package names in the README supported-download table.

## Evidence

The current README contains:

- `android-release-doctor-v0.1.1-linux-x86_64.tar.gz`
- `android-release-doctor-v0.1.1-windows-x86_64.zip`
- `android-release-doctor-v0.1.1-macos-x86_64.tar.gz`

while the same README identifies the release target as `v0.1.2`, and the current workspace version is `0.1.2`.

The v0.1.1 tag is not a release and must not be reused.

## Root cause

The release rebaseline changed the surrounding README release references from `v0.1.1` to `v0.1.2` but did not replace the three literal package filenames in the table.

## Scope

### In scope

Replace only the three stale package filenames in the README with their `v0.1.2` equivalents.

### Out of scope

- Any product or Rust source changes.
- Distribution workflow changes.
- Package matrix changes.
- Checksum changes.
- CLI or engine-version changes.
- Changelog changes.
- Historical release or tag changes.

## Acceptance

- README contains no stale `v0.1.1` package filename.
- The three supported-download package names exactly match the `v0.1.2` release target.
- No unrelated README content changes.
- Rust CI Build/Test/Format/Clippy passes.
- A second audit confirms the exact three-line correction.
