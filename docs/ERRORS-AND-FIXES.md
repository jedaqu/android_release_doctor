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

## ERR-040 — M0.7 Block 1 test borrow-checker failure

- **Milestone:** M0.7 Block 1 / implementation validation
- **Type:** Test build failure
- **Actions run:** #296 / 36779727793
- **Problem:** The new tampered-signature regression test indexed `signature[signature.len() - 1]`, creating overlapping mutable and immutable borrows under the Rust borrow checker.
- **Cause:** The test combined an immutable length query and mutable indexed assignment in the same expression.
- **Correction:** Store the final index in a local variable before mutating the signature byte, without changing test behavior.
- **Correction commit pending:** current follow-up implementation commit
- **Validation:** Actions run #296 passed Build but failed Test compilation at the new regression assertion; the corrected commit must pass the full gate before this entry is resolved.
- **Status:** OPEN


## ERR-041 — M0.7 Block 1 crypto fixture encoding was malformed

- **Milestone:** M0.7 Block 1 / implementation validation
- **Type:** Test-fixture correctness defect
- **Actions run:** #297 / 36779970794
- **Problem:** The newly added v2/v3 ECDSA/SHA-512 fixtures had an incorrect signer encoding: the signed-data payload was not length-prefixed inside the signer structure. This caused the verifier to interpret digest/certificate bytes as signer-level signature/public-key fields.
- **Cause:** The fixture builder constructed the signer payload as raw signed-data bytes followed by signatures and public key instead of encoding the signed-data field as the required length-prefixed sequence.
- **Correction:** Regenerate both deterministic v2 and v3 fixtures with the correct signed-data sequence framing, while retaining the same bounded P-384 / ECDSA-SHA-512 scope. Regenerate the direct unit-test signature so it matches its embedded certificate and test message.
- **Correction pending validation:** current follow-up implementation commit
- **Validation:** Actions run #297 / 36779970794 passed Build but failed three focused crypto tests; 62 tests passed and the three failures were the malformed fixture/direct-signature cases described above.
- **Status:** OPEN


## ERR-042 — M0.7 Block 1 rustfmt failure

- **Milestone:** M0.7 Block 1 / implementation validation
- **Type:** CI formatting failure
- **Actions run:** #300 / 36781123329
- **Problem:** `cargo fmt --all -- --check` reported three deterministic layout differences in the new ECDSA/SHA-512 implementation.
- **Cause:** The implementation was logically valid, but the new import grouping and two long error expressions had not yet been formatted by rustfmt.
- **Correction:** Apply only the rustfmt-indicated import grouping and line wrapping; no logic or test behavior changes.
- **Correction commit pending:** current follow-up formatting commit
- **Validation:** Actions run #300 / 36781123329 passed Build and Test (65 unit tests + 16 integration tests) and failed only Format on the three reported layout differences.
- **Status:** OPEN


## ERR-043 — M0.7 Block 1 second audit found missing focused regressions

- **Milestone:** M0.7 Block 1 / second audit
- **Type:** Test coverage defect
- **Problem:** The implementation had positive v2/v3 ECDSA/SHA-512 P-384 coverage and a direct tampered-signature test, but the acceptance matrix lacked a focused v2 APK tampering regression and an explicit regression proving a non-P-384 `0x0202` case returns `Unsupported`.
- **Cause:** The first implementation focused on proving the positive path and cryptographic failure path before completing every bounded acceptance cell from the pre-change audit.
- **Required correction:** Add the two focused regressions without changing production verification behavior.
- **Second-audit document:** f11226a47b33f7812a8c487cc5c15a94652ab62b
- **Status:** OPEN

## ERR-044 — M0.7 Block 1 Cargo.lock was stale after adding crypto dependencies

- **Milestone:** M0.7 Block 1 / implementation validation
- **Type:** Reproducibility / dependency-lock defect
- **Problem:** After adding `p384` and `sha2` as direct dependencies, the tracked `Cargo.lock` still described the pre-implementation dependency graph.
- **Cause:** The dependency declarations were committed before regenerating and committing the lockfile.
- **Correction:** Generate the lockfile with the repository toolchain, commit the generated dependency graph, and remove the temporary CI-only lock-generation workflow.
- **Correction commit:** 3880246bc00ea45d78d04ea6b30449fa84bde54d
- **Validation:** Final Rust CI run #305 / 36782365269 passed Build, Test, Format, and Clippy with the tracked lockfile present.
- **Status:** RESOLVED

## ERR-045 — M0.7 Block 1 second-audit correction closure

- **Milestone:** M0.7 Block 1 / second audit
- **Type:** Validation closure
- **Problem addressed:** ERR-039 through ERR-043 represented the implementation compile/test/fixture/formatting sequence and the second-audit regression-coverage finding encountered during Block 1.
- **Correction chain:** `68f9810ddc41d1782b0b8cb7625770baaec8d560`; `cfefb487ebdeaa6afe9a5544f5de2fb6b6e40bac`; `0c1c6f3a21165c4a1d9e5522ef85d15dafc275b7`; `0de900ec5590bcf2d3a18012de23a7b9d4e92a29`; `a446e92ebf62d17e53d5ed3657af58e6447337ca`; `597f497499910b5269278ed97130d1d071b49119`; `e8996c7ffce11be146c1d04b49a0e575f2c3fe1a`.
- **Validation:** Final second-audit regression run #308 / 36782554714 passed Build, Test, Format, and Clippy. The final second-audit document records PASS.
- **Status:** RESOLVED

## ERR-046 — M0.7 Block 1 final second-audit result

- **Milestone:** M0.7 Block 1 / second audit
- **Type:** Audit closure
- **Result:** The bounded ECDSA/SHA-512 P-384 increment satisfies the pre-change acceptance matrix, including v2/v3 positive fixtures, v2/v3 tamper rejection, explicit Unsupported semantics for non-P-384 curves, current dependency locking, and complete CI validation.
- **Second-audit commit:** 26f9f956d0bf32080881bbbbbcc6d8b0687c1284
- **Status:** PASS — ready for checkpoint

## ERR-047 — README status stale after M0.7 Block 1 implementation

- **Milestone:** M0.7 Block 1 / checkpoint preparation
- **Type:** Documentation synchronization defect
- **Problem:** README still identified M0.6 Block 5 as the current repository status after the M0.7 Block 1 implementation and second audit had passed.
- **Correction:** Update the status and add the bounded M0.7 Block 1 capability section without rewriting historical M0.5/M0.6 scope.
- **Validation:** Final checkpoint validation will exercise the corrected README state through Build, Test, Format, and Clippy.
- **Status:** OPEN

## ERR-048 — M0.7 Block 1 README synchronization resolved

- **Milestone:** M0.7 Block 1 / checkpoint preparation
- **Type:** Documentation closure
- **Correction:** README status and M0.7 Block 1 capability scope were synchronized with the validated implementation.
- **Correction commit:** a65d613fbd28cc133fa09d891c3213bfbe386f24
- **Validation:** Actions run #313 / 36782863855 passed Build, Test, Format, and Clippy with the synchronized documentation state.
- **Status:** RESOLVED


## ERR-049 — M0.7 Block 2 proof-of-rotation capability flags are parsed but discarded

