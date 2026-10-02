# AUDIT — ERR-038-A Second Audit / ECDSA P-521

Date: 2026-10-02
Repository: jedaqu/android_release_doctor
Base: main at 67a107fbf9fe53711276cdcf7aad056d0fa828cf
Branch: fix/err-038-a-p521-coverage-2026-10-02

## Scope

Second audit of the ERR-038-A implementation after code changes and before PR CI.

The only authorized implementation increment is ECDSA P-521 coverage for APK Signature Scheme v2/v3 algorithm IDs:

- 0x0201 — ECDSA/SHA-256
- 0x0202 — ECDSA/SHA-512

## Changed files

The implementation diff is bounded to:

1. Cargo.toml — add p521 0.14.0 workspace dependency.
2. Cargo.lock — add the exact p521 0.14.0 registry package entry and direct doctor-core dependency edge.
3. crates/doctor-core/Cargo.toml — wire the workspace dependency.
4. crates/doctor-core/src/signature_verify.rs — add P-521 curve detection and verification path plus deterministic regression tests.

A fifth documentation file contains this second-audit record only.

## Implementation audit

### P-521 detection

The verifier now recognizes the NIST P-521 curve OID 1.3.132.0.35.

### 0x0201

For ECDSA/SHA-256:

- P-256 continues through the existing ring backend.
- P-384 continues through the existing ring backend.
- P-521 uses the dedicated RustCrypto P-521 verification path.
- Other curves remain Unsupported.

### 0x0202

For ECDSA/SHA-512:

- P-384 continues through the existing dedicated P-384 path.
- P-521 uses the dedicated P-521 verification path.
- Other curves remain Unsupported.

### Cryptographic semantics

The P-521 path:

- parses the EC public key as a SEC1 point;
- parses the ECDSA signature as ASN.1 DER;
- hashes the exact signed-data bytes with SHA-256 or SHA-512 according to the selected Android algorithm ID;
- invokes P-521 prehash verification;
- reports failed signatures as Invalid.

Certificate/SPKI binding remains unchanged and is not bypassed by the new path.

## Regression correction discovered during second audit

The historical test for unsupported ECDSA/SHA-512 curves changed the P-384 OID byte 0x22 to 0x23.

0x23 is precisely the NIST P-521 OID, so after ERR-038-A that test no longer represented an unsupported curve.

The test was corrected to use 0x21 (P-224) and now verifies that a genuinely unsupported curve remains explicitly Unsupported.

No production behavior was weakened to satisfy the test.

## Tests added

The implementation adds deterministic cryptographic tests for:

- real ECDSA/SHA-256 P-521 verification;
- real ECDSA/SHA-512 P-521 verification;
- tampered ECDSA/SHA-256 P-521 signature rejection;
- preservation of the existing unsupported-curve boundary.

The test certificates and signatures are repository-contained in source as deterministic test vectors.

## Explicitly out of scope

This diff does not implement or alter:

- RSA 1024-bit;
- RSA 16384-bit;
- DSA;
- v3.1 cryptographic completion;
- v3.2/PQC;
- AAB cryptographic verification;
- proof-of-rotation semantics;
- signer aggregation;
- workflow behavior;
- release/distribution behavior.

## Acceptance review

The implementation satisfies the pre-audit acceptance design:

- P-521 is a real cryptographic verification path, not a structural pass.
- Invalid P-521 signatures remain Invalid.
- Unsupported curves remain Unsupported.
- P-256/P-384 paths are preserved.
- Existing aggregation and content-digest code paths are untouched.
- The dependency and lockfile changes are bounded to P-521.
- No unrelated refactoring is present.

## Second-audit conclusion

ERR-038-A SECOND AUDIT: PASS — pending terminal GitHub Actions validation.

Required CI gate:

Build → Test → Format → Clippy

Only after that terminal gate passes is the implementation eligible for merge.

After merge, the post-merge main CI must also pass before ERR-038-A is considered fully closed.

ERR-038 itself remains open until the remaining RSA and DSA capability cells are separately resolved or explicitly dispositioned.