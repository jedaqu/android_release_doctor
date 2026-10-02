# AUDIT — ERR-038-B Pre-Audit / RSA 1024 and 16384

Date: 2026-10-02
Repository: jedaqu/android_release_doctor
Baseline main: 17db749982e27be80e1c98ff31d8f6615991de65
Historical issue: ERR-038
Previous closed increment: ERR-038-A / ECDSA P-521
Audit branch: audit/err-038-b-pre-audit-2026-10-02

## 1. Objective

Determine whether the remaining RSA boundary cells in ERR-038 can be implemented safely as a separate bounded increment without changing the already validated 2048–8192 RSA path.

Target capability:

ERR-038-B — RSA 1024-bit and RSA 16384-bit verification for Android APK v2/v3 signature algorithm IDs 0x0101, 0x0102, 0x0103, and 0x0104.

## 2. Current repository boundary

The current verifier uses ring for all four Android RSA signature IDs.

ring is deliberately bounded by the repository implementation to RSA key sizes 2048–8192 bits. Therefore:

- 1024-bit RSA is currently Unsupported.
- 2048/4096/8192-bit RSA remains supported by the existing ring path.
- 16384-bit RSA is currently Unsupported.

ERR-038-B must not widen the ring boundary itself.

## 3. Android signing semantics

Android APK Signature Scheme v2 defines:

- 0x0101 — RSASSA-PSS, SHA-256 message digest, MGF1-SHA-256, 32-byte salt, trailer 0xbc.
- 0x0102 — RSASSA-PSS, SHA-512 message digest, MGF1-SHA-512, 64-byte salt, trailer 0xbc.
- A 1024-bit RSA modulus is too short to encode a valid RSASSA-PSS/SHA-512 signature with the required 64-byte salt. AOSP therefore does not provide a 1024-bit PSS/SHA-512 fixture.
- 0x0103 — RSASSA-PKCS1-v1_5 with SHA-256.
- 0x0104 — RSASSA-PKCS1-v1_5 with SHA-512.

Android documents RSA key sizes of 1024, 2048, 4096, 8192, and 16384 bits for the matrix.

Authoritative references:

- https://source.android.com/docs/security/features/apksigning/v2
- https://source.android.com/docs/security/features/apksigning/v3
- https://android.googlesource.com/platform/tools/apksig/

## 4. Backend evaluation

### Candidate: RustCrypto rsa 0.9.10

The current stable rsa 0.9.10 release provides public RSA verification support for PKCS#1 v1.5 and PSS, including configurable PSS salt length. It supports SHA-256 and SHA-512 through its sha2 integration.

References:

- https://docs.rs/rsa/0.9.10
- https://docs.rs/rsa/0.9.10/rsa/pss/struct.VerifyingKey.html

The crate's current security status must be recorded explicitly: RustSec RUSTSEC-2023-0071 reports a Marvin timing-side-channel issue, with no patched version currently available. The advisory describes possible private-key recovery through timing information. This project will not expose private-key operations through this dependency; ERR-038-B uses it only for verification against public keys. The dependency is therefore an explicit, bounded engineering tradeoff rather than an unqualified claim of security.

Reference:

- https://rustsec.org/advisories/RUSTSEC-2023-0071.html

### Decision

Use rsa 0.9.10 only for the two currently unsupported RSA key-size boundaries (1024 and 16384), while preserving ring for 2048–8192.

Do not replace the existing RSA backend globally in ERR-038-B.

## 5. Proposed implementation architecture

At the existing verify_signature_bytes() boundary:

1. Parse the X.509 certificate as today.
2. Enforce RSA public-key type as today.
3. Determine RSA key size from the parsed certificate public key.
4. If key size is 1024 or 16384, dispatch to a dedicated RustCrypto RSA public-verification helper.
5. Otherwise retain the current ring path and its 2048–8192 boundary.

The dedicated helper must:

- build an RsaPublicKey from the X.509 modulus and exponent;
- verify algorithm 0x0101 using PSS/SHA-256 with 32-byte salt;
- verify algorithm 0x0102 using PSS/SHA-512 with 64-byte salt;
- verify algorithm 0x0103 using PKCS#1 v1.5/SHA-256;
- verify algorithm 0x0104 using PKCS#1 v1.5/SHA-512;
- verify the signed-data digest with the same SHA-256/SHA-512 inputs already selected by the existing Android signature-algorithm parser;
- return cryptographic failure as Invalid, not Verified;
- preserve the existing Unsupported semantics for RSA key sizes outside 1024/2048/4096/8192/16384.

Certificate/public-key binding remains enforced by the existing verify_certificate_and_public_key() path.

## 6. Acceptance boundary

ERR-038-B may be considered complete only when all of the following pass:

1. 1024-bit RSA verifies successfully for 0x0101.
2. 1024-bit RSA + 0x0102 is explicitly Unsupported because the Android PSS/SHA-512/64-byte-salt encoding cannot fit a 1024-bit modulus.
3. 1024-bit RSA verifies successfully for 0x0103.
4. 1024-bit RSA verifies successfully for 0x0104.
5. 16384-bit RSA verifies successfully for 0x0101.
6. 16384-bit RSA verifies successfully for 0x0102.
7. 16384-bit RSA verifies successfully for 0x0103.
8. 16384-bit RSA verifies successfully for 0x0104.
9. Tampered signatures remain Invalid for both key-size boundaries.
10. PSS signatures with the wrong salt length do not verify.
11. Existing 2048/4096/8192 RSA behavior remains unchanged.
12. RSA sizes outside the documented Android matrix remain Unsupported.
13. Certificate/SPKI binding remains enforced.
14. v2/v3 content digest verification remains unchanged.
15. Deterministic repository-contained regression vectors cover all cryptographically valid target combinations.
16. Build → Test → Format → Clippy pass in GitHub Actions.
17. A second audit confirms the diff does not replace the existing 2048–8192 backend or broaden scope.
18. Post-merge main CI passes.

## 7. Explicit exclusions

ERR-038-B does not include:

- DSA 1024/2048/3072;
- v3.1 cryptographic expansion;
- v3.2/PQC;
- AAB verification;
- changes to proof-of-rotation;
- signer aggregation redesign;
- changes to release/distribution workflows;
- replacing ring for the already-supported RSA range;
- private-key operations or signing functionality;
- unrelated dependency upgrades.

## 8. Security decision boundary

The rsa dependency warning must remain visible in project documentation.

This increment is acceptable only because the dependency is used exclusively for local/offline-style public-key verification and the project does not expose a private-key signing/decryption service through this code path. Any future reuse of this backend for private RSA operations must trigger a separate security review and must not be inferred from ERR-038-B.

## 9. Pre-audit conclusion

ERR-038-B PRE-AUDIT: PASS.

Compatibility correction: the initial generic acceptance wording for 1024-bit 0x0102 has been narrowed to the actual Android/AOSP compatibility boundary; this is not an implementation defect.

The remaining RSA coverage gap is technically correctable through a dedicated public-verification backend while preserving the current validated ring path.

Next bounded implementation:

ERR-038-B — RSA 1024/16384 public verification.

ERR-038-C DSA remains separate and is not authorized by this document.