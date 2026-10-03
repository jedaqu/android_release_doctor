# Audit — Service Prebuilt CLI Runtime — 2026-10-03

## Scope

This audit covers the next service-architecture block after the repository-root Action correction.

Objective: remove the Rust/Cargo toolchain requirement from consumer runners while preserving the existing audit engine, Report v1 schema, exit-code contract, local CLI, and public/private boundary.

The product remains a GitHub Actions service. No installable application or product release model is introduced.

## Baseline

- Repository: `jedaqu/android_release_doctor`
- Base main: `21e0d16c63ff67c70ed0155d5f2d869a0b3b10c3`
- Working branch: `service/prebuilt-cli-runtime-2026-10-03`
- Architecture implementation baseline: `c18ef723c5fb488a071299914362018ba8193b5d`

## Existing gap

### SVC-001 — consumer Action depends on Rust/Cargo

The previous root Action executed `cargo run` on the consumer runner. This makes Rust/Cargo part of the consumer environment and conflicts with the current GitHub guidance for third-party CLI Actions, which emphasizes distributing/selecting a CLI build rather than compiling the CLI in every consumer workflow.

## Selected architecture

The Action is migrated from a composite/Cargo implementation to a JavaScript Action running on Node 24.

Runtime flow:

```text
consumer workflow
    ↓
root action.yml
    ↓
Node 24 bootstrapper
    ↓
runtime/manifest.json
    ↓
OS/architecture resolution
    ↓
prebuilt engine download
    ↓
SHA-256 verification
    ↓
runner-local cache
    ↓
android-release-doctor executable
    ↓
Report v1 + exit code
```

The Rust workspace remains the authoritative source/build/local-validation surface for the engine. It is removed from the consumer execution path, not from the project.

## Runtime version separation

Three technical identities remain separate:

1. Action reference: the immutable Git commit consumed through `uses:`.
2. Engine version: the prebuilt CLI version selected through `engine-version` or the manifest default.
3. Technical engine distribution tag: `engine-v<version>`.

The third is only a transport/versioning mechanism for the service implementation. It does not redefine the product as a release or installable application.

## Integrity controls

The Action:

- reads a committed runtime manifest;
- resolves one exact asset for the runner platform;
- downloads only the declared asset;
- computes SHA-256 locally;
- refuses execution on a digest mismatch;
- verifies cached binaries again before reuse;
- launches the binary directly without a shell.

The manifest remains `draft` until all declared runtime assets exist and their SHA-256 values have been independently verified.

## Platform scope

Initial runtime targets:

- linux-x64
- linux-arm64
- macos-x64
- macos-arm64
- windows-x64
- windows-arm64

The selected GitHub-hosted runner labels exist in the current runner matrix. ARM64 coverage is retained because the current GitHub runner documentation exposes corresponding Linux, Windows, and macOS ARM64 labels.

## Consumer-runner requirements after migration

Required:

- a compatible GitHub Actions runner;
- Node 24 support through the Actions runtime;
- network access to the public technical engine distribution when the binary is not already cached.

No Rust/Cargo toolchain is required to execute the Action.

## Cache boundary

The cache is an optimization, not a trust root.

Cache lookup is scoped by:

`android-release-doctor/<engine-version>/<platform>/<asset>`

A cached executable is accepted only when its SHA-256 equals the committed manifest value.

## Security boundary

The implementation does not:

- execute downloaded content through a shell;
- accept an arbitrary download URL as an Action input;
- trust a release asset without digest verification;
- upload APK/AAB artifacts or reports to an Android Release Doctor server;
- add telemetry.

The public Action and runtime metadata contain no private continuity information, credentials, commercial strategy, pricing, or secrets.

## Current implementation status

Implemented on the architecture branch:

- Node 24 root Action metadata;
- optional `engine-version` input;
- runtime platform resolver;
- draft runtime manifest;
- manifest schema;
- runtime-manifest structural validator;
- verified-download and cache loader;
- direct engine process execution;
- platform-targeted engine build workflow;
- architecture documentation;
- branch-local Action Validation path using a locally built executable as a test-only runtime override.

Not yet complete:

- production engine binaries for all six platforms;
- independently recorded SHA-256 values;
- production manifest publication;
- final immutable-SHA consumer validation;
- second audit;
- main integration.

## Findings

| Finding | State |
|---|---|
| SVC-001 Cargo dependency on consumer runner | Architecture correction implemented; operational closure pending runtime distribution |
| Runtime version selection | Implemented through manifest + optional `engine-version` |
| Cross-platform runtime matrix | Declared and backed by current GitHub runner labels |
| Binary integrity | SHA-256 verification implemented |
| Cache trust model | Digest verification required before reuse |
| Action output metadata | Corrected for JavaScript Action semantics |
| Product identity | Preserved as GitHub Actions service |
| Private/public boundary | Preserved |

## Decision

Proceed with prebuilt runtime implementation and validation on the architecture branch.

Do not merge to `main` while the manifest is `draft` and runtime assets are absent.

## References

- https://docs.github.com/en/actions/how-tos/create-and-publish-actions/create-a-cli-action
- https://docs.github.com/en/actions/tutorials/create-actions/create-a-javascript-action
- https://docs.github.com/en/actions/reference/workflows-and-actions/metadata-syntax
- https://docs.github.com/en/actions/reference/security/secure-use
- https://docs.github.com/en/actions/reference/runners/github-hosted-runners
