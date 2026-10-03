# Service Runtime Architecture — Prebuilt CLI Runtime — 2026-10-03

## Purpose

This block removes the remaining runtime dependency that requires a consumer runner to have Rust/Cargo installed.

GitHub's current third-party CLI Action guidance recommends that a CLI Action let consumers select the CLI version, support multiple operating systems, run efficiently, work on hosted and self-hosted runners, and retrieve a specific CLI build with optional caching.

The Action therefore becomes a small Node.js 24 bootstrapper. The Rust CLI remains the local development and engine implementation surface, but it is no longer compiled by the consumer Action.

Sources:

- https://docs.github.com/en/actions/how-tos/create-and-publish-actions/create-a-cli-action
- https://docs.github.com/en/actions/tutorials/create-actions/create-a-javascript-action
- https://docs.github.com/en/actions/reference/workflows-and-actions/metadata-syntax
- https://docs.github.com/en/actions/reference/security/secure-use
- https://docs.github.com/en/actions/reference/runners/github-hosted-runners

## Target architecture

```text
Consumer workflow
      |
      | uses: jedaqu/android_release_doctor@<reviewed-ref>
      v
Root JavaScript Action (Node 24)
      |
      +-- resolve engine version
      +-- detect OS/architecture
      +-- read runtime/manifest.json
      +-- download immutable prebuilt CLI
      +-- verify SHA-256
      +-- cache runner-local copy
      |
      v
Prebuilt android-release-doctor engine
      |
      v
Report v1 + exit code
```

The Action never executes cargo on the consumer runner.

## Separation of version identities

The Action and the engine runtime have independent technical versions:

- Action version: the Git ref or commit consumed through uses.
- Engine version: the prebuilt Rust CLI selected through engine-version or the runtime manifest default.
- Runtime distribution tag: engine-v<version>.

A runtime distribution tag or release is a technical transport/versioning mechanism for the service implementation. It is not a product identity change and does not turn Android Release Doctor into an installable application.

## Runtime assets

Initial runtime target matrix:

| Runtime key | Target |
|---|---|
| linux-x64 | x86_64 Linux |
| linux-arm64 | aarch64 Linux |
| macos-x64 | x86_64 macOS |
| macos-arm64 | Apple Silicon macOS |
| windows-x64 | x86_64 Windows |
| windows-arm64 | ARM64 Windows |

GitHub currently documents x64 and ARM64 support for self-hosted runners across Linux, Windows, and macOS, with ARM64 on self-hosted runners noted as public preview. GitHub-hosted runners currently expose standard x64 and ARM64 Linux/Windows images and Intel/ARM64 macOS images.

The runtime loader rejects unsupported platform and architecture combinations instead of silently falling back to another binary.

## Integrity model

The Action does not trust a downloaded binary only because the URL is under the expected repository.

For every published engine version, runtime/manifest.json records the technical engine version, engine release tag, exact asset filename for each supported platform, and expected SHA-256 digest.

The Action resolves the requested version, resolves the exact platform asset, downloads the raw executable, computes SHA-256 locally, refuses execution on mismatch, and caches only the verified binary.

The current branch deliberately keeps the manifest in draft state until the first technical engine runtime distribution has been built and its hashes independently checked. A draft manifest is not a production-ready runtime source.

GitHub recommends pinning Actions themselves to full commit SHAs when immutable references are required.

## Cache model

The loader first checks RUNNER_TOOL_CACHE when available and falls back to RUNNER_TEMP or the operating-system temporary directory.

The cache key includes android-release-doctor/<engine-version>/<platform-key>/<asset-file>. A cached executable is reused only when its SHA-256 exactly matches the manifest.

This makes the cache an optimization, not a trust anchor.

## Distribution workflow

```text
Rust source
   |
   v
engine runtime build workflow
   |
   +-- linux-x64
   +-- linux-arm64
   +-- macos-x64
   +-- macos-arm64
   +-- windows-x64
   +-- windows-arm64
   |
   v
raw executable assets + SHA-256 sidecars
   |
   v
technical GitHub engine distribution: engine-v<version>
   |
   v
runtime/manifest.json receives verified hashes
   |
   v
Action version is reviewed and published
```

The engine build workflow is separated from the Action runtime path. Rust/Cargo exists only in the source/build/validation side of the service.

## Migration boundary

Main remains unchanged until this block has:

- a prebuilt engine for every declared platform;
- verified SHA-256 values in the manifest;
- Action execution validated without Cargo inside the Action itself;
- consumer-style immutable-SHA validation;
- second audit;
- documentation reconciliation;
- a final checkpoint.

Until those conditions are met, this branch is architecture-in-progress rather than a production runtime migration.

## Explicit non-goals

This block does not change the audit engine's Report v1 contract, change exit-code semantics, introduce telemetry, upload APK/AAB artifacts to Android Release Doctor servers, turn the product into an installable application, rewrite historical milestone documents, or create a paid distribution mechanism.

## Architectural outcome

Once the manifest is populated and this block is integrated, a consumer only needs a compatible GitHub Actions runner and network access to retrieve the pinned engine binary. A Rust/Cargo toolchain is no longer part of the consumer Action prerequisite.
