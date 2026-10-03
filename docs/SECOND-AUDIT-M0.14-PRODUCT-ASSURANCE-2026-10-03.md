# M0.14 — Product Assurance Expansion Second Audit — 2026-10-03

## Baseline and head

- Base main: `8668b8defb30caae699e1aa134bc01d65a4737c3`
- Final M0.14 head: `af250d870fac15f90fc06dabccbd9241c56e85e9`
- Pull request: #62
- Scope: test/assurance infrastructure only

## Validation evidence

### Direct Report v1 schema validation

Using the published `docs/report-schema-v1.0.json` as a draft 2020-12 schema and the `jsonschema` validator:

- success APK report — PASS;
- AAB report — PASS;
- project report — PASS;
- project + Play report — PASS;
- non-default Play platform report — PASS.

The actual generated files were validated, not merely hand-constructed report objects.

### Consumer-style immutable SHA Action

The workflow invoked:

`jedaqu/android_release_doctor/.github/actions/android-release-doctor@8668b8defb30caae699e1aa134bc01d65a4737c3`

The remote immutable-SHA Action completed successfully and its generated Report v1 passed the published schema validation.

This provides consumer-style reference evidence, although the invocation still occurs from the product repository's CI rather than a separately owned consumer repository.

### Deterministic artifact corpus

Twelve maintained APK/AAB corpus cases were exercised through the CLI. Every case returned an allowed audit result (exit 0 or blocker exit 1) and every generated Report v1 passed schema validation:

- minimal-release.apk
- minimal-release.aab
- crypto-v2-release.apk
- crypto-v2-empty-element-release.apk.b64 (materialized to APK at runtime)
- crypto-v3-release.apk
- crypto-v31-release.apk
- crypto-v31-lineage-mismatch.apk
- crypto-v31-no-v3-attr.apk
- crypto-v31-no-v3.apk
- crypto-v31-wrong-rotation-min-sdk.apk
- m07-crypto-v2-ecdsa-sha512-p384.apk
- m07-crypto-v3-ecdsa-sha512-p384.apk

### Existing product suite

The pre-existing suite remains green:

- 115 doctor-core unit tests;
- 25 doctor-core integration tests;
- 7 CLI integration tests;
- Build;
- Format;
- Clippy.

Total automated Rust/CLI tests: 147 passing.

## Findings

### ASSURE-001 — Published Report v1 schema is now runtime-validated

Resolved. Generated Action reports are checked against the published schema.

### ASSURE-002 — Consumer-style immutable Action reference

Resolved at the same-repository CI level. A true separately-owned external consumer repository is not available through the current connected GitHub write surface, so no claim of a separate consumer repository is made.

### ASSURE-003 — Broader deterministic corpus validation

Resolved. Twelve maintained APK/AAB cases now exercise the CLI and the published schema contract in CI.

## Product-defect assessment

No production product defect was exposed by M0.14.

The only initial failure was a test-harness assumption about `crypto-v2-empty-element-release.apk`: the repository stores that fixture as a `.b64` representation. The test was corrected to materialize it at runtime. The rerun passed.

The several exit-code 1/2 lines visible in the Action log correspond to intentionally exercised blocker and invalid-input paths; their surrounding validation steps all passed.

## Remaining assurance gap

The strongest remaining validation improvement is a genuinely separate consumer repository executing the Action from an immutable SHA, followed by a broader corpus sourced from independent third-party Android artifacts with explicit provenance and checksums.

## Disposition

M0.14 is technically validated and ready for integration.
