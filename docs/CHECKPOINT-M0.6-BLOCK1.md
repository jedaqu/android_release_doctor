# Checkpoint M0.6 Block 1 — Audit-state and certificate-chain hardening

Date: 2026-09-30
Branch: `m06-block1-state-hardening`
Validated implementation baseline: `6d1fd773185936f29c1baa7a986b8744ff39d4a4`
Base: `2e76c460d8e1ed190e7594e12e9bb9f98f0c8b42`
PR: #8

## Status

M0.6 Block 1 is a validated implementation baseline.

## Scope

- First-class `MANUAL-REVIEW` audit state.
- `SIGNING-003` preserves the distinction between cryptographic failure and incomplete/unsupported verification.
- v2/v3 signer certificate sequences accept additional chain certificates while retaining the first certificate for signer binding/fingerprint evidence.
- Focused regression coverage for both changes.
- CI trigger coverage for the stacked M0.6 development path.

## Validation

Pre-change audit:
- `docs/AUDIT-M0.6-BLOCK1.md`

Second audit:
- `docs/SECOND-AUDIT-M0.6-BLOCK1.md`

Actions:
- Run #150 (`36735470654`) exposed one formatting-only failure.
- The failure was followed to the exact rustfmt diff.
- No test/build failure was bypassed.
- The individual formatting defect was corrected.
- Run #151 (`36735612710`) passed Build, Test, Format and Clippy.

## Boundary

This checkpoint does not claim:
- complete proof-of-rotation validation;
- complete v3.1 cryptographic verification;
- complete v3.2 hybrid/PQC verification;
- AAB cryptographic signature verification;
- new Play policy coverage.

The existing M0.5 cryptographic capability boundaries remain in force.

## Recovery

Use this checkpoint's validated implementation baseline as the starting point for the next M0.6 block.

Do not begin the next block from an unvalidated intermediate commit.

No merge is performed as part of this checkpoint.
