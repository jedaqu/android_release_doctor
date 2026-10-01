# SECOND AUDIT M0.8 Block 4 — Distribution / Release Packaging

Date: 2026-10-01
Branch: `m08-block4-distribution-release`
Baseline: `2310111e84ddc3a785523159cc69a37e889b0d08`
Implementation review head before ERR-084 correction: `baa8c065e34397e9f71c3fac28fc08ecef461fd6`
ERR-084 correction head: `56dcf8e7ab978288e535585aacf9c852b5109d2c`
ERR-085 correction head: `28c82e3c1b6c022548da0747ae1e85176b06f16a`
ERR-086 correction head: `e440b1e1e00d9f163e5373a331e35ad89448b183`

## Result

**PASS — the implementation is bounded and matches the frozen Block 4 contract. External Actions validation is the remaining gate.**

## Workflow review

### `.github/workflows/m08-block4-distribution.yml`

Verified:
- branch push coverage for `m08-block4-distribution-release`;
- tag coverage for `v*`;
- pull-request target coverage for `m08-block3-github-action`;
- read-only top-level permissions;
- dedicated `contents: write` permission only on publish job.

### Version contract

The validation job extracts the root workspace version from `Cargo.toml`.

For tag events it strips the leading `v` from `GITHUB_REF_NAME` and requires exact equality with the workspace version.

No tag is created or modified by the workflow.

### Build contract

Each platform performs:
`cargo build --locked --release -p doctor-cli`

The workflow does not change Cargo profiles, dependencies, or workspace versioning.

### Platform matrix

The exact matrix is:
- Ubuntu 24.04 / Linux x86_64;
- Windows 2025 / Windows x86_64;
- macOS 15 Intel / macOS x86_64.

The matrix is fail-fast disabled so a failure on one platform remains visible while other platform jobs finish.

### Package contents

Linux/macOS package root:
- `android-release-doctor`;
- `LICENSE`;
- `README.md`.

Windows package root:
- `android-release-doctor.exe`;
- `LICENSE`;
- `README.md`.

The workflow validates an exact three-file payload before uploading each archive.

### Binary verification

Each binary is executed from the packaged directory:
- `--version` is checked against the workspace version;
- `--help` must exit successfully.

This verifies the packaged executable rather than only the Cargo build output.

### Archive verification

The created archive is opened/inspected on the same native runner.

The expected sorted file set is asserted for tarballs and ZIP.

### Cross-job artifact handling

Each platform uses a unique Actions artifact name. This is compatible with the current immutable artifact model.

The publish job downloads all three package artifacts with `merge-multiple: true`, then creates `SHA256SUMS`.

### Release publication

The release job can run only for a tag push.

It:
1. requires all package jobs to succeed;
2. verifies all three expected archive names;
3. verifies the generated checksums;
4. calls `gh release create` with `--verify-tag`;
5. publishes the three archives plus `SHA256SUMS`.

No release operation is executed for branch or PR events.

## Rust CI trigger coverage

The existing `rust.yml` now explicitly covers:
- push on `m08-block4-distribution-release`;
- pull-request targets on `m08-block3-github-action`.

The Build → Test → Format → Clippy job graph is unchanged.

This applies the preventive CI trigger observation recorded after ERR-079.

## Scope review

### Changed

- Block 4 distribution workflow;
- Rust CI trigger coverage;
- README distribution section;
- Block 4 preflight and frozen contract;
- this second audit.

### Not changed

- audit engine;
- Report v1;
- CLI implementation/contract;
- GitHub Action semantics;
- cryptographic verification;
- Cargo dependencies;
- package version;
- release tag;
- existing GitHub Releases.

## Security review

- no third-party release action;
- no user-supplied shell fragments;
- no automatic tag creation;
- release job is the only job with write permission;
- no secrets are copied into archives;
- package contents are explicitly constrained.

## CI correction review — ERR-084

The first external validation failed because the historical `Cargo.lock` was stale relative to the already-declared Block 1 report serialization dependencies.

The correction is bounded to `Cargo.lock` and preserves the Block 4 reproducibility intent:
- `--locked` remains enabled;
- no dependency declaration changed;
- no product behavior changed;
- no packaging scope changed.

The corrected lockfile now contains the direct `doctor-core` dependency edges for `serde` and `serde_json` plus the required registry records.

## CI correction review — ERR-085

The first multi-platform packaging validation reached the macOS archive-content verification step. The package itself built successfully and binary checks passed; only the locale-sensitive textual ordering assertion failed.

The correction is bounded to deterministic verification:
`tar -tzf "$archive" | LC_ALL=C sort`.

Linux behavior remains unchanged and the package payload contract is unchanged.

## CI correction review — ERR-086

The Windows packaging job reached native binary build and ZIP creation successfully, then failed only because the verification compared locale/order-dependent filename sequences.

The bounded correction removes ordering as a correctness requirement and compares the exact expected filename set case-sensitively.

## Final static verdict after ERR-086 correction

**PASS — no scope violation or contract mismatch found.**

External validation remains mandatory:
- Rust CI Build/Test/Format/Clippy;
- Block 4 three-platform packaging workflow.

If Actions exposes a defect, record the next ERR-NNN and apply one bounded correction before revalidation.

## Final external validation evidence

After ERR-084, ERR-085, and ERR-086 corrections, the corrected implementation was validated on the branch head used for closure:

- Distribution run #34 / `36890483899`: PASS
  - validate: PASS
  - Linux x86_64 package: PASS
  - Windows x86_64 package: PASS
  - macOS x86_64 package: PASS
  - publish job not exercised because this was not a version-tag push
- Rust CI #533 / `36890483719`: PASS
  - Build: PASS
  - Test: PASS
  - Format: PASS
  - Clippy: PASS

ERR-084, ERR-085, and ERR-086 are therefore **RESOLVED**. No audit engine, Report v1, CLI, or Block 3 Action behavior changed as part of these corrections.

