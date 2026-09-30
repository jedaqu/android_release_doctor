# Incremental Error & Fix Log

This file is the chronological engineering ledger for failures, errors, their root causes, and the corrective action taken during the repository's audit/validation process.

## Rules

- Append new entries; do not rewrite or renumber historical entries.
- Record the first observed failure, root cause, correction, validation result, and relevant commit/run.
- Distinguish implementation defects from CI-only formatting/lint failures.
- A failure is considered resolved only after the corresponding validation step passes.
- This log complements the detailed audit and checkpoint documents; it does not replace them.

---

## ERR-001 — AXML parser attribute_size advancement

- **Milestone:** M0.1
- **Type:** Implementation defect
- **Problem:** The AXML parser read a fixed attribute structure but advanced according to value_size, which could misalign subsequent parsing.
- **Impact:** Malformed or unusual AXML could cause incorrect parsing boundaries.
- **Correction:** Align attribute parsing and cursor advancement with the actual AXML attribute structure and validate structural sizes.
- **Validation:** Covered by the M0.1 parser hardening/regression work.
- **Status:** RESOLVED

## ERR-002 — android:debuggable absent treated as unknown

- **Milestone:** M0.1
- **Type:** Semantic defect
- **Problem:** Absence of android:debuggable was treated as an unknown/warning condition instead of distinguishing explicit declaration from Android's effective default.
- **Correction:** Model explicit debuggable state separately from the effective Android value; absence therefore does not imply an unknown runtime state.
- **Status:** RESOLVED

## ERR-003 — Dead MANIFEST-002 evaluation path

- **Milestone:** M0.1
- **Type:** Control-flow defect
- **Problem:** audit_path() returned AuditError::ManifestParse before the evaluation path could receive None, making MANIFEST-002 unreachable.
- **Correction:** Rework the error/evaluation flow so the intended rule can actually be evaluated.
- **Status:** RESOLVED

## ERR-004 — Insufficient negative AXML coverage

- **Milestone:** M0.1
- **Type:** Test coverage
- **Problem:** Tests concentrated on successful extraction and did not sufficiently exercise malformed/truncated AXML.
- **Correction:** Add negative parser cases and structural validation around headers, string pools, typed values, bounds, and terminators.
- **Status:** RESOLVED

## ERR-005 — Manual-review boundaries represented as warnings

- **Milestone:** M0.6 Block 2 / AUDIT-005
- **Type:** Semantic defect
- **Problem:** Conditions requiring external/artifact-proof verification were represented as ordinary Warning values even though they were not confirmed artifact failures.
- **Examples:** PLAY-002/003/004, PLAY-005 evidence gaps, NATIVE-002/003 evidence gaps, SIGNING-002 evidence gaps, SIGNING-003 verification-unavailable cases.
- **Correction:** Introduce and propagate first-class Severity::ManualReview for those boundaries while preserving blockers and ordinary warnings.
- **Validation:** Focused regression tests and Actions Build/Test/Format/Clippy passed.
- **Status:** RESOLVED

## ERR-006 — SIGNING-001 misleading for valid v2/v3 APKs

- **Milestone:** M0.6 Block 2 / AUDIT-006
- **Type:** Semantic defect
- **Problem:** Absence of legacy META-INF v1 signature files could produce a warning even when the APK had valid modern v2/v3 signing evidence.
- **Correction:** Define SIGNING-001 specifically around legacy/v1 signature material; modern v2/v3 evidence prevents the misleading unsigned-artifact warning.
- **Validation:** Focused tests cover modern signed and unsigned/minimal artifacts.
- **Status:** RESOLVED

## ERR-007 — Existing tests encoded obsolete warning semantics

- **Milestone:** M0.6 Block 2 / AUDIT-007
- **Type:** Regression coverage
- **Problem:** Tests expected old Warning semantics for conditions converted to ManualReview.
- **Correction:** Update focused assertions to encode the new severity model and preserve confirmed blockers/advisory warnings.
- **Status:** RESOLVED

## ERR-008 — M0.6 Block 2 CI trigger coverage

- **Milestone:** M0.6 Block 2 / AUDIT-008
- **Type:** CI configuration
- **Problem:** Stacked M0.6 validation paths were not fully represented in workflow triggers.
- **Correction:** Align push/PR trigger branches with the stacked M0.6 validation path without redesigning the job graph.
- **Status:** RESOLVED

