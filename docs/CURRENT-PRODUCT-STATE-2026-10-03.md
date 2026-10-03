# Current Product State — 2026-10-03

## Canonical current baseline

- Repository: `jedaqu/android_release_doctor`
- Visibility: public
- Default branch: `main`
- Current functional product/code baseline: `4dc8c27a3efbce45f5930d2b3b35baa33ffc39ea`
- Final runtime-integration main commit: `4dc8c27a3efbce45f5930d2b3b35baa33ffc39ea`
- Documentation-only reconciliation commits after runtime integration: PR #77 (`efa46e530cda020e50b7db3aecc017dae1c9e78a`) and PR #78 (`19c322e01e5e83099a814f926f36351a82be9135`).
- This document is a state snapshot; documentation-only commits do not redefine the functional product/code baseline.
- M0.16 readiness-gate baseline: `1298c73d3019799e76cf7ace9706c8f880a98d72`
- M0.11-A implementation integration baseline: `8e9f639ce71ce06e211718187d3079a5d68458f5`

The functional product/code baseline is kept distinct from later documentation-only commits. This document is a state snapshot and does not attempt to chase its own future documentation commits.

This document is the current-state companion to the historical milestone/audit records. It is not a replacement for them.

## Active service boundary

Android Release Doctor is a GitHub Actions service.

Its primary consumer interface is the repository-root Node 24 JavaScript GitHub Action defined by `action.yml`.

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
- the repository-root Node 24 GitHub Action with success, blocker, and operational-error self-test coverage exercised by the Action validation workflow targeting `main`;
- a published prebuilt engine runtime resolved from `runtime/manifest.json`, verified by SHA-256, with six intentional OS/architecture targets.

## Current runtime/distribution state

The current production runtime state is CLOSED and is not a branch-local migration in progress.

- Action execution: repository-root Node 24 JavaScript Action; no consumer-side Cargo execution.
- Engine version: `0.1.3`, selected from the published runtime manifest unless overridden by the supported `engine-version` input.
- Active technical distribution: `engine-v0.1.3-build3`.
- Runtime targets: Linux x64, Linux ARM64, macOS x64, macOS ARM64, Windows x64, Windows ARM64.
- Runtime integrity: immutable manifest SHA-256 verification on download and cached-runtime reuse.
- Production runtime retrieval, cache recovery, and intentional digest-mismatch rejection: validated.
- PR #76 integrated the runtime architecture at product/code baseline `4dc8c27a3efbce45f5930d2b3b35baa33ffc39ea`.
- PR #77 is a docs-only post-merge reconciliation; final repository `main` is `efa46e530cda020e50b7db3aecc017dae1c9e78a`.
- Final post-PR #77 Rust CI `37157332204` and Action Validation `37157332205`: SUCCESS.

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



## Historical milestone record — M0.12 — Consolidated product surface (CLOSED)

At M0.12, the maintained product surface was explicitly consolidated into:

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


## Historical branch-state record — Service runtime migration — 2026-10-03

At this stage, the service branch was migrating the public Action from consumer-side Cargo execution to a Node 24 Action that retrieves a prebuilt Rust engine runtime. This section is historical branch-state evidence; it does not describe the current `main` execution model.

Branch-local architecture:

- repository-root `action.yml` using Node 24;
- `runtime/manifest.json` selecting the engine version and exact platform asset;
- SHA-256 verification before execution and on cached binaries;
- runner-local runtime cache;
- six declared runtime targets: Linux x64, Linux ARM64, macOS x64, macOS ARM64, Windows x64, and Windows ARM64;
- technical engine distribution tags of the form `engine-v<version>` as transport/versioning mechanisms only;
- `.github/workflows/engine-runtime.yml` builds the six native binaries and gates publication on asset and checksum validation.

The six-target matrix is an intentional service architecture decision: the Action is designed to be multiplatform, while each consumer runner retrieves only the runtime matching its own operating system and architecture.

The production runtime gate remains open until a real technical engine distribution is built, its six SHA-256 values are independently verified, the manifest leaves `draft`, and real consumer retrieval through the published runtime path is validated.

This section records what was branch-local active work at that point. It does not retroactively rewrite the historical main-state sections above.

## Historical branch-state validation — six-platform build/hash gate — 2026-10-03

The controlled runtime build/hash gate has now been validated independently on the active service branch.

- Validation Run: `37154787258` — SUCCESS.
- All six native build jobs succeeded.
- Aggregate job downloaded all six GitHub Actions artifacts and independently verified the packaged binary files against their sidecar SHA-256 values.
- Exactly six entries were accepted into `SHA256SUMS.txt`.

