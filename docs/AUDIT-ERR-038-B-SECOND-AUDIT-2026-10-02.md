# AUDIT — ERR-038-B Second Audit / RSA 1024 and 16384

Date: 2026-10-02
Repository: jedaqu/android_release_doctor
Base main: 1c05a76527f1800940edc8f431a1a9fd56f4cb66
Branch: fix/err-038-b-rsa-1024-16384-2026-10-02

## 1. Scope reviewed

This second audit reviews the final ERR-038-B implementation against the approved pre-audit.

Implemented:

- RSA 1024-bit public verification for 0x0101, 0x0103, and 0x0104.
- RSA 1024-bit 0x0102 explicitly reported as Unsupported because the required PSS/SHA-512/64-byte-salt encoding cannot fit a 1024-bit modulus.
- RSA 16384-bit public verification for 0x0101, 0x0102, 0x0103, and 0x0104.
- RustCrypto rsa 0.9.10 used only for the two previously unsupported RSA key-size boundaries.
- Existing ring verification retained for 2048–8192-bit RSA.

## 2. Cryptographic semantics audit

The extended verifier uses:

- 0x0101: RSA-PSS/SHA-256 with 32-byte salt.
- 0x0102: RSA-PSS/SHA-512 with 64-byte salt.
- 0x0103: RSA-PKCS1-v1.5/SHA-256.
- 0x0104: RSA-PKCS1-v1.5/SHA-512.

The RSA public key is reconstructed from the certificate modulus and exponent and the existing certificate/public-key binding path remains in place.

The existing 2048–8192 ring path was not modified.

## 3. Test-vector audit

1024-bit vectors are deterministic, repository-contained cryptographic test vectors generated for this project.

16384-bit vectors are deterministic signatures generated from the public AOSP RSA-16384 test key material during the bounded fixture-harvest procedure. The private key was used only transiently during fixture generation and was never uploaded as a repository artifact, fixture, or source file.

Four RSA-16384 signature fixtures are now stored under:

- tests/fixtures/err-038-b/rsa-16384/sig-pss-sha256.bin
- tests/fixtures/err-038-b/rsa-16384/sig-pss-sha512.bin
- tests/fixtures/err-038-b/rsa-16384/sig-pkcs1-sha256.bin
- tests/fixtures/err-038-b/rsa-16384/sig-pkcs1-sha512.bin

The temporary fixture-harvest workflow was removed before the technical PR was opened.

## 4. Negative coverage

The test suite includes:

- explicit Unsupported behavior for RSA-1024 + 0x0102;
- tampered RSA-1024 PSS/SHA-256 rejection;
- tampered RSA-16384 PSS/SHA-256 rejection;
- preservation of the existing RSA 2048–8192 ring boundary test.

## 5. Dependency/security boundary

The new dependency is rsa 0.9.10 and is used only for public-key signature verification.

The RustSec advisory RUSTSEC-2023-0071 remains an explicit documented caveat for this dependency. ERR-038-B does not add private-key signing/decryption operations and does not expose a service boundary where private-key timing leakage could be exploited.

Any future reuse of this dependency for private-key operations requires a separate security audit.

## 6. Changed-scope review

The final diff contains only:

- Cargo.toml
- Cargo.lock
- crates/doctor-core/Cargo.toml
- crates/doctor-core/src/signature_verify.rs
- docs/AUDIT-ERR-038-B-PRE-AUDIT-2026-10-02.md
- this second-audit document
- four RSA-16384 signature fixture binaries.

No workflow changes remain in the final diff.

No DSA, v3.1, v3.2, AAB, proof-of-rotation, signer aggregation, release, or private-key behavior was added.

## 7. Acceptance gate

Before merge, GitHub Actions must pass:

Build → Test → Format → Clippy

After merge, main CI must pass the same four gates.

## 8. Second-audit conclusion

ERR-038-B SECOND AUDIT: PASS — pending terminal GitHub Actions validation.

The implementation is bounded to the approved RSA 1024/16384 public-verification capability and does not replace the validated 2048–8192 ring backend.

ERR-038 parent remains active after ERR-038-B because DSA 1024/2048/3072 is deliberately separate.