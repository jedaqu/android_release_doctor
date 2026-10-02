# AUDIT — ERR-104 Final Release Validation

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Current main: `54641233485755b42d3569297afbd13bd7b72536`
Validated release tag: `v0.1.2`
Distribution workflow run: `37027265016` (run #6)

## Purpose

This audit performs the final validation required to close ERR-104 after the implementation correction was merged and a subsequent real three-platform release was successfully published.

The prior ERR-104 second audit passed the scoped implementation but explicitly left the error OPEN pending re-execution of the distribution package matrix.

## Implementation integrity

The corrected implementation remains:

```rust
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
```

The focused invariant test remains:

```rust
#[test]
fn engine_version_tracks_package_version() {
    assert_eq!(ENGINE_VERSION, env!("CARGO_PKG_VERSION"));
}
```

Current `main` contains that implementation and no separate hard-coded engine version remains in the inspected production constant.

ERR-104 correction commit:

`933eccba12b0e40162dede4f1b6666952ecd79a2`

PR #26 was merged and its post-merge Rust CI passed before the v0.1.2 release-validation cycle.

## Final distribution validation

The tag-driven distribution workflow for `v0.1.2` completed successfully:

- run #6
- run id: `37027265016`
- overall conclusion: **success**
- `validate`: PASS
- Linux package: PASS
- Windows package: PASS
- macOS package: PASS
- `publish`: PASS

The three package jobs each passed the binary version consistency guard.

### Linux x86_64

The package job logged:

```text
VERSION: 0.1.2
android-release-doctor 0.1.2
path: android-release-doctor-v0.1.2-linux-x86_64.tar.gz
```

The executed guard was:

```bash
"$package_root/android-release-doctor" --version | grep -F "android-release-doctor $VERSION"
```

Result: **PASS**.

### Windows x86_64

The package job executed the PowerShell guard requiring the output to match `android-release-doctor $env:VERSION`.

The terminal log recorded:

```text
VERSION: 0.1.2
path: android-release-doctor-v0.1.2-windows-x86_64.zip
```

The job concluded **success**, so the binary version guard passed without raising `Binary version mismatch`.

### macOS x86_64

The package job logged:

```text
VERSION: 0.1.2
android-release-doctor 0.1.2
path: android-release-doctor-v0.1.2-macos-x86_64.tar.gz
```

The executed guard was the same `--version`/grep contract used for the Unix package path.

Result: **PASS**.

## Published release confirmation

GitHub Release `v0.1.2` exists and is published:

- release id: `401939254`
- tag: `v0.1.2`
- draft: false
- prerelease: false
- published: `2026-10-02T15:32:25Z`
- target commit: `main`

The release contains exactly:

1. `android-release-doctor-v0.1.2-linux-x86_64.tar.gz`
2. `android-release-doctor-v0.1.2-windows-x86_64.zip`
3. `android-release-doctor-v0.1.2-macos-x86_64.tar.gz`
4. `SHA256SUMS`

The publish job also completed **Generate and verify checksums — PASS** and **Publish GitHub Release — PASS**.

## Acceptance recheck

ERR-104 required final validation that the corrected engine version propagates into packaged binaries across the supported release matrix.

That condition is now demonstrated directly by the successful `v0.1.2` distribution run:

- workspace/tag version: `0.1.2`;
- Linux binary: reports `android-release-doctor 0.1.2`;
- Windows binary: passes the `0.1.2` version-match guard;
- macOS binary: reports `android-release-doctor 0.1.2`;
- all three package jobs: terminal success;
- release publication: terminal success.

The originally observed stale-version defect from `v0.1.1` is therefore no longer present in the corrected release path.

## Scope and regression boundary

No additional source, workflow, checksum, package-matrix, archive, signing, or publication-command change was required for ERR-104 closure.

The failed remote tag `v0.1.1` remains untouched. Historical `v0.1.0` remains untouched.

## Conclusion

**FINAL AUDIT: PASS.**

The implementation correction for ERR-104 is validated end-to-end by the real `v0.1.2` three-platform distribution and publication. The remaining closure requirement is documentary: mark ERR-104 RESOLVED and create the formal closure checkpoint.
