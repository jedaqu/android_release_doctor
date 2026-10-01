# AUDIT M0.7 Block 2 — Proof-of-rotation semantic evidence

**Date:** 2026-09-30  
**Branch:** `m07-block2-rotation-semantics`  
**Base:** `m07-block1-crypto-coverage`  
**Scope:** Pre-change audit only. No production implementation is included in this audit.

## 1. Audit purpose

M0.7 Block 2 extends the already-validated M0.6 proof-of-rotation verification with explicit semantic evidence for lineage-node capability flags.

The audit establishes, before changing production code:

- the current proof-of-rotation verification boundary;
- the Android-defined lineage capability flags;
- what can be determined locally from the artifact;
- the exact evidence gap caused by currently discarded flags;
- the smallest bounded implementation increment for Block 2;
- the required fixtures and regressions.

The Block 2 definition explicitly forbids treating arbitrary combinations of independent capability bits as inherently invalid. Semantic conclusions must be limited to rules that Android exposes deterministically from the artifact.

## 2. Ledger review

The incremental ledger was reviewed through **ERR-048**.

Relevant constraints carried forward:

- **ERR-024..029:** preserve proof-of-rotation verification, structured evidence, and evidence through signer-local failures.
- **ERR-035:** do not invent invalid flag combinations; capability bits must be modeled explicitly.
- **ERR-036:** v3.1 cross-block semantics remain a separate Block 3 scope.
- **ERR-045..048:** Block 1 is closed and its evidence/CI/documentation state must remain unchanged.

No historical entry authorizes changing the existing lineage cryptographic verification semantics.

## 3. Current implementation coverage

The current `validate_proof_of_rotation()` implementation already performs the following:

- parses version 1 lineage structure;
- parses each lineage node;
- parses the node's signed data;
- parses the certificate;
- parses the parent signature algorithm;
- parses the node signature algorithm;
- verifies parent-to-child lineage signatures;
- rejects duplicate certificates;
- enforces first-node/no-parent-signature semantics;
- enforces final-node/no-next-algorithm semantics;
- checks the final lineage certificate against the current v3 signer certificate;
- represents the result as `Verified`, `Invalid`, or `Unsupported`;
- preserves structured proof-of-rotation evidence through later signer-local failures.

However, the node flags are currently read into `_flags` and discarded. `ProofOfRotationInfo` contains only:

- verification state;
- lineage level count;
- textual detail.

Therefore the artifact's capability-bit evidence is not represented in the public evidence model.

## 4. Android-defined capability flags

The Android/AOSP lineage implementation defines these capability bits:

| Bit | AOSP meaning |
|---:|---|
| `0x00000001` | past certificate may be trusted for installed-data continuity |
| `0x00000002` | past certificate may participate in shared UID relationships |
| `0x00000004` | past certificate may receive/grant signature-permission trust |
| `0x00000008` | past certificate permits rollback/update to the old certificate |
| `0x00000010` | past certificate retains authenticator-module access |

These are independent capability bits. A combination of set bits is not, by itself, evidence of malformed or invalid lineage data.

The AOSP implementation also treats these bits as capability properties exposed through the lineage model rather than as a closed enumeration of mutually exclusive states.

Sources:

- Android APK Signature Scheme v3:
  https://source.android.com/docs/security/features/apksigning/v3
- AOSP `SigningCertificateLineage.java`:
  https://android.googlesource.com/platform/tools/apksig/+/refs/heads/main/src/main/java/com/android/apksig/SigningCertificateLineage.java

## 5. Deterministic local semantic boundary

The following facts are locally verifiable from the artifact:

1. each node contains a uint32 capability-flags field;
2. the five documented capability bits can be decoded independently;
3. bits outside the documented mask are unknown/reserved from this verifier's current specification boundary;
4. known and unknown bits can be reported without changing cryptographic validity;
5. capability flags are protected by the v3 signed-data structure already verified by the current implementation.

The following conclusions are **not** safe to infer solely from the artifact:

