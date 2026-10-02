# PRE-AUDIT — ERR-038 Public Cryptographic Documentation Synchronization

**Date:** 2026-10-02  
**Repository:** `jedaqu/android_release_doctor`  
**Baseline:** `main` at `0070bbcc37d47780159f97f68898e6b4c5e0c3dc`  
**Branch:** `fix/err-038-public-crypto-docs-2026-10-02`

## 1. Objective

Reconcile the public, current-state cryptographic documentation with capabilities that are already implemented and terminally validated in the repository.

This block is documentation-only. It does not add or change cryptographic behavior.

## 2. Evidence reviewed

The audit reviewed:

- current `main` implementation in `crates/doctor-core/src/signature_verify.rs`;
- deterministic P-521 regression tests;
- deterministic RSA-1024 regression tests;
- repository-contained RSA-16384 certificate and signature fixtures;
- `docs/AUDIT-ERR-038-A-SECOND-AUDIT-2026-10-02.md`;
- `docs/CHECKPOINT-ERR-038-A-CLOSED.md`;
- `docs/AUDIT-ERR-038-B-PRE-AUDIT-2026-10-02.md`;
- `docs/AUDIT-ERR-038-B-SECOND-AUDIT-2026-10-02.md`;
- `docs/ERRORS-AND-FIXES.md`;
- current `main` Rust CI terminal result;
- current README and public corpus;
- current Android/AOSP v2, v3 and v3.2 documentation;
- current Google Play App Signing documentation.

## 3. Current implementation evidence

### Implemented and validated

- RSA 0x0101, 0x0103 and 0x0104: 1024-bit public verification.
- RSA 0x0101, 0x0102, 0x0103 and 0x0104: 16384-bit public verification.
- RSA 0x0101–0x0104: existing 2048–8192-bit ring-backed verification remains intact.
- ECDSA 0x0201: P-256, P-384 and P-521.
- ECDSA 0x0202: P-384 and P-521.

ERR-038-A and ERR-038-B both have terminal Build → Test → Format → Clippy validation and terminal post-merge main validation.

### Deliberately unsupported or incomplete

- RSA 1024 + 0x0102 remains explicit Unsupported because the required SHA-512 PSS salt encoding does not fit the 1024-bit modulus boundary.
- DSA 0x0301 is recognized at the algorithm/digest identification layer but is not cryptographically verified.
- ECDSA 0x0202 with P-256 is unsupported.
- v3.2/PQC cryptographic verification is not implemented; the current verifier records v3.2 block presence separately.

## 4. Documentation discrepancy

The README's historical M0.7 capability table is a historical record of that milestone and must not be rewritten.

However, the README also exposes a current capability statement that still describes:

- P-521 as outside ECDSA coverage;
- RSA 1024/16384 as outside the current supported range.

Those statements are now stale because ERR-038-A and ERR-038-B were subsequently implemented and validated.

The required correction is therefore limited to adding a clearly labeled current-state cryptographic matrix and updating the current "Still not implemented" list.

## 5. DSA boundary

Android/AOSP continues to define DSA 0x0301 and DSA key sizes 1024/2048/3072 in the v2/v3 signature-algorithm matrix, while Google Play App Signing's current key requirements use RSA and its quantum-ready option uses a hybrid RSA 4096 + ML-DSA-65 configuration.

DSA therefore remains a legitimate Android compatibility/legacy verification question, but it is not required by the current Play App Signing key model and it introduces a separate backend/security evaluation.

Decision for this documentation block:

**DSA remains cryptographically Unsupported and explicitly deferred.**

No DSA implementation is authorized by this block.

## 6. v3.2 / PQC boundary

Android 17 introduces APK Signature Scheme v3.2 as a hybrid classical/PQC scheme. AOSP documents ML-DSA-65 and ML-DSA-87 as the initial PQC algorithms, with a separate v3.2 block.

This is architecturally distinct from historical DSA 0x0301.

Decision for this documentation block:

**v3.2/PQC remains a separate future capability boundary.**

No PQC implementation is authorized by this block.

## 7. Scope boundary

Included:

- current README cryptographic matrix;
- current README "Still not implemented" wording;
- this pre-audit record;
- a second-audit record after the documentation change;
- a final checkpoint after terminal CI.

Excluded:

- Rust production code;
- cryptographic behavior;
- tests and fixtures;
- workflows;
- changelog history;
- historical M0.7 audit/checkpoint documents;
- DSA implementation;
- v3.2/PQC implementation;
- new ERR entry.

## 8. Acceptance

The documentation block is acceptable only if:

1. the current README describes the actual validated P-521 and RSA 1024/16384 coverage;
2. DSA remains explicitly Unsupported/deferred;
3. v3.2/PQC remains explicitly separate;
4. historical M0.7 documentation remains untouched;
5. no source/test/workflow files are changed;
6. the second audit confirms scope integrity;
7. Build → Test → Format → Clippy pass on the resulting branch/PR;
8. post-merge main CI passes.

## 9. PRE-AUDIT CONCLUSION

**PASS TO PROCEED WITH THE SCOPED DOCUMENTATION CORRECTION**

The evidence is sufficient for one small, non-cryptographic correction: synchronize the current public README with the already implemented and validated ERR-038-A/ERR-038-B capabilities.

## References

- https://source.android.com/docs/security/features/apksigning/v2
- https://source.android.com/docs/security/features/apksigning/v3
- https://source.android.com/docs/security/features/apksigning/v3-2
- https://support.google.com/googleplay/android-developer/answer/9842756
