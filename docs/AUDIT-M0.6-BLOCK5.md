# M0.6 Block 5 — Pre-change audit

Date: 2026-09-30
Validated baseline: `d1dd8cbf596f06572ba9b883528b5133ca28d176`
Base branch: `m06-block4-signer-error-isolation`
Working branch: `m06-block5-verification-completeness`

## Audit method

This audit follows the mandatory project discipline:

ledger review → audit → scoped changes → second audit → Actions → follow-up → individual corrections → new validation → checkpoint

Before this audit, the complete `docs/ERRORS-AND-FIXES.md` ledger was reviewed. ERR-001 through ERR-023 remain present and resolved. No historical entry is rewritten or removed.

Reviewed:

- `crates/doctor-core/src/signature_verify.rs`
- `crates/doctor-core/src/lib.rs`
- `.github/workflows/rust.yml`
- `rules/README.md`
- `README.md`
- M0.6 Block 4 final checkpoint and second-audit records
- existing v2/v3 verification tests
- current AOSP documentation for APK Signature Scheme v3 and v3.1/v3.2 boundaries

The audit is intentionally limited to the next coherent verification-hardening gap. It does not assume that every currently unsupported Android signing capability must be implemented in this block.

## Findings

### AUDIT-024 — v3 proof-of-rotation is detected but not verified

The current v3 parser detects the proof-of-rotation attribute by its ID (`0x3ba06f8c`) and records only a boolean `has_proof_of_rotation`.

When the attribute is present, the signer is ultimately reported as `CryptoVerificationState::Unsupported` rather than having its proof-of-rotation structure validated.

This is conservative because the tool does not claim a cryptographic pass. However, it leaves an important part of the v3 signed-data evidence unverified.

AOSP defines the proof-of-rotation attribute as a signed, singly-linked sequence of previous signing certificates. During v3 verification, when the attribute exists, the verifier is expected to validate the structure and verify that the current signer is the final certificate in the lineage. citeturn0search4

The current implementation therefore cannot distinguish between:

- a valid proof-of-rotation chain;
- a structurally malformed proof-of-rotation attribute;
- a valid-looking chain whose signatures or certificate relationships do not establish the required lineage;
- a chain whose final certificate does not correspond to the current signer.

Required change:

- parse the proof-of-rotation structure rather than only detecting its attribute ID;
- validate its length-prefixed structure and certificate entries;
- validate the signing relationship between consecutive lineage levels using the applicable previous certificate and signature algorithm;
- verify that the final lineage certificate corresponds to the current v3 signer certificate;
- retain a conservative `MANUAL-REVIEW`/unsupported boundary for any proof-of-rotation algorithm or structure the verifier cannot safely validate;
- preserve the existing signer isolation, SDK-range evidence, and Invalid/Unsupported/Verified aggregation semantics from Blocks 3 and 4.

This block must not claim Android runtime trust semantics beyond what the artifact evidence proves.

### AUDIT-025 — proof-of-rotation evidence is not exposed as structured signer evidence

Even when a proof-of-rotation attribute is present, the current `CryptoSchemeInfo` contains only the generic `Unsupported` state and detail text.

After validation is added, downstream audit/reporting cannot distinguish a verified signer with a valid rotation lineage from a signer for which rotation verification remains incomplete unless the evidence model records that distinction.

Required change:

- expose structured proof-of-rotation evidence sufficient to report whether the lineage was verified;
- retain the current certificate fingerprint evidence;
- preserve the current SDK-range evidence;
- avoid exposing internal cryptographic parsing details as if they were Android platform decisions.

The evidence model should remain minimal: only facts that are actually verified by the artifact should be retained.

## Existing boundaries explicitly confirmed

The following are **not Block 5 findings**:

### v3.1

The current implementation detects the v3.1 signing-block ID and surfaces its presence through manual review rather than pretending to verify it. AOSP describes v3.1 as a separate signing block used for SDK-targeted key rotation. citeturn0search1

Complete v3.1 verification remains a future scope item.

### v3.2

The current implementation detects the v3.2 signing-block ID and surfaces its presence through manual review. Android 17 introduces v3.2 as a hybrid classical/PQC signing scheme with specific two-signer requirements and ML-DSA algorithms. citeturn0search3

Complete v3.2/PQC verification remains a future scope item.

### Additional signature algorithms

The verifier intentionally supports a bounded subset of Android signing algorithms. Unsupported algorithms remain manual-review evidence rather than being reported as verified.

Expanding the cryptographic backend or adding broad algorithm coverage is not part of Block 5.

## M0.6 Block 5 scope

Only these findings are in scope:

1. structurally and cryptographically validate the v3 proof-of-rotation attribute when present and safely supported;
2. verify the required relationship between consecutive lineage certificates;
3. verify that the current v3 signer corresponds to the final proof-of-rotation certificate;
4. expose minimal structured proof-of-rotation verification evidence;
5. add focused regression fixtures/tests for valid and malformed/invalid rotation evidence;
6. preserve existing signer isolation and severity semantics;
7. update only the documentation necessary to describe the new evidence boundary.

## Out of scope

- complete v3.1 cryptographic verification;
- complete v3.2 hybrid/PQC verification;
- new cryptographic algorithms or replacement of the `ring` backend;
- AAB cryptographic verification;
- new Google Play policy rules;
- broad CLI/report redesign;
- unrelated refactoring;
- PR merge.

## Acceptance criteria

1. A v3 signer without proof-of-rotation continues to verify exactly as before.
2. A valid proof-of-rotation chain is parsed and cryptographically validated.
3. The final certificate in the lineage must match the current v3 signer certificate.
4. Malformed or cryptographically invalid proof-of-rotation evidence cannot produce a cryptographic PASS.
5. Unsupported proof-of-rotation algorithms remain explicitly unsupported/manual-review rather than guessed as valid.
6. Existing v2/v3 single-signer and multi-signer verification remains green.
7. Existing SDK-range, signer-count, certificate-fingerprint, and signer-isolation evidence is preserved.
8. Focused tests cover valid lineage, malformed lineage, invalid lineage signature, and final-certificate mismatch.
9. The incremental error ledger is updated only for newly discovered defects/corrections during the Block 5 cycle; historical ERR-001 through ERR-023 remain unchanged.
10. No merge is performed as part of this block.
