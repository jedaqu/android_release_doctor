# M0.6 Block 2 — Second audit

Date: 2026-09-30
Validated implementation candidate before Actions: `6e35d6299880f05fe24c9d6eec34bb99fe61e5fc`
Base: `3b50eab248359e1b2b923993c97a2b64e6874158`
Branch: `m06-block2-signing-evidence`

## Audit result

The four pre-change findings were rechecked after implementation.

### AUDIT-005 — Manual-review semantics

Status: **IMPLEMENTED**

First-class `Severity::ManualReview` is now used for artifact-proof/external-verification boundaries in:
- `PLAY-002`
- `PLAY-003`
- `PLAY-004`
- `PLAY-005` when native evidence is unavailable
- `NATIVE-002` when ELF evidence is unavailable
- `NATIVE-003` when ZIP alignment evidence is unavailable
- `SIGNING-002` when the APK signing block cannot be validated
- `SIGNING-003` when cryptographic verification cannot be completed or no supported v2/v3 scheme can be verified

Confirmed release-breaking conditions remain blockers.

### AUDIT-006 — SIGNING-001 semantics

Status: **IMPLEMENTED**

`SIGNING-001` now describes legacy/v1 `META-INF` signature material.

A modern APK with v2/v3 signing-block evidence and no `META-INF` files no longer receives an unsigned-artifact warning from `SIGNING-001`.

An artifact with neither legacy signature material nor modern APK signing evidence still receives an actionable warning.

### AUDIT-007 — Regression coverage

Status: **IMPLEMENTED**

Focused coverage now asserts:
- Play Console external requirements are `ManualReview`;
- Play native evidence gaps are `ManualReview`;
- ELF evidence gaps are `ManualReview`;
- AAB ZIP alignment evidence gaps are `ManualReview`;
- unreadable APK signing blocks are `ManualReview`;
- unavailable APK cryptographic verification is `ManualReview`;
- valid v2/v3 fixtures do not warn through `SIGNING-001`;
- unsigned/minimal artifacts retain the signing warning.

Confirmed ZIP/ELF alignment failures and signing failures remain tested as their existing warning/blocker conditions.

### AUDIT-008 — Rule registry alignment

Status: **IMPLEMENTED**

The rule registry now documents:
- explicit severity semantics;
- `MANUAL-REVIEW` for external/artifact-proof boundaries;
- the updated `SIGNING-001` legacy/v1 interpretation;
- updated M0.4/M0.5 wording consistent with the runtime model.

## CI trigger audit

The workflow was also reviewed before Actions:

- push validation includes `m06-block2-signing-evidence`;
- pull-request validation includes the stacked M0.6 base branches;
- the duplicate `m05-crypto-signatures` push entry inherited from the previous workflow edit was removed.

## Scope check

No changes were introduced for:
- new cryptographic algorithms;
- proof-of-rotation completion;
- v3.1/v3.2 cryptographic verification;
- AAB cryptographic verification;
- new Play policy requirements;
- new artifact inspection capabilities;
- broad report/CLI redesign.

## Final pre-Actions conclusion

No unresolved implementation or audit finding remains inside the defined M0.6 Block 2 scope.

The branch is ready for GitHub Actions validation.

No merge is performed as part of this audit.
