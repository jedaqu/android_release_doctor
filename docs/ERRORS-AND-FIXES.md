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

## ERR-024 — v3 proof-of-rotation was detected but not verified

- **Milestone:** M0.6 Block 5 / AUDIT-024
- **Type:** Verification boundary defect
- **Problem:** The v3 verifier only detected the proof-of-rotation attribute and treated its presence as Unsupported, without validating the lineage structure, parent-to-child signatures, or final-certificate relationship to the current signer.
- **Correction:** Parse and validate the v3 proof-of-rotation lineage, preserve the existing unsupported boundary for algorithms the verifier cannot safely validate, and prevent invalid lineage evidence from producing a cryptographic pass.
- **Correction commits:** b5089e6daec53c12d4f8beecce22b158b006188d; 20de0254e62920f8d1e9b6c6690e677ef27cabc7
- **Validation:** Actions run #233 / 36751272856 passed Build, Test, Format, and Clippy; focused valid/malformed/invalid-signature/final-certificate-mismatch lineage tests passed.
- **Status:** RESOLVED
## ERR-025 — proof-of-rotation evidence was not represented structurally

- **Milestone:** M0.6 Block 5 / AUDIT-025
- **Type:** Evidence-model defect
- **Problem:** CryptoSchemeInfo exposed only a generic Unsupported state/detail when proof-of-rotation was present and had no structured lineage-verification evidence.
- **Correction:** Add minimal structured proof-of-rotation evidence while preserving signer count, SDK ranges, certificate fingerprints, and existing aggregation semantics.
- **Correction commits:** b5089e6daec53c12d4f8beecce22b158b006188d; a8bbe905faae4a8fb2549625aeec184f8ab8e88b
- **Validation:** Actions run #233 / 36751272856 passed all four gates, including the focused evidence-propagation regression test.
- **Status:** RESOLVED
## ERR-026 — M0.6 Block 5 stacked PR validation is not yet wired for the Block 4 base

- **Milestone:** M0.6 Block 5 / CI follow-up audit
- **Type:** CI configuration / validation gap
- **Problem:** The current workflow did not initially include the Block 5 source branch in push coverage, and the validated Block 4 base workflow did not initially target m06-block4-signer-error-isolation in pull_request coverage for a stacked Block 5 PR.
- **Correction:** Add the Block 5 branch to push coverage and update the validated Block 4 base workflow so m06-block4-signer-error-isolation is covered as a pull-request target.
- **Correction commits:** 097e4c6c4f7dbee67e508b57a58f898ffe3dad5f; 0ec5d0b9ebb644eba0a5ca9c8855a2322bf4c4f9
- **Validation:** Block 4 base workflow Actions run #212 / 36750176041 passed Build, Test, Format, and Clippy. Draft PR #12 stacked on that base triggered pull-request validation run #236 / 36751740474, which passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED
## ERR-027 — Block 5 audit document contained non-repository citation markers

- **Milestone:** M0.6 Block 5 / audit documentation correction
- **Type:** Documentation defect
- **Problem:** The initial Block 5 audit document contained ChatGPT/web UI citation markers instead of stable repository-readable source URLs.
- **Required correction:** Replace those markers with plain AOSP source URLs and keep repository documentation independent of chat UI formatting.
- **Correction commit:** f3886a25e8d8321832d04d575f302c15c4880384
- **Validation:** Re-read of `docs/AUDIT-M0.6-BLOCK5.md` confirmed the citation markers are absent and the AOSP references are represented as plain URLs.
- **Status:** RESOLVED
## ERR-028 — Block 5 proof-of-rotation fixture commit is not reachable from the working branch

- **Milestone:** M0.6 Block 5 / second audit
- **Type:** Test-fixture / Git history defect
- **Problem:** The proof-of-rotation fixture was created in an orphaned commit and was not initially connected to the m06-block5-verification-completeness branch. Tests referenced the missing path.
- **Correction:** Regenerate the deterministic two-level proof-of-rotation fixture and attach it directly to the Block 5 branch tree.
- **Correction commit:** 38dc4991cb0c1249581f16565aa741bb6b8d7bfd
- **Validation:** Current branch tree contains tests/fixtures/proof-rotation-valid.bin, and Actions run #233 / 36751272856 passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED
## ERR-029 — Proof-of-rotation evidence is dropped on later signer verification failures

- **Milestone:** M0.6 Block 5 / second audit
- **Type:** Evidence-model propagation defect
- **Problem:** After parse_signed_data_v3() successfully established proof-of-rotation evidence, later signer-local failures could convert through error_to_scheme_info_with_evidence() without forwarding the parsed proof evidence.
- **Correction:** Introduce a rotation-aware signer error conversion and pass parsed proof-of-rotation evidence through every post-parse signer-local failure path.
- **Correction commit:** dc6d4eb4d241155bb9413e2de38839240901e60b
- **Validation:** Actions run #233 / 36751272856 passed Build, Test, Format, and Clippy; focused signer-error evidence test passed.
- **Status:** RESOLVED
## ERR-030 — proof-of-rotation evidence helper call-site correction was incomplete

