# Current Product State — 2026-10-03

## Canonical current baseline

- Repository: `jedaqu/android_release_doctor`
- Visibility: public
- Default branch: `main`
- Current validated main HEAD: `6fec2fe8fe0eb71fd01cd59cba69f059dc1f5b91`
- M0.16 readiness-gate baseline: `1298c73d3019799e76cf7ace9706c8f880a98d72`
- M0.11-A implementation integration baseline: `8e9f639ce71ce06e211718187d3079a5d68458f5`

The exact moving `main` SHA is recorded by the latest validated checkpoint. This document records the current validated product state and is updated as validated work advances.

This document is the current-state companion to the historical milestone/audit records. It is not a replacement for them.

## Active service boundary

Android Release Doctor is a GitHub Actions service.

Its primary consumer interface is the reusable GitHub composite Action:
`action.yml` at the repository root.

The service is backed by:

1. Rust audit engine: `crates/doctor-core`.
2. Local CLI interface: `crates/doctor-cli`.
3. Report v1 machine-readable output with `schema_version = "1.0"` and the public JSON Schema at `docs/report-schema-v1.0.json`.
4. Public technical fixtures and engineering evidence.
5. Permanent CI validation for the maintained service surface.
6. `docs/ERRORS-AND-FIXES.md` as append-only engineering memory.

Historical campaigns, temporary experiments, and superseded validation workflows are not part of the active service surface.

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

### Report v1 runtime contract

Report v1 output is validated against the published Draft 2020-12 schema in permanent assurance tests.

### Remaining assurance breadth

The current service evidence includes deterministic maintained fixtures plus independent public APK/AAB artifacts. Broader corpus expansion remains optional evidence work, not a missing service contract.

## Current documentation hierarchy

For understanding the repository today, use this order:

1. `README.md` — public product entry point.
2. `docs/CURRENT-PRODUCT-STATE-2026-10-03.md` — current product truth.
3. Latest checkpoint / validated audit for the current closed milestone.
4. `docs/ERRORS-AND-FIXES.md` — append-only chronology plus later reconciliations.
5. Historical milestone/pre-audit/second-audit documents — engineering history, not current product contract.

## Coherence rule

A product capability is not considered current merely because code exists.

The current state must be coherent across:

`implementation + tests + CI evidence + public documentation + schemas/contracts + active checkpoint`.

When a validated change alters product behavior or scope, the corresponding current documentation and contractual artifacts must be updated in the same controlled change or in the explicitly linked documentation follow-up before the block is closed.

Historical audit observations are preserved. Later status changes are recorded through append-only reconciliation or a newer current-state document; historical evidence is not silently rewritten.

## Service contract boundary

The public service contract is the Action input/output interface plus the Report v1 schema and exact exit semantics. Changes to these interfaces require controlled audit, validation, documentation, and second-audit closure.

## Public/private boundary

Public repository documentation must contain only material appropriate for the public service and engineering surface. Private continuity, passwords, credentials, commercial strategy, pricing, private metrics, and internal conversations remain outside this repository.



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

M0.13 implementation was integrated by PR #60 at `70fc650cfad15974f06e5b58b5ab3efcdfcf28f9`; closure documentation was reconciled by PR #61 at final M0.13 main `8668b8defb30caae699e1aa134bc01d65a4737c3`. Post-merge Rust CI and Action Validation both completed successfully.


## M0.14 — Product Assurance Expansion (CLOSED)

M0.14 moved validation beyond internal correctness into release-assurance evidence.

Validated:
- generated Report v1 documents are checked at runtime against the published `docs/report-schema-v1.0.json` Draft 2020-12 schema;
- the reusable Action executes successfully through an immutable repository-SHA reference;
- a deterministic 12-case APK/AAB corpus is audited through the CLI and its reports are schema-validated;
- the existing 147-test Rust/CLI suite remains green.

M0.14 assurance integration was followed by documentation/head reconciliation to final `main` at `d1d681e46391c95fd77152779ff4e3656995837f`. Post-merge Rust CI and Action Validation both completed successfully.

M0.15 subsequently resolved the remaining separate-consumer validation gap without changing production logic.

## M0.15 — Consumer Contract Validation (CLOSED)

M0.15 validated the published reusable GitHub Action from a genuinely separate consumer repository using the immutable current product revision:

`jedaqu/android_release_doctor/.github/actions/android-release-doctor@d1d681e46391c95fd77152779ff4e3656995837f`

Consumer validation evidence:
- a real third-party Meshtastic v2.8.1 APK was processed through the immutable Action reference;
- the artifact SHA-256 observed by the consumer workflow was `7f42735fd1c17c7e6a64d3a48ae1e4baf22cab778e994eb8331d3b993e42eb00`;
- successful path: Action output `exit-code=0`;
- blocking Play path: Action output `exit-code=1`;
- invalid format path: Action output `exit-code=2`;
- success and blocker Report v1 outputs both validated against the published schema;
- consumer-visible `report-path` and `exit-code` outputs were verified for each exercised path.

The consumer repository remains outside the public product repository; its identity and private project material are intentionally not recorded in this public document.

M0.15 introduced no production-code changes.

Final M0.15 integration:
- PR #65 merged at `937fe5059c4535a407c962893646a0d0b64de036`.
- External consumer Run `37141021253`: SUCCESS.
- Post-PR #65 Action Validation Run `37141293283`: SUCCESS.
- Post-PR #65 Rust CI Run `37141293281`: SUCCESS.
- Post-PR #66 Action Validation Run `37141546195`: SUCCESS.
- Post-PR #66 Rust CI Run `37141546206`: SUCCESS.

M0.15 is CLOSED. The next work should begin with a fresh checkpoint and pre-audit rather than modifying this closed milestone.


## M0.16 — Final Product Readiness Gate (CLOSED)

M0.16 performed the final engineering readiness gate against main `1298c73d3019799e76cf7ace9706c8f880a98d72`.

Validated:
- 147 automated Rust/CLI tests: 7 CLI + 115 unit + 25 integration, all passing;
- current permanent Rust CI and Action Validation both passing;
- Action Validation complete graph including Report v1 schema validation, immutable-SHA consumer-style path, deterministic corpus, and operational error path;
- external consumer Run `37141021253` successful across exit codes 0, 1, and 2;
- independent public APK/AAB evidence from Meshtastic, Reticulum, and Ventoid;
- exactly two maintained workflow files remain active;
- public repository scan found no tested restricted commercial/private-boundary terms.

M0.16 introduced no production-code changes. Its engineering disposition is CLOSED after the final readiness gate. Historical evidence remains preserved and the public/private boundary remains unchanged.
