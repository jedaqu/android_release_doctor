# M0.13 — Validation Ledger Reconciliation

## VAL-001 — AAB validation expectation corrected

The first M0.13 Action validation attempt assumed `minimal-release.aab` would exit 0. The actual product returned exit code 1 for the fixture, which is a valid blocker outcome under the stable CLI contract.

Correction: the AAB test now expects exit code 1 while separately validating Report v1 and artifact kind AAB.

Status: RESOLVED — test-harness correction, not product defect.

## VAL-002 — Expected nonzero AAB outcome allowed through composite Action

The initial AAB test did not use `continue-on-error`, so the expected product exit code 1 stopped the Action job before its validation step.

Correction: the AAB invocation now uses `continue-on-error: true` and validates the Action output explicitly.

Status: RESOLVED — test-harness correction, not product defect.

## VAL-003 — Expanded Action boundary validation

Added and successfully exercised project, AAB, combined project+Play, non-default Play platform, invalid input, invalid format, invalid platform, and output-path safety cases.

Status: RESOLVED — validated in PR #60 and post-merge CI.
