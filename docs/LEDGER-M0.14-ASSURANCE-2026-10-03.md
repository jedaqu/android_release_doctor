# M0.14 — Assurance Ledger Reconciliation

## ASSURE-001 — Report v1 schema runtime validation

Generated success, AAB, project, project+Play, automotive, and immutable-SHA consumer-style Action reports were validated against the published Draft 2020-12 schema.

Status: RESOLVED.

## ASSURE-002 — Immutable SHA Action reference

The Action was executed through `jedaqu/android_release_doctor/.github/actions/android-release-doctor@8668b8defb30caae699e1aa134bc01d65a4737c3` and the resulting report passed schema validation.

Status: RESOLVED at same-repository consumer-style level; separate consumer repository remains unverified.

## ASSURE-003 — Deterministic artifact corpus

Twelve maintained APK/AAB cases were audited through the CLI, accepting documented exit 0 or blocker exit 1, and every report passed the published schema.

Status: RESOLVED.

## ASSURE-004 — Encoded fixture handling

The first corpus run exposed that `crypto-v2-empty-element-release.apk` is stored in the repository as `.b64`. The assurance workflow now materializes the encoded fixture at runtime before auditing.

Status: RESOLVED — test-harness correction, not product defect.