- **Milestone:** M0.7 Block 2 / AUDIT-M0.7-B2-001
- **Type:** Evidence-model defect
- **Problem:** The existing proof-of-rotation verifier parses each lineage node's flags field but stores it as a discarded value. The structured proof-of-rotation evidence therefore cannot report the capabilities declared for historical signing certificates.
- **Cause:** M0.6 Block 5 correctly focused on lineage structure, certificate uniqueness, parent-to-child signatures, algorithm linkage, and final-certificate binding, but did not model the independent self-trust capability bits.
- **Required correction:** Represent the Android-defined capability bits explicitly per lineage node and preserve unknown/reserved bits as evidence without treating unusual combinations as inherently invalid.
- **Audit:** docs/AUDIT-M0.7-BLOCK2.md
- **Validation:** Pre-change audit passed; production implementation has not yet been changed.
- **Status:** OPEN — scoped for M0.7 Block 2 implementation


## ERR-050 — M0.7 Block 2 fixture helper borrowed and mutated the same buffer

- **Milestone:** M0.7 Block 2 / second audit
- **Type:** Test compilation risk
- **Problem:** The new proof-of-rotation fixture helper retained a slice borrowed from the output buffer and then attempted to mutate that same buffer to replace the node flags.
- **Cause:** The helper used the borrowed `read_sequence()` result directly while calculating the mutation offset.
- **Correction:** Copy the selected node into an owned `Vec<u8>` before mutating the original fixture buffer; no production verification behavior changes.
- **Correction commit:** 766e190e2d90a677c4844c4fb65ce8a925a54128
- **Validation:** Second-audit re-review required before Actions.
- **Status:** RESOLVED — pending CI validation


## ERR-051 — M0.7 Block 2 stacked CI trigger coverage was missing

- **Milestone:** M0.7 Block 2 / second-audit CI review
- **Type:** CI configuration / validation gap
- **Problem:** The Block 2 source branch was not included in the Rust workflow push trigger, and the inherited workflow did not include the validated Block 1 branch as a pull-request target for the stacked Block 2 PR.
- **Cause:** The workflow had been updated for Block 1 but had not yet been extended to the next stacked block.
- **Correction:** Add `m07-block2-rotation-semantics` to push coverage and `m07-block1-crypto-coverage` to pull-request target coverage without changing the job graph.
- **Correction commit:** c93f52f01af86d7c6498fdf86e7a7a6ab9fd9cc6
- **Validation:** Final workflow review and stacked PR-event validation required.
- **Status:** RESOLVED — pending Actions validation


## ERR-052 — Block 2 PR-event validation required base-branch workflow coverage

- **Milestone:** M0.7 Block 2 / CI follow-up
- **Type:** CI configuration / validation gap
- **Problem:** The Block 2 PR targets `m07-block1-crypto-coverage`, but the workflow stored on that validated base branch did not yet include `m07-block1-crypto-coverage` in its `pull_request.branches` filter. The Block 2 branch workflow alone cannot establish PR-event coverage for a PR whose base is Block 1.
- **Correction:** Add `m07-block1-crypto-coverage` to the pull-request target list on the validated Block 1 base workflow, without changing the job graph or production code.
- **Correction commit on base branch:** 6395c5fbf65d0a40060998646cb62fe07bf9f6fd
- **Validation:** New PR-event execution required.
- **Status:** RESOLVED — pending Actions validation


## ERR-053 — M0.7 Block 2 missing capability field in duplicate proof-of-rotation initializer

- **Milestone:** M0.7 Block 2 / Actions run #324
- **Type:** Build failure
- **Actions run:** #324 / 36799430192
- **Problem:** After extending `ProofOfRotationInfo` with `capabilities`, the duplicate proof-of-rotation attribute error path in `parse_signed_data_v3()` still used the old struct initializer and omitted the new field.
- **Cause:** The new evidence field was added to normal validation/fixtures but one existing error initializer was missed.
- **Correction:** Add `capabilities: Vec::new()` to that duplicate-attribute error initializer. No verification behavior changes beyond restoring compilation.
- **Correction commit:** 751fdb4a4917316b276473478514d55374654b5b
- **Validation:** Required in new Actions execution.
- **Status:** RESOLVED — pending new CI execution


## ERR-054 — M0.7 Block 2 fixture helper wrote flags at the wrong offset

- **Milestone:** M0.7 Block 2 / Actions run #326
- **Type:** Test-fixture correctness defect
- **Actions run:** #326 / 36799721854
- **Problem:** The new fixture helper intended to mutate a lineage node's capability flags, but its calculated offset was four bytes too far into the node. The tests therefore overwrote the node's signature algorithm field instead of the flags field.
- **Observed effects:** Four capability-evidence tests failed with `Unsupported`, `Invalid`, or unchanged flags evidence while the remaining 67 unit tests passed.
- **Cause:** The node layout contains a four-byte length prefix followed by the signed-data payload; the flags field begins immediately after that payload. The helper added an unnecessary second four-byte offset.
- **Correction:** Change the fixture mutation offset from `4 + signed_data.len() + 4` to `4 + signed_data.len()`. Test-only correction; no production verification behavior changed.
- **Correction commit:** fd6dfa1e275c75a4bc5d28fa1c42b6e969119497
- **Validation:** Required in new Actions execution.
- **Status:** RESOLVED — pending new CI execution


## ERR-055 — M0.7 Block 2 rustfmt failure in capability regression tests

- **Milestone:** M0.7 Block 2 / Actions run #333
- **Type:** CI formatting failure
- **Actions run:** #333 / 36799797451
- **Problem:** `cargo fmt --all -- --check` reported three deterministic layout differences in the newly added proof-of-rotation fixture helper/tests.
- **Cause:** The implementation was logically valid, but three new lines had not yet been formatted according to repository rustfmt output.
- **Correction:** Apply only the three rustfmt-indicated line-wrap/layout changes. No logic, fixture data, or production behavior changed.
- **Correction commit:** f0bcd2652dab80507cf62b5d51ebaaba848f2539
- **Validation:** Required in new Actions execution.
- **Status:** RESOLVED — pending new CI execution


## ERR-056 — M0.7 Block 2 correction-chain validation closure

- **Milestone:** M0.7 Block 2 / final Actions validation
- **Type:** Validation closure
- **Scope:** ERR-050 through ERR-055, including the scoped CI trigger corrections and test/format corrections recorded during this block.
- **Validation run:** Actions #342 / 36799881789
- **Result:** Build PASS; Test PASS; Format PASS; Clippy PASS.
- **Additional validation:** The final capability model and regression suite remain green, including all existing M0.6 proof-of-rotation tests and the new known/unknown capability-flag cases.
- **Status:** RESOLVED — Block 2 ready for checkpoint


## ERR-057 — M0.7 Block 3 v3.1 verification and cross-block semantics are not implemented

- **Milestone:** M0.7 Block 3 / pre-change audit
- **Type:** Verification capability gap
- **Problem:** The verifier detects the v3.1 signing-block ID but does not cryptographically verify the v3.1 block or enforce the required v3/v3.1 rotation-target semantics.
- **Cause:** M0.6 and M0.7 Blocks 1–2 deliberately stopped at the v2/v3 and proof-of-rotation boundaries and retained v3.1 as an explicit unsupported/manual-review boundary.
- **Required correction:** Add v3.1 verification for the existing supported crypto subset plus deterministic cross-block checks for the v3 base block, rotation-min-SDK stripping protection, targeted SDK ranges, and lineage consistency.
- **Audit:** docs/AUDIT-M0.7-BLOCK3.md
- **Validation:** Pre-change audit passed; production implementation is now authorized on `m07-block3-v31-verification`.
- **Status:** OPEN — scoped for M0.7 Block 3 implementation


