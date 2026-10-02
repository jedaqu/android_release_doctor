# CHECKPOINT — ERR-104 CLOSED

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Scope: M0.9 / AUDIT-005 release binary version propagation

## Closure state

ERR-104 completed:

**PRE-AUDIT → minimal implementation correction → focused test → Rust CI → second audit → real three-platform distribution validation → final audit → documentation closure → PR CI → merge → post-merge CI → checkpoint.**

## Defect

The first real `v0.1.1` distribution attempt built the supported Linux, Windows, and macOS binaries but rejected each package because the binary `--version` output remained `0.1.0` while the workspace/tag version was `0.1.1`.

The release workflow correctly prevented publication.

## Correction

The independent hard-coded engine version was replaced with the Cargo package version:

```rust
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
```

A focused regression test enforces the invariant:

```rust
assert_eq!(ENGINE_VERSION, env!("CARGO_PKG_VERSION"));
```

Correction commit:

`933eccba12b0e40162dede4f1b6666952ecd79a2`

PR #26 and its post-merge Rust CI passed all four required gates.

## Final release validation

The corrected release target was `v0.1.2`.

Distribution workflow:

- run #6
- run id: `37027265016`
- overall: **SUCCESS**
- validate: PASS
- Linux package: PASS
- Windows package: PASS
- macOS package: PASS
- publish: PASS

Binary version evidence:

- Linux: `android-release-doctor 0.1.2`
- Windows: `0.1.2` binary-version consistency guard passed
- macOS: `android-release-doctor 0.1.2`

Published release:

- `v0.1.2`
- release id: `401939254`
- draft: false
- prerelease: false
- three platform packages plus `SHA256SUMS`

## Closure documentation validation

Final release validation:

- `docs/AUDIT-ERR-104-FINAL-RELEASE-VALIDATION.md`
- result: **PASS**

Closure commit sequence is documentation-only after the already-validated implementation and release:

- ERR-104 ledger update recorded as RESOLVED.
- This checkpoint records the complete terminal evidence.

## Tag integrity

- `v0.1.2` remains the successful corrected publication.
- `v0.1.1` remains untouched.
- Historical `v0.1.0` remains untouched.
- No force-push or tag movement was used.

## Final conclusion

**ERR-104 CLOSED.**

The stale release-binary version defect has been corrected, regression-covered, revalidated across all supported package platforms by a real `v0.1.2` publication, and formally closed after CI verification.
