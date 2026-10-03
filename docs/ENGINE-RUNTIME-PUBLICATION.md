# Technical Engine Runtime Publication — 2026-10-03

## Purpose

This document records the controlled publication procedure for the prebuilt engine used internally by the Android Release Doctor GitHub Actions service.

The technical engine distribution is an implementation transport mechanism. It is not a separate product, installable application, or product release.

## Preconditions

- PR #76 remains open and draft.
- Branch: `service/prebuilt-cli-runtime-2026-10-03`
- Validated branch head: `0d879c179a02bac7c11c3b0090d982805299e13e`
- Action Validation Run `37153230600` — SUCCESS.
- Rust CI Run `37153230605` — SUCCESS.
- Engine source version in the workspace: `0.1.3`.
- Runtime manifest is intentionally still `draft`.

## Technical tag

The production build is tag-driven and technical tags are immutable.

`engine-v0.1.3` already exists and intentionally points to the earlier failed gate commit. It must not be moved or reused.

The corrected immutable publication revision uses the technical tag `engine-v0.1.3-build2`. The tag suffix is a transport revision; the actual engine version remains the workspace version `0.1.3`.

The runtime workflow validates that a technical tag is either exactly `engine-v<engine-version>` or `engine-v<engine-version>-build<N>`, while asset names and manifest keys remain based on the actual engine version.

## Expected workflow

The tag starts `.github/workflows/engine-runtime.yml`.

The workflow:

1. builds `doctor-cli` natively on each declared hosted runner;
2. verifies that the Cargo package version matches `0.1.3`;
3. creates exactly these six runtime assets:
   - `linux-x64`
   - `linux-arm64`
   - `macos-x64`
   - `macos-arm64`
   - `windows-x64`
   - `windows-arm64`
4. computes a SHA-256 sidecar for every asset;
5. independently recomputes and verifies all six hashes before publication;
6. creates the technical GitHub distribution `engine-v0.1.3`.

## After publication

The next controlled block must:

- record the six published asset hashes in `runtime/manifest.json` with `release_tag` set to `engine-v0.1.3-build2`;
- change the manifest status from `draft` to `published`;
- validate real Action retrieval on at least the matching consumer runner path;
- validate cached-runtime reuse and hash mismatch rejection;
- perform the production-integrated second audit;
- reconcile `main` and close the runtime migration checkpoint.

No merge into `main` occurs before these gates are closed.