## ERR-058 — M0.7 Block 3 implementation substitution corruption caught before validation

- **Milestone:** M0.7 Block 3 / implementation audit
- **Type:** Implementation correction before CI
- **Problem:** An automated source transformation temporarily replaced v3 parser/error labels with literal `${scheme_name}` text and also inserted v3.1 evidence defaults into one helper's parameter list instead of only into struct initializers.
- **Cause:** A broad text substitution was used while parameterizing the shared v3/v3.1 verification path.
- **Impact:** The intermediate source was not suitable for compilation and could have changed unrelated diagnostic text.
- **Correction:** Restore all affected v3 labels, repair the helper signature, complete the evidence-model initializers, and continue with smaller targeted edits.
- **Validation:** Detected during implementation review before Actions; no corrupted intermediate state was used for CI.
- **Status:** RESOLVED — pre-CI correction


## ERR-059 — M0.7 Block 3 stacked CI trigger coverage was missing

- **Milestone:** M0.7 Block 3 / second-audit CI review
- **Type:** CI configuration / validation gap
- **Problem:** The Block 3 source branch was not present in the Rust workflow push trigger, and the active Block 2 workflow did not yet target the Block 3 branch for the stacked pull request.
- **Correction:** Add `m07-block3-v31-verification` to the Block 3 push workflow, `m07-block2-rotation-semantics` to the Block 3 PR target list, and add `m07-block3-v31-verification` to the validated Block 2 base workflow.
- **Correction commits:** `e1d87aab495ed152c6dc3c2aa247175d6e51a8fc` on Block 3; `c747e02aefb506700b1d5c7ce45661fd33b14e7d` on Block 2.
- **Validation:** PR-event Actions validation required before checkpoint.
- **Status:** RESOLVED — pending Actions validation


## ERR-060 — M0.7 Block 3 build failure from dynamic LengthReader label

- **Milestone:** M0.7 Block 3 / Actions run #356
- **Type:** Build failure
- **Actions run:** #356 / 36801502384
- **Problem:** `LengthReader::read_sequence()` accepts `&str`, but the parameterized v3/v3.1 entry point passed `format!("{scheme_name} signers")`, producing a `String`.
- **Correction:** Use a static `"v3.1 signers"` or `"v3 signers"` label selected from the block ID.
- **Correction commit:** 8971a7c0cf55eb2deb4c2a1c6565baf2c1d67da3
- **Validation:** Required in new Actions execution.
- **Status:** RESOLVED — pending new CI execution


## ERR-061 — M0.7 Block 3 test parser call sites still used the old signature

- **Milestone:** M0.7 Block 3 / Actions run #358
- **Type:** Test compilation failure
- **Actions run:** #358 / 36801575904
- **Problem:** After parameterizing `parse_signed_data_v3()` with `scheme_block_id`, three existing tests still called it with only the signed-data bytes.
- **Affected call sites:** lines 2069, 2197, and 2482 at the time of the failing run.
- **Correction:** Pass `V3_BLOCK_ID` at all three existing v3 test call sites. No production verification semantics changed.
- **Correction commit:** 9ff5dbd80188eb4e08560075f524496bd06221c8
- **Validation:** Required in new Actions execution.
- **Status:** RESOLVED — pending new CI execution


## ERR-062 — M0.7 Block 3 over-constrained v3.1 rotation minimum SDK

- **Milestone:** M0.7 Block 3 / Actions run #364
- **Type:** Semantic-specification correction
- **Actions run:** #364 / 36801696008
- **Problem:** The first Block 3 implementation rejected a v3.1 signer when its rotation-target minimum SDK was below 33. The authoritative AOSP verifier/test corpus contains valid v3.1 configurations targeting SDK 32, where SDK 32 is the finalized predecessor SDK used during platform development.
- **Observed effect:** The valid v3.1 fixture `v31-rsa-2048_2-tgt-33-1-tgt-28.apk` exposed the overly strict condition; the fixture's v3.1 signer range is 32..MAX.
- **Correction:** Remove the hard-coded `min_sdk >= 33` rejection and treat the v3.1 signer range as authoritative evidence. Keep the required cross-block checks for v3 base presence, range ordering/continuity, stripping-protection equality, and lineage consistency.
- **Correction commit:** 039301636517d71c763502b0eb07dfb74b1c338e
- **Validation:** Required in new Actions execution.
- **Status:** RESOLVED — pending new CI execution


## ERR-063 — M0.7 Block 3 rejected an allowed v3/v3.1 SDK boundary overlap

- **Milestone:** M0.7 Block 3 / Actions run #370
- **Type:** Cross-block semantic correction
- **Actions run:** #370 / 36801841509
- **Problem:** The cross-block validator rejected any case where v3 maxSDK equaled v3.1 minSDK. The authoritative v3.1 corpus includes a valid development-era configuration with both values at SDK 32.
- **Correction:** Permit equality at SDK 32 and permit equality for later SDK versions only when the v3.1 signer explicitly carries the development-release rotation attribute. Unjustified overlap above that boundary remains invalid; ordinary gaps remain invalid.
- **Correction commit:** a6cb7b42acda17f5d9a11dc1ffce07bc418f7302
- **Validation:** Required in new Actions execution.
- **Status:** RESOLVED — pending new CI execution


## ERR-064 — M0.7 Block 3 required v3 proof-of-rotation evidence when v3.1 already carried the lineage

- **Milestone:** M0.7 Block 3 / Actions run #378
- **Type:** Cross-block lineage semantic correction
- **Actions run:** #378 / 36801940959
- **Problem:** The cross-block validator required both v3 and v3.1 signers to expose structured proof-of-rotation evidence. The authoritative valid v3.1 fixture has the lineage evidence in the v3.1 signer while the v3 certificate identity is still sufficient as the cross-block bridge.
- **Correction:** Require verified lineage evidence from v3.1; when v3 also exposes lineage, require prefix consistency. When v3 has no structured lineage, require every v3 signer certificate fingerprint to be represented in the verified v3.1 lineage.
- **Correction commit:** 88ca7a5a8db12b71d17dd845ab53103e54abfdea
- **Validation:** Required in new Actions execution.
- **Status:** RESOLVED — pending new CI execution


## ERR-065 — M0.7 Block 3 lineage negative fixture was classified incorrectly

- **Milestone:** M0.7 Block 3 / Actions run #384
- **Type:** Regression-test specification correction
- **Actions run:** #384 / 36802039515
- **Problem:** The `v31-2elem-incorrect-lineage.apk` corpus fixture is a malformed-lineage case, not a cross-block semantic mismatch. The test incorrectly expected the semantic message `lineages are inconsistent`.
- **Correction:** Rename the fixture regression to `rejects_v31_with_malformed_lineage_fixture` and assert cryptographic/lineage validation failure. Add a separate unit test for genuine v3/v3.1 lineage-prefix inconsistency using synthetic evidence objects at the semantic helper boundary, without bypassing APK cryptographic verification.
- **Correction commit:** 013dfbe2d4433f861a7b2f59bebc12bc581f5c5a
- **Additional correction:** The valid authoritative v3.1 fixture's v3 rotation-min-sdk is SDK 32, so its existing expectation was corrected from 33 to 32.
- **Validation:** Required in new Actions execution.
- **Status:** RESOLVED — pending new CI execution


