# CHECKPOINT M0.8 Block 4 — Distribution / Release Packaging — Final 3

Date: 2026-10-01
Branch: `m08-block4-distribution-release`
Validated implementation baseline: `6e610201302bd565cedd009c96b425d41cb41006`
Final documentation baseline before this checkpoint: `881b6b0342ed36a867a465b8a550f1e86445e752`
PR: #20 — open, unmerged
Base: `m08-block3-github-action`

## Status

**CLOSED — terminal CI evidence confirms the complete M0.8 Block 4 gate.**

## Final correction chain

- ERR-084: stale `Cargo.lock` exposed by the new `--locked` reproducibility gate.
- ERR-085: macOS archive verification made locale-dependent by plain `sort`.
- ERR-086: Windows ZIP verification made filename-order dependent by `Sort-Object`.

All three are resolved in the validated implementation state.

## Final external validation of implementation state

Before this documentation-only checkpoint:
- Distribution run #34 / `36890483899`: PASS
  - validate: PASS
  - Linux x86_64 package: PASS
  - Windows x86_64 package: PASS
  - macOS x86_64 package: PASS
  - publish not exercised because no version tag was pushed
- Rust CI #533 / `36890483719`: PASS
  - Build: PASS
  - Test: PASS
  - Format: PASS
  - Clippy: PASS

## Frozen distribution contract

Supported packages:
- `android-release-doctor-v<version>-linux-x86_64.tar.gz`
- `android-release-doctor-v<version>-windows-x86_64.zip`
- `android-release-doctor-v<version>-macos-x86_64.tar.gz`

Payload:
- Linux/macOS: binary + `LICENSE` + `README.md`
- Windows: `android-release-doctor.exe` + `LICENSE` + `README.md`

Build:
`cargo build --locked --release -p doctor-cli`

The workspace version is authoritative. A release tag must match it exactly.

## Release publication boundary

Only a pushed `v*` tag can enter the publication job.

The publication job:
- downloads all three validated package artifacts;
- generates and verifies `SHA256SUMS`;
- invokes `gh release create ... --verify-tag --generate-notes`;
- has `contents: write` permission.

No release tag or GitHub Release is created during this checkpoint.

## Terminal CI closure evidence

Checkpoint commit: `0756a30eb63e0861c71b7da9aa00794bb7262abd`

- Rust CI #538 / `36891513281`: **SUCCESS**
  - Build: PASS
  - Test: PASS
  - Format: PASS
  - Clippy: PASS
- M0.8 Block 4 Distribution #40 / `36891522180`: **SUCCESS**
  - validate: PASS
  - Linux x86_64 package: PASS
  - Windows x86_64 package: PASS
  - macOS x86_64 package: PASS
  - publish: SKIPPED (no version-tag push)

All required terminal gates are green for the exact checkpoint commit.

## Explicitly out of scope

- ARM64/additional architectures;
- cross-compilation;
- package-manager integrations;
- installers;
- OS signing/notarization;
- cryptographic binary signatures;
- provenance/attestations;
- Docker images;
- automatic version bumping;
- audit engine changes;
- Report v1 changes;
- CLI changes;
- Block 3 Action behavior changes.

## Exact closure gate for this checkpoint

This checkpoint becomes formally **CLOSED** only when the GitHub runs associated with this exact checkpoint commit are terminal and successful for:

1. Rust CI Build;
2. Rust CI Test;
3. Rust CI Format;
4. Rust CI Clippy;
5. Block 4 Distribution validate;
6. Linux x86_64 package;
7. Windows x86_64 package;
8. macOS x86_64 package.

The publish job must be skipped.

PR #20 remains open and unmerged.


## Final closure evidence

Checkpoint commit: `0756a30eb63e0861c71b7da9aa00794bb7262abd`

- Rust CI push #538 / `36891513281`: PASS — Build, Test, Format, Clippy.
- Distribution push #39 / `36891513353`: PASS — validate, Linux x86_64, Windows x86_64, macOS x86_64.
- Distribution PR #40 / `36891522180`: PASS — validate, Linux x86_64, Windows x86_64, macOS x86_64; publish skipped.
- No version tag was pushed; therefore publication was intentionally not exercised.
- ERR-084, ERR-085, ERR-086: RESOLVED.

**M0.8 Block 4 is formally CLOSED.**
