# Service Productization Audit — 2026-10-03

## Scope

This audit evaluates Android Release Doctor as a GitHub Actions service.

The audit covers:

- current service identity and consumer surface;
- Action execution model;
- service contract boundaries;
- GitHub Marketplace/service-delivery constraints;
- version/reference handling;
- runner dependencies;
- public/private separation;
- current documentation coherence.

This document does not define commercial pricing or private business strategy. Those decisions remain outside the public repository.

Historical milestone, audit, checkpoint, and ledger documents are preserved as engineering history. This audit changes only the current service framing and identifies future productization work.

## Current service identity

Android Release Doctor is a GitHub Actions service for auditing Android APK and AAB artifacts in CI.

The primary consumer interface is:

`.github/actions/android-release-doctor/action.yml`

The Action is backed by:

- `crates/doctor-core` — audit engine;
- `crates/doctor-cli` — local execution and validation interface;
- Report v1 — machine-readable service result;
- exact exit-code semantics;
- permanent Action and Rust validation.

The service receives workflow inputs, performs evidence-based auditing, and returns structured results to the calling workflow.

## Current service contract

### Inputs

- `artifact` — required APK/AAB path;
- `project` — optional static Android project path;
- `play` — optional Play readiness profile;
- `play-platform` — selected Play platform;
- `format` — text or Report v1 JSON;
- `output` — optional report destination.

### Outputs

- `exit-code` — exact CLI status;
- `report-path` — effective requested report path.

### Result contract

Report v1 remains the machine-readable contract at:

`docs/report-schema-v1.0.json`

The service distinguishes:

- PASS;
- WARNING;
- BLOCKER;
- MANUAL-REVIEW.

Exit semantics remain:

- `0` — audit completed without blockers;
- `1` — audit completed with one or more blockers;
- `2` — usage, input/audit, output, serialization, or internal error.

## Current implementation risks for service delivery

### SVC-001 — Action is coupled to the workspace

The Action currently runs the validated CLI through Cargo:

`cargo run --locked --manifest-path <workspace>/Cargo.toml -p doctor-cli`

This means the consumer runner must provide a usable Rust/Cargo toolchain.

This is a service-delivery dependency, not an audit-engine defect.

### SVC-002 — Action metadata is nested inside the engineering repository

The public repository currently contains:

- engine;
- CLI;
- Action;
- tests;
- extensive engineering documentation.

GitHub's current Marketplace guidance says an Action repository should contain the metadata, code, and files necessary for the Action, and that each repository must contain one Action metadata file at its root for automatic Marketplace listing. GitHub also recommends a single repository for an Action so it can be versioned and packaged as one unit.

Current state:

`.github/actions/android-release-doctor/action.yml`

Therefore the current monorepo is not yet shaped as a dedicated Marketplace Action repository.

No repository split is authorized by this audit.

Source:
https://docs.github.com/en/actions/how-tos/create-and-publish-actions/publish-in-github-marketplace

### SVC-003 — Reference/version governance is not yet formalized

Consumers can pin the Action to an immutable commit SHA, and the current validation workflow already exercises an immutable-SHA invocation.

Before public service distribution is formalized, the project still needs a controlled policy for:

- supported Git references;
- compatibility expectations;
- update cadence;
- rollback procedure;
- breaking versus compatible service changes.

GitHub documents immutable releases and tags as mechanisms for managing Action versions. These are transport/versioning mechanisms for the Action and do not redefine Android Release Doctor's service identity.

Source:
https://docs.github.com/en/actions/how-tos/create-and-publish-actions/using-immutable-releases-and-tags-to-manage-your-actions-releases

### SVC-004 — Marketplace commercial mechanism requires explicit verification

GitHub's current Marketplace documentation distinguishes GitHub Actions from Marketplace Apps.

The Marketplace Terms and Developer Agreement define Actions as Developer Products and state that Developer Products may be distributed for free or for a fee. The Developer Agreement also defines GitHub as merchant of record for paid Developer Products and describes payment/remittance terms.

At the same time, GitHub's detailed pricing-plan documentation is explicitly scoped to Marketplace Apps, not Actions.

Therefore the exact paid-service onboarding path for a paid Action must be verified against GitHub's current Marketplace flow before any commercial architecture is committed. The project must not assume that the App pricing-plan workflow applies to an Action.

Sources:
https://docs.github.com/en/site-policy/github-terms/github-marketplace-terms-of-service
https://docs.github.com/en/site-policy/github-terms/github-marketplace-developer-agreement
https://docs.github.com/en/apps/github-marketplace/selling-your-app-on-github-marketplace/pricing-plans-for-github-marketplace-apps

### SVC-005 — Privacy/service-data contract needs to be explicit before external service expansion

The current product is local-first and the Action does not upload audited artifacts or telemetry.

If a future service architecture introduces a hosted component, the project will need an explicit contract for:

- what data leaves the runner;
- what identifiers are processed;
- retention;
- support diagnostics;
- deletion;
- secret handling;
- customer-visible privacy terms.

No hosted-data behavior is introduced by this audit.

### SVC-006 — Support and security intake are not yet productized

The current engineering surface contains strong validation evidence but does not yet establish the complete service-support layer.

Before broad external service use, define:

- support contact;
- vulnerability reporting path;
- service issue taxonomy;
- compatibility reporting;
- service-status communication.

## Current documentation findings

Two stale references were found in the current-state document:

1. the documented current `main` SHA was one commit behind the real validated head;
2. the documented M0.16 baseline SHA contained a transcription error.

Both are documentation-only findings and were corrected on the controlled service-normalization branch.

## Public/private boundary

The public repository now frames the product around:

- GitHub Actions service consumption;
- audit engine;
- CLI validation interface;
- Report v1;
- Action validation.

Private continuity, credentials, internal strategy, pricing decisions, and private metrics remain outside the public repository.

## Historical-material rule

Historical audits, checkpoints, ledgers, and milestone documents are not rewritten solely to make current service terminology appear uniform.

Current service truth is defined by:

1. the README;
2. the current-state document;
3. the current Action contract;
4. the maintained tests/fixtures;
5. the active validation workflows.

Historical documents retain their original chronology.

## Audit conclusion

The engineering core is sufficiently validated to begin service productization work.

The main productization questions are now concentrated in the service layer:

- Action packaging;
- runner dependency;
- reference/version governance;
- Marketplace onboarding;
- service support;
- privacy/data boundaries.

No Rust audit logic, Report v1 schema, fixture semantics, or Action behavior was changed by this audit.

## Next controlled step

The next authorized implementation block should be a dedicated service-architecture pre-audit focused on the Action distribution unit and execution model.

That block must begin from a fresh checkpoint and must not modify audit semantics unless a separate finding explicitly authorizes it.
