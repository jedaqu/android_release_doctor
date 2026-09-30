# M0.6 Block 4 — Pre-change audit

Date: 2026-09-30
Validated baseline: 238c0403378c09f1175dd89f0cfd5edbfcf53eaa
Base branch: m06-block3-verification-hardening
Working branch: m06-block4-signer-error-isolation

## Audit method

This audit follows the project discipline:
ledger review → audit → scoped changes → second audit → Actions → follow-up → individual correction → new validation → checkpoint

The current docs/ERRORS-AND-FIXES.md ledger was reviewed before this audit. All prior entries ERR-001 through ERR-015 are resolved; no historical entry is rewritten.

Reviewed:
- crates/doctor-core/src/signature_verify.rs
- .github/workflows/rust.yml
- README.md
- docs/CHECKPOINT-M0.6-BLOCK3.md
- current v2/v3 verification tests and targeted-signer regression coverage
- M0.6 Block 3 audit and second-audit records

The Block 4 audit is intentionally limited to signer-level verification isolation/evidence retention and the validation/documentation plumbing needed for the new stacked branch. It does not expand cryptographic algorithm coverage.

## Findings

### AUDIT-014 — A signer-local verification failure aborts the entire multi-signer verification pass

The current v2 and v3 verification loops can abort the whole scheme on the first signer-local error because signer verification uses ? directly.

In v2, the signer loop currently performs:

    results.push(verify_v2_signer(file, block, signer)?);

In v3, the per-signer verification path similarly propagates errors directly from operations such as signature selection, certificate/public-key binding, cryptographic signature verification, content-digest verification, and certificate fingerprint extraction.

This means that, in a multi-signer artifact, a failure in one signer can prevent already-valid or later signers from being independently represented in the merged evidence.

This is especially important for the v3 targeted-signer model implemented in Block 3: multiple signer entries may carry different SDK ranges, so the verifier should retain the evidence for each signer it inspected rather than collapsing the whole block at the first signer-local error.

Required change:
- contain verification errors at the individual signer boundary;
- append a signer-level Invalid/Unsupported result for the failing signer;
- continue evaluating remaining signers;
- merge all signer results using the existing state aggregation semantics;
- do not turn a cryptographic failure into a pass or manual-review state.

The overall state semantics remain unchanged:
- any Invalid result keeps the merged scheme Invalid;
- otherwise any Unsupported result keeps it Unsupported;
- otherwise the merged scheme is Verified.

### AUDIT-015 — v3 signer-local failure can lose known SDK-range evidence

For v3, the outer signer structure contains minSDK/maxSDK before the cryptographic verification operations that may fail.

When one of those later operations fails, the current direct error propagation discards the already-parsed signer range because the error is converted only at the top-level through error_to_scheme_info.

That conversion produces empty sdk_ranges and signer_count: 0, which is inconsistent with the Block 3 evidence model for a signer that was actually parsed and inspected.

Required change:
- when the v3 outer signer range has been successfully parsed, retain (minSDK, maxSDK) in the signer-level result even if later verification fails;
- retain signer count as one for that signer-level result;
- preserve an empty SDK range only when the signer cannot be parsed far enough to establish the range;
- retain certificate evidence when it was successfully obtained before a later failure.

### AUDIT-016 — M0.6 Block 4 branch is absent from PR CI trigger coverage

The workflow currently includes the Block 3 branch in the push trigger, but the new Block 4 branch is not yet present in either the push list or the stacked pull-request target list.

This would break the project's established stacked-validation discipline for the new block.

Required change:
- add m06-block4-signer-error-isolation to the push trigger;
- add it to the PR trigger target list for the active stacked M0.6 path;
- do not redesign the CI job graph.

### AUDIT-017 — Top-level repository status is stale after the Block 3 checkpoint

README.md still says M0.6 Block 3 in progress.

The Block 3 checkpoint is now complete and validated, and the current work is M0.6 Block 4.

Required change:
- update only the top-level status text to indicate M0.6 Block 4 in progress;
- preserve historical checkpoint and audit records unchanged.

## Scope for M0.6 Block 4

Only these findings are in scope:
1. isolate v2/v3 signer-level verification failures;
2. retain known v3 SDK-range evidence when a signer fails after its range is parsed;
3. add focused regression tests for multi-signer failure isolation/evidence retention;
4. add the Block 4 branch to the existing CI push/PR trigger coverage;
5. update the top-level repository status.

## Out of scope

- new cryptographic algorithms or key-size support;
- replacement of the ring verification backend;
- proof-of-rotation completion;
- complete v3.1 or v3.2 verification;
- AAB cryptographic verification;
- new Google Play policy rules;
- broad CLI/report redesign;
- unrelated refactoring;
- merge of PRs.

## Acceptance criteria

1. A failing signer does not prevent later signers from being independently verified and represented in the merged result.
2. Existing single-signer v2/v3 fixtures remain unchanged and verified.
3. A mixed multi-signer result can represent both a valid signer and a failing signer.
4. A v3 signer failure occurring after outer minSDK/maxSDK parsing retains that SDK range in the signer-level evidence.
5. Existing Invalid/Unsupported/Verified aggregation semantics remain unchanged.
6. CI push and pull-request triggers cover the Block 4 stacked branch.
7. README status matches M0.6 Block 4 in progress.
8. Focused regression tests cover the changed signer-level behavior.
9. No merge is performed as part of this block.