# AUDIT — ERR-104 Second Audit

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Baseline: `main` / `a00529824f61795e97979d9289ad54b1e504c662`
Correction commit: `933eccba12b0e40162dede4f1b6666952ecd79a2`
PR: #26

## Scope recheck

The implementation remains limited to the demonstrated version-propagation defect.

Diff against the pre-audit branch:

- `crates/doctor-core/src/lib.rs`: 1 production deletion + 6 test lines.
- No workflow changes.
- No package-matrix changes.
- No checksum changes.
- No publication-command changes.
- No ERR-093 production changes.
- No ERR-095/ERR-096 changes.

## Corrected implementation

The independent hard-coded engine version:

```rust
pub const ENGINE_VERSION: &str = "0.1.0";
```

is now:

```rust
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
```

The existing CLI continues to print the same contract:

```text
android-release-doctor {ENGINE_VERSION}
```

Therefore the engine version now derives from the package version rather than maintaining a second literal.

## Regression coverage

Added one focused unit test:

```rust
#[test]
fn engine_version_tracks_package_version() {
    assert_eq!(ENGINE_VERSION, env!("CARGO_PKG_VERSION"));
}
```

This test directly encodes the demonstrated invariant and will fail if the engine version is reintroduced as an independent value.

## CI evidence

PR #26 Rust CI run:

- Run: `37023893859`
- Run number: `105`
- Job: `test`
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

The corrected commit therefore passes the mandatory four CI gates.

## Remaining release validation

This audit does not claim ERR-104 closed yet because the distribution workflow's three-platform package matrix has not been re-exercised after the correction.

The required next release-validation step is to run the existing Block 4 distribution workflow against the corrected release state and confirm that Linux, Windows, and macOS all pass their binary `--version` consistency checks.

## Release/tag integrity

The existing remote `v0.1.1` tag remains untouched.

No historical release or tag has been rewritten.

## Conclusion

**SECOND AUDIT: PASS for the scoped ERR-104 implementation.**

The implementation is minimal and technically validated by Rust CI. ERR-104 remains OPEN only pending final distribution-matrix validation and post-merge validation.