## ERR-066 — M0.7 Block 3 fixture max SDK expectation used the wrong integer width

- **Milestone:** M0.7 Block 3 / Actions run #390
- **Type:** Regression-test expectation correction
- **Actions run:** #390 / 36802149071
- **Problem:** The valid v3.1 fixture reports maxSDK 2,147,483,647, matching the signed 32-bit Android SDK range convention used by the source corpus; the test expected u32::MAX.
- **Correction:** Assert the fixture range as `(32, i32::MAX as u32)`.
- **Correction commit:** 390ed764d3bbdce14e402b28c52cb192cb41b3fa
- **Validation:** Required in new Actions execution.
- **Status:** RESOLVED — pending new CI execution


## ERR-067 — M0.7 Block 3 rustfmt corrections after functional tests passed

- **Milestone:** M0.7 Block 3 / Actions run #396
- **Type:** CI formatting failure
- **Actions run:** #396 / 36802218920
- **Problem:** Build and Test passed, but `cargo fmt --all -- --check` reported eight deterministic formatting differences in `signature_verify.rs`.
- **Correction:** Apply the exact rustfmt line wrapping at v3/v3.1 dispatch, cross-block conditions, signer-label parsing, and two test assertions/calls.
- **Correction commit:** e41dc1e378b15ad65bf404c3e7381985178934cf
- **Validation:** Required in new Actions execution.
- **Status:** RESOLVED — pending new CI execution


## ERR-068 — M0.7 Block 3 correction-chain validation closure

- **Milestone:** M0.7 Block 3 / final Actions validation
- **Type:** Validation closure
- **Scope:** ERR-060 through ERR-067, including build, test, semantic, fixture, and formatting corrections.
- **Final validation run:** Actions #402 / 36802297492
- **Result:** Build PASS; Test PASS; Format PASS; Clippy PASS.
- **Status:** RESOLVED — Block 3 ready for checkpoint


## ERR-069 — M0.7 Block 4 stale top-level SIGNING-003 integration

- **Milestone:** M0.7 Block 4 / integration audit
- **Type:** Production integration defect
- **Problem:** Block 3 added verified v3.1 evidence to `ApkSignatureVerification`, but the top-level `SIGNING-003` audit path still evaluated only v2/v3 and unconditionally treated v3.1 presence as manual review with a stale “not cryptographically verified” message.
- **Impact:** The core verifier and the user-facing release audit could disagree about a valid v3.1 APK's verification state.
- **Correction:** Include v3.1 in the `SIGNING-003` scheme evaluation, preserve Invalid/Unsupported semantics, keep v3.2 as a manual-review boundary, and add focused regression tests for Verified/Invalid/Unsupported v3.1 plus v3.2 presence.
- **Correction commit:** `8ec343a95798d59ec577a3612b151f430ddb6291` and `554e1c472222a3f5ca96b33bc1d7e1008b6f449b`.
- **Validation:** Required in Block 4 Actions validation.
- **Status:** CORRECTION APPLIED — pending final Actions validation

## ERR-070 — M0.7 Block 4 stacked CI trigger coverage

- **Milestone:** M0.7 Block 4 / integration audit
- **Type:** CI configuration / validation gap
- **Problem:** The inherited Rust workflow did not yet include the Block 4 branch as a push target or Block 4 as a pull-request target from the Block 3 base.
- **Correction:** Add `m07-block4-integration-completeness` to the active workflow push targets and add `m07-block3-v31-verification` as its PR target; then update the validated Block 3 base workflow to recognize the Block 4 PR target using the established stacked-branch pattern.
- **Validation:** Required in Block 4 Actions validation.
- **Status:** OPEN — correction in progress
## ERR-071 — M0.7 Block 4 first SIGNING-003 branch-structure failure

- **Milestone:** M0.7 Block 4 / Actions run #36868550017
- **Type:** Build failure
- **Actions job:** 110390082400
- **Problem:** The first Block 4 integration correction left the nested `SIGNING-003` manual-review/pass branches structurally unbalanced, producing a Rust parser error in `crates/doctor-core/src/lib.rs`.
- **Cause:** The scoped integration edit introduced an incorrect brace/`else` relationship around the new v3.1 manual-review path.
- **Initial correction:** Commit `b81d2710a23763cb573060906d26ac30feb1870b` attempted to close the branch structure locally.
- **Validation:** Follow-up Actions run #36868916143 still failed at the same area, showing that the structural correction was incomplete.
- **Status:** SUPERSEDED BY ERR-072 — retained as the first observed failure in the correction chain.

## ERR-072 — M0.7 Block 4 residual extra `else` after SIGNING-003 correction

- **Milestone:** M0.7 Block 4 / Actions run #36868916143
- **Type:** Build failure
- **Actions job:** 110391334448
- **Problem:** Build failed with `expected expression, found keyword else` at `crates/doctor-core/src/lib.rs:1219`.
- **Root cause:** The previous local brace edit removed one unmatched delimiter but left an additional outer `} else {`; the `else` belonged to the inner unsupported/manual-review condition.
- **Correction:** Rebind the manual-review/pass `else` directly to the `schemes.iter().any(Unsupported) || v31_requires_manual || verification.v32_present` condition and remove the extra outer `else`.
- **Correction commit:** `e3ae2c9be9c4a1a5431a53b6a2948ab6c512eac3`
- **Validation:** Corrected source must pass Build, Test, Format, and Clippy before this entry can be marked resolved.
- **Status:** CORRECTION APPLIED — pre-Actions second audit PASS; pending Actions validation.


## ERR-073 — M0.7 Block 4 second-audit terminology clarification

- **Milestone:** M0.7 Block 4 / second audit after ERR-072 correction
- **Type:** Audit clarification
- **Problem:** The corrected Block 4 source still contains a conditional v3.1 manual-review detail stating that a present v3.1 block without a verified scheme result requires manual review. That text could be mistaken for the obsolete Block 3 behavior that unconditionally treated every v3.1 block as unverified.
- **Clarification:** The current path first evaluates the distinct verified v3.1 scheme result. Only a present v3.1 block without a Verified result takes the manual-review fallback; a verified v3.1 scheme can contribute to PASS and an invalid v3.1 scheme remains a BLOCKER.
- **Validation:** Confirmed by source-level second audit and focused SIGNING-003 regression coverage.
- **Status:** RESOLVED — audit clarification; no production behavior change.


## ERR-074 — SIGNING-003 unclosed delimiter after Block 4 branch repair

- **Milestone:** M0.7 Block 4
- **Type:** Build failure
- **Actions run:** #36869780405 / 36869780405
- **Job:** `test` / 110394269323
- **Problem:** `crates/doctor-core/src/lib.rs` failed to compile with `this file contains an unclosed delimiter`. The compiler traced the unmatched structure to the `SIGNING-003` branch beginning at line 696 and the nested conditional around line 1218.
- **Root cause:** The previous bounded branch repair closed the inner manual-review conditional but left the surrounding `else` branch open before the final `else` that handles unavailable verification.
- **Correction:** Restore the missing closing delimiter for the surrounding `else` branch and normalize indentation only within that affected `SIGNING-003` block.
- **Correction commit:** `89753fc376e8fd4381d67c5b4c72b0ac5aff6324`
- **Validation:** Actions run #428 / `36873605656` passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED


## ERR-075 — M0.7 Block 4 stacked PR target missing from validated Block 3 base workflow

- **Milestone:** M0.7 Block 4 / final stacked-CI audit
- **Type:** CI configuration / validation gap
- **Problem:** The validated m07-block3-v31-verification base workflow did not list m07-block3-v31-verification itself in pull_request.branches. Therefore PR #15, whose target is Block 3, could not exercise the required pull-request workflow from the base-branch configuration.
- **Root cause:** The base workflow contained the future Block 4 branch as a pull-request target, but omitted the actual Block 3 target required by PR #15. The head workflow alone could not establish that event coverage.
- **Correction:** Add m07-block3-v31-verification to the Block 3 base workflow pull-request targets while preserving the existing job graph and historical targets.
- **Correction commit:** 2d506d98e7892e602f4f2c4a99cbcab0b1d190b9
- **Validation:** Actions #430 / 36874487972 (push) and #431 / 36874495416 (pull_request) both passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED

## ERR-076 — PR #15 merge conflict blocked stacked pull-request validation

- **Milestone:** M0.7 Block 4 / final stacked-CI audit
- **Type:** CI configuration / branch synchronization defect
- **Problem:** PR #15 reported mergeable=false with mergeable_state=dirty after ERR-075. The base workflow and Block 4 head workflow had diverging pull_request.branches lists, producing a workflow-file merge conflict. GitHub does not run pull_request workflows while the PR has a merge conflict.
- **Root cause:** ERR-075 corrected the validated Block 3 base workflow by adding m07-block3-v31-verification, while the Block 4 head retained its own scoped target list without the future Block 4 target present on the base side.
- **Correction:** Synchronize the Block 4 head workflow with the validated base target list by retaining the existing Block 4 push trigger and including both m07-block3-v31-verification and m07-block4-integration-completeness in pull_request.branches. The Build → Test → Format → Clippy graph is unchanged.
- **Correction commit:** 50c3735d04d9c7532ffad13a8946bfaadcecf9c9
- **Validation:** Actions #433 / 36875170256 passed Build, Test, Format, and Clippy; PR #15 returned mergeable=true and mergeable_state=clean. Actions #434 / 36875368536 then executed with event pull_request for PR #15 and passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED

## ERR-077 — Report v1 implementation imported proof-of-rotation DTO sources from the crate root

- **Milestone:** M0.8 Block 1 / Report v1 production serialization
- **Type:** Build defect / import path
- **Observed in:** Actions #444 / run 36883291509, Build step
- **Commit:** 63aea724a4b3e052a037f3de4acaae14b2bc63d0
- **Problem:** `crates/doctor-core/src/report.rs` imported `ProofOfRotationCapabilities` and `ProofOfRotationInfo` from `crate::`, but those domain types are defined in `signature_verify.rs` and are not re-exported by the crate root.
- **Secondary warning:** `ArtifactKind` was imported in `report.rs` but only needed by the test module, producing an unused-import warning during Build.
- **Root cause:** The new explicit DTO mapper was aligned to the frozen evidence model but used the crate-root import surface without verifying which signature-domain types are publicly re-exported.
- **Correction:** Import the proof-of-rotation domain types directly from `crate::signature_verify` and keep `ArtifactKind` scoped to the tests where it is used.
- **Validation:** Corrected commit `2a0b42b911c185ca31eb265d1fc3a13b2c5ab165`; Actions #449 / run `36883817080` passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED


## ERR-078 — Report v1 implementation failed rustfmt gate

- **Milestone:** M0.8 Block 1 / Report v1 production serialization
- **Type:** Formatting gate failure
- **Observed in:** Actions #447 / run 36883538774, Format step
- **Commit:** 339ed88510fa77b17a6e3357f68aa9e1935d8019
- **Problem:** `crates/doctor-core/src/report.rs` contained five rustfmt differences after the ERR-077 correction.
- **Scope:** Import ordering/line wrapping, iterator formatting, a long constructor expression, and two assertion expressions. Build and Test both passed before Format stopped the job.
- **Root cause:** The minimal correction was applied manually after the first Build failure; the resulting file had not yet been normalized by `cargo fmt`.
- **Correction:** Apply exactly the formatter-indicated changes in `report.rs`; no behavioral or architectural change.
- **Validation:** Corrected commit `2a0b42b911c185ca31eb265d1fc3a13b2c5ab165`; Actions #449 / run `36883817080` passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED


## ERR-079 — M0.8 stacked PR had no Actions trigger on its base branch

- **Milestone:** M0.8 Block 2 / CI validation infrastructure
- **Type:** CI configuration / validation coverage gap
- **Observed:** PR #18 target `m08-block1-report-contract` produced no associated Rust CI run for commit `9c0a5b2b436487677c69a905f8fdf6890c4d2f91`.
- **Problem:** `.github/workflows/rust.yml` did not include the new M0.8 stacked branch in its `pull_request.branches` or `push.branches` trigger lists.
- **Root cause:** The workflow branch matrix was frozen at the M0.7 branch set before the M0.8 stacked productization branches were created.
- **Correction:** Add `m08-block1-report-contract` and `m08-block2-cli-output-contract` to the existing push/PR branch lists without changing the Build → Test → Format → Clippy job graph.
- **Scope note:** This is validation infrastructure only; it adds no product behavior and does not alter the CLI/report contract.
- **Validation:** Actions #466 / run 36885586680 on corrected commit c5411b59286617b83ccd8079dce4469c3c64a25a passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED


## ERR-080 — CLI stored a validated artifact path as an optional field

- **Milestone:** M0.8 Block 2 / CLI output contract
- **Type:** Build failure / type mismatch
- **Observed in:** Actions #457 / run 36885100911, Build step
- **Commit:** 629e5a42cba7facf24c1df2449f861b863a238d9
- **Problem:** `CliOptions::artifact_path` remained `Option<PathBuf>` even after argument parsing had guaranteed an artifact path, causing five compile errors when passing it to `doctor-core` functions requiring `AsRef<Path>` and when constructing `CliOptions`.
- **Root cause:** The new parser retained the pre-existing optional storage shape after moving the missing-artifact validation to the parser result boundary.
- **Correction:** Store the validated artifact path as `PathBuf` in `CliOptions`; no runtime behavior or contract semantics change.
- **Validation:** Actions #466 / run 36885586680 on corrected commit c5411b59286617b83ccd8079dce4469c3c64a25a passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED


## ERR-081 — CLI implementation failed rustfmt gate

- **Milestone:** M0.8 Block 2 / CLI output contract
- **Type:** CI formatting failure
- **Observed in:** Actions #461 / run 36885388868, Format step
- **Commit:** 39cf70a123155eedebcc06e7ef47fbd2c2f3aca8
- **Problem:** `cargo fmt --all -- --check` reported formatter differences in `crates/doctor-cli/src/main.rs` and `crates/doctor-cli/tests/cli.rs`.
- **Root cause:** The previous ERR-080 correction was applied without running the repository formatter.
- **Correction:** Apply only the rustfmt-indicated import and match-arm formatting.
- **Validation:** Actions #466 / run 36885586680 on corrected commit c5411b59286617b83ccd8079dce4469c3c64a25a passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED


