# Pre-Audit — Complete Classical Crypto Matrix

Date: 2026-10-02
Baseline: `main` at `cff74b5a8bba80b16639a9e622163d62fb0e4513`

## Authoritative matrix

Android's APK Signature Scheme v2 documentation defines these signature algorithm IDs:

- `0x0101` RSA-PSS / SHA-256
- `0x0102` RSA-PSS / SHA-512
- `0x0103` RSA-PKCS1-v1.5 / SHA-256
- `0x0104` RSA-PKCS1-v1.5 / SHA-512
- `0x0201` ECDSA / SHA-256
- `0x0202` ECDSA / SHA-512
- `0x0301` DSA / SHA-256

The same authoritative document lists RSA key sizes 1024, 2048, 4096, 8192, 16384 and EC curves P-256, P-384, P-521.

Sources:

- https://source.android.com/docs/security/features/apksigning/v2
- https://source.android.com/docs/security/features/apksigning/v3

## Current implementation audit

The current verifier already contains:

- RSA-PSS SHA-256/SHA-512 verification.
- RSA-PKCS1-v1.5 SHA-256/SHA-512 verification.
- RSA extended verification for 1024-bit and 16384-bit keys.
- Ring-backed RSA verification for 2048 through 8192 bits.
- ECDSA SHA-256 for P-256, P-384 and P-521.
- ECDSA SHA-512 for P-384 and P-521.
- Explicit Unsupported handling for DSA `0x0301`.
- Explicit Unsupported handling for RSA-1024 + PSS/SHA-512 because the required 64-byte salt does not fit the 1024-bit modulus.
- Explicit Unsupported handling for unsupported ECDSA curves.

## Coverage gap identified

The implementation capability is broader than the currently documented/test-demonstrated matrix.

Missing explicit positive evidence at the cryptographic primitive boundary:

- RSA 2048 × 4 algorithms
- RSA 4096 × 4 algorithms
- RSA 8192 × 4 algorithms
- explicit ECDSA P-256 SHA-256 primitive vector
- complete cross-product assertions for the supported EC cells

Existing positive evidence already covers:

- RSA 1024: PKCS1/SHA-256, PKCS1/SHA-512, PSS/SHA-256; PSS/SHA-512 explicitly unsupported.
- RSA 16384: all four algorithms.
- ECDSA P-384: SHA-512, with existing broader SHA-256 implementation coverage.
- ECDSA P-521: SHA-256 and SHA-512.
- Existing v2/v3 APK fixtures provide higher-level integration evidence.

## Negative/malformed coverage gap

The campaign should explicitly assert:

- tampered signature rejection for representative RSA and ECDSA cells;
- malformed/truncated signature rejection;
- algorithm/key-family mismatch;
- DSA `0x0301` remains Unsupported;
- unsupported algorithm IDs remain Unsupported;
- RSA-1024 PSS/SHA-512 remains Unsupported;
- ECDSA/SHA-512 P-256 remains Unsupported;
- key-size boundaries 1024/2048/8192/16384 remain explicit.

## Scope boundary

This campaign does not add:

- DSA cryptographic verification;
- v3.2/PQC cryptographic verification;
- AAB cryptographic signing verification;
- runtime Android trust-state simulation.

Those remain explicit product boundaries.

## Acceptance oracle

Supported positive cells:

| Family | Variant | Expected |
|---|---|---|
| RSA | 1024 × 0101 | PASS |
| RSA | 1024 × 0102 | UNSUPPORTED |
| RSA | 1024 × 0103 | PASS |
| RSA | 1024 × 0104 | PASS |
| RSA | 2048/4096/8192 × 0101..0104 | PASS |
| RSA | 16384 × 0101..0104 | PASS |
| EC | P-256 × 0201 | PASS |
| EC | P-256 × 0202 | UNSUPPORTED |
| EC | P-384 × 0201/0202 | PASS |
| EC | P-521 × 0201/0202 | PASS |
| DSA | 0301 | UNSUPPORTED |

For every supported positive cell, a tampered or malformed signature must not verify.

## Authorization boundary

No production behavior change is authorized until this pre-audit is used as the implementation/test boundary. Documentation updates must reflect the verified implementation rather than expanding implementation scope.

## Implementation and Validation

PR: #51

Final test head:
`f19027efc783c1c4f9c8061332f39c1fbb9c82af`

The campaign added public cryptographic test vectors containing only certificates and signatures:

- RSA 2048 × `0x0101`..`0x0104`
- RSA 4096 × `0x0101`..`0x0104`
- RSA 8192 × `0x0101`..`0x0104`
- ECDSA P-256 × `0x0201`
- ECDSA P-384 × `0x0201`, `0x0202`
- ECDSA P-521 × `0x0201`, `0x0202`

The regression tests additionally cover:

- positive verification for every supported new cell;
- tampered signature rejection;
- truncated/malformed signature rejection;
- RSA/ECDSA key-family mismatch rejection;
- DSA `0x0301` remaining Unsupported;
- unknown algorithm IDs remaining Unsupported;
- RSA-1024 PSS/SHA-512 remaining Unsupported;
- ECDSA P-256 SHA-512 remaining Unsupported;
- explicit RSA key-size boundaries.

### Campaign corrections

1. CI run `37086633173` exposed a test-fixture parser defect: the pre-section `MESSAGE=` line was ignored. The parser was corrected; no production verification logic changed.
2. CI run `37086707555` exposed three corrupted/truncated public RSA fixture values introduced during manual fixture insertion:
   - RSA-4096 `0x0102`
   - RSA-8192 `0x0101`
   - RSA-8192 `0x0104`
   The values were restored from the locally generated OpenSSL-verified source fixtures. No production verification logic changed.
3. CI run `37086980714` on final head `f19027efc783c1c4f9c8061332f39c1fbb9c82af` passed Build, Test, Format and Clippy.

### Second Audit

- The production verifier files were not modified by this campaign.
- All new fixture material contains public certificates and signatures only; no private keys were committed.
- The tested matrix matches the intended classical algorithm boundary in v2/v3.
- DSA, v3.2/PQC, AAB cryptographic signing, and runtime Android trust simulation remain outside the supported verification scope.

## Final Result

**PASS — Complete classical cryptographic matrix is now explicitly tested and documented.**

The verifier's existing implementation capability is now demonstrated by deterministic positive and negative tests across the supported RSA and ECDSA cells. No new cryptographic algorithm capability was added in this campaign.
