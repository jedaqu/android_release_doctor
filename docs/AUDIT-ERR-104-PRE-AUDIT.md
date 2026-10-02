# AUDIT — ERR-104 Pre-Audit: Engine Version Propagation

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Baseline: `main` / `a00529824f61795e97979d9289ad54b1e504c662`
Branch: `audit/err-104-engine-version-propagation-2026-10-02`

## Status

**PRE-AUDIT COMPLETE — implementation not started.**

ERR-093 remains **OPEN**. No production correction has been applied for ERR-104.

## Observed failure

The tag-driven distribution workflow was executed for `v0.1.1` in Actions run `37022419003`.

Validation passed:

- Read workspace version
- Test
- Format
- Clippy

All three package jobs built the release binary successfully but then failed at the binary-version consistency check:

- Linux x86_64: `"$package_root/android-release-doctor" --version | grep -F "android-release-doctor $VERSION"`
- macOS x86_64: same check
- Windows x86_64: `Binary version mismatch`

The `publish` job was skipped because the package matrix did not complete successfully.

## Demonstrated root cause

The release workspace version is `0.1.1`:

```toml
[workspace.package]
version = "0.1.1"
```

Both `doctor-core` and `doctor-cli` inherit that workspace package version.

However, `crates/doctor-core/src/lib.rs` contains:

```rust
pub const ENGINE_VERSION: &str = "0.1.0";
```

The CLI imports that constant and implements:

```rust
println!("android-release-doctor {ENGINE_VERSION}");
```

Therefore the release inputs disagree:

| Source | Observed value |
|---|---|
| Workspace/package version | `0.1.1` |
| `ENGINE_VERSION` | `0.1.0` |
| CLI `--version` source | `ENGINE_VERSION` |
| Release tag | `v0.1.1` |

The same failure on Linux, Windows, and macOS establishes that this is a shared source/version-propagation defect, not a runner-specific packaging defect.

## Why the workflow is correct to fail

The distribution workflow intentionally checks that the packaged binary reports the same version as the workspace version before creating each release archive. The failure is therefore a legitimate release-integrity gate, not a packaging false positive.

The relevant checks are in:

`.github/workflows/m08-block4-distribution.yml`

The workflow's tag/version validation had already passed, so the remaining inconsistency is between the package metadata and the binary-reported engine version.

## Scope

### In scope

1. Remove the independent hard-coded engine-version source.
2. Make `ENGINE_VERSION` derive from the Cargo package version so workspace version changes propagate automatically.
3. Preserve the existing public constant, CLI output format, report rendering, and release workflow contract.
4. Add only focused regression coverage needed to prevent the demonstrated version drift from returning.
5. Re-run the complete Build → Test → Format → Clippy gates.
6. Exercise the distribution package matrix on the corrected branch without publication before attempting the next real release.

### Out of scope

- Changes to audit semantics.
- Changes to package matrix or archive formats.
- Changes to checksum generation beyond the already-merged ERR-093 correction.
- Changes to GitHub Action interfaces.
- Changes to signing, permissions, publication commands, or trigger design.
- Rewriting or moving the already-created `v0.1.1` tag.
- Rebuilding historical `v0.1.0`.

## Proposed minimal correction boundary

The smallest architectural correction is to make the existing constant derive from Cargo's package version rather than storing a second literal version.

Candidate implementation:

```rust
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
```

Because `doctor-core` already uses `version.workspace = true`, this keeps the existing API and output path while eliminating the duplicated version literal.

This is a proposal for implementation delimitation, not yet an applied change.

## Release-version consequence discovered during pre-audit

The failed validation has already created the remote annotated tag `v0.1.1`, pointing to commit `a00529824f61795e97979d9289ad54b1e504c662`.

GitHub currently reports:

- `v0.1.1` tag exists.
- No GitHub Release exists for `v0.1.1`.
- The published GitHub Release list still contains the historical `v0.1.0` release.

Accordingly, the `v0.1.1` tag must not be moved or rewritten as part of the correction. The eventual successful publication must use a new release version/tag after the release metadata is re-baselined. The precise release-version edit belongs to the implementation/release-preparation step, not this pre-audit.

The current `README.md` and `CHANGELOG.md` on `main` describe `v0.1.1` as published. That statement is not currently supported by the GitHub release state and must be corrected as part of the next release preparation.

## Validation plan

1. Apply only the scoped version-propagation correction and required release-version rebaseline.
2. Run focused tests.
3. Run Rust CI and require terminal Build/Test/Format/Clippy PASS.
4. Use the Block 4 workflow's manual dispatch on the corrected branch to exercise validate + all three package jobs without publishing.
5. Confirm each package job reaches archive verification/upload instead of failing at `--version`.
6. Perform a second audit of the exact diff.
7. Only after ERR-104 is closed, resume ERR-093 real-publication validation with the new release tag.
8. Do not retry the failed `v0.1.1` tag publication.

## Acceptance criteria

ERR-104 may be marked **RESOLVED** only when all of the following are demonstrated:

- no independent hard-coded engine version remains on the release path;
- workspace/package version and CLI `--version` agree;
- focused tests pass;
- Build/Test/Format/Clippy all pass on the corrected commit;
- the three platform package jobs complete their binary-version checks successfully on the corrected branch;
- no historical release/tag has been rewritten.

## Conclusion

**ERR-104 is a confirmed version-propagation defect discovered during ERR-093 real-publication validation.**

No production implementation has been performed in this pre-audit.