- **Milestone:** M0.6 Block 5 / Actions run #223
- **Type:** Build failure
- **Actions run:** #223 / 36750840753
- **Problem:** The new error_to_scheme_info_with_rotation() helper was introduced with six parameters, but five v3 signer-local call sites still supplied only five arguments.
- **Cause:** The scoped evidence-propagation correction converted the helper name without consistently appending the already-parsed proof-of-rotation evidence argument.
- **Correction:** Add parsed.proof_of_rotation to every post-parse signer-local error conversion and keep pre-parse paths on the original helper.
- **Correction commit:** dc6d4eb4d241155bb9413e2de38839240901e60b
- **Validation:** Actions run #224 / 36750878135 passed Build and Test after the correction. Format failed independently on rustfmt-only layout differences.
- **Status:** RESOLVED
## ERR-031 — rustfmt failure after Block 5 proof-of-rotation implementation

- **Milestone:** M0.6 Block 5 / Actions run #224
- **Type:** CI formatting failure
- **Actions run:** #224 / 36750878135
- **Problem:** `cargo fmt --all -- --check` reported deterministic layout differences in the new proof-of-rotation implementation and tests.
- **Correction:** Apply only the rustfmt-indicated formatting changes; no production logic or test behavior changed.
- **Correction commit:** 9d0527a34924bc22b1de469281c7b706b0e727b7
- **Validation:** Actions run #228 / 36751061388 passed Format. Build and Test also passed on the same commit; Clippy failed independently on a separate lint.
- **Status:** RESOLVED

## ERR-032 — Clippy redundant-guard after Block 5 proof-of-rotation implementation

- **Milestone:** M0.6 Block 5 / Actions run #228
- **Type:** CI lint failure
- **Actions run:** #228 / 36751061388
- **Problem:** Clippy reported a redundant guard in the proof-of-rotation certificate DER validation match.
- **Correction:** Replace the guard pattern `Ok((remaining, _)) if remaining.is_empty()` with the equivalent slice pattern `Ok(([], _))`; no logic change.
- **Correction commit:** 5e35593c1bd4319c64404feefe5cc7f5073826f3
- **Validation:** Actions run #234 / 36751391708 passed Build, Test, Format, and Clippy after the subsequent formatting correction.
- **Status:** RESOLVED
## ERR-033 — rustfmt failure in the new proof-of-rotation evidence regression test

- **Milestone:** M0.6 Block 5 / Actions run #230
- **Type:** CI formatting failure
- **Actions run:** #230 / 36751168580
- **Problem:** `cargo fmt --all -- --check` required one line-wrap change in the newly added `signer_error_evidence_preserves_proof_of_rotation` test.
- **Correction:** Apply only the rustfmt-indicated string-literal layout change; no logic or test behavior change.
- **Correction commit:** 3318ab3cc3c173e0bdfa9855d9c567b041ebc00b
- **Validation:** Actions run #234 / 36751391708 passed Format.
- **Status:** RESOLVED
## ERR-034 — README historical M0.5 scope included M0.6 proof-of-rotation verification

- **Milestone:** M0.6 / global evaluation
- **Type:** Documentation chronology defect
- **Problem:** The README M0.5 Block 2 capability list stated that v3 proof-of-rotation lineage structure and parent-to-child signatures were verified, even though the validated M0.5 Block 2 checkpoint explicitly kept proof-of-rotation as a manual-review boundary. The M0.6 Block 5 section correctly assigns that verification capability to Block 5.
- **Correction:** Remove only the proof-of-rotation verification bullet from the historical M0.5 Block 2 capability list and keep the capability documented under M0.6 Block 5.
- **Correction commit:** 97c2748a475bebd55a7150f5ea136eec223e2467
- **Validation:** Push Actions run #251 / 36753228659 passed the complete Rust CI gate (Build, Test, Format and Clippy) on the branch after the documentation correction.
- **Status:** RESOLVED

## Current validation state

M0.6 Block 5 implementation and stacked pull-request validation have passed the complete validation gate.

Block 5 push run #235 / 36751525004:
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

Stacked PR-event run #236 / 36751740474 for draft PR #12:
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

Block 4 base workflow run #212 / 36750176041:
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

Final ledger-state push run #237 / 36751850590:
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

M0.6 Block 5 is validated and its final checkpoint documentation is now being recorded. No merge is performed as part of the checkpoint cycle.


## Maintenance rule

Every future Actions failure or audit-discovered defect must append a new ERR-NNN entry rather than editing an older entry. Resolutions should reference the correction commit and the validating Actions run whenever available.


