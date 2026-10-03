# Checkpoint — Service Prebuilt CLI Runtime Architecture — 2026-10-03

## Checkpoint identity

- Repository: `jedaqu/android_release_doctor`
- Base main: `21e0d16c63ff67c70ed0155d5f2d869a0b3b10c3`
- Branch: `service/prebuilt-cli-runtime-2026-10-03`
- Purpose: remove the consumer Action's Rust/Cargo runtime dependency.

## Product boundary

Android Release Doctor remains a GitHub Actions service.

The prebuilt engine is an implementation/runtime transport. A technical `engine-v<version>` distribution is not a product release and does not create an installable application.

## Implemented branch state

- root `action.yml` uses Node 24;
- the Action launches `dist/index.js`;
- `dist/index.js` mirrors `src/action.js`;
- `engine-version` is an explicit optional input;
- `runtime/manifest.json` defines six initial platform targets and remains `draft`;
- runtime assets are selected by OS/architecture;
- SHA-256 is required before production execution;
- verified runtime binaries are cached locally;
- the Action never invokes Cargo;
- `.github/workflows/engine-runtime.yml` defines the platform build/distribution path;
- service-runtime architecture and audit documents are present.

## Validation boundary

The Action Validation workflow builds the Rust CLI as a separate CI build step and injects that binary only through the test-only `ANDROID_RELEASE_DOCTOR_TEST_RUNTIME_PATH` environment variable.

This is deliberately different from production execution: the Action itself never runs Cargo.

## Remaining gate

The branch is not production-ready.

Before main integration:

1. build and validate the declared runtime targets;
2. publish technical runtime assets;
3. populate and verify manifest SHA-256 values;
4. pin consumer-style validation to an immutable commit SHA;
5. complete second audit;
6. reconcile current-state documentation;
7. obtain final closure.

## Main protection

No main modification is part of this checkpoint.

Historical milestone documents remain unchanged.