## ERR-009 — v3 verification rejected targeted multi-signer configurations

- **Milestone:** M0.6 Block 3 / AUDIT-009
- **Type:** Implementation defect
- **Problem:** v3 verification rejected a block unless exactly one signer was present.
- **Correction:** Verify each declared v3 signer independently, retain its minSDK/maxSDK range, and merge the evidence without claiming a runtime-platform-specific signer selection.
- **Status:** RESOLVED in Block 3 implementation

## ERR-010 — v3 signer SDK-range evidence was discarded

- **Milestone:** M0.6 Block 3 / AUDIT-010
- **Type:** Evidence-model defect
- **Problem:** minSDK/maxSDK values were checked but discarded from CryptoSchemeInfo.
- **Correction:** Add sdk_ranges to the evidence model; keep v2 output unchanged and report conservatively.
- **Status:** RESOLVED in Block 3 implementation

## ERR-011 — M0.6 Block 3 CI trigger coverage

- **Milestone:** M0.6 Block 3 / AUDIT-011
- **Type:** CI configuration
- **Problem:** PR trigger coverage did not fully include the active stacked M0.6 base branches.
- **Correction:** Add the required M0.6 stacked branches to push/PR triggers and remove the duplicate obsolete entry.
- **Status:** RESOLVED in Block 3 implementation

## ERR-012 — Repository status documentation stale

- **Milestone:** M0.6 Block 3 / AUDIT-012
- **Type:** Documentation
- **Problem:** README status still identified M0.5 Block 2 after M0.6 Blocks 1 and 2 had been validated.
- **Correction:** Update the top-level status to M0.6 Block 3 in progress while preserving historical audit/checkpoint records.
- **Status:** RESOLVED in Block 3 implementation

## ERR-013 — Compile error after v3 verification change

- **Milestone:** M0.6 Block 3
- **Type:** Build failure
- **Actions run:** #167 / 36742972007
- **Problem:** The v3 verification return path used merge_scheme_results("v3", results)? where the enclosing function expected a Result.
- **Correction:** Adjust the return expression so the compiler receives the expected Result<CryptoSchemeInfo, ...>.
- **Correction commit:** 1e7f1b6d4f306759558a0489a5acbde4bd3d288d
- **Validation:** Subsequent run Build/Test passed.
- **Status:** RESOLVED

## ERR-014 — rustfmt failure

- **Milestone:** M0.6 Block 3
- **Type:** CI formatting failure
- **Actions run:** #169 / 36743321096
- **Problem:** cargo fmt --all -- --check found one extra blank line before the final closing brace in the test module.
- **Correction:** Remove only that blank line; no logic or behavior changed.
- **Correction commit:** 2962984babeea8f6ab86ee0df9da99bdbef44782
- **Validation:** Subsequent run #173 Format passed.
- **Status:** RESOLVED

## ERR-015 — Clippy needless_question_mark

- **Milestone:** M0.6 Block 3
- **Type:** CI lint failure
- **Actions run:** #173 / 36743785845
- **Problem:** Clippy rejected Ok(merge_scheme_results("v3", results)?) as an unnecessary enclosing Ok plus ?.
- **Required correction:** Replace the expression with merge_scheme_results("v3", results).
- **Correction:** Applied the one-line return-expression correction only; no logic, API, or unrelated formatting changes.
- **Correction commit:** 9126fd8d20fc49d410f640c345da15f3529a9f49
- **Validation:** Awaiting the next complete Build/Test/Format/Clippy Actions run.
- **Status:** CORRECTED — VALIDATION PENDING

---

## Current validation state

ERR-015 has been corrected in commit 9126fd8d20fc49d410f640c345da15f3529a9f49.

The latest completed validation run remains #173 / 36743785845:
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: FAIL due to ERR-015

A new complete Build/Test/Format/Clippy Actions run is required. M0.6 Block 3 is not yet checkpointed.

## Maintenance rule

Every future Actions failure or audit-discovered defect must append a new ERR-NNN entry rather than editing an older entry. Resolutions should reference the correction commit and the validating Actions run whenever available.
