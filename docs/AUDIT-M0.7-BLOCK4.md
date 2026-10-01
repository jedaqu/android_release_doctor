# AUDIT M0.7 Block 4 — Integration and completeness

**Date:** 2026-10-01  
**Branch:** `m07-block4-integration-completeness`  
**Base checkpoint:** `834e86503311f346e7d5b764286687be83aa0b5f`  
**Scope:** Pre-change milestone-wide audit. No Block 4 production correction is included in this document.

## 1. Audit purpose

M0.7 Block 4 is the formal integration and completeness checkpoint defined in `docs/M0.7-DEFINITION.md`.

This block does not authorize a new APK signature algorithm, v3.2/PQC implementation, AAB cryptographic verification, or unrelated refactoring.

The objective is to verify that the capabilities delivered by M0.7 Blocks 1–3 are actually integrated into the release audit/reporting surface, remain aligned with the declared Android specification boundaries, preserve the complete incremental ledger, and have complete stacked CI coverage before the M0.7 final checkpoint.

## 2. Ledger-first review

The complete incremental ledger through ERR-068 was reviewed before this audit.

The Block 3 chain ends with:

- ERR-060 — dynamic parser label build failure;
- ERR-061 — old parser test call sites;
- ERR-062 — over-constrained v3.1 minimum SDK;
- ERR-063 — SDK-32 boundary handling;
- ERR-064 — v3.1 lineage bridge requirement;
- ERR-065 — malformed-lineage fixture classification;
- ERR-066 — maxSDK expectation width;
- ERR-067 — rustfmt correction;
- ERR-068 — final Block 3 correction-chain closure.

Historical entries remain unchanged and chronological.

## 3. Milestone capability inventory

The validated M0.7 implementation currently contains:

### Block 1
- v2/v3 algorithm coverage extension for ECDSA/SHA-512 P-384 (`0x0202`);
- real signature verification;
- explicit Unsupported handling for non-P-384 `0x0202`;
- preservation of existing supported RSA and ECDSA/SHA-256 paths.

### Block 2
- structured proof-of-rotation capability flags;
- explicit known and unknown/reserved flag evidence;
- preservation of lineage verification semantics.

### Block 3
- v3.1 block detection and separate verification;
- reuse of the existing supported cryptographic verification path;
- v3.1 signer SDK-range evidence;
- rotation-min-SDK stripping protection;
- v3/v3.1 targeted-range checks;
- v3 base-block dependency;
- signer-count compatibility;
- lineage consistency/bridge evidence;
- explicit v3.2/PQC boundary.

## 4. Findings

### AUDIT-M0.7-B4-001 — top-level SIGNING-003 does not consume verified v3.1 evidence

`crates/doctor-core/src/signature_verify.rs` now verifies and stores `ApkSignatureVerification.v31`.

However, `crates/doctor-core/src/lib.rs` still builds the `SIGNING-003` scheme list from only v2 and v3. It then treats `verification.v31_present` as an unconditional reason for manual review and states that the v3.1 block is “not cryptographically verified”.

This is stale integration logic left over from the pre-Block-3 boundary.

**Impact:** the core verifier and the user-facing audit report disagree about the actual M0.7 capability. A valid v3.1 APK can be cryptographically verified internally and still be surfaced as manual review by the main audit.

**Required correction:** include v3.1 in the `SIGNING-003` evaluated schemes, preserve Invalid/Unsupported handling, keep v3.2 as an explicit manual-review boundary, and update the corresponding remediation/report wording.

### AUDIT-M0.7-B4-002 — top-level SIGNING-003 regression coverage does not exercise the v3.1 integration state

The existing `lib.rs` test suite covers unavailable signing verification, but it does not exercise a populated `ApkSignatureVerification` containing a Verified v3.1 result.

**Impact:** the Block-3 capability could regress at the audit-report layer without being caught by the current top-level finding tests.

**Required correction:** add focused tests for:
- verified v3.1 being accepted by `SIGNING-003`;
- invalid v3.1 remaining a blocker;
- unsupported v3.1 remaining manual review;
- v3.2 remaining manual review.

No broad report redesign is authorized.

### AUDIT-M0.7-B4-003 — Block 4 stacked CI trigger coverage is not yet configured

The current workflow inherited from Block 3 has Block-3 push coverage and Block-2 PR coverage, but no Block-4 push branch and no Block-4 PR target.

**Impact:** the Block-4 branch cannot yet demonstrate the required stacked validation path.

**Required correction:** extend the active workflow only with the Block-4 branch and Block-3 PR target. Preserve the existing Build → Test → Format → Clippy job graph.

The validated Block-3 base workflow must also be updated to recognize Block-4 as its PR target, following the established stacked-branch pattern.

### AUDIT-M0.7-B4-004 — milestone capability documentation is not yet closed at M0.7 level

README capability history currently ends with Block 3. There is no final M0.7 integration/completeness section, and there is no final milestone audit/checkpoint document.

**Impact:** repository status does not yet explicitly establish the M0.7 completion boundary or the consolidated supported/unsupported capability matrix.

**Required correction:** update README with the final M0.7 milestone status and a concise consolidated boundary summary; create final Block-4 audit/checkpoint documents.

### AUDIT-M0.7-B4-005 — supported/unsupported cryptographic boundary should be consolidated

The implementation currently supports the following signature algorithm IDs in the v2/v3/v3.1 verification path:

- 0x0101 — RSA-PSS/SHA-256;
- 0x0102 — RSA-PSS/SHA-512;
- 0x0103 — RSA-PKCS1-v1_5/SHA-256;
- 0x0104 — RSA-PKCS1-v1_5/SHA-512;
- 0x0201 — ECDSA/SHA-256 for the currently supported P-256/P-384 curves;
- 0x0202 — ECDSA/SHA-512 only for P-384.

DSA 0x0301 remains an explicit Unsupported boundary. RSA keys outside the current ring-backed 2048–8192-bit range remain Unsupported. ECDSA/SHA-512 curves other than P-384 remain Unsupported. v3.2/PQC remains outside M0.7.

The milestone documentation should consolidate these facts so the README, audit, and implementation tell the same story.

## 5. Cross-check against current Android specification

The current AOSP documentation confirms:

- v3.1 is a distinct signing-block ID while reusing the v3 signer verification machinery;
- Android 13+ uses v3.1 and older Android versions use the v3 block;
- the v3 block carries the rotation-min-SDK stripping-protection attribute;
- the v3.1 development-release attribute is specific to the v3.1 signer;
- supported Android v2/v3 signature algorithm IDs include RSA-PSS, RSA-PKCS1, ECDSA/SHA-256, ECDSA/SHA-512 and DSA/SHA-256, with key/curve constraints defined separately.

References:

- https://source.android.com/docs/security/features/apksigning/v3
- https://source.android.com/docs/security/features/apksigning/v3-1
- https://android.googlesource.com/platform/tools/apksig/+/master/src/main/java/com/android/apksig/internal/apk/SignatureAlgorithm.java
- https://android.googlesource.com/platform/tools/apksig/+/e5be60383ef2d04462add5299634bcb27f3ac07b/src/main/java/com/android/apksig/internal/apk/v3/V3SchemeConstants.java

The implementation's declared Unsupported boundaries remain intentional and do not need to be expanded by Block 4.

## 6. Scope integrity check

No Block-4 finding currently requires:

- v3.2/PQC;
- AAB cryptographic verification;
- Gradle execution;
- Play policy automation;
- HTML/SARIF;
- permission-risk classification;
- CLI redesign;
- unrelated refactoring.

The only identified production correction is an integration-layer correction in `SIGNING-003`, plus its focused regression tests.

## 7. CI acceptance

The Block 4 validation must demonstrate:

1. Block-4 push workflow execution;
2. Block-4 PR-event coverage against the Block-3 base;
3. Build PASS;
4. Test PASS;
5. Format PASS;
6. Clippy PASS.

Any newly discovered failure receives the next chronological ERR entry.

## 8. Pre-change audit conclusion

**PRE-CHANGE AUDIT: PASS TO PROCEED WITH SCOPED BLOCK 4 CORRECTIONS**

The milestone is functionally complete at the cryptographic core, but the audit/reporting layer and stacked CI closure still require integration work.

The bounded Block 4 correction is therefore:

**integrate verified v3.1 evidence into the top-level audit result, add focused regression coverage, close stacked CI trigger coverage, consolidate README capability boundaries, and produce the final M0.7 audit/checkpoint.**

No new cryptographic algorithm or adjacent capability is authorized by this audit.
