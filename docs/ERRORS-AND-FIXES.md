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
- **Validation:** Actions run #184 / 36745400537 passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED

---

## ERR-016 — signer-local verification aborted multi-signer evidence

- **Milestone:** M0.6 Block 4 / AUDIT-014
- **Type:** Implementation defect
- **Problem:** v2/v3 signer verification propagated signer-local errors with `?`, so one failing signer could abort the whole multi-signer verification pass.
- **Correction:** Contain signer-level failures, retain each failure as an Invalid/Unsupported signer result, continue with remaining signers, and merge the evidence using existing state semantics.
- **Correction commit:** 135e3df81507ad48d6b03608f282da7154c028af
- **Validation:** Actions run #198 / 36746555546 passed Build, Test, Format, and Clippy with the correction present.
- **Status:** RESOLVED

## ERR-017 — v3 signer failure lost known SDK-range evidence

- **Milestone:** M0.6 Block 4 / AUDIT-015
- **Type:** Evidence-model defect
- **Problem:** A signer failure after outer minSDK/maxSDK parsing was converted at the top level, losing the already-known signer range and signer count.
- **Correction:** Preserve signer count, SDK range, algorithms, and certificate fingerprint whenever those values were successfully parsed before the failure.
- **Correction commit:** 135e3df81507ad48d6b03608f282da7154c028af
- **Validation:** Actions run #198 / 36746555546 passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED

## ERR-018 — M0.6 Block 4 branch missing from CI trigger coverage

- **Milestone:** M0.6 Block 4 / AUDIT-016
- **Type:** CI configuration
- **Problem:** The new Block 4 branch was not yet covered by the workflow's required stacked validation path.
- **Correction:** Add the Block 4 branch to push coverage and add the active Block 3 stacked base to pull-request target coverage.
- **Validation:** Final workflow configuration inspected; Actions run #198 / 36746555546 passed all four gates.
- **Status:** RESOLVED

## ERR-019 — Repository status stale after Block 3 checkpoint

- **Milestone:** M0.6 Block 4 / AUDIT-017
- **Type:** Documentation
- **Problem:** README still described M0.6 Block 3 as in progress after its checkpoint.
- **Correction:** Update the top-level status to M0.6 Block 4 in progress.
- **Correction commit:** 0b43ac43a74b1bee8df6c9d86dd68036bac9a1ec
- **Status:** RESOLVED

## ERR-020 — Block 4 CI trigger edit duplicated push entry and omitted PR entry

- **Milestone:** M0.6 Block 4 / second audit
- **Type:** CI configuration correction defect
- **Problem:** The scoped workflow edit inserted `m06-block4-signer-error-isolation` twice under push.branches and did not add it to pull_request.branches.
- **Correction:** Keep the Block 4 branch exactly once in push.branches and use the correct stacked Block 3 base in pull_request.branches.
- **Correction commit:** f71c16ae43cd9f57748ad494c10ada9f4a7f6dd4
- **Validation:** Final workflow configuration inspected; Actions run #198 / 36746555546 passed all four gates.
- **Status:** RESOLVED
## ERR-021 — rustfmt layout after Block 4 test addition

- **Milestone:** M0.6 Block 4
- **Type:** CI formatting failure
- **Actions run:** #190 / 36746316865 and #191 / 36746324902
- **Problem:** `cargo fmt --all -- --check` required a multi-line layout for the final `assert!(result.detail.contains(...))` in the new regression test.
- **Correction:** Apply only the rustfmt-indicated line wrap; no test logic or production behavior changes.
- **Correction commit:** af70dcb1175c092de319eee9c601ff1830b42a1e
- **Validation:** Actions run #196 / 36746459364 and #198 / 36746555546 passed Format.
- **Status:** RESOLVED
## ERR-022 — PR trigger audit targeted the source branch instead of the stacked base branch

- **Milestone:** M0.6 Block 4 / CI follow-up
- **Type:** Audit/CI-scope defect
- **Problem:** The Block 4 CI audit treated pull_request.branches as if it filtered the PR source branch. For the stacked workflow, that filter selects the PR target/base branch. A Block 4 PR stacked on Block 3 therefore requires m06-block3-verification-hardening in the PR trigger list.
- **Correction:** Keep m06-block4-signer-error-isolation in push.branches, and use m06-block3-verification-hardening as the Block 4 pull-request target branch. Preserve the previously validated Block 1 and Block 2 PR targets.
- **Correction commit:** f71c16ae43cd9f57748ad494c10ada9f4a7f6dd4
- **Validation:** Final workflow configuration inspected; Actions run #198 / 36746555546 passed all four gates. A Block 4 pull-request event itself will be exercised when the stacked PR is opened.
- **Status:** RESOLVED — PR-event validation pending PR creation
## ERR-023 — Block 4 PR validation is gated by the base branch workflow configuration

- **Milestone:** M0.6 Block 4 / CI follow-up
- **Type:** CI configuration / validation gap
- **Problem:** Draft PR #11 targets m06-block3-verification-hardening, but the workflow on that base branch did not include m06-block3-verification-hardening in its pull_request.branches filter. The Block 4 branch workflow alone could not establish PR-event coverage for a PR whose base is Block 3.
- **Correction:** Updated the validated Block 3 base branch workflow so its pull_request.branches includes m06-block3-verification-hardening, without changing the Block 3 job graph or production code.
- **Correction commit:** c2fb2ae978a3eb924b954cb4266fdbccebeffa14
- **Validation:** Block 3 base branch Actions run #205 / 36747047095 passed Build, Test, Format, and Clippy. Draft PR #11 subsequently received pull-request validation in runs #202 / 36746979474 and #204 / 36747038783, both successful.
- **Status:** RESOLVED
## Current validation state

M0.6 Block 4 implementation and the current CI correction cycle have passed the complete validation gate.

Actions run #198 / 36746555546:
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

Final branch workflow configuration places m06-block4-signer-error-isolation in push coverage and m06-block3-verification-hardening in the pull-request target coverage for the stacked path.

A Block 4 checkpoint may now be created. No merge is performed as part of this checkpoint.

## Maintenance rule

Every future Actions failure or audit-discovered defect must append a new ERR-NNN entry rather than editing an older entry. Resolutions should reference the correction commit and the validating Actions run whenever available.