- whether a capability is actually exercised by the Android platform for a particular installed-app state;
- whether a rollback is operationally possible on a particular device;
- whether a shared-UID relationship exists elsewhere;
- whether a signature permission is actually declared or granted;
- whether an authenticator module is present or applicable.

Those remain evidence boundaries, not failures.

## 6. Concrete Block 2 gap

### AUDIT-M0.7-B2-001 — Proof-of-rotation flags are parsed but discarded

The current verifier consumes the node flags field but stores no structured representation of it.

**Impact:** a successful proof-of-rotation verification can prove the lineage cryptography and certificate relationship, but the audit cannot report which self-trust capabilities each historical certificate declares.

This is a concrete evidence-model gap and is exactly within the declared Block 2 scope.

### AUDIT-M0.7-B2-002 — Unknown/reserved flag bits have no explicit evidence state

The current verifier has no representation for bits outside the documented capability mask.

**Impact:** an artifact may contain unknown bits, but the current evidence model cannot distinguish "no known capability bits set" from "known bits plus unknown/reserved bits present."

The correction must expose unknown bits explicitly without declaring the lineage invalid merely because such bits are present.

## 7. Selected bounded increment

The selected Block 2 increment is:

> **Add structured proof-of-rotation capability evidence for each lineage node: raw flags, decoded known capabilities, and unknown/reserved bits, while preserving all existing cryptographic and structural verification semantics.**

The increment must:

- preserve the existing `CryptoVerificationState` values;
- preserve all M0.6 lineage cryptographic checks;
- preserve signer isolation and evidence aggregation;
- decode the five AOSP-defined capability bits independently;
- retain unknown/reserved bits explicitly;
- treat unusual combinations of known bits as valid capability evidence unless Android defines a deterministic contradiction;
- avoid inferring runtime trust outcomes;
- preserve existing positive, malformed, invalid-signature, duplicate-certificate, and final-certificate tests.

No v3.1 behavior is introduced by this block.

## 8. Fixture and regression plan

The implementation must add deterministic coverage for at least:

1. existing valid two-level lineage remains `Verified`;
2. a lineage node with a representative single known capability bit exposes that capability;
3. a lineage node with multiple known capability bits exposes each bit independently;
4. zero known capability bits is represented distinctly from unknown bits;
5. an unknown/reserved bit is preserved as explicit evidence without forcing `Invalid`;
6. malformed flags framing remains `Invalid`;
7. cryptographically invalid lineage remains `Invalid`;
8. final-certificate mismatch remains `Invalid`;
9. signer-local failure still preserves the structured proof-of-rotation evidence.

Fixtures created for this block must be reachable from the active branch before the second audit.

## 9. Explicitly out of scope

This Block 2 increment does **not** implement:

- v3.1 verification;
- v3.2/PQC;
- cross-block v3/v3.1 consistency;
- runtime Android trust-state simulation;
- Play policy conclusions;
- changes to cryptographic algorithm support;
- AAB cryptographic verification;
- broad evidence-model refactoring.

## 10. Acceptance semantics

The implementation is acceptable only when:

- every lineage node's capability flags are represented structurally;
- known capabilities are decoded correctly;
- unknown/reserved bits remain explicit evidence;
- arbitrary combinations of independent capability bits are not rejected merely because they are unusual;
- no valid cryptographic lineage becomes `Invalid` solely because of a flag combination;
- malformed/cryptographically invalid lineage remains `Invalid`;
- existing M0.6 and M0.7 Block 1 regressions remain green;
- signer-local evidence retention remains intact;
- no unrelated production behavior is changed.

## 11. Pre-change audit conclusion

**PRE-CHANGE AUDIT: PASS TO PROCEED WITH THE SCOPED IMPLEMENTATION**

The current verifier has a precise, specification-backed evidence gap: it parses proof-of-rotation flags but discards them.

The bounded Block 2 increment is therefore limited to **capability-bit semantic evidence and unknown-bit handling**. No cryptographic redesign or v3.1 work is authorized in this block.

The next engineering movement is:

**scoped implementation → focused tests → second audit → Actions → individual corrections if required → new validation → Block 2 checkpoint.**