Verified binary SHA-256 values:

| Target | Binary SHA-256 |
|---|---|
| Linux x64 | `44e7a0fc87fdbed57962acf7bf7d6491f4987bc2316cb8c652d0a4576f794760` |
| Linux ARM64 | `6f96be49ca16e0cf3562f3809a412c4c243fd0feffa091246b006088a52bc48d` |
| macOS x64 | `513f11988de3faf41e7d62cd1132693271bc25c32bd3ebdd2a80b2659c68116e` |
| macOS ARM64 | `197475e7e59d331db2e5aa934c0c8919d87dc19cb42008d68db54ee33179752c` |
| Windows x64 | `392380402e5e2c157ef190b5a97efe69847ef02334a664be965a552b09752331` |
| Windows ARM64 | `0e772cb45a4c589691cac1d4017859cd9f469bba88cd477a78657d2ae885b103` |

These are the hashes of the six packaged runtime binaries themselves. The GitHub Actions artifact ZIP digests are separate transport-level digests and are not used as the runtime manifest values.

The temporary branch-only trigger used to exercise this gate has been removed from `.github/workflows/engine-runtime.yml`. The production workflow remains tag-driven for technical publication.

### Remaining production publication gate

The six-platform binary build and independent hash verification are closed.

Still open:

- technical publication under the intended `engine-v0.1.3` distribution tag;
- population of `runtime/manifest.json` from the verified six hashes and transition from `draft` to `published`;
- real Action download/cache execution and intentional digest-mismatch rejection;
- final reconciliation into `main`;
- second audit and final checkpoint before merge/closure.

The existing public `engine-v0.1.3` tag currently points to the earlier failed gate commit; it was intentionally not moved during this validation because the technical distribution was not yet published. A corrected immutable publication reference must be established before the manifest can be activated.


## Historical branch closure record — Service runtime migration — 2026-10-03

The Node 24 prebuilt-runtime migration is validated through the complete production runtime path.

- Six native targets remain the active multiplatform runtime architecture.
- Runtime manifest is published and points to engine-v0.1.3-build3.
- Real Action retrieval, cache digest revalidation, and intentional download digest mismatch rejection are validated.
- Permanent Rust CI and Action Validation both pass on branch head 5096bc489f9046489b0fcf5acde82caad7745909.
- The canonical main HEAD and post-merge CI results will be reconciled immediately after PR #76 integration.


## PR #76 main integration — historical closure record — 2026-10-03

PR #76 was merged into main at merge commit 4dc8c27a3efbce45f5930d2b3b35baa33ffc39ea.

Post-merge validation:
- Rust CI Run 37156909779 — SUCCESS.
- Action Validation Run 37156909843 — SUCCESS.

The Node 24 prebuilt-runtime architecture is now integrated into main. The production runtime manifest is published and points to engine-v0.1.3-build3. The six-platform runtime distribution and production Action retrieval integrity checks are closed.

The main-state current truth is now:
- Android Release Doctor remains a GitHub Actions service.
- The reusable Action resolves and verifies a prebuilt engine matching the runner platform and architecture.
- The runtime manifest is published with immutable SHA-256 values.
- Production cache revalidation and digest mismatch rejection are validated.
- The six-target multiplatform service runtime remains intentional and unchanged.

This section records the historical branch-to-main reconciliation for PR #76. It is preserved as engineering history and is not the final current-state baseline.


## Final post-PR #77 reconciliation — 2026-10-03

PR #77 completed the final documentation reconciliation after the PR #76 runtime integration.

- PR #77 — “Finalize main integration reconciliation” — MERGED.
- Final main merge commit: `efa46e530cda020e50b7db3aecc017dae1c9e78a`.
- Rust CI Run `37157332204` — SUCCESS on the final main commit.
- Action Validation Run `37157332205` — SUCCESS on the final main commit.
- The production runtime distribution remains `engine-v0.1.3-build3`.
- The runtime manifest remains published and resolves the six official targets with immutable SHA-256 values.
- No product-code or runtime-behavior change was introduced by PR #77; it completed the post-merge documentation/current-state reconciliation.

The canonical current-state baseline is therefore:

`main @ efa46e530cda020e50b7db3aecc017dae1c9e78a`

The prebuilt-runtime migration, technical publication, main integration, post-merge validation, and documentation reconciliation are CLOSED. Any next work begins as a new product phase and must not reopen the completed runtime migration without new evidence of a defect.
