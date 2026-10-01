# CHECKPOINT M0.8 Block 4 — Distribution / Release Packaging — Final 2

Date: 2026-10-01
Branch: `m08-block4-distribution-release`
Block 3 baseline: `2310111e84ddc3a785523159cc69a37e889b0d08`
ERR-084 correction baseline: `56dcf8e7ab978288e535585aacf9c852b5109d2c`
Documentation head before this checkpoint: `fedc67ea3b958e6cf5945fd402355ec9c0810f31`
PR: #20 — open, unmerged
Base: `m08-block3-github-action`

## Status

**SUPERSEDED — this checkpoint candidate was invalidated by ERR-085 before terminal closure.**

ERR-085 exposed a locale-dependent macOS archive verification assertion. See the chronological ledger entry and the correction in `28c82e3c1b6c022548da0747ae1e85176b06f16a`.

## Correction history

- Initial Block 4 implementation was audited and bounded.
- ERR-084 was discovered when `--locked` exposed that the historical `Cargo.lock` did not contain the already-declared `serde`/ `serde_json` dependencies.
- Only `Cargo.lock` was corrected.
- ERR-084 is recorded chronologically and remains pending only fresh Actions validation.
- The prior checkpoint candidate was explicitly superseded.

## Frozen implementation

- tag-driven distribution workflow;
- Linux x86_64 package;
- Windows x86_64 package;
- macOS Intel x86_64 package;
- exact archive payload validation;
- packaged binary `--version` / `--help` validation;
- SHA256SUMS generation and verification;
- GitHub Release publication guarded by an existing matching version tag;
- read-only build jobs and `contents: write` only on publication;
- stacked Rust CI trigger coverage.

## Cargo reproducibility

The distribution workflow and its validation use `--locked`.

The corrected lockfile now includes:
- direct `doctor-core` edges for `serde` and `serde_json`;
- `itoa 1.0.15`;
- `serde_json 1.0.151`;
- `zmij 1.0.23`;
- required registry checksums.

No `Cargo.toml` dependency declaration or package version was changed by the correction.

## Explicitly out of scope

- ARM64 and additional architectures;
- cross-compilation;
- package managers;
- installers;
- OS signing/notarization;
- cryptographic binary signatures;
- provenance/attestations;
- Docker images;
- automatic version bumping;
- audit engine changes;
- Report v1 changes;
- CLI changes;
- GitHub Action behavior changes;
- release tag creation during checkpoint validation.

## Closure condition

This checkpoint becomes formally **CLOSED** only when checks associated with this exact checkpoint commit are terminal and successful for:

1. Rust CI Build;
2. Rust CI Test;
3. Rust CI Format;
4. Rust CI Clippy;
5. M0.8 Block 4 validate job;
6. Linux x86_64 package job;
7. Windows x86_64 package job;
8. macOS x86_64 package job.

The publish job must remain skipped because this checkpoint is not a tag push.

No further implementation or documentation changes are permitted after this checkpoint commit before terminal validation is evaluated.


## ERR-086 follow-up

The subsequent Windows ZIP verification failure is recorded as ERR-086. A new final checkpoint is required after its correction.

## Supersession note

A new final checkpoint is required after ERR-085 correction and fresh terminal Actions validation.