## ERR-035 — M0.7 definition treated proof-of-rotation flags as inherently valid/invalid combinations

- **Milestone:** M0.7 definition / second audit
- **Type:** Definition defect
- **Problem:** Block 2 required fixtures for “invalid flag combinations” without defining an Android semantic rule that makes arbitrary capability-bit combinations invalid.
- **Cause:** The planning text treated the lineage capability flags as a closed set of mutually constrained states instead of independent capability bits.
- **Correction:** Redefine Block 2 around explicit capability-bit modeling, known/unknown-bit evidence, and only deterministic semantic rules that Android exposes locally.
- **Validation:** Corrected definition will be re-audited and validated through Actions before Etapa 3 closure.
- **Status:** RESOLVED

## ERR-036 — M0.7 v3.1 definition did not enumerate required v3/v3.1 cross-block constraints

- **Milestone:** M0.7 definition / second audit
- **Type:** Definition defect
- **Problem:** Block 3 described the v3.1 signer/range relationship too generically and did not explicitly name the required v3/v3.1 rotation-target and stripping-protection consistency checks.
- **Cause:** The scope text summarized v3.1 as an extension of v3 without listing the cross-block verification boundaries that distinguish v3.1 behavior.
- **Correction:** Enumerate v3.1/v3 presence rules, rotation-min-SDK consistency, stripping-protection attribute linkage, targeted SDK coverage, and lineage/signer consistency as explicit acceptance boundaries.
- **Validation:** Corrected definition will be re-audited and validated through Actions before Etapa 3 closure.
- **Status:** RESOLVED

## ERR-037 — M0.7 Block 1 ECDSA/SHA-512 coverage gap

- **Milestone:** M0.7 Block 1 / AUDIT-M0.7-B1-001
- **Type:** Cryptographic coverage defect
- **Problem:** Signature algorithm `0x0202` (ECDSA with SHA-512) is defined and supported by Android v2/v3, but the current verifier excludes it from supported algorithm selection and returns `Unsupported` before cryptographic verification.
- **Cause:** The initial M0.6 verifier implementation covered ECDSA/SHA-256 plus RSA variants but did not implement an ECDSA/SHA-512 backend.
- **Required correction:** Add real ECDSA/SHA-512 verification for the bounded Block 1 P-384 coverage cell while preserving explicit Unsupported semantics outside that cell.
- **Audit commit:** a016971b3052c29873b79111b2300307855e1071
- **Validation:** Pre-change audit completed against Android v2/v3 specifications and AOSP apksig fixtures; production code not changed by the audit commit.
- **Status:** OPEN — scoped for Block 1 implementation

## ERR-038 — M0.7 Block 1 Android key/curve coverage gaps

- **Milestone:** M0.7 Block 1 / AUDIT-M0.7-B1-003
- **Type:** Cryptographic coverage defect
- **Problem:** The current verifier intentionally narrows the Android v2/v3 matrix: ECDSA accepts only P-256/P-384, and RSA accepts only 2048–8192-bit keys, leaving Android-documented P-521 plus RSA 1024/16384 cases outside the current verification boundary.
- **Cause:** The M0.6 verifier backend uses ring coverage that is narrower than the full Android signature/key matrix.
- **Required correction:** Track the remaining key/curve cells explicitly and implement them only through future bounded increments; do not broaden the Block 1 scope beyond the selected 0x0202/P-384 increment.
- **Audit commit:** a016971b3052c29873b79111b2300307855e1071
- **Validation:** Pre-change audit completed against Android v2/v3 specifications and AOSP apksig verification fixtures; production code not changed by the audit commit.
- **Status:** OPEN — explicitly out of scope for the selected first increment

## ERR-039 — M0.7 Block 1 ECDSA/SHA-512 backend compile errors

- **Milestone:** M0.7 Block 1 / implementation validation
- **Type:** Build failure
- **Actions run:** #295 / 36779609901
- **Problem:** The first ECDSA/SHA-512 implementation commit did not compile: the `PrehashVerifier` trait was imported from the wrong module path, and the X.509 EC public-key bytes were passed as `Cow<[u8]>` instead of a byte slice.
- **Cause:** The initial integration used the underlying `ecdsa` module structure from documentation rather than the re-exported trait path exposed by `p384::ecdsa`, and omitted a borrow on the parsed SubjectPublicKey BIT STRING data.
- **Correction:** Import `p384::ecdsa::signature::hazmat::PrehashVerifier` and pass `&cert.public_key().subject_public_key.data` to `P384VerifyingKey::from_sec1_bytes`.
- **Correction pending validation:** 37139b0eda12765365a153854afd774a7b0bf29b1
- **Validation:** Build failed before Test/Format/Clippy on Actions run #295 / 36779609901; the corrected commit must pass the full gate before this entry is resolved.
- **Status:** OPEN
