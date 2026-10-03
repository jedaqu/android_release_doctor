# AUDIT — Product Surface Cleanup — 2026-10-03

## Scope

This cleanup removes historical binary-release and publication machinery that no longer defines Android Release Doctor.

Product surface retained:

- `.github/actions/android-release-doctor/action.yml`
- CLI and core audit engine
- Report v1
- Play readiness profiles
- Gradle static cross-checks
- signing and cryptographic verification
- native/16 KiB inspection
- APK/AAB inventory and manifest inspection
- public technical fixtures
- `docs/ERRORS-AND-FIXES.md` and current engineering-discipline documentation

## Removed from the active tree

- historical distribution workflow `.github/workflows/m08-block4-distribution.yml`
- public release changelog `CHANGELOG.md`
- M0.8 Block 4 distribution contract/preflight/checkpoints
- M0.8 manual distribution workflow documentation
- M0.9 public-release onboarding documents
- historical v0.1.2/v0.1.3 distribution/publication audits and checkpoints
- publication/readiness exercise documents that describe the former release-packaging product surface

These files remain recoverable through Git history but are no longer presented as current product documentation.

## README normalization

The README now defines the active product as:

**GitHub Action + CLI + audit engine**

It no longer defines:

- a public binary download matrix;
- a tag-driven publication contract;
- a current `v0.1.x` release target;
- packaged Linux/Windows/macOS archives;
- release-tag Action examples.

The Action example now uses `@main` and explicitly recommends pinning a reviewed commit SHA for supply-chain control.

Cargo workspace version metadata remains because Cargo packages require version metadata; that field is explicitly not treated as a public release identifier.

## Historical ledger rule

`docs/ERRORS-AND-FIXES.md` is intentionally retained and not rewritten. Historical entries are part of the project's engineering memory and may still contain references to former release work.

## Validation requirement

The cleaned branch must pass:

- Rust format check
- workspace tests
- Clippy
- Action success/blocker/operational-error paths
- workflow syntax lint
- absence of the removed release/distribution files
- absence of active README references to the old v0.1.x publication surface

## Product finding carried forward

The second validation campaign retained one unresolved product/specification candidate:

- 16 KiB native ABI scope: current PLAY-005 evaluation treats a misaligned 32-bit ABI as a blocker even when the 64-bit ABIs are correctly aligned.

No code change for that finding is included in this cleanup.
