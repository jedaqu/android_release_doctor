# Current Product State — 2026-10-03

## Canonical current baseline

- Repository: `jedaqu/android_release_doctor`
- Visibility: public
- Default branch: `main`
- Current main HEAD: `b2baf2823d5bb5472a3c978ce092d75d81de94f0`
- M0.11-A implementation integration baseline: `8e9f639ce71ce06e211718187d3079a5d68458f5`

The exact moving `main` SHA is recorded by the latest validated checkpoint. This document records the current validated product state and is updated as validated work advances.

This document is the current-state companion to the historical milestone/audit records. It is not a replacement for them.

## Active product boundary

Android Release Doctor is currently the following product surface:

1. CLI: `android-release-doctor`.
2. Rust audit engine: `crates/doctor-core`.
3. Reusable GitHub composite Action: `.github/actions/android-release-doctor/action.yml`.
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

The remaining items below are product-hardening boundaries, not completed capabilities:

### Play policy semantics

The Play profile evaluates target API and 16 KiB evidence using deterministic artifact rules. Policy-context nuances that depend on submission context, dates, extensions, or Play Console state remain outside artifact-only proof.

### AAB final-APK equivalence

Bundle contents can be inspected, but raw AAB ZIP offsets do not prove the final generated APK's native packaging alignment. This remains a manual-review boundary.

### Report v1 schema validation

The product emits Report v1 JSON and maintains the published JSON Schema. Direct runtime validation of every generated report against the schema file is a future validation improvement.

### External consumer black-box coverage

The reusable Action is validated in-repository. A separate external consumer repository remains a future validation improvement.

## Current documentation hierarchy

For understanding the repository today, use this order:

1. `README.md` — public product entry point.
2. `docs/CURRENT-PRODUCT-STATE-2026-10-03.md` — current product truth.
3. Latest checkpoint / validated audit for the closed M0.12 Product Surface Consolidation block.
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



## M0.12 — Consolidated product surface (CLOSED)

The maintained product surface is explicitly consolidated into:

- `doctor-core`: the reusable Rust audit engine;
- `doctor-cli`: the local command-line interface;
- `.github/actions/android-release-doctor/action.yml`: the reusable GitHub composite Action;
- Report v1 and `docs/report-schema-v1.0.json`;
- fixtures/regression tests supporting the maintained behavior;
- permanent CI validation through `.github/workflows/rust.yml` and `.github/workflows/android-release-doctor-action.yml`.

Development campaigns, stress tests, temporary fixture-harvest workflows, and historical validation workflows are not current product interfaces.


## M0.12 closure evidence

The consolidated technical surface was integrated by PR #58 at main commit `508edc22c7111aaada06fe885b93f224a1780853`.

Validated pre-merge on consolidation head `58449d35ce3e1b81025962f93d0d604cc3cf10ec`:

- Rust CI Run `37129863362` — SUCCESS.
- Action validation Run `37129863355` — SUCCESS.

Permanent workflow files after consolidation:

- `.github/workflows/rust.yml`
- `.github/workflows/android-release-doctor-action.yml`

The historical M0.8 workflow path is no longer present in the current repository tree. Historical execution records may still remain visible in GitHub Actions history.


## M0.13 — Product Validation Expansion (CLOSED)

M0.13 expanded the permanent Action validation without changing production audit behavior.

The validated suite now includes:
- APK success and Play-blocker paths;
- AAB input;
- project input;
- combined project + Play input;
- non-default Play platform;
- operational error;
- invalid `play` input;
- invalid output format;
- invalid Play platform;
- CR/LF output-path rejection.

Automated Rust/CLI validation remains:
- 115 doctor-core unit tests;
- 25 doctor-core integration tests;
- 7 CLI integration tests;
- Build, Format and Clippy all passing.

M0.13 was integrated by PR #60 at main commit `70fc650cfad15974f06e5b58b5ab3efcdfcf28f9`, followed by successful post-merge Rust CI Run `37132758711` and Action Validation Run `37132758674`.


## M0.14 — Product Assurance Expansion (CLOSED)

M0.14 moved validation beyond internal correctness into release-assurance evidence.

Validated:
- generated Report v1 documents are checked at runtime against the published `docs/report-schema-v1.0.json` Draft 2020-12 schema;
- the reusable Action executes successfully through an immutable repository-SHA reference;
- a deterministic 12-case APK/AAB corpus is audited through the CLI and its reports are schema-validated;
- the existing 147-test Rust/CLI suite remains green.

Post-merge validation on `main`:
- Android Release Doctor — Action Validation Run `37135654381`: SUCCESS.
- Rust CI Run `37135654379`: SUCCESS.

The remaining assurance boundary is a genuinely separate consumer repository executing the Action from an immutable SHA. The current connected GitHub write surface does not provide repository-creation/write support needed to establish that separate consumer repository without modifying an unrelated repository, so no separate-consumer result is claimed.


Post-closure coherence: M0.14 closure documentation was integrated by PR #63 at main commit `b2baf2823d5bb5472a3c978ce092d75d81de94f0`. Post-merge Rust CI and Action Validation both completed successfully.
