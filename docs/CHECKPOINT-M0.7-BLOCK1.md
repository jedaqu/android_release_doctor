# CHECKPOINT M0.7 Block 1 — Cryptographic coverage

**Branch:** `m07-block1-crypto-coverage`  
**M0.7 baseline:** `25c9bf07ab6b5747f11b11ecc9e01933ae2ead81`  
**Implementation head validated by final second audit:** `e8996c7ffce11be146c1d04b49a0e575f2c3fe1a`  
**Documentation/ledger head before this checkpoint:** `2bb93421df98fe9fdda3c0491b8af02df36dd285`

## 1. Scope delivered

M0.7 Block 1 delivered one bounded cryptographic coverage increment:

> ECDSA with SHA-512 (`0x0202`) over NIST P-384 for APK Signature Scheme v2 and v3.

The implementation performs real signature verification and preserves the existing evidence-first result model.

## 2. Acceptance evidence

Validated:

- v2 `0x0202` / P-384 positive APK fixture → `Verified`;
- v3 `0x0202` / P-384 positive APK fixture → `Verified`;
- tampered v2 `0x0202` fixture → `Invalid`;
- tampered v3 `0x0202` fixture → `Invalid`;
- non-P-384 `0x0202` → explicit `Unsupported`;
- direct deterministic P-384 ECDSA/SHA-512 verification → pass;
- direct tampered ECDSA/SHA-512 verification → `Invalid`;
- existing M0.6 signer isolation/evidence regressions → pass;
- tracked `Cargo.lock` contains the declared `p384` and `sha2` dependency graph.

Final second-audit validation run:

**Actions #308 / 36782554714**
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

## 3. Documentation

- Pre-change audit: `docs/AUDIT-M0.7-BLOCK1.md`
- Second audit: `docs/SECOND-AUDIT-M0.7-BLOCK1.md`
- Incremental ledger: `docs/ERRORS-AND-FIXES.md`

The second audit is explicitly **PASS**.

## 4. Explicit boundaries retained

This checkpoint does not claim support for:

- ECDSA/SHA-512 P-256;
- ECDSA/SHA-512 P-521;
- DSA/SHA-256;
- P-521 ECDSA/SHA-256;
- RSA 1024-bit or 16384-bit expansion;
- v3.1/v3.2;
- AAB cryptographic signing verification.

Those remain future bounded coverage cells.

## 5. CI and repository state

The permanent Rust workflow remains the existing job graph with the Block 1 source branch covered for push validation.

Temporary lockfile-generation workflows used during validation were removed.

No merge is performed.

## 6. Checkpoint rule

This document records the Block 1 checkpoint candidate. The checkpoint becomes **FINALIZADO** only after the validation run for this exact documentation state passes Build, Test, Format, and Clippy.

After that validation, M0.7 Block 1 is formally closed and the next block may begin only from this recorded lineage.
