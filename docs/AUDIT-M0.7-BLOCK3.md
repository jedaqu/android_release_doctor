# AUDIT M0.7 Block 3 — APK Signature Scheme v3.1 verification

**Date:** 2026-10-01  
**Branch:** `m07-block3-v31-verification`  
**Base checkpoint:** `m07-block2-rotation-semantics`  
**Scope:** Pre-change audit only. No production implementation is included in this document.

## 1. Audit purpose

M0.7 Block 3 is the next bounded increment after the validated M0.7 Block 2 checkpoint.

The objective is to move the existing v3 cryptographic verifier across the explicit v3.1 boundary without weakening the evidence-first model or silently treating v3.1 as ordinary v3.

The current Android documentation states that v3.1 is used on Android 13+ for SDK-targeted key rotation, while older Android releases ignore the v3.1 block and use the v3 block. The v3.1 block reuses the v3 signer structure but has distinct block semantics and requires cross-block validation. citeturn371363view0

## 2. Ledger-first review

The Block 2 checkpoint and the complete incremental ledger were reviewed before this audit.

The latest Block 2 correction chain ends with:

- ERR-050 — fixture helper borrow conflict;
- ERR-051 — Block 2 stacked CI trigger coverage;
- ERR-052 — Block 1 base workflow PR-event coverage;
- ERR-053 — missed `capabilities` field in an existing initializer;
- ERR-054 — incorrect proof-of-rotation fixture offset;
- ERR-055 — rustfmt-only correction;
- ERR-056 — final Block 2 correction-chain validation closure.

Historical entries remain unchanged and chronological.

## 3. Current implementation coverage

The current verifier already contains:

- a v3 signing-block ID;
- a v3.1 signing-block ID;
- a v3.2 signing-block ID retained as an explicit boundary;
- detection booleans for v3.1 and v3.2 presence;
- complete v2 verification;
- complete bounded v3 verification for the currently supported algorithm subset;
- SDK-range evidence;
- multi-signer isolation;
- structured proof-of-rotation evidence;
- ECDSA/SHA-512 P-384 support.

However, `verify_apk_signatures()` currently detects v3.1 presence and then does nothing with the v3.1 block. It verifies only the v2 and v3 blocks.

Therefore an APK containing a v3.1 block currently receives presence evidence but no v3.1 cryptographic verification.

## 4. Authoritative v3.1 boundary

The current AOSP specification establishes the following facts:

1. Android 13 introduced APK Signature Scheme v3.1.
2. The v3.1 block uses a distinct signing-block ID and is consumed by Android 13+.
3. Android 12 and lower ignore the v3.1 block and use the v3 block.
4. The v3.1 block has the same internal signer format as v3 but exists specifically to target key rotation from a selected minimum SDK.
5. A v3.1 block is expected together with the required v3 base block.
6. The v3 signer carries a rotation-min-SDK stripping-protection attribute.
7. The value in that v3 attribute must agree with the minimum SDK targeted by the v3.1 block.
8. The v3.1 and v3 targeted ranges must cover the appropriate SDK intervals without overlapping or leaving unintended holes.
9. v3/v3.1 lineage information must remain mutually consistent.
10. A v3.1 signer may carry a development-release rotation attribute; that attribute has v3.1-specific semantics.

AOSP's current `apksig` verifier exposes explicit issue categories for missing v3 base blocks, rotation-min-SDK mismatch, missing stripping-protection attributes, and v3.1 development-release misuse. citeturn716886view1turn716886view3

The AOSP constants identify the stripping-protection attribute as `0x559f8b02`, with the v3.1 development-release attribute `0xc2a6b3ba`. The current verifier already uses the v3.1 block ID `0x1b93ad61`. citeturn512235search0

## 5. Findings

### AUDIT-M0.7-B3-001 — v3.1 presence is detected but the block is not verified

The current implementation sets `v31_present` from the APK signing block but never parses or cryptographically verifies the v3.1 signer structure.

**Impact:** a v3.1 APK currently receives only presence evidence. Its signer signature, certificate/public-key binding, SDK range, digest and APK content digest are not independently verified.

### AUDIT-M0.7-B3-002 — the existing v3 verifier cannot currently express the v3/v3.1 distinction

`verify_v3_block()` is hard-wired to v3 terminology and does not accept a block ID or scheme identity.

**Impact:** simply invoking it for the v3.1 bytes would verify the basic signer cryptography but would not establish the required v3.1 cross-block semantics.

The correction must therefore parameterize the reusable v3 verification path or create the smallest equivalent bounded v31 path without duplicating cryptographic logic.

### AUDIT-M0.7-B3-003 — rotation-min-SDK stripping protection is not parsed

The v3 signed-data parser currently recognizes proof-of-rotation but discards all other additional attributes.

The required v3.1 protection attribute `0x559f8b02` is therefore invisible to the verifier.

**Impact:** the verifier cannot establish whether a v3.1 block was intentionally protected against stripping or whether the protected rotation target matches the v3.1 range.

### AUDIT-M0.7-B3-004 — v3.1 rotation target is not represented separately from ordinary SDK range

The current `CryptoSchemeInfo.sdk_ranges` records a signer's outer min/max SDK range, but there is no explicit evidence identifying that a v3.1 range is the rotation-target range.

**Impact:** the verifier cannot currently distinguish:

