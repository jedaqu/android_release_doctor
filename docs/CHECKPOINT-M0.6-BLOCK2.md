# Checkpoint M0.6 Block 2 — Signing evidence semantics

Date: 2026-09-30
Branch: `m06-block2-signing-evidence`
Validated implementation baseline: `5316d157c1cb2f32cfe0e996226cb0012dc7ac96`
Base: `3b50eab248359e1b2b923993c97a2b64e6874158`
PR: #9

## Status

M0.6 Block 2 is a validated implementation baseline.

## Scope

- First-class `MANUAL-REVIEW` semantics propagated to artifact-proof and external-verification boundaries.
- `SIGNING-001` reconciled with modern APK v2/v3 signing evidence.
- Confirmed release-breaking conditions remain blockers.
- Ordinary advisory conditions remain warnings.
- Focused regression coverage added for manual-review and signing-evidence semantics.
- Rule registry severity semantics aligned with runtime behavior.
- Rust CI trigger coverage aligned with the stacked M0.6 branch sequence.

## Validation sequence

### Pre-change audit
`docs/AUDIT-M0.6-BLOCK2.md`

### Second audit
`docs/SECOND-AUDIT-M0.6-BLOCK2.md`

### Actions follow-up

Run #158 (`36737009775`) initially failed in Test.

All 50 unit tests passed. One integration assertion failed because the existing `minimal-release.apk` fixture contains legacy `META-INF` signature files, so `SIGNING-001` correctly returned `PASS` rather than `WARNING`.

Correction:
- changed only that test expectation;
- commit: `9621feffc71ce74d59a0ba88f31f866b73ef56b4`.

Run #159 (`36737113317`) then passed Build and Test but failed Format.

The log identified one rustfmt-only layout difference in the new `SIGNING-001` expression.

Correction:
- changed only the formatting of that expression;
- commit: `5316d157c1cb2f32cfe0e996226cb0012dc7ac96`.

Run #160 (`36737237511`) passed:
- Build
- Test
- Format
- Clippy

No failing production behavior was bypassed.

## Important boundaries

This checkpoint does not claim:
- new cryptographic algorithms;
- complete proof-of-rotation validation;
- complete v3.1 cryptographic verification;
- complete v3.2 hybrid/PQC verification;
- AAB cryptographic signature verification;
- new Play policy coverage.

The M0.5 cryptographic capability boundaries remain unchanged.

## Recovery

Use `5316d157c1cb2f32cfe0e996226cb0012dc7ac96` as the validated M0.6 Block 2 code baseline after the final checkpoint CI pass.

Do not begin the next block from an unvalidated intermediate commit.

No merge is performed as part of this checkpoint.
