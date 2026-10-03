# Current Product State — 2026-10-03

## Canonical current baseline

- Repository: `jedaqu/android_release_doctor`
- Visibility: public
- Default branch: `main`
- Current main HEAD: `4a41b80415b94ffe1748ec07a10a4cbe0224ee1a`

This document is the current-state companion to the historical milestone/audit records. It is not a replacement for them.

## Active product boundary

Android Release Doctor is currently the following product surface:

1. Reusable GitHub composite Action:
   `.github/actions/android-release-doctor/action.yml`
2. CLI:
   `android-release-doctor`
3. Rust audit engine:
   APK/AAB inventory, manifest/component evidence, static Gradle cross-checks, Play readiness, signing evidence, classical cryptographic verification, proof-of-rotation evidence, and native/16 KiB inspection.
4. Report v1 machine-readable output:
   `schema_version = "1.0"`
   with the public JSON Schema at `docs/report-schema-v1.0.json`.
5. Public technical fixtures and engineering evidence.
6. `docs/ERRORS-AND-FIXES.md` as append-only engineering memory.

Historical binary distribution, public-release packaging, and publication workflows are not part of the active product surface.

## Demonstrated current capabilities

The current validated implementation includes:

- APK and AAB inventory and base-module manifest inspection;
- package/version/SDK/debuggable evidence;
- component/exported checks and permission inventory;
- static Groovy/Kotlin Gradle cross-checks;
- versioned Google Play readiness profile with platform selection;
- ELF native inspection;
- 16 KiB ZIP/native checks with the applicable 64-bit ABI boundary `arm64-v8a` / `x86_64`;
- APK v2/v3/v3.1 cryptographic verification for the supported classical RSA/ECDSA matrix;
- supported v3 proof-of-rotation verification;
- explicit unsupported/manual-review boundaries rather than guessed passes;
- Report v1 JSON and stable CLI exit semantics;
- a reusable GitHub Action with success, blocker, and operational-error self-test coverage on its dedicated historical validation workflow.

## Current hardening gaps

The following are open product-hardening candidates and are intentionally not represented as completed capabilities:

### Action validation on current main

The dedicated Action self-test workflow is currently scoped to the historical `m08-block3-github-action` branch and its stacked PR target. Current `main` Rust CI does not execute the Action self-test.

### Action dependency reproducibility

The Action invokes the repository CLI through Cargo using `cargo run` without `--locked`. The normal Rust CI now enforces `--locked`. This is a current consistency/hardening gap and not yet a correction.

### Play policy semantics

The Play profile currently evaluates target API and 16 KiB evidence using deterministic artifact rules. A separate product audit is still required for policy-context nuances that depend on submission context, dates, extensions, or other Play Console state that the artifact cannot prove.

### AAB final-APK equivalence

AAB inspection remains intentionally bounded. Bundle contents can be inspected, but raw AAB ZIP offsets do not prove the final generated APK's native packaging alignment. The current manual-review boundary is intentional.

## Current documentation hierarchy

For understanding the repository today, use this order:

1. `README.md` — public product entry point.
2. `docs/CURRENT-PRODUCT-STATE-2026-10-03.md` — current product truth.
3. Latest checkpoint / validated audit for the active work block.
4. `docs/ERRORS-AND-FIXES.md` — append-only chronology plus later reconciliations.
5. Historical milestone/pre-audit/second-audit documents — engineering history, not current product contract.

## Coherence rule

A product capability is not considered current merely because code exists.

The current state must be coherent across:

`implementation + tests + CI evidence + public documentation + schemas/contracts + active checkpoint`.

When a validated change alters product behavior or scope, the corresponding current documentation and contractual artifacts must be updated in the same controlled change or in the explicitly linked documentation follow-up before the block is closed.

Historical audit observations are preserved. Later status changes are recorded through append-only reconciliation or a newer current-state document; historical evidence is not silently rewritten.

## Public/private boundary

Public repository documentation must contain only material appropriate for the public project surface. Private continuity, passwords, credentials, commercial strategy, pricing, private metrics, and internal conversations remain outside this repository.

