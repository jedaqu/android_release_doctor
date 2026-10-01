# SECOND-AUDIT M0.7 Block 2 — Proof-of-rotation semantic evidence

**Date:** 2026-09-30  
**Branch:** `m07-block2-rotation-semantics`  
**Scope:** Re-audit of the completed Block 2 implementation against `docs/AUDIT-M0.7-BLOCK2.md`.

## 1. Audit baseline

The second audit re-reviewed:

- the incremental ledger through ERR-050;
- the Block 2 pre-change audit;
- the M0.7 definition;
- the implementation diff against the validated Block 1 base;
- the proof-of-rotation evidence model;
- focused regression coverage;
- scope boundaries.

The implementation remains restricted to proof-of-rotation capability evidence.

## 2. Implementation verification

The implementation now:

1. defines the five AOSP capability bits explicitly;
2. preserves the raw flags value for every lineage node;
3. exposes the known-bit mask;
4. exposes unknown/reserved bits explicitly;
5. decodes each known capability independently;
6. preserves capability evidence through lineage-local failures after flags have been parsed;
7. leaves all existing certificate, signature-algorithm, duplicate-certificate, final-certificate, and signer-isolation checks intact;
8. does not change `CryptoVerificationState` semantics;
9. does not introduce v3.1 behavior;
10. does not introduce a cryptographic backend or algorithm change.

The Android/AOSP sources define the capability bits as independent capabilities associated with past signing certificates. The implementation therefore does not reject unusual combinations merely because multiple independent bits are set.

## 3. Regression verification

Focused tests were added for:

- all five known capability bits simultaneously;
- a representative known capability combined with an unknown/reserved bit;
- zero flags versus unknown-only flags;
- preservation of capability evidence when lineage signature verification fails;
- continued validation of the existing valid two-level lineage;
- existing malformed-lineage and final-certificate mismatch behavior.

The tests derive deterministic variants from the existing real proof-of-rotation fixture, so the cryptographic lineage material remains unchanged while the capability evidence is varied.

## 4. Second-audit finding and correction

### AUDIT-M0.7-B2-003 — fixture mutation helper borrow conflict

The initial implementation of the new deterministic fixture helper retained a borrowed node slice and then attempted to mutate the backing fixture buffer.

This was identified during the second audit before CI execution.

**Correction:** copy the selected node to an owned buffer before mutating the original fixture. This is test-only and does not alter production verification behavior.

**Correction commit:** `766e190e2d90a677c4844c4fb65ce8a925a54128`

The ledger records this as ERR-050.

### AUDIT-M0.7-B2-004 — missed struct field in duplicate-attribute error path

The first Actions execution exposed one remaining compile-time initializer missed by the implementation audit: the duplicate proof-of-rotation attribute path did not initialize the new `capabilities` field.

**Correction:** add an empty capability vector to that error evidence object only.

**Actions run:** #324 / `36799430192`

**Correction commit:** `751fdb4a4917316b276473478514d55374654b5b`

The finding is recorded as ERR-053. No production verification behavior outside the intended evidence model was changed.

### AUDIT-M0.7-B2-005 — fixture flags mutation used the wrong node offset

Actions run #326 compiled successfully but four new capability-evidence tests failed because the fixture helper wrote the requested flags four bytes too far into the node, overwriting `signature_algorithm` instead of `flags`.

**Correction:** change the fixture offset to `4 + signed_data.len()` so the mutation targets the flags field exactly.

**Actions run:** #326 / `36799721854`

**Correction commit:** `fd6dfa1e275c75a4bc5d28fa1c42b6e969119497`

The finding is recorded as ERR-054. No production verification behavior changed.

### AUDIT-M0.7-B2-006 — rustfmt-only CI correction

Actions run #333 passed Build and Test, then failed only at Format with three deterministic layout differences in the new helper/tests.

**Correction:** apply exactly the three rustfmt-indicated layout changes and no behavioral changes.

**Actions run:** #333 / `36799797451`

**Correction commit:** `f0bcd2652dab80507cf62b5d51ebaaba848f2539`

The finding is recorded as ERR-055.

## 5. Scope review

The diff against `m07-block1-crypto-coverage` contains:

- the Block 2 pre-change audit;
- the Block 2 ledger entry/correction;
- the targeted proof-of-rotation capability model and tests.

No unrelated production subsystem, CI graph, cryptographic algorithm support, v3.1 logic, or AAB behavior was introduced.

## 6. Acceptance matrix

| Requirement | Result |
|---|---|
| Known capability bits represented | PASS |
| Raw flags retained | PASS |
| Unknown/reserved bits retained | PASS |
| Independent bit combinations accepted | PASS |
| Malformed lineage remains Invalid | PASS by preserved regression |
| Invalid lineage signature remains Invalid | PASS by preserved regression |
| Final certificate mismatch remains Invalid | PASS by preserved regression |
| Capability evidence retained after lineage-local failure | PASS |
| Existing proof-of-rotation evidence model preserved | PASS |
| v3.1 excluded | PASS |
| M0.7 Block 1 behavior untouched | PASS |
| No unrelated production change | PASS |

## 7. Second-audit conclusion

## 8. CI trigger correction before validation

The inherited workflow initially did not include the Block 2 source branch in push coverage or the Block 1 branch as the stacked pull-request target. This was corrected without changing the job graph.

**Correction commit:** `c93f52f01af86d7c6498fdf86e7a7a6ab9fd9cc6`

The ledger records this as ERR-051. The workflow now covers:

- push: `m07-block2-rotation-semantics`;
- pull request target: `m07-block1-crypto-coverage`.

The CI correction is limited to the active M0.7 stack.

## 9. Base-workflow PR-event correction

The stacked PR also required the validated Block 1 base workflow to target `m07-block1-crypto-coverage` in its `pull_request.branches` filter. This was corrected on the base branch in commit `6395c5fbf65d0a40060998646cb62fe07bf9f6fd` and recorded as ERR-052.

## 11. Final second-audit conclusion

**SECOND AUDIT: PASS TO CI**

The scoped Block 2 implementation satisfies the declared semantic-evidence boundary after the identified test-helper correction.

The next movement is the established Actions validation gate:

**Build → Test → Format → Clippy**

After Actions, any failure will be handled individually, appended to the ledger, corrected without unrelated changes, and revalidated in a new execution.
