# M0.6 Block 4 — Second audit before Actions

Date: 2026-09-30
Baseline reviewed: 238c0403378c09f1175dd89f0cfd5edbfcf53eaa
Implementation commits reviewed:
- 135e3df81507ad48d6b03608f282da7154c028af — signer error isolation
- d6ae40578e2edbc54d2566a9bb774a99fc181cc1 — regression tests
- 927ed9b809bb491c688869ea1024ce82c9f55e78 — CI trigger update
- 0b43ac43a74b1bee8df6c9d86dd68036bac9a1ec — README status update

## Ledger-first review

Before this second audit, the current incremental ledger was reviewed. The historical entries remain unchanged.

## Audit result

### AUDIT-014 / AUDIT-015 implementation review

PASS. v2 and v3 signer-local verification failures are converted into signer-level results instead of aborting the entire verification pass. The v3 path retains the already-parsed outer SDK range, algorithms, and certificate fingerprint when those values are available before the failure.

The merged Invalid/Unsupported/Verified state aggregation semantics remain unchanged.

### Regression coverage review

PASS. Focused unit coverage verifies that signer-level error evidence retains signer count, algorithms, certificate fingerprint, and v3 SDK range, and that a mixed verified/failed multi-signer result remains Invalid while preserving evidence for both signers.

### AUDIT-016 implementation review

FAIL — the CI edit is incorrect and must be corrected before Actions.

The current .github/workflows/rust.yml contains m06-block4-signer-error-isolation twice in the push trigger and does not yet contain the Block 4 branch in the pull_request.branches list.

This is a scoped CI-edit defect only. No production verification logic is implicated.

### AUDIT-017 implementation review

PASS. README now reports M0.6 Block 4 in progress and identifies the signer verification/evidence-hardening focus.

## Required correction before Actions

Correct only .github/workflows/rust.yml:
- keep m06-block4-signer-error-isolation exactly once in push.branches;
- add m06-block4-signer-error-isolation exactly once in pull_request.branches;
- preserve all existing validated branch entries;
- do not redesign the workflow.

No checkpoint is allowed until the correction is validated by complete Build/Test/Format/Clippy Actions.

## Scope remains bounded

No changes are required to the production signer-isolation implementation before the CI correction. No new cryptographic algorithms, AAB verification, proof-of-rotation, v3.1/v3.2 completion, Play-policy expansion, or merge are in scope.