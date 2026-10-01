# SECOND-AUDIT M0.7 Block 4 — Integration and completeness

**Date:** 2026-10-01  
**Branch:** `m07-block4-integration-completeness`  
**Base checkpoint:** `834e86503311f346e7d5b764286687be83aa0b5f`  
**Scope:** Re-audit of the Block 4 corrections before Actions.

## 1. Audit baseline

The second audit reviewed:

- `docs/AUDIT-M0.7-BLOCK4.md`;
- `docs/M0.7-DEFINITION.md`;
- `docs/SECOND-AUDIT-M0.7-DEFINITION.md`;
- the complete incremental ledger through ERR-070;
- the M0.7 Block 3 checkpoint and final CI validation;
- the Block 4 production diff against the Block 3 checkpoint;
- the Block 4 workflow and the validated Block 3 base workflow;
- the consolidated README capability boundary.

The Block 4 production correction is limited to top-level APK audit integration. No new cryptographic algorithm, v3.2/PQC implementation, AAB signing verification, or CLI redesign was introduced.

## 2. ERR-069 re-audit — SIGNING-003 integration

Block 3 produces:

- `ApkSignatureVerification.v31`;
- `v31_present`;
- a distinct v3.1 `CryptoSchemeInfo` state.

Block 4 now includes v2, v3 and v3.1 in the `SIGNING-003` evaluated scheme set.

The finding path now behaves as follows:

- Verified v3.1 contributes to a PASS;
- Invalid v3.1 contributes to a BLOCKER;
- Unsupported v3.1 contributes to MANUAL-REVIEW;
- an unexpected present-but-unverified v3.1 block remains MANUAL-REVIEW;
- v3.2 remains an explicit MANUAL-REVIEW boundary.

The stale Block-3 wording is removed from the v3.1 path.

**AUDIT-M0.7-B4-001 re-check: PASS.**

## 3. SIGNING-003 regression coverage

Focused tests were added to `crates/doctor-core/src/lib.rs` for:

1. verified v3.1 accepted by `SIGNING-003`;
2. invalid v3.1 remaining a blocker;
3. unsupported v3.1 remaining manual review;
4. v3.2 presence remaining manual review.

The test fixture is intentionally at the audit-result boundary: it verifies integration of the already cryptographically verified evidence rather than duplicating APK cryptographic fixture tests.

**AUDIT-M0.7-B4-002 re-check: PASS.**

## 4. Cryptographic coverage reconciliation

The implementation and documentation now agree on the M0.7 supported algorithm boundary:

- 0x0101 — RSA-PSS/SHA-256;
- 0x0102 — RSA-PSS/SHA-512;
- 0x0103 — RSA PKCS#1 v1.5/SHA-256;
- 0x0104 — RSA PKCS#1 v1.5/SHA-512;
- 0x0201 — ECDSA/SHA-256 for P-256/P-384;
- 0x0202 — ECDSA/SHA-512 for P-384 only.

Explicit Unsupported boundaries remain:

- 0x0301 DSA/SHA-256;
- RSA key sizes outside the current 2048–8192-bit verifier range;
- ECDSA/SHA-512 curves other than P-384.

The broader Android algorithm/key matrix remains outside the current implementation boundary where the project has not established real cryptographic support.

No unsupported case is promoted to Verified merely because the algorithm ID can be parsed.

**AUDIT-M0.7-B4-005 re-check: PASS.**

## 5. v3.1 semantic boundary reconciliation

The Block 3 implementation remains responsible for:

- required v3 base-block presence;
- v3 stripping-protection rotation-min-SDK;
- v3.1 rotation target range;
- targeted-range overlap/gap rules;
- development-era SDK-32 compatibility already established by the authoritative test corpus;
- signer-count compatibility;
- verified lineage bridge/prefix consistency.

Block 4 does not duplicate or broaden those semantics.

The v3.1 development-release attribute remains treated as a v3.1-specific control for the cross-block range boundary. The milestone does not claim runtime Android trust simulation.

**Milestone semantic boundary: PASS.**

## 6. CI stacked-branch reconciliation

The Block 4 workflow now contains:

- push target: `m07-block4-integration-completeness`;
- pull-request target: `m07-block3-v31-verification`.

The validated Block 3 base workflow now also contains the Block 4 PR target, preserving the established stacked-branch event model.

The job graph remains exactly:

**Build → Test → Format → Clippy**

No CI architecture redesign was introduced.

**AUDIT-M0.7-B4-003 re-check: PASS pending Actions execution.**

## 7. Documentation reconciliation

README now contains a consolidated M0.7 milestone section covering:

- Block 1;
- Block 2;
- Block 3;
- Block 4 integration role;
- supported cryptographic algorithm IDs;
- explicit Unsupported boundaries;
- explicit v3.2/PQC and AAB boundaries.

The final M0.7 milestone status remains pending until the final Block 4 checkpoint Actions run succeeds.

**AUDIT-M0.7-B4-004 re-check: PASS pending final checkpoint.**

## 8. Scope integrity

The Block 4 diff contains:

- one top-level audit integration correction;
- focused regression tests;
- workflow trigger coverage;
- README capability consolidation;
- audit/ledger documentation.

