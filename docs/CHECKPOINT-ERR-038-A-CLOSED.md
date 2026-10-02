# CHECKPOINT — ERR-038-A Closed / ECDSA P-521

Date: 2026-10-02
Repository: jedaqu/android_release_doctor
Validated main commit: 20694b62e8af1797d3d339dedb9986d55860db46
PR: #37

## State

ERR-038-A — ECDSA P-521 coverage: CLOSED.
ERR-038 — Android cryptographic/key coverage: ACTIVE / PENDING.

## What was corrected

The verifier now cryptographically verifies Android APK v2/v3 ECDSA P-521 signatures for:

- 0x0201 — ECDSA with SHA-256
- 0x0202 — ECDSA with SHA-512

The implementation uses the existing certificate/SPKI binding path, SEC1 P-521 public-key parsing, DER ECDSA signatures, and prehashed SHA-256/SHA-512 verification.

Existing P-256/P-384 ECDSA and RSA behavior remains unchanged.

## Validation

PR #37 terminal validation:

- Rust CI #136 / 37047107679: SUCCESS
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

Post-merge main validation:

- Rust CI #137 / 37047269534: SUCCESS
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

## Correction history

ERR-106 recorded the implementation compile corrections encountered during CI.

ERR-107 recorded the rustfmt-only correction.

Both are RESOLVED in docs/ERRORS-AND-FIXES.md.

The correction chain remained bounded to ERR-038-A. No RSA, DSA, v3.1, v3.2, AAB signing verification, proof-of-rotation, signer aggregation, or workflow/release changes were introduced.

## Remaining ERR-038 scope

ERR-038 remains active because the following capability cells were deliberately not included in ERR-038-A:

- RSA 1024-bit
- RSA 16384-bit
- DSA 1024-bit
- DSA 2048-bit
- DSA 3072-bit

These remain separate future increments and require their own security/backend evaluation before implementation.

## Engineering result

ERR-038-A is formally closed with terminal PR CI and terminal post-merge main CI.

The next ERR-038 movement, if authorized, must begin with a new pre-audit and a separately bounded scope. No RSA or DSA implementation is authorized by this checkpoint.

**CHECKPOINT RESULT: ERR-038-A CLOSED — PASS**