## Process Observation — recurrent stacked-CI trigger coverage gaps

- **Scope:** M0.6–M0.8 historical correction chains
- **Type:** Process observation / preventive improvement candidate
- **Observation:** The ledger contains multiple independent CI trigger-coverage corrections for stacked branches (including ERR-079 in M0.8). These are not duplicate errors and must remain as separate historical records.
- **Pattern:** A newly created stacked branch/PR can inherit a workflow whose push or pull-request branch filters do not yet cover the new branch/base target, preventing the expected validation run.
- **Action:** Keep the historical ERR entries unchanged. Before future stacked-block implementation, explicitly verify push coverage on the head branch and pull-request target coverage on the validated base workflow during the pre-audit, before implementation.
- **Status:** Recorded as a preventive process observation; no production or workflow change made by this observation.


## ERR-082 — M0.8 Block 3 action metadata syntax failure

- **Milestone:** M0.8 Block 3 / GitHub Action
- **Type:** CI validation / action metadata parse failure
- **Observed in:** Actions #10 / run 36887334695, job `action`
- **Head:** `38713f931de66e8848358e913fb0bd3055fa1827`
- **Problem:** GitHub failed during action metadata loading before job setup. `.github/actions/android-release-doctor/action.yml` was rejected with `While scanning a simple key, could not find expected ':'` at line 134.
- **Root cause:** The bounded CR/LF validation insertion was malformed during file generation and introduced a literal fragment into the YAML/run block, making the action manifest syntactically invalid.
- **Correction:** Replaced only `.github/actions/android-release-doctor/action.yml` with the intended composite-action metadata and Bash validation, preserving the existing input/output contract and repository Cargo path.
- **Correction commit:** `38f33f261bc5300f4d1b05afde627b1f54f49030`
- **Validation:** A fresh Actions run is required. No product/audit/CLI behavior was changed by the correction.
- **Validation:** Actions #18 / run `36887625607` passed the Block 3 action self-test, including success, blocker, and operational-error paths. Rust CI run #489 / `36887626517` passed Build, Test, Format, and Clippy on the corrected branch.
- **Status:** RESOLVED


## ERR-083 — M0.8 Block 3 action command expression malformed after ERR-082 correction

- **Milestone:** M0.8 Block 3 / GitHub Action correction chain
- **Type:** CI validation / action manifest expression parse failure
- **Observed in:** Actions #12 / run 36887450382, job `action`
- **Head:** `38f33f261bc5300f4d1b05afde627b1f54f49030`
- **Problem:** GitHub rejected `.github/actions/android-release-doctor/action.yml` because the Bash command invocation contained an accidental unclosed GitHub expression: `"${{CMDVAR${{"`.
- **Root cause:** The corrective file-generation substitution converted the internal Bash array expansion `"${cmd[@]}"` into a malformed GitHub expression.
- **Correction:** Replace only the malformed command invocation with the intended Bash array expansion `"${cmd[@]}"`.
- **Correction commit:** `f56eb1448264ef3ce09ccb07b436bceed02388a0`
- **Validation:** Actions #18 / run `36887625607` passed the Block 3 action self-test, including success, blocker, and operational-error paths. Rust CI run #489 / `36887626517` passed Build, Test, Format, and Clippy on the corrected branch.
- **Status:** RESOLVED


## ERR-084 — M0.8 Block 4 `--locked` rejected stale Cargo.lock

- **Milestone:** M0.8 Block 4 / Distribution and Release Packaging
- **Type:** CI validation / reproducibility gate
- **Observed in:** Block 4 validation runs `36888781506` and `36888775173`, job `validate`
- **Head:** `2bc955d5a11ca7e15d3cecf05793af7ab8da31a5`
- **Problem:** `cargo test --workspace --locked` attempted to update `Cargo.lock` and exited with code 101 before the package matrix could start.
- **Evidence:** CI reported that `Cargo.lock` was not synchronized with the workspace dependency graph.
- **Root cause:** `crates/doctor-core/Cargo.toml` declares direct dependencies on `serde` and `serde_json`, but the historical lockfile did not include those dependencies in the `doctor-core` package entry or their registry package records. Earlier CI did not expose this because the Rust workflow did not use `--locked`.
- **Correction:** Updated only `Cargo.lock`:
  - added `serde` and `serde_json` to `doctor-core` lock dependencies;
  - added registry records for `itoa 1.0.15`, `serde_json 1.0.151`, and `zmij 1.0.23` with their verified registry checksums;
  - left `Cargo.toml` and product code unchanged.
- **Correction commit:** `56dcf8e7ab978288e535585aacf9c852b5109d2c`
- **Further correction:** The resolver diagnosis showed the historical lockfile also required `itoa 1.0.18`, `lazy_static 1.5.1`, and the `serde_derive` edge. The exact lockfile reconciliation was applied in the later corrected branch state.
- **Validation:** Block 4 distribution run #34 / `36890483899`: validate PASS; Linux x86_64 package PASS; Windows x86_64 package PASS; macOS x86_64 package PASS. Rust CI #533 / `36890483719`: Build PASS; Test PASS; Format PASS; Clippy PASS.
- **Status:** RESOLVED


## ERR-085 — M0.8 Block 4 macOS archive verification was locale-dependent

- **Milestone:** M0.8 Block 4 / Distribution and Release Packaging
- **Type:** CI validation / cross-platform packaging verification
- **Observed in:** Block 4 run `36889294750`, job `package (macos-x86_64, macos-15-intel, tar.gz)`
- **Head:** `c2170b3bb6a96286bc7621cd9575ad932b25a2f7`
- **Problem:** The macOS package was built successfully and its binary passed `--version` and `--help`, but the exact archive-content comparison exited with code 1.
- **Evidence:** The failing step was the non-Windows verification command using `tar -tzf "$archive" | sort`. Linux passed the identical check; the failure was isolated to macOS verification.
- **Root cause:** The archive-content comparison relied on the runner's locale-dependent `sort` ordering. The package contract is platform-independent, so verification must use a deterministic byte ordering.
- **Correction:** Change the archive verification pipeline to `LC_ALL=C sort`. This affects only deterministic verification ordering; archive contents and package structure remain unchanged.
- **Correction commit:** `28c82e3c1b6c022548da0747ae1e85176b06f16a`
- **Validation:** Block 4 distribution run #34 / `36890483899`: validate PASS; Linux x86_64 package PASS; Windows x86_64 package PASS; macOS x86_64 package PASS. Rust CI #533 / `36890483719`: Build PASS; Test PASS; Format PASS; Clippy PASS.
- **Status:** RESOLVED


## ERR-086 — M0.8 Block 4 Windows ZIP verification was order-dependent

- **Milestone:** M0.8 Block 4 / Distribution and Release Packaging
- **Type:** CI validation / cross-platform packaging verification
- **Observed in:** Block 4 corrected run `36889890808`, job `package (windows-x86_64, windows-2025, zip)`
- **Head:** `d4a4bb26cbd6f5eaae013a7133eff84ae1e8a95f`
- **Problem:** Windows built the release binary and created the ZIP successfully, but the archive-content verification failed on the ordered string comparison.
- **Root cause:** The verification depended on `Sort-Object` ordering for filenames. Package correctness does not depend on filename order, and locale-aware ordering can vary across environments.
- **Correction:** Replace ordered string comparison with an order-independent, case-sensitive set comparison using PowerShell arrays and `-cnotcontains`. The expected payload remains exactly `LICENSE`, `README.md`, and `android-release-doctor.exe`.
- **Correction commit:** `e440b1e1e00d9f163e5373a331e35ad89448b183`
- **Validation:** Block 4 distribution run #34 / `36890483899`: validate PASS; Linux x86_64 package PASS; Windows x86_64 package PASS; macOS x86_64 package PASS. Rust CI #533 / `36890483719`: Build PASS; Test PASS; Format PASS; Clippy PASS.
- **Status:** RESOLVED


