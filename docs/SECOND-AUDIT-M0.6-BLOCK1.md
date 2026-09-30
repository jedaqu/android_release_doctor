# M0.6 Block 1 — Second audit

Date: 2026-09-30
Validated implementation candidate: `6d1fd773185936f29c1baa7a986b8744ff39d4a4`
Base: `2e76c460d8e1ed190e7594e12e9bb9f98f0c8b42`
PR: #8

## Audit result

The four pre-change findings were rechecked after implementation.

### AUDIT-001 — Manual-review state

Status: **IMPLEMENTED**

- `Severity::ManualReview` is now a first-class finding state.
- `SIGNING-003` uses `ManualReview` for unsupported/incomplete cryptographic verification.
- Manual-review findings have a dedicated rendered label.
- Manual-review findings are counted separately from warnings and blockers.
- CLI blocker exit behavior remains unchanged because only `Severity::Blocker` drives the non-zero release-blocking result.

### AUDIT-002 — v2/v3 certificate sequences

Status: **IMPLEMENTED**

- v2 and v3 signed-data parsing still treats the first certificate as the signer certificate.
- Additional length-prefixed certificates are now consumed as certificate-chain entries.
- Extra certificates no longer cause a trailing-bytes failure.
- Focused tests cover both v2 and v3 parsing.

### AUDIT-003 — CI trigger coverage

Status: **IMPLEMENTED**

- The Rust CI push trigger includes `m06-block1-state-hardening`.
- The pull-request trigger includes `m05-crypto-signatures`, allowing the stacked M0.6 PR to receive automatic validation.

### AUDIT-004 — Regression coverage

Status: **IMPLEMENTED**

Focused tests were added for:
- first-class manual-review reporting;
- v2 additional certificate-chain entries;
- v3 additional certificate-chain entries.

Existing M0.1-M0.5 tests remain intact.

## Scope check

No changes were introduced for:
- proof-of-rotation completion;
- v3.1 cryptographic verification;
- v3.2 hybrid/PQC verification;
- AAB cryptographic signing verification;
- new Play policy rules;
- broad architecture refactoring.

## Pre-CI review result

The implementation matches the audited scope and is ready for Actions validation.

The first Actions execution exposed exactly one formatting defect in the newly added manual-review test: an extra blank line. Build and all tests had already passed.

That formatting defect was corrected individually in commit `6d1fd773185936f29c1baa7a986b8744ff39d4a4`.

## Final Actions result

Run #151 (`36735612710`) completed successfully.

Job `test`:
- Build: success
- Test: success
- Format: success
- Clippy: success

The preceding run #150 (`36735470654`) failed only at Format; no test or build failure was bypassed. The failure was followed to its exact formatting diff and corrected before the new execution.

## Conclusion

M0.6 Block 1 satisfies its defined acceptance criteria and is a validated implementation candidate pending checkpoint creation.

No merge is performed as part of this block.
