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

**SECOND AUDIT: PASS TO CI**

The scoped Block 2 implementation satisfies the declared semantic-evidence boundary after the identified test-helper correction.

The next movement is the established Actions validation gate:

**Build → Test → Format → Clippy**

After Actions, any failure will be handled individually, appended to the ledger, corrected without unrelated changes, and revalidated in a new execution.