## ERR-087 — M0.9 Block 1 macOS checksum command was not portable

- **Milestone:** M0.9 Block 1 / Public Release & Onboarding
- **Type:** Documentation defect
- **Observed during:** second implementation audit of the public-release onboarding changes
- **Problem:** `README.md` initially documented `sha256sum -c SHA256SUMS` for both Linux and macOS. Stock macOS provides `shasum` rather than the GNU `sha256sum` command by default.
- **Root cause:** The onboarding checksum instructions were written from the Linux verification path used by the release workflow without accounting for the native macOS command surface.
- **Correction:** Separate the verification instructions by host and use `shasum -a 256 -c SHA256SUMS` for macOS.
- **Validation:** Re-read the corrected onboarding section and verify the command against the documented x86_64 macOS distribution path; full repository CI remains required for the implementation checkpoint.
- **Status:** RESOLVED — Rust CI #547 / run `36898715401` passed Build, Test, Format, and Clippy.


## ERR-088 — M0.9 Block 1 branch missing from Rust CI push coverage

- **Milestone:** M0.9 Block 1 / Public Release & Onboarding
- **Type:** CI configuration / validation coverage gap
- **Observed:** No Rust CI workflow run was associated with the first M0.9 implementation head `2ab57c2c653a394505be564c70921955515cc114`.
- **Problem:** The validated `.github/workflows/rust.yml` on the M0.8 Block 4 base did not include the new M0.9 implementation branch in `push.branches`.
- **Root cause:** The recurring stacked-branch trigger coverage pattern documented in the ledger was correctly identified during pre-audit but the new head branch had not yet been added to the active push matrix before implementation validation.
- **Correction:** Add `m09-block1-public-release-onboarding` to the existing Rust CI `push.branches` list without changing the Build → Test → Format → Clippy job graph.
- **Scope:** Validation infrastructure only; no product behavior, CLI behavior, Action behavior, or audit logic changes.
- **Validation:** A new push from the corrected branch must produce terminal Rust CI Build/Test/Format/Clippy evidence before this entry is marked resolved.
- **Status:** RESOLVED — Rust CI #547 / run `36898715401` passed Build, Test, Format, and Clippy.


## ERR-089 — M0.9 public documentation integrity

- **Milestone:** M0.9 public release onboarding / documentation integrity
- **Type:** Documentation / public traceability and boundary defect
- **Observed:** The public M0.9 checkpoint contained references to documentation and development context that were not required for the published release surface.
- **Problem:** The checkpoint was not fully self-contained as public product/release documentation and exposed unnecessary process context.
- **Root cause:** The checkpoint retained development-process wording beyond the information needed to describe the validated public release state.
- **Correction:** Remove the unnecessary process-context references and replace the audit wording with a self-contained statement of the completed second-audit result. No product, CLI, Action, test, or workflow behavior changes.
- **Scope:** Public documentation only. No product, CLI, Action, test, or release behavior changed.
- **Validation:** Second audit of the corrected public documentation, public-surface review, and terminal Rust CI on the corrected commit.
- **Status:** RESOLVED — corrected public documentation reviewed and terminal Rust CI run `36923956731` on corrected commit `95e1a7bafe62eaa56bb9c94f77711021ed06152d` passed Build, Test, Format, and Clippy.


## ERR-091 — AUDIT-002 public documentation boundary

- **Milestone:** AUDIT-002 / public information boundary
- **Type:** Documentation / public-boundary defect
- **Observed:** Two public documentation surfaces contained development-process references that were not required for the public product and release surface.
- **Problem:** The published documentation exposed unnecessary internal process context.
- **Root cause:** Historical checkpoint/ledger wording exceeded the information required to describe the validated public state.
- **Correction:** Remove the unnecessary references and retain only self-contained public release and validation information.
- **Scope:** Public documentation only. No product, CLI, Action, test, workflow, or release behavior changed.
- **Validation:** Exact correction diff reviewed; public-boundary second audit completed; terminal Rust CI passed Build, Test, Format, and Clippy on the correction commit.
- **Status:** RESOLVED — correction commit `e140d7242e8fadc003774c0e7425ee75e4a66bfb` merged to `main` by PR #5 as `b22616a92c406448a146876292f6bfa1fc3cfca3`. CI run `36941326086` completed successfully.


## ERR-092 — M0.9 AUDIT-005 publication job lacked repository context

- **Milestone:** M0.9 / AUDIT-005 real release publication
- **Type:** CI publication workflow defect
- **Observed in:** Distribution run `36945562430`, rerun attempt `2`, job `publish`, step `Publish GitHub Release`
- **Head:** `c3a4095838e3030ebe945d91bf417dc5aa84cf8c`
- **Problem:** The release command `gh release create` failed with `fatal: not a git repository` because the artifact-only `publish` job did not establish local Git repository context and did not explicitly identify the repository.
- **Root cause:** The publication command relied on implicit repository context that is unavailable in the `publish` job after artifact download.
- **Evidence distinction:** The checksum failure in attempt `1` was not reproduced in the controlled rerun and is therefore not attributed to this root cause.
- **Correction:** Pass the workflow-provided repository identifier explicitly to `gh release create` using `--repo "$GITHUB_REPOSITORY"`. No checkout, publication-guard change, package-format change, checksum-algorithm change, permission change, platform-matrix change, or version-contract change.
- **Reference:** https://cli.github.com/manual/gh_release_create
- **Correction branch:** `fix/audit-005-publish-repo-context`
- **Correction commit:** `a3f9ffd1f0c2124415133d7c19004b15d065c1ae`
- **Validation:** Second audit `docs/AUDIT-005-SECOND-AUDIT.md` passed; PR #12 CI run `36948346937` passed Build, Test, Format, and Clippy; PR #12 merged as `4d14ae288882667f23fdd241d26481e215163318`; main CI run `36948475354` passed Build, Test, Format, and Clippy.
- **Status:** RESOLVED


## ERR-093 — Nondeterministic SHA256SUMS self-inclusion in publication checksum generation

- **Milestone:** M0.9 / AUDIT-005 post-closure corrective audit
- **Type:** CI publication workflow defect
- **Observed in:** Distribution run `36945562430`, rerun attempt `2`, job `publish`, step `Generate and verify checksums`
- **Observed head:** `c3a4095838e3030ebe945d91bf417dc5aa84cf8c`
- **Problem:** The checksum command creates `SHA256SUMS` through shell redirection while concurrently enumerating regular files with `find`. The output file can therefore be included in its own checksum input. In the observed execution, the three package checksums reported `OK`, while `SHA256SUMS: FAILED` caused the step to exit with code 1.
- **Root cause:** The checksum input set is not explicitly bounded to the package files and races with creation of the checksum output file. The behavior is nondeterministic: the final publication run `36949141163` succeeded, while the later rerun of the older commit reproduced the failure.
- **Relation to prior closure:** The prior AUDIT-005 closure recorded the original mismatch as a non-reproduced anomaly. The later rerun provides additional evidence that the mismatch is reproducible under the affected workflow, so this entry records a distinct post-closure workflow defect without rewriting the historical closure entry.
- **Correction:** Exclude `SHA256SUMS` explicitly from the `find` input set. Preserve the existing three-package count, SHA-256 algorithm, verification command, and filename assertions.
- **Correction commit:** `4c849b2a0e007270b32d53c4bd64a17da171ab5e`
- **Validation:** Pending PR CI and post-correction distribution validation.
- **Status:** OPEN


