# AUDIT M0.7 Block 1 — Cryptographic coverage matrix

**Date:** 2026-09-30  
**Branch:** `m07-block1-crypto-coverage`  
**Exact baseline SHA:** `25c9bf07ab6b5747f11b11ecc9e01933ae2ead81`  
**Scope:** Pre-change audit only. No production implementation is included in this commit.

## 1. Audit purpose

M0.7 Block 1 is the cryptographic coverage-matrix increment defined by `docs/M0.7-DEFINITION.md`.

This audit establishes, before changing production code:

- the current verifier coverage;
- the authoritative Android v2/v3 algorithm and key/curve coverage;
- the concrete gaps that are currently reported as `Unsupported`;
- the smallest bounded implementation increment selected for Block 1;
- the regression and fixture requirements that will be used after implementation.

The evidence-first model from M0.6 remains unchanged: parsing an algorithm identifier is not sufficient for a `Verified` result; the implementation must perform real cryptographic verification and enforce the applicable artifact constraints.

## 2. Baseline and ledger review

The first action for this block was review of the incremental engineering ledger through **ERR-036**.

Relevant historical patterns carried forward:

- ERR-005..010: keep verification boundaries explicit and preserve evidence instead of converting uncertainty into ordinary warnings.
- ERR-016..017: isolate signer-local failures and retain evidence already established before the failure.
- ERR-024..029: cryptographic evidence must be structurally represented and preserved through later signer-local failures.
- ERR-035..036: M0.7 scope must be expressed as exact, locally verifiable rules rather than broad assumptions.

No historical entry authorizes changing the evidence model or multi-signer aggregation semantics in this block.

## 3. Current implementation coverage

The current implementation in `crates/doctor-core/src/signature_verify.rs` has this effective signature-algorithm coverage:

| Algorithm ID | Android meaning | Current selection | Current verification coverage |
|---|---|---|---|
| `0x0101` | RSA-PSS + SHA-256 | Supported | Ring RSA verifier, 2048–8192 bit RSA |
| `0x0102` | RSA-PSS + SHA-512 | Supported | Ring RSA verifier, 2048–8192 bit RSA |
| `0x0103` | RSA-PKCS#1 v1.5 + SHA-256 | Supported | Ring RSA verifier, 2048–8192 bit RSA |
| `0x0104` | RSA-PKCS#1 v1.5 + SHA-512 | Supported | Ring RSA verifier, 2048–8192 bit RSA |
| `0x0201` | ECDSA + SHA-256 | Supported | NIST P-256 and P-384 only |
| `0x0202` | ECDSA + SHA-512 | **Not supported** | Explicit `UNSUPPORTED` result |
| `0x0301` | DSA + SHA-256 | **Not supported** | Parsed as a known digest mapping but excluded from supported selection and verification |

The current code also limits RSA verification through:

`ring_rsa_key_size_supported(bits) => 2048..=8192`

and the ECDSA `0x0201` path accepts only the P-256 and P-384 curve OIDs. Other EC curves are returned as `Unsupported`.

The existing integration tests exercise real v2 and v3 cryptographic fixtures and the tampered-artifact `Invalid` path. They do not currently provide a direct `0x0202` verification fixture.

## 4. Authoritative Android coverage

The Android Open Source Project v2 specification lists these APK Signature Scheme signature algorithm IDs:

- `0x0101` RSA-PSS/SHA-256
- `0x0102` RSA-PSS/SHA-512
- `0x0103` RSA-PKCS1-v1_5/SHA-256
- `0x0104` RSA-PKCS1-v1_5/SHA-512
- `0x0201` ECDSA/SHA-256
- `0x0202` ECDSA/SHA-512
- `0x0301` DSA/SHA-256

The same AOSP page states that all of those algorithms are supported by the Android platform. It also documents the supported key/curve families as RSA 1024/2048/4096/8192/16384, EC P-256/P-384/P-521, and DSA 1024/2048/3072.

Source:
https://source.android.com/docs/security/features/apksigning/v2

The AOSP v3 specification explicitly states that v3 uses the same signature algorithm IDs, key sizes, and EC curves as v2, while adding SDK-range and proof-of-rotation information.

Source:
https://source.android.com/docs/security/features/apksigning/v3

The current AOSP `apksig` implementation also defines `0x0202` as ECDSA with SHA-512 and `0x0301` as DSA with SHA-256.

Source:
https://android.googlesource.com/platform/tools/apksig/+/master/src/main/java/com/android/apksig/internal/apk/SignatureAlgorithm.java

The AOSP verification test corpus contains concrete v2 and v3 fixtures for:

- ECDSA/SHA-512 with P-256, P-384 and P-521;
- DSA/SHA-256 with 1024, 2048 and 3072-bit keys;
- RSA-PKCS1/SHA-256 and SHA-512 with 1024-bit and 16384-bit RSA keys.

Sources:
https://android.googlesource.com/platform/tools/apksig/+/refs/heads/main/src/test/resources/com/android/apksig
https://android.googlesource.com/platform/tools/apksig/+/master/src/test/java/com/android/apksig/ApkVerifierTest.java

## 5. Findings

### AUDIT-M0.7-B1-001 — ECDSA/SHA-512 is a real Android-supported verification case

`0x0202` is a defined Android v2/v3 signature algorithm, but the current implementation returns `Unsupported` before attempting cryptographic verification.

This is therefore a concrete M0.7 Block 1 coverage gap, not a parsing-only or documentation-only discrepancy.

**Evidence:** current `supported_signature_algorithm` excludes `0x0202`; `verify_signature_bytes` explicitly returns `UNSUPPORTED` for `0x0202`.

### AUDIT-M0.7-B1-002 — DSA/SHA-256 is recognized but not actually supported

