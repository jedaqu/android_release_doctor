# Checkpoint — Service Prebuilt CLI Runtime — Final Branch State — 2026-10-03

## Identity

- Repository: `jedaqu/android_release_doctor`
- Main baseline: `21e0d16c63ff67c70ed0155d5f2d869a0b3b10c3`
- Branch: `service/prebuilt-cli-runtime-2026-10-03`
- Final branch state for this checkpoint: current branch head after the checkpoint commit
- PR: #76 — draft, not merged

## Product boundary

Android Release Doctor is a GitHub Actions service.

The user-facing product is the Action/service contract. Runtime binaries, technical engine tags, hashes, and distribution assets are implementation mechanisms.

No installable application or product-release packaging has been introduced.

## Validated architecture

```text
consumer workflow
      ↓
root Node 24 Action
      ↓
runtime manifest
      ↓
platform-specific prebuilt engine
      ↓
SHA-256 verification
      ↓
runner-local cache
      ↓
engine execution
      ↓
Report v1 + exit code
```

The consumer Action no longer needs Rust/Cargo.

## Evidence

- Action Validation Run `37152271333` — SUCCESS.
- Rust CI Run `37152271324` — SUCCESS.
- Immutable-SHA consumer validation — SUCCESS.
- Non-default Play platform path — SUCCESS.
- Runtime manifest schema validation — SUCCESS.
- Action package Cargo guard — SUCCESS.
- Direct prebuilt engine control — SUCCESS.
- Report v1 validation and deterministic corpus tests — SUCCESS.

## Errors resolved in this block

ERR-123, ERR-124, ERR-125, and ERR-126 were recorded and subsequently reconciled as resolved in `docs/ERRORS-AND-FIXES.md`.

## Branch contents

- root `action.yml` — Node 24 JavaScript Action;
- `src/action.js` — source runtime;
- `dist/index.js` — packaged runtime;
- `runtime/manifest.json` — draft engine manifest;
- `runtime/manifest.schema.json` — manifest schema;
- `scripts/verify-runtime-manifest.cjs` — manifest validator;
- `.github/workflows/engine-runtime.yml` — technical runtime build/distribution workflow;
- architecture audit and second-audit documents.

## Main protection

`main` remains at the pre-migration baseline.

No production manifest publication, technical engine distribution, or main integration is included in this checkpoint.

## Next authorized block

The next block is the real engine-runtime distribution:

- produce the declared six platform binaries;
- collect and independently verify their SHA-256 values;
- populate the manifest;
- validate the retrieval path from the Action;
- then prepare the final main-state reconciliation.

No merge to main is implied by this checkpoint.
