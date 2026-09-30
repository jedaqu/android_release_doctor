# Second Audit — M0.7 Definition

Date: 2026-09-30
Status: **FINDINGS IDENTIFIED — CORRECTION REQUIRED**

## 1. Purpose

This second audit reviews `docs/M0.7-DEFINITION.md` against:

- the completed M0.6 → M0.7 transition audit;
- the current M0.6 implementation boundaries;
- the established ledger-first engineering discipline;
- the Android APK Signature Scheme v3/v3.1 reference behavior relevant to the proposed scope.

No production code is changed by this audit.

## 2. Ledger-first verification

Before this audit, `docs/ERRORS-AND-FIXES.md` was reviewed.

Historical entries remain chronological and are not rewritten or renumbered.

## 3. Audit result

The overall M0.7 axis remains coherent:

> increase the completeness and precision of APK cryptographic verification without weakening the evidence-first model.

Blocks 1–4 are also the correct high-level order. However, two definition-level findings must be corrected before Etapa 3 can be closed.

## 4. Findings

### AUDIT-M0.7-001 — Proof-of-rotation flags were specified as “invalid flag combinations” without a defined Android semantic basis

**Related ledger entry:** ERR-035.

The definition currently asks Block 2 to add fixtures for “valid and invalid flag combinations”.

The Android `apksig` implementation models the lineage capability flags as independent bit capabilities:

- installed-data: bit 1;
- shared-user-id: bit 2;
- permission: bit 4;
- rollback: bit 8;
- auth: bit 16.

The official implementation exposes these capabilities independently; combinations are therefore not, by themselves, a sufficient definition of malformed or invalid semantics.

The M0.7 boundary should instead require:

- explicit modeling of the defined capability bits;
- explicit preservation/reporting of unknown or reserved bits, if encountered;
- deterministic tests for known capability-state evidence;
- semantic validation only where Android defines a deterministic rule that can be checked from the artifact alone;
- no claim that a bit combination is invalid merely because it is unusual.

Reference:
https://android.googlesource.com/platform/tools/apksig/+/refs/heads/main/src/main/java/com/android/apksig/SigningCertificateLineage.java

### AUDIT-M0.7-002 — v3.1 verification requirements were too broad and did not enumerate the v3/v3.1 relationship constraints

**Related ledger entry:** ERR-036.

The definition says Block 3 must “verify the applicable signer/range relationship”, but does not explicitly require the important v3.1 cross-block constraints.

The official `apksig` verifier treats v3.1 as more than a second copy of v3: it validates the relationship between the v3 and v3.1 blocks, including:

- v3.1 presence together with the required v3 base block;
- rotation-min-SDK targeting consistency;
- the v3 stripping-protection attribute and its relationship to the v3.1 rotation target;
- the absence/presence rules for the v3.1 block;
- signer/lineage consistency across platform-targeted signers.

The M0.7 definition must name these as acceptance boundaries rather than leaving them under the generic phrase “signer/range relationship”.

References:
https://source.android.com/docs/security/features/apksigning/v3-1
https://android.googlesource.com/platform/tools/apksig/+/refs/heads/main/src/com/android/apksig/ApkVerifier.java

## 5. Scope integrity check

No finding requires expanding M0.7 beyond the cryptographic axis already selected.

The following remain correctly outside the milestone:

- full v3.2/PQC verification;
- AAB signing/final-generated-APK workflow;
- Gradle/variant execution;
- complete Play policy automation;
- HTML/SARIF;
- permission-risk classification;
- broad CLI redesign;
- unrelated refactoring.

No production-code correction is authorized by this audit.

## 6. Required correction

Correct `docs/M0.7-DEFINITION.md` only:

1. replace the vague “invalid flag combinations” fixture requirement with explicit capability-bit/unknown-bit evidence semantics;
2. make the v3.1 cross-block and rotation-target constraints explicit in Block 3;
3. preserve all previously declared boundaries and acceptance criteria.

After correction:

- append ERR-035 and ERR-036 with cause/correction/validation fields;
- re-audit this document;
- run Actions;
- update the roadmap only after the corrected definition passes;
- create the final Etapa 3 checkpoint.

**Production implementation remains prohibited until Etapa 3 is formally closed.**