The current code maps `0x0301` to SHA-256 in `signature_algorithm_digest`, but it is excluded from `supported_signature_algorithm` and falls through to an `Unsupported` verification result.

Android defines `0x0301` and AOSP maintains v2/v3 DSA fixtures, so this is a second real coverage gap.

### AUDIT-M0.7-B1-003 — EC and RSA key coverage is narrower than the Android matrix

The current verifier intentionally narrows the Android matrix:

- P-521 is not accepted for ECDSA;
- RSA verification is limited to 2048–8192 bits;
- consequently, Android-supported 1024-bit and 16384-bit RSA cases are outside the current verifier boundary.

These are valid future Block 1 increments, but expanding all of them together would violate the bounded-increment requirement for the first implementation step.

## 6. Selected bounded increment for M0.7 Block 1

The selected implementation increment is:

> **Add real cryptographic verification for signature algorithm `0x0202` (ECDSA with SHA-512) for NIST P-384, for both v2 and v3, while preserving the existing ring-based verification paths for all already-supported algorithms.**

Rationale for this boundary:

1. `0x0202` is explicitly defined by Android.
2. AOSP contains real v2 and v3 P-384/SHA-512 fixtures.
3. The current parser already recognizes P-384 public keys.
4. The increment changes one algorithm/curve coverage cell instead of redesigning the verifier.
5. The unsupported boundary remains explicit for P-256/P-521 `0x0202`, DSA, and the remaining RSA/EC key-size gaps.
6. RustCrypto `p384` exposes ECDSA verification and ASN.1 DER signature support, providing a narrowly isolated implementation backend for this cell.

Backend reference:
https://docs.rs/p384/latest/p384/ecdsa/index.html

This block will **not** replace `ring` globally.

## 7. Implementation constraints

The subsequent implementation must:

- keep `CryptoVerificationState::{Verified, Invalid, Unsupported}` unchanged;
- keep signer isolation and evidence aggregation unchanged;
- perform real ECDSA/SHA-512 verification over the v2/v3 signed-data bytes;
- preserve the existing certificate-to-public-key binding check;
- preserve the existing APK content-digest verification;
- preserve exact digest/signature algorithm list matching;
- return `Invalid` for cryptographic failure or artifact inconsistency;
- return `Unsupported` only for a case that is structurally understood but outside this implementation's declared cryptographic capability;
- avoid fallback behavior that could convert an invalid `0x0202` signature into `Verified`.

## 8. Fixture and regression plan

The implementation step must add deterministic coverage for at least:

1. **Valid v2 / ECDSA-SHA512 / P-384** → `Verified`
2. **Valid v3 / ECDSA-SHA512 / P-384** → `Verified`
3. **Tampered `0x0202` signature** → `Invalid`
4. **Tampered signed data or content digest** → `Invalid`
5. **A declared but out-of-scope ECDSA-SHA512 curve** → `Unsupported`, without falling back to `Verified`
6. **Existing v2/v3 P-256/P-384 SHA-256 fixtures** → unchanged `Verified`
7. **Existing signer-isolation regressions** → unchanged evidence preservation

Where practical, the positive fixtures should be derived from the AOSP test cases or generated deterministically from equivalent signing material. The fixture must be reachable from the working branch itself, following the M0.6 fixture-history correction pattern.

## 9. Explicitly out of scope for this increment

The following remain outside the first implementation increment and must stay explicit in the coverage matrix:

- ECDSA/SHA-512 with P-256;
- ECDSA/SHA-512 with P-521;
- DSA/SHA-256;
- P-521 support for ECDSA/SHA-256;
- RSA 1024-bit expansion;
- RSA 16384-bit expansion;
- v3.1 implementation;
- v3.2/PQC;
- AAB cryptographic verification;
- broad dependency or verifier refactoring.

These are not hidden assumptions. They remain future coverage cells.

## 10. Acceptance semantics

The implementation is acceptable only when:

- the selected v2/v3 P-384 ECDSA-SHA512 cases are verified by real cryptography;
- cryptographic tampering cannot produce `Verified`;
- unsupported curves/algorithms remain explicit `Unsupported`;
- the existing multi-signer evidence behavior is unchanged;
- M0.6 cryptographic regressions remain green;
- no unrelated production code is modified.

## 11. Pre-change audit conclusion

**PRE-CHANGE AUDIT: PASS TO PROCEED WITH THE SCOPED IMPLEMENTATION**

The audit identified concrete, specification-backed coverage gaps and selected one bounded increment: **ECDSA/SHA-512 (`0x0202`) over P-384 for v2/v3**.

No production-code change is included in this audit commit.

The next engineering movement is the scoped implementation, followed by:

**second audit → focused regression validation → Actions → individual corrections if required → new validation → Block 1 checkpoint.**

## 12. Sources

- Android APK Signature Scheme v2:
  https://source.android.com/docs/security/features/apksigning/v2
- Android APK Signature Scheme v3:
  https://source.android.com/docs/security/features/apksigning/v3
- AOSP `SignatureAlgorithm.java`:
  https://android.googlesource.com/platform/tools/apksig/+/master/src/main/java/com/android/apksig/internal/apk/SignatureAlgorithm.java
- AOSP `ApkVerifierTest.java`:
  https://android.googlesource.com/platform/tools/apksig/+/master/src/test/java/com/android/apksig/ApkVerifierTest.java
- AOSP apksig test resources:
  https://android.googlesource.com/platform/tools/apksig/+/refs/heads/main/src/test/resources/com/android/apksig
- RustCrypto P-384 ECDSA:
  https://docs.rs/p384/latest/p384/ecdsa/index.html
- ring 0.17 verification algorithms:
  https://docs.rs/ring/0.17.14/ring/signature/index.html
