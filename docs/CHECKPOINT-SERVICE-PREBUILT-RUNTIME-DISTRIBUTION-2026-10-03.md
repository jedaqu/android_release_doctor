# Checkpoint — Six-Platform Prebuilt Engine Distribution Gate — 2026-10-03

## Authority and scope

- Product: Android Release Doctor.
- Product form: GitHub Actions service.
- This checkpoint continues PR #76 without changing the product identity.
- Dani explicitly confirmed that the prebuilt engine is an internal service component and that the six-platform runtime matrix must be retained for the multiplatform service goal.
- No reduction to Linux-only runtime support is authorized.

## Baseline

- Repository: `jedaqu/android_release_doctor`
- Main baseline: `21e0d16c63ff67c70ed0155d5f2d869a0b3b10c3`
- Branch: `service/prebuilt-cli-runtime-2026-10-03`
- PR: #76 — draft, open, not merged
- Branch head at checkpoint creation: `ea195a04c5017b689dc5366359f9e1e8e6a4ffdd`

## Pre-audit

The previous architecture and second-audit work already established:

- root Node 24 Action;
- prebuilt engine resolution;
- runtime manifest;
- SHA-256 verification;
- runner-local cache;
- six-platform target mapping;
- consumer-style immutable-SHA Action validation;
- no Cargo invocation by the consumer Action.

Fresh current verification before continuing confirmed GitHub-hosted runner labels for the six targets:

- `ubuntu-24.04` — Linux x64;
- `ubuntu-24.04-arm` — Linux ARM64;
- `macos-15-intel` — macOS x64;
- `macos-15` — macOS ARM64;
- `windows-2025` — Windows x64;
- `windows-11-arm` — Windows ARM64.

GitHub's current runner documentation lists these hosted runner families. ARM64 Linux is documented as public preview.

## Controlled implementation in this continuation

The runtime distribution workflow was hardened without changing the Action input/output or audit-engine contract.

Added:

- `scripts/verify-engine-assets.cjs` — verifies all six expected runtime assets and independently recomputes each SHA-256 from the packaged file.
- Engine source/version consistency gate before compilation.
- Non-empty packaged binary checks.
- Six-asset verification gate before technical distribution publication.
- Six-entry `SHA256SUMS.txt` generation check.
- Duplicate technical-release rejection.
- Current branch-state reconciliation in `docs/CURRENT-PRODUCT-STATE-2026-10-03.md`.
- Append-only scope reconciliation in `docs/SECOND-AUDIT-SERVICE-PREBUILT-RUNTIME-2026-10-03.md`.

## Remaining gate

The production runtime is not yet considered published.

Still required:

1. Execute the real engine runtime build for `engine-v0.1.3`.
2. Verify all six produced binaries and SHA-256 values.
3. Create the technical GitHub engine distribution.
4. Populate `runtime/manifest.json` with the verified SHA-256 values and move it from `draft` to `published`.
5. Validate real Action retrieval and execution from the published runtime distribution.
6. Reconcile the final integrated `main` state.
7. Run the second audit against the production-integrated state and close with a final checkpoint.

## Non-goals

This checkpoint does not:

- create an installable application;
- redefine Android Release Doctor as a release/package;
- introduce telemetry or server-side upload of artifacts;
- change Report v1;
- change exit-code semantics;
- introduce pricing or monetization;
- expose private continuity or commercial material.

## Discipline

No production integration into `main` occurs while the runtime distribution gate remains open.

## Validation after controlled implementation

The continuation changes were validated by the permanent PR workflows on this branch:

- Action Validation Run `37153106170` — SUCCESS.
- Rust CI Run `37153106161` — SUCCESS.

This closes the branch-local implementation/test portion of the checkpoint. The only remaining production gate is the real six-platform technical engine distribution and subsequent manifest population/retrieval validation.