## ERR-094 — AAB manifest parsing compatibility gap

- **Milestone:** Post-release real-world artifact validation
- **Type:** Artifact parsing / Android App Bundle compatibility
- **Observed:** A real AAB audit reached manifest evaluation but the manifest parser reported an invalid Android binary XML root.
- **Problem:** The previous AAB path passed the protobuf manifest representation through the APK-specific Android Binary XML parser.
- **Impact:** Manifest-derived evidence such as target SDK was unavailable for affected AAB audits and could cascade into downstream Play-readiness findings.
- **Correction:** Added a focused protobuf manifest decoder for the AAPT2 `XmlNode`/`XmlElement`/`XmlAttribute` representation used by AAB manifests. APK manifests continue to use the existing binary-XML parser unchanged. The decoder extracts only the existing `ManifestInfo` fields and component evidence required by the current audit.
- **Validation:** Public Rust CI run `36960802366` and final post-merge Rust CI run `36961337709` both completed Build, Test, Format, and Clippy successfully. Focused parser tests, the generated-protobuf AAB regression, and existing APK manifest tests pass.
- **Second audit:** Exact implementation diff remained limited to `crates/doctor-core/src/lib.rs`, new `crates/doctor-core/src/manifest_proto.rs`, and the focused AAB fixture updates in `crates/doctor-core/tests/fixtures.rs`. No Play, project, signing, or APK AXML logic was changed.
- **Final real-world validation:** A real-world AAB regression check confirmed the corrected parser exposes the application manifest data, including targetSdk, with no `MANIFEST-002` finding.
- **Status:** RESOLVED
- **Roadmap:** docs/ROADMAP-ERR-094-096.md

## ERR-095 — APK v2 signature false negative

- **Milestone:** Post-release real-world artifact validation
- **Type:** Cryptographic verification / compatibility defect
- **Observed:** A real APK audit reported SIGNING-003 with `v2 signed data contains trailing bytes`.
- **Independent evidence:** The same APK was accepted by the Android SDK apksigner verifier for APK Signature Scheme v2 and v3.
- **Root cause:** Byte-level reproduction demonstrated a concrete compatible v2 `signedData` structure in which the three known length-prefixed fields are followed by exactly one additional empty length-prefixed element encoded as `00 00 00 00`. Those four bytes are part of the signed `signedData` and are accepted by the Android signing/reference-verifier path. ADR currently requires complete consumption immediately after the three known fields and therefore reports the valid structure as trailing bytes.
- **Correction boundary:** Accept only that exact fourth empty element after the three known fields, then require complete consumption. Preserve strict rejection of short residuals, non-empty fourth elements, multiple residual elements, malformed lengths, and arbitrary trailing bytes. Preserve cryptographic verification over the complete `signedData`.
- **Out of scope:** v3/v3.1/v3.2/v4 behavior, certificate/public-key binding, content digest verification, SIGNING-001/002 semantics, and ERR-096.
- **Implementation:** Applied on the ERR-095 branch in `crates/doctor-core/src/signature_verify.rs`; focused parser and integration regressions added. Final validation remains pending.
- **Regression target:** Conventional three-field v2 data remains Verified; the compatible fourth-empty-element structure becomes Verified; malformed/truncated/non-empty/multiple residual variants remain Invalid.
- **Public-boundary note:** The real-world artifact and private evidence remain private. The public regression will encode the demonstrated structural contract rather than publish the external artifact.
- **Status:** OPEN — root cause demonstrated; correction delimited.
- **Delimitation:** docs/AUDIT-ERR-095-DELIMITATION.md
- **Roadmap:** docs/ROADMAP-ERR-094-096.md

## ERR-097 — ERR-095 focused regression tests used `expect_err` on a non-Debug result type

- **Milestone:** ERR-095 implementation / Actions run #371 / run `37013840670`
- **Type:** Test-only validation failure
- **Problem:** Three new ERR-095 parser-negative tests used `Result::expect_err`. Rust requires the `Ok` type to implement `Debug` for that method, but `ParsedSignedData` intentionally does not implement `Debug`.
- **Evidence:** Build passed; the Test step failed to compile `doctor-core` with E0277 at the three new assertions. Format and Clippy were not executed because Test stopped the workflow.
- **Correction:** Replace only those three `expect_err` assertions with explicit `match` expressions that extract the expected `Err` without adding a production `Debug` implementation or changing parser behavior.
- **Scope:** Test code only. No production behavior, fixture bytes, workflow, or public API changes.
- **Validation:** Required in the next Actions execution.
- **Status:** CORRECTION APPLIED — pending validation

## ERR-098 — ERR-095 public compatibility fixture omitted the signer length prefix

- **Milestone:** ERR-095 implementation / Actions run #74 / run `37014155122`
- **Type:** Test fixture structural defect
- **Problem:** The first public ERR-095 APK fixture encoded the v2 signers sequence as a length-prefixed signer body without the required length prefix around the signer itself. The fixture therefore did not represent a valid v2 signer structure.
- **Evidence:** Build passed and all 92 `doctor-core` unit tests passed, but the integration fixture failed with `v2 signer contains trailing bytes; v2 signatures is truncated; v2 signed data exceeds its containing structure`. Byte-level inspection identified the missing signer length prefix.
- **Correction:** Replace only the public fixture bytes with the correctly nested v2 structure: signedData + signatures + publicKey inside the signer, the signer length-prefixed inside the signers sequence, and the signers sequence length-prefixed in the v2 block. The corrected fixture contains exactly one empty fourth signed-data element (`00 00 00 00`) and has SHA-256 `596db1d5efe23cf8a5227e430123595464c319f673cfc37f275f04c09efabdc5`.
- **Scope:** Test fixture bytes and checksum documentation only. No production behavior or workflow changes.
- **Validation:** Required in the next Actions execution.
- **Status:** CORRECTION APPLIED — pending validation

## ERR-096 — Android application plugin alias discovery gap

- **Milestone:** Post-release real-world artifact validation
- **Type:** Project parser / Kotlin DSL compatibility
- **Observed:** A real Android application project using build.gradle.kts and a version-catalog plugin alias was not recognized by project discovery.
- **Problem:** Current discovery relies on finding the literal com.android.application in the module build file and therefore misses supported Kotlin DSL/version-catalog alias usage.
- **Correction:** Pending focused audit of safe application-plugin detection for direct plugin IDs and version-catalog aliases, without introducing broad text heuristics.
- **Validation target:** Representative Groovy/Kotlin DSL direct-ID fixtures, version-catalog alias fixtures, and negative cases that must remain non-application projects.
- **Status:** OPEN
- **Roadmap:** docs/ROADMAP-ERR-094-096.md
