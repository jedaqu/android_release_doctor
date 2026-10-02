# AUDIT — ERR-038 Pre-Audit / Cryptographic Coverage

**Date:** 2026-10-02  
**Repository:** `jedaqu/android_release_doctor`  
**Baseline:** public `main` at `cb58125dc1ec398b7ecb431cb63d857b0853c43d`  
**Historical ledger entry:** ERR-038  
**Audit branch:** `audit/err-038-pre-audit-2026-10-02`

## 1. Objective

Determine whether ERR-038 can be corrected safely and incrementally, identify its exact remaining cryptographic coverage cells, and freeze a bounded first implementation scope.

This pre-audit does not mark ERR-038 resolved.

## 2. Historical problem statement

ERR-038 records a cryptographic coverage gap remaining after M0.7 Block 1:

> The verifier intentionally narrowed the Android v2/v3 matrix. ECDSA was limited to P-256/P-384 and RSA to 2048–8192-bit keys, leaving P-521 and RSA 1024/16384 outside the current verification boundary.

The historical entry explicitly required future bounded increments rather than broadening the original M0.7 Block 1 scope.

## 3. Current verifier baseline

The current `doctor-core` signature verifier supports:

### Signature algorithm IDs recognized by the current digest/selection layer

- `0x0101` — RSA-PSS / SHA-256
- `0x0102` — RSA-PSS / SHA-512
- `0x0103` — RSA-PKCS1-v1.5 / SHA-256
- `0x0104` — RSA-PKCS1-v1.5 / SHA-512
- `0x0201` — ECDSA / SHA-256
- `0x0202` — ECDSA / SHA-512

The verifier does not currently implement `0x0301` DSA cryptographic verification even though the digest/algorithm table recognizes the ID for format/selection purposes.

### Current EC coverage

- P-256: supported for `0x0201`
- P-384: supported for `0x0201`
- P-384: supported for `0x0202`
- P-521: unsupported

### Current RSA coverage

The current `ring` backend is explicitly bounded to 2048–8192-bit RSA keys for all four supported RSA algorithm IDs.

Therefore:

- 1024-bit RSA: unsupported
- 2048-bit RSA: supported
- 4096-bit RSA: supported
- 8192-bit RSA: supported
- 16384-bit RSA: unsupported

### Current DSA coverage

- `0x0301`: not cryptographically implemented.

### Separate capability boundaries

v3.1 and v3.2 remain separate capability areas. Their block IDs are recognized structurally, but they are not part of ERR-038's historical definition and must not be silently folded into this correction.

## 4. Android reference matrix

The Android v2 documentation defines these signature algorithm IDs and key/curve sets:

- RSA: 1024, 2048, 4096, 8192, 16384
- EC: P-256, P-384, P-521
- DSA: 1024, 2048, 3072

The v3 documentation states that v3 uses the same signature algorithm IDs, key sizes, and EC curves as v2.

Authoritative references:

- https://source.android.com/docs/security/features/apksigning/v2
- https://source.android.com/docs/security/features/apksigning/v3
- https://android.googlesource.com/platform/tools/apksig/

## 5. Capability gap decomposition

ERR-038 should be treated as three independent bounded capability increments:

### ERR-038-A — ECDSA P-521

Target cells:

- `0x0201` + P-521
- `0x0202` + P-521

Existing architecture already contains a dedicated RustCrypto path for P-384 under `0x0202`. The `p521` crate currently provides ECDSA/P-521 verification and ASN.1 DER signature support.

Reference:

- https://docs.rs/p521/latest/p521/ecdsa/

**Feasibility: HIGH.**

### ERR-038-B — RSA 1024/16384

Target cells:

- `0x0101` / `0x0102` / `0x0103` / `0x0104` with 1024-bit RSA
- same four IDs with 16384-bit RSA

The current `ring` verifier path intentionally rejects both boundaries because its available RSA verification algorithms are bounded to 2048–8192 bits.

A separate backend is therefore required. This increment should not be implemented by weakening the existing `ring` boundary or by pretending that an unsupported key size has been verified.

**Feasibility: MEDIUM.**

Required before implementation:

- select and audit the alternate Rust RSA verification backend;
- verify exact PSS parameter semantics for `0x0101`/`0x0102`;
- verify PKCS#1 v1.5 semantics for `0x0103`/`0x0104`;
- establish explicit key-size acceptance boundaries;
- add deterministic regression coverage for all four algorithm IDs at both edge sizes.

### ERR-038-C — DSA 1024/2048/3072

Target cell:

- `0x0301` with DSA key sizes 1024, 2048, 3072

This is a separate implementation path. The currently available RustCrypto `dsa` crate supports DSA verification but currently warns that its implementation has not been independently audited for security.

Therefore it must not be adopted automatically merely to claim matrix completeness.

**Feasibility: POSSIBLE BUT SECURITY-GATED.**

A trusted backend, or a separately accepted security rationale and validation strategy, is required before implementation.

## 6. Recommended first correction

The first implementation increment should be **ERR-038-A: ECDSA P-521**.

Reason for the bounded choice:

- it closes a documented Android EC coverage gap;
- it uses the same high-level cryptographic model already used by the P-384 implementation;
- `p521` is available as a current RustCrypto crate with ECDSA verification and DER signature support;
- no RSA backend replacement is needed;
- no DSA backend with an explicit security warning is introduced;
- the existing SHA-512 content-digest path can remain unchanged;
- the change can be isolated to signature verification, dependency declaration/lockfile, focused fixtures/tests, and public documentation.

## 7. Proposed acceptance boundary for ERR-038-A

The implementation may be considered complete only when all of the following are true:

1. `0x0201` + P-521 verifies a real valid signature.
2. `0x0202` + P-521 verifies a real valid signature.
3. Invalid P-521 signatures produce `Invalid`, never `Verified`.
4. P-521 public keys presented under unsupported/non-P-521 curves remain `Unsupported`.
5. Existing P-256/P-384 behavior remains unchanged.
6. Existing RSA behavior remains unchanged.
7. Certificate/SPKI binding remains enforced.
8. APK content-digest verification remains unchanged.
9. v2 and v3 signer aggregation semantics remain unchanged.
10. v3 proof-of-rotation behavior remains unchanged.
11. Focused positive and negative regression fixtures are deterministic and repository-contained.
12. Build → Test → Format → Clippy pass in GitHub Actions.
13. A second audit confirms the diff contains no unrelated cryptographic or workflow expansion.

## 8. Explicit exclusions

ERR-038-A must not include:

- RSA 1024/16384 implementation;
- DSA implementation;
- v3.1 cryptographic completion;
- v3.2/PQC implementation;
- AAB cryptographic verification;
- changes to signer aggregation semantics;
- unrelated refactoring;
- changes to release/distribution workflows.

## 9. Audit conclusion

**ERR-038 PRE-AUDIT: PASS**

ERR-038 is technically correctable, but it should not be treated as one monolithic change.

The recommended next implementation block is:

**ERR-038-A — ECDSA P-521 coverage**

The remaining RSA and DSA gaps stay explicitly bounded for later decisions.

No historical ERR-038 ledger status is changed by this document.