- ordinary v3 platform coverage;
- v3.1 rotation-target coverage;
- cross-block interval relationships.

### AUDIT-M0.7-B3-005 — v3.1 without v3 base block is not rejected

AOSP explicitly treats a v3.1 block without the required v3 base block as an error. citeturn716886view0

The current implementation only exposes `v31_present`; it does not enforce the dependency.

### AUDIT-M0.7-B3-006 — v3 and v3.1 targeted ranges are not checked for coverage/non-overlap

The current implementation can collect v3 ranges, but no cross-block interval validation exists.

AOSP documents the targeted arrangement in which the v3 block covers the lower platform interval and v3.1 covers the rotation-target interval beginning at the selected rotation minimum SDK. citeturn371363view0

### AUDIT-M0.7-B3-007 — v3/v3.1 lineage consistency is not checked

The current implementation independently verifies a v3 proof-of-rotation lineage but has no comparison between the v3 and v3.1 lineage/signers.

The Block 3 correction must compare lineage evidence at the certificate identity level without inventing runtime trust semantics.

### AUDIT-M0.7-B3-008 — deterministic v3.1 fixtures are absent

No active fixture currently exercises a genuine v3.1 signing block.

Existing v3 fixtures must remain unchanged as regressions.

The Block 3 fixture strategy therefore requires deterministic v3.1 evidence covering at least:

1. valid v3+v3.1 rotation-target configuration;
2. valid v3.1 cryptographic signer with supported algorithms;
3. malformed v3.1 structure;
4. invalid v3.1 signature;
5. unsupported v3.1 algorithm/key combination;
6. v3.1 block without v3;
7. rotation-min-SDK mismatch;
8. missing v3 stripping-protection attribute;
9. v3/v3.1 range overlap or unintended gap;
10. v3/v3.1 lineage mismatch.

The fixture method must not weaken cryptographic verification by replacing signatures with synthetic acceptance shortcuts.

## 6. Selected bounded increment

The authorized Block 3 increment is:

> **Implement v3.1 verification for the existing supported cryptographic algorithm subset, then add the deterministic v3↔v3.1 cross-block semantics required by Android.**

The increment must:

- reuse the existing v3 cryptographic verification path;
- support the same currently supported signature algorithms/key combinations unless a v3.1-specific restriction is established;
- parse v3.1 signers with their own scheme identity;
- expose v3.1 presence and verification evidence;
- parse the v3 rotation-min-SDK stripping-protection attribute;
- derive the v3.1 rotation target from its signer range;
- require the v3 base block when v3.1 is present;
- validate rotation-min-SDK consistency;
- validate targeted-range non-overlap and intended coverage;
- validate v3/v3.1 lineage consistency;
- preserve multi-signer isolation and SDK-range evidence;
- preserve proof-of-rotation evidence;
- distinguish malformed, cryptographically invalid, and unsupported cases;
- leave v3.2/PQC outside this block.

## 7. Evidence model boundary

The local verifier may prove:

- whether a v3.1 block exists;
- whether its signer structure is well formed;
- whether its signature cryptographically verifies;
- whether its certificate/public-key binding verifies;
- whether its content digest verifies;
- which SDK range the v3.1 signer targets;
- whether v3 contains the required stripping-protection attribute;
- whether the declared rotation minimum matches the v3.1 targeted range;
- whether v3 and v3.1 ranges have a deterministic structural relationship;
- whether the v3 and v3.1 lineages are cryptographically/structurally consistent.

The verifier must not claim to simulate the runtime behavior of every Android platform version.

## 8. Explicitly out of scope

This block does not implement:

- v3.2;
- PQC;
- v4;
- AAB cryptographic signing verification;
- runtime PackageManager or SigningInfo behavior simulation;
- Play policy automation;
- unrelated signature algorithm expansion;
- broad CLI/reporting redesign;
- unrelated refactoring.

## 9. Regression and fixture acceptance

The final implementation must preserve every existing M0.6 and M0.7 Block 1/2 test.

New Block 3 tests must distinguish:

- valid supported v3.1;
- malformed v3.1;
- invalid v3.1 signature;
- unsupported v3.1 crypto;
- missing v3 base;
- rotation-min-SDK mismatch;
- missing stripping protection;
- range inconsistency;
- lineage inconsistency;
- evidence retention after signer-local failure.

A v3.1 cryptographic failure must never be silently ignored merely because a v3 block exists.

## 10. CI acceptance

Block 3 follows the existing stacked workflow:

**ledger review → pre-change audit → implementation → focused tests → second audit → Actions → individual correction if required → new execution → checkpoint**

The active Block 3 source branch must receive push coverage.

The validated Block 2 base workflow must explicitly accept the Block 3 PR target.

The job graph remains:

**Build → Test → Format → Clippy**

No CI redesign is authorized.

## 11. Pre-change audit conclusion

**PRE-CHANGE AUDIT: PASS TO PROCEED WITH SCOPED IMPLEMENTATION**

The existing code has a precise gap: v3.1 presence is detected but cryptographic verification and cross-block semantics are absent.

The bounded correction is therefore limited to:

**v3.1 supported-crypto verification + required v3/v3.1 rotation-target semantics + deterministic regression evidence.**

The implementation is now authorized on `m07-block3-v31-verification`.

No v3.2/PQC or unrelated cryptographic expansion is authorized by this audit.
