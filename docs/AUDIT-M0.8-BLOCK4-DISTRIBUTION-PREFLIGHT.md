# M0.8 Block 4 — Distribution / Release Packaging Preflight Audit

Date: 2026-10-01
Branch: `m08-block4-distribution-release`
Baseline: `2310111e84ddc3a785523159cc69a37e889b0d08`

## Result

**PASS — a bounded first distribution increment is defined and does not require changes to the audit engine, Report v1, CLI behavior, or GitHub Action semantics.**

## Baseline verification

- M0.8 Block 3 is formally closed.
- Final Block 3 checkpoint: `2310111e84ddc3a785523159cc69a37e889b0d08`.
- Rust CI #500 / `36887897432`: Build, Test, Format, Clippy all PASS.
- Block 3 Action self-test #29 / `36887897470`: PASS.
- PR #19 remains intentionally open and unmerged.
- Block 4 starts from the exact Block 3 checkpoint.

## Repository state relevant to distribution

- Workspace version: `0.1.0`.
- Binary package: `doctor-cli`.
- Binary name: `android-release-doctor`.
- License: Apache-2.0.
- `Cargo.lock` is present.
- No repository Rust toolchain file is currently pinned.
- No existing release/distribution workflow exists on the Block 3 baseline.
- Block 3 deliberately builds the CLI from source through Cargo and explicitly defers prebuilt binary distribution to this block.

Cargo's documented release build output is produced under `target/release`, and binary targets are configured in Cargo package metadata. The repository's CLI package already defines the `android-release-doctor` binary, so packaging can remain a packaging concern rather than a product-code change.

## Selected distribution boundary

The first packaged binary matrix is:

| Host | GitHub runner | Package |
|---|---|---|
| Linux x86_64 | `ubuntu-24.04` | `.tar.gz` |
| Windows x86_64 | `windows-2025` | `.zip` |
| macOS x86_64 | `macos-15-intel` | `.tar.gz` |

GitHub currently documents these hosted runner labels as available standard x64 environments.
Reference: https://docs.github.com/en/actions/reference/runners/github-hosted-runners

ARM64, other architectures, and cross-compilation are intentionally deferred.

## Package contents

Each archive contains exactly:
- `android-release-doctor` (or `.exe` on Windows);
- `LICENSE`;
- `README.md`.

No source tree, Cargo cache, target directory, debug symbols, credentials, or workflow metadata is packaged.

## Versioning

The distribution workflow is tag-driven.

Required tag format:
`v<workspace-version>`

For the current baseline:
`v0.1.0`

The workflow validates the pushed tag version against the workspace package version before building release artifacts.

The release workflow must not create a tag implicitly. It must verify that the supplied tag already exists before publishing the GitHub Release.

## Integrity

For every release:
- each platform archive is produced independently on its native x64 runner;
- the archive is tested by executing `--version` and `--help`;
- `SHA256SUMS` is generated over the final archives;
- checksums are verified before release publication.

## Release mechanism

GitHub CLI `gh release create <tag> <assets...>` is used for publication with `--verify-tag`. This keeps the release operation within the GitHub-provided CLI rather than adding a third-party release action.

GitHub's current CLI documentation confirms that `gh release create` can upload assets and `--verify-tag` refuses to create a release when the tag is absent.
Reference: https://cli.github.com/manual/gh_release_create

## Trigger boundary

The packaging workflow has two purposes:

1. branch/PR validation:
   - builds and verifies all three packages;
   - does not publish a release.
2. tag publication:
   - performs the same package validation;
   - publishes the verified archives and `SHA256SUMS`.

No release is published from branch or pull-request execution.

## Permissions

Build jobs use read-only repository access.

Only the final release job requires:
`contents: write`

No package registry, Pages, deployment, or secret permissions are required.

## Explicitly out of scope

- ARM64 packages;
- Linux musl packages;
- cross-compilation;
- crates.io publishing;
- Homebrew/Scoop/Chocolatey/package-manager integration;
- OS code signing/notarization;
- cryptographic binary signatures;
- SLSA/provenance/attestations;
- Docker images;
- installer/MSI/DMG generation;
- automatic version bumping;
- changes to Cargo workspace version;
- changes to audit engine, Report v1, CLI, or Block 3 Action behavior.

## Acceptance boundary

Block 4 may close only after:
1. packaging workflow implementation matches this preflight;
2. branch/PR packaging validation passes on all three platforms;
3. generated archives contain the exact expected file set;
4. packaged binaries return the expected `--version` and `--help`;
5. tag/version guard is validated;
6. checksums are generated and verified;
7. second audit passes;
8. Rust CI Build/Test/Format/Clippy passes;
9. documentation and ledger are updated;
10. final checkpoint has terminal successful CI.

No release tag or published GitHub Release is created as part of this Block 4 checkpoint unless explicitly required by a later release operation.
