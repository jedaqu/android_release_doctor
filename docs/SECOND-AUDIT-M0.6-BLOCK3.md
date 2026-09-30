# M0.6 Block 3 — Second audit

Date: 2026-09-30
Implementation branch: `m06-block3-verification-hardening`
Validated M0.6 Block 2 baseline: `c99ff30c28022d70ebc2ce20e73becf99f97475e`

## Audit result

All four pre-change findings were rechecked after implementation.

### AUDIT-009 — v3 targeted multi-signer handling

Status: **IMPLEMENTED**

`verify_v3_block()` now parses every v3 signer entry instead of rejecting the block because more than one signer exists.

Each signer is independently checked for:
- outer/inner minSDK/maxSDK consistency;
- supported signature selection;
- signer certificate/public-key binding;
- cryptographic signature validity;
- APK content digest;
- digest/signature algorithm-list equality.

The per-signer results are merged without claiming runtime-platform signer selection.

### AUDIT-010 — v3 SDK-range evidence

Status: **IMPLEMENTED**

`CryptoSchemeInfo` now exposes:
`sdk_ranges: Vec<(u32, u32)>`.

v3 results retain each signer's declared SDK range. v2 results keep an empty range list because v2 has no equivalent signer-range field.

The real v3 fixture test now asserts that a verified v3 result retains one valid SDK range.

A focused unit test also verifies that multiple verified targeted signer results merge while retaining both ranges.

### AUDIT-011 — stacked CI trigger coverage

Status: **IMPLEMENTED**

The workflow now:
- triggers push validation for `m06-block1-state-hardening`;
- triggers push validation for `m06-block2-signing-evidence`;
- triggers push validation for `m06-block3-verification-hardening`;
- retains the validated M0.5 branches;
- triggers pull-request validation when targeting the stacked M0.6 base branches, including `m06-block2-signing-evidence`.

Duplicate `m06-block2-signing-evidence` push entries were removed.

No CI job graph redesign was introduced.

### AUDIT-012 — repository status

Status: **IMPLEMENTED**

The top-level README now identifies M0.6 Block 3 as in progress. Historical checkpoint and audit records remain unchanged.

## Scope check

No changes were introduced for:
- new cryptographic algorithms;
- cryptographic backend replacement;
- complete proof-of-rotation verification;
- v3.1 cryptographic verification;
- v3.2 hybrid/PQC verification;
- AAB cryptographic verification;
- new Play policy rules;
- broad report/CLI redesign.

## Remaining validation

The implementation must still pass GitHub Actions Build, Test, Format and Clippy.

Any Actions failure will be handled individually and minimally before the next run.

No merge is performed as part of this audit.
