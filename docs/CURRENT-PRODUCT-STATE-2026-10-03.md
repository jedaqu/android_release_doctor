# Current Product State — 2026-10-03

## Canonical current baseline

- Repository: `jedaqu/android_release_doctor`
- Visibility: public
- Default branch: `main`
- Current main HEAD: `325154034f965de475d4214959d9608c60f82de8`
- M0.11-A implementation integration baseline: `8e9f639ce71ce06e211718187d3079a5d68458f5`

The exact moving `main` SHA is recorded by the latest validated checkpoint. This document records the current validated product state and is updated as validated work advances.

This document is the current-state companion to the historical milestone/audit records. It is not a replacement for them.

## Active product boundary

Android Release Doctor is currently the following product surface:

1. CLI: `android-release-doctor`
2. Rust audit engine: `crates/doctor-core`
3. Reusable GitHub composite Action: `.github/actions/android-release-doctor/action.yml`
4. Report v1 machine-readable output:
   `android-release-doctor`
3. Rust audit engine:
   APK/AAB inventory, manifest/component evidence, static Gradle cross-checks, Play readiness, signing evidence, classical cryptographic verification, proof-of-rotation evidence, and native/16 KiB inspection.
4. Report v1 machine-readable output:
   `schema_version = "1.0"`
   with the public JSON Schema at `docs/report-schema-v1.0.json`.
5. Public technical fixtures and engineering evidence.
6. Permanent CI validation for the maintained product surface.
7. `docs/ERRORS-AND-FIXES.md` as append-only engineering memory.

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
- a reusable GitHub Action with success, blocker, and operational-error self-test coverage exercised by the Action validation workflow targeting `main`.

## Current hardening gaps

The following are open product-hardening candidates and are intentionally not represented as completed capabilities:

### Action validation on current main

M0.11-A integrated the existing bounded Action self-test into the `main` push and `main` pull-request trigger paths. The pull-request-to-`main` validation for PR #55 completed successfully on Run `37127173597`, covering success, blocker, Report v1, output-path, exit-code, and operational-error behavior.

### Action dependency reproducibility

The reusable Action now invokes Cargo with `--locked`, matching the locked dependency-resolution boundary enforced by the normal Rust CI. This correction was validated on Action Run `37127173597` and Rust CI Run `37127173629` before integration.

### Play policy semantics

The Play profile currently evaluates target API and 16 KiB evidence using deterministic artifact rules. A separate product audit is still required for policy-context nuances that depend on submission context, dates, extensions, or other Play Console state that the artifact cannot prove.

### AAB final-APK equivalence

AAB inspection remains intentionally bounded. Bundle contents can be inspected, but raw AAB ZIP offsets do not prove the final generated APK's native packaging alignment. The current manual-review boundary is intentional.

## Current documentation hierarchy

For understanding the repository today, use this order:

1. `README.md` — public product entry point.
2. `docs/CURRENT-PRODUCT-STATE-2026-10-03.md` — current product truth.
3. Latest checkpoint / validated audit for the active work block (`M0.12 product surface consolidation is the current active technical block; M0.11-A remains closed.).
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



## M0.12 — Consolidated product surface

The maintained product surface is explicitly consolidated into:

- `doctor-core`: the reusable Rust audit engine;
- `doctor-cli`: the local command-line interface;
- `.github/actions/android-release-doctor/action.yml`: the reusable GitHub composite Action;
- Report v1 and `docs/report-schema-v1.0.json`;
- fixtures/regression tests supporting the maintained behavior;
- permanent CI validation through `.github/workflows/rust.yml` and `.github/workflows/android-release-doctor-action.yml`.

Development campaigns, stress tests, temporary fixture-harvest workflows, and historical validation workflows are not current product interfaces.