No v3.2/PQC, AAB signing verification, Gradle execution, Play policy automation, HTML/SARIF, permission-risk classification, broad CLI redesign, or unrelated refactor was introduced.

**Scope integrity: PASS.**

## 9. Second-audit conclusion

All Block 4 findings have a bounded correction.

- ERR-069 correction: PASS.
- ERR-070 correction: PASS pending Actions.
- M0.7 capability matrix: PASS.
- v3.1 semantic boundary: PASS.
- README synchronization: PASS pending final checkpoint status update.
- stacked CI configuration: PASS pending Actions.
- no unresolved production-code finding remains.

**SECOND AUDIT: PASS TO PROCEED TO ACTIONS**

The remaining acceptance gate is empirical CI validation of the exact Block 4 branch and its stacked pull-request path.


## 10. Re-audit after ERR-071/ERR-072 correction

The exact Block 4 head was re-reviewed after the correction commit and before Actions.

### Structural check

The SIGNING-003 nested branches now have the intended structure:

- verified/invalid/unsupported v3.1 evidence is evaluated through the unified scheme list;
- the manual-review condition owns the explicit else branch for the verified-pass case;
- the extra outer else that caused Actions run 36868916143 to fail is no longer present.

The focused regression tests remain present:

- verified_v31_is_accepted_by_signing_003;
- invalid_v31_remains_a_signing_003_blocker;
- unsupported_v31_remains_manual_review;
- v32_presence_remains_manual_review_boundary.

### Ledger reconciliation

ERR-071 records the first observed branch-structure failure from Actions run 36868550017.
ERR-072 records the residual failure from Actions run 36868916143 and the precise correction now applied.
ERR-073 records the terminology clarification so the remaining conditional manual-review text is not confused with the removed unconditional Block 3 behavior.

### Final pre-Actions audit result

- production scope: PASS;
- v3.1 integration semantics: PASS;
- focused regression coverage: PASS;
- cryptographic capability boundary: PASS;
- stacked CI triggers: PASS;
- documentation/ledger reconciliation: PASS;
- no unresolved production-code finding identified by source review.

**SECOND-AUDIT AFTER ERR-072: PASS TO PROCEED TO ACTIONS**

The remaining gate is GitHub Actions validation of the exact branch head.


## 11. Re-audit after ERR-074 correction

The exact production correction for ERR-074 was reviewed before the next Actions execution.

### Structural check

The `SIGNING-003` control flow now has the complete intended nesting:

- APK verification-unavailable remains the outer manual-review path;
- an available verification result builds the unified v2/v3/v3.1 scheme set;
- empty schemes remain manual review;
- any Invalid scheme remains a blocker;
- the remaining branch evaluates Unsupported, present-but-unverified v3.1, or v3.2 as manual review;
- the verified path reaches PASS;
- the surrounding scheme-evaluation branch is explicitly closed before the final unavailable-verification `else`.

The previous unclosed-delimiter condition is therefore structurally corrected without changing the intended finding semantics.

### Scope check

The correction changes no cryptographic algorithm support, v3.1 semantics, v3.2/PQC boundary, AAB behavior, workflow architecture, or CLI behavior. Only the affected `SIGNING-003` branch structure and its formatting were corrected.

### Ledger reconciliation

ERR-074 records Actions run 36869780405 and job 110394269323 as a distinct build failure after the ERR-071/ERR-072 branch repair sequence. It is not merged into the historical ERR-071/ERR-072 entries.

### Pre-Actions result

- production-code structural repair: PASS;
- intended SIGNING-003 semantics: PASS by source review;
- scope integrity: PASS;
- ledger reconciliation: PASS;
- empirical CI validation: PENDING.

**SECOND-AUDIT AFTER ERR-074: PASS TO PROCEED TO ACTIONS**

The exact correction commit `89753fc376e8fd4381d67c5b4c72b0ac5aff6324` was validated by Actions run #428 / `36873605656` with Build, Test, Format, and Clippy all PASS.

**SECOND-AUDIT AFTER ERR-074: PASS**

The next acceptance gate is the final checkpoint commit carrying this ledger/audit closure through GitHub Actions.


## 12. ERR-075 re-audit and final checkpoint readiness

The final stacked-CI audit identified one configuration gap in the validated Block 3 base workflow: the Block 3 branch itself was absent from the pull_request.branches filter.

The correction commit 2d506d98e7892e602f4f2c4a99cbcab0b1d190b9 added the missing target without changing the Build → Test → Format → Clippy job graph.

Validation evidence:

- Actions #430 / 36874487972: push — PASS;
- Actions #431 / 36874495416: pull_request — PASS.

The existing Block 4 head workflow already targets m07-block3-v31-verification, so the head-side configuration remains aligned after the base-side correction.

### Final pre-checkpoint audit result

- production code: PASS;
- ERR-074 correction: PASS;
- ERR-075 correction: PASS;
- v3.1 integration semantics: PASS;
- focused regression coverage: PASS;
- capability boundary: PASS;
- stacked CI configuration: PASS;
- documentation/ledger reconciliation: PASS;
- PR #15 remains open, draft, and unmerged by design.

**SECOND-AUDIT AFTER ERR-075: PASS TO FINAL CHECKPOINT**

The only remaining gate is Actions validation of the formal Block 4 checkpoint commit itself.