# M0.6 Block 4 — CI trigger follow-up audit

Date: 2026-09-30
Scope: correction of the pull-request trigger target identified after the second audit.

## Finding

The workflow field pull_request.branches filters the pull request's target/base branch, not the source branch containing the Block 4 changes.

For the project's stacked branch sequence, a future M0.6 Block 4 PR should target the validated M0.6 Block 3 branch. Therefore the required PR trigger entry is:

m06-block3-verification-hardening

The Block 4 source branch belongs in push.branches:

m06-block4-signer-error-isolation

## Correction boundary

Only .github/workflows/rust.yml is corrected.

- push.branches: Block 4 remains present exactly once;
- pull_request.branches: Block 3 is added as the active stacked base;
- previously validated Block 1 and Block 2 PR targets remain unchanged;
- no workflow job graph changes are made.

## Validation

A new complete Build/Test/Format/Clippy Actions run is required after this correction. No checkpoint is created before the correction is validated.