# CHECKPOINT M0.8 Block 4 — Distribution / Release Packaging

Date: 2026-10-01
Branch: `m08-block4-distribution-release`
Block 3 baseline: `2310111e84ddc3a785523159cc69a37e889b0d08`
Implementation/documentation head before checkpoint: `e254a98b48fad80478a2491f347bcd1108c3b783`
PR: #20 — open, unmerged
Base: `m08-block3-github-action`

## Status

**SUPERSEDED — this checkpoint candidate was invalidated by ERR-084 before terminal closure.**

ERR-084 exposed that the historical `Cargo.lock` was stale relative to declared report dependencies. See the chronological ledger entry and the correction in `56dcf8e7ab978288e535585aacf9c852b5109d2c`.

## Completed sequence

1. Block 3 final state verified.
2. Block 4 preflight audit: PASS.
3. Distribution contract frozen.
4. Tag-driven packaging workflow implemented.
5. Native x86_64 packages defined for Linux, Windows, and macOS Intel.
6. Exact archive-content validation implemented.
7. Packaged binary `--version` and `--help` validation implemented.
8. SHA256 checksum generation and verification implemented.
9. GitHub Release publication restricted to version-tag pushes and `contents: write`.
10. Rust CI push/stacked PR trigger coverage extended.
11. README distribution documentation added.
12. Second audit: PASS.

## Frozen distribution contract

Packages:
- `android-release-doctor-v<version>-linux-x86_64.tar.gz`
- `android-release-doctor-v<version>-windows-x86_64.zip`
- `android-release-doctor-v<version>-macos-x86_64.tar.gz`

Each archive contains exactly the binary, `LICENSE`, and `README.md`.

The workspace version is authoritative. A tag `v<version>` must match it exactly.

## Release publication

Branch/PR executions validate package creation only.

Only a pushed version tag can reach the publish job.

The publish job:
- downloads all three packages;
- generates `SHA256SUMS`;
- verifies checksums;
- verifies the existing tag through `gh release create --verify-tag`;
- publishes the three archives plus `SHA256SUMS`.

No release tag is created or modified by the workflow.

## Rust CI

The existing Build → Test → Format → Clippy graph is unchanged.

Coverage added for this block:
- push on `m08-block4-distribution-release`;
- pull-request target `m08-block3-github-action`.

## Explicitly out of scope

- ARM64;
- additional Linux targets;
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
- GitHub Action behavior changes.

## Closure condition

This checkpoint becomes formally **CLOSED** only when the GitHub checks associated with this exact checkpoint commit are terminal and successful for:

1. Rust CI — Build;
2. Rust CI — Test;
3. Rust CI — Format;
4. Rust CI — Clippy;
5. M0.8 Block 4 Distribution validation across:
   - Linux x86_64;
   - Windows x86_64;
   - macOS x86_64.

For tag publication behavior, the workflow's publish branch is guarded and is not exercised by this checkpoint; creating a real release tag is a separate release operation.

No further implementation or documentation changes are permitted after this checkpoint commit before the terminal validation is evaluated.


## Supersession note

A new final checkpoint is required after ERR-084 correction and fresh terminal Actions validation.
