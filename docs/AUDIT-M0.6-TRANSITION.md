# Transition Audit — M0.6 → M0.7

Date: 2026-09-30
Status: **EN CURSO**
Baseline under audit: `m06-block5-verification-completeness`
Baseline commit: `dea8bbec0e4b9dbb16100305a96dbd6c43630568`

## 1. Purpose

This audit determines what the repository actually has after M0.6 and separates the remaining capability space into:

1. **Implemented and verified**
2. **Implemented but deliberately limited**
3. **Not implemented**

The audit does not start M0.7 implementation. It does not modify production code.

The goal is to establish evidence and dependencies first, then define M0.7 in a separate stage.

## 2. Ledger-first verification

The current `docs/ERRORS-AND-FIXES.md` ledger was reviewed before this audit.

ERR-001 through ERR-034 remain chronologically represented. No historical entry was removed, renumbered, or rewritten.

The latest closure validation is Actions run #266 / 36758086894:
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

## 3. Repository state entering the audit

M0.6 Blocks 1–5 are complete within their declared scopes.

The validated chain covers:
- artifact/manifest hardening;
- static project ↔ artifact comparison;
- Play readiness checks;
- native ELF and ZIP alignment evidence;
- APK v2/v3 structural and cryptographic verification within the supported verifier subset;
- v3 targeted signer SDK-range evidence;
- signer-local error isolation;
- supported v3 proof-of-rotation verification;
- first-class ManualReview semantics.

No M0.7 production implementation is present in this audit step.

---

## 4. Implemented and verified

### 4.1 APK/AAB artifact inspection

**State: IMPLEMENTED AND VERIFIED**

The core inspects APK/AAB ZIP contents, locates the expected manifest, inventories DEX and native libraries, and records signing-related artifact evidence.

For AABs, M0.1 remains intentionally focused on the base module manifest.

### 4.2 Android manifest evidence

**State: IMPLEMENTED AND VERIFIED**

The artifact parser extracts package identity, SDK levels, effective debuggable state, versions, permissions, and supported component/export information.

Malformed manifest handling and negative AXML cases were hardened in M0.1 and retained through M0.6.

### 4.3 Static project ↔ artifact cross-check

**State: IMPLEMENTED AND VERIFIED WITH DECLARED LIMITATIONS**

Literal Gradle/Groovy and Kotlin DSL values are parsed and compared with the final artifact for application identity, SDK levels, version values, and release debuggable state.

This is sufficient for the declared static cross-check capability.

### 4.4 Native 16 KB evidence

**State: IMPLEMENTED AND VERIFIED**

The tool inspects ELF PT_LOAD alignment and applicable APK ZIP data offsets.

For AABs, bundle entry offsets are not incorrectly treated as proof of generated APK ZIP alignment; the result remains manual review where final APK evidence is unavailable.

### 4.5 APK v2/v3 verification

**State: IMPLEMENTED AND VERIFIED WITH A DELIBERATE CRYPTOGRAPHIC SUBSET**

The verifier covers:
- v2 and v3 signing structures;
- strongest supported signature selection;
- signature verification;
- first-certificate/public-key binding;
- APK content digest verification;
- algorithm-list equality/order checks;
- v3 SDK range consistency;
- signer certificate SHA-256 evidence;
- multi-signer aggregation without aborting the complete pass on one signer-local failure.

### 4.6 v3 proof-of-rotation

**State: IMPLEMENTED AND VERIFIED WITH A DELIBERATE LIMITATION**

The current verifier validates supported lineage structure, certificate uniqueness, parent-to-child signatures, algorithm linkage, and final-certificate/current-signer equality.

The proof-of-rotation node flags are parsed but intentionally not interpreted as a separate semantic capability model. The implementation therefore verifies the cryptographic lineage evidence it currently models, not every possible semantic consequence of the lineage flags.

### 4.7 Manual-review evidence model

**State: IMPLEMENTED AND VERIFIED**

`Severity::ManualReview` is first-class and is propagated across the artifact-proof and external-verification boundaries audited in M0.6.

### 4.8 Versioned Google Play target-API profile

**State: IMPLEMENTED AND VERIFIED FOR THE CURRENT TARGET-API PROFILE**

The repository's profile is versioned as `2026-08-31` and currently uses:
- mobile: API 36;
- Wear OS: API 35;
- Automotive: API 35;
- Android TV: API 34;
- Android XR: API 34.

These values match the current Google Play target-API requirements published by Google as of the transition audit date.

The profile also emits manual review for Play Console declarations that cannot be proven from the artifact.

### 4.9 Current text report

**State: IMPLEMENTED AND VERIFIED**

The CLI produces a structured human-readable text report with artifact/project evidence, findings, severities, remediation text, and summary counts including Manual Review.

---

## 5. Implemented but deliberately limited

### 5.1 Complete Android v2/v3 algorithm and key-size coverage

**State: IMPLEMENTED BUT DELIBERATELY LIMITED**

The current verifier supports a subset of the Android platform's documented v2/v3 signature algorithms and key ranges.

The repository currently handles RSA PSS/PKCS#1 variants and selected ECDSA paths through the available cryptographic backend. Other documented platform algorithms and/or key ranges remain Unsupported rather than being guessed as valid or invalid.

Android's v2 documentation currently lists these platform signature algorithm IDs:
- 0x0101 RSASSA-PSS SHA-256
- 0x0102 RSASSA-PSS SHA-512
- 0x0103 RSASSA-PKCS1-v1_5 SHA-256
- 0x0104 RSASSA-PKCS1-v1_5 SHA-512
- 0x0201 ECDSA SHA-256
- 0x0202 ECDSA SHA-512
- 0x0301 DSA SHA-256

The same documentation lists RSA 1024/2048/4096/8192/16384, EC P-256/P-384/P-521, and DSA 1024/2048/3072 as platform-supported key sizes/curves.

The current code intentionally does not claim complete coverage. Unsupported combinations stay explicit Unsupported/manual-review cases.

### 5.2 Proof-of-rotation semantics

**State: IMPLEMENTED BUT DELIBERATELY LIMITED**

Cryptographic lineage verification is present, but node capability flags are not currently modeled and semantically evaluated.

This is a bounded evidence limitation, not a claim that every Android signing-trust consequence has been reproduced.

### 5.3 v3.1

**State: IMPLEMENTED BUT DELIBERATELY LIMITED**

The repository detects the v3.1 block and prevents it from being silently reported as fully cryptographically verified. It remains a manual-review boundary.

### 5.4 v3.2 / PQC

**State: IMPLEMENTED BUT DELIBERATELY LIMITED**

The repository detects the v3.2 block and keeps it as an explicit manual-review boundary.

Current Android 17 documentation defines v3.2 as a hybrid signature scheme and documents ML-DSA-65 and ML-DSA-87 as the initial PQC algorithms. Full v3.2 verification is outside M0.6.

### 5.5 Gradle project evaluation

**State: IMPLEMENTED BUT DELIBERATELY LIMITED**

The parser is static. It does not execute Gradle and does not resolve:
- product flavors;
- build variants;
- convention plugins;
- generated values;
- arbitrary Kotlin/Groovy expressions;
- CI-injected versioning.

The limitation is explicit in the README and in the project parser behavior.

### 5.6 Google Play policy automation

**State: IMPLEMENTED BUT DELIBERATELY LIMITED**

The current profile verifies the artifact target API requirement and records manual review for Play Console information that the local artifact cannot prove.

It is not a complete automated representation of all current Play policies, declarations, review requirements, account state, or policy exceptions.

### 5.7 Legacy v1 signature evidence

**State: IMPLEMENTED BUT DELIBERATELY LIMITED**

The tool detects legacy `META-INF` signature material and avoids confusing its absence with an unsigned modern APK. It does not provide complete cryptographic v1/JAR signature verification.

### 5.8 AAB native packaging evidence

**State: IMPLEMENTED BUT DELIBERATELY LIMITED**

The tool can inspect AAB native payloads and ELF alignment, but an AAB ZIP entry offset is not treated as proof of final APK packaging alignment. Generated-APK verification remains external/manual.

---

## 6. Not implemented

### 6.1 Full v3.1 cryptographic verification

**State: NOT IMPLEMENTED**

No complete v3.1 signer verification path exists.

### 6.2 Full v3.2/PQC verification

**State: NOT IMPLEMENTED**

No complete hybrid two-signer v3.2 verification exists, including the documented PQC verification and implicit-rotation checks.

### 6.3 Full AAB signing verification

**State: NOT IMPLEMENTED**

The tool does not cryptographically verify the uploaded AAB's signing material as a separate bundle-signing capability, nor does it derive and cryptographically verify every final APK generated from the bundle.

This area must be defined carefully because Google Play uses the uploaded AAB to generate APKs and performs app-signing operations on those generated APKs.

### 6.4 Full Gradle/variant evaluator

**State: NOT IMPLEMENTED**

There is no Gradle execution/evaluation engine or complete variant/flavor resolution engine.

### 6.5 Complete current Google Play policy engine

**State: NOT IMPLEMENTED**

Beyond the versioned target-API profile and explicit manual-review checks, there is no general engine representing the complete set of current Play submission/account/policy conditions.

### 6.6 Permission-risk classification

**State: NOT IMPLEMENTED**

Permissions are inventoried and counted, but the tool explicitly does not classify their risk.

### 6.7 HTML output

**State: NOT IMPLEMENTED**

The current reporting layer does not provide an HTML renderer.

### 6.8 SARIF output

**State: NOT IMPLEMENTED**

The current reporting layer does not provide SARIF output.

---

## 7. Transition dependencies

The audit identifies six capability clusters. They should not be treated as one undifferentiated implementation task.

### Cluster A — Cryptographic completeness

Includes:
- remaining v2/v3 algorithms and key ranges;
- proof-of-rotation flag semantics;
- v3.1;
- v3.2/PQC.

Dependencies:
- cryptographic backend capability;
- deterministic fixtures for each supported/unsupported boundary;
- explicit Android specification mapping;
- preservation of the existing Verified / Invalid / Unsupported model.

### Cluster B — AAB release verification

Includes:
- bundle signing evidence;
- clearer distinction between upload-bundle integrity and final APK app-signing verification;
- optional generated-APK verification workflow if a bundletool-based boundary is introduced.

Dependencies:
- precise definition of what the local tool is proving;
- avoidance of treating the AAB itself as equivalent to the final installed APK signature.

### Cluster C — Project/variant resolution

Includes:
- variants;
- flavors;
- generated version values;
- convention plugins;
- CI-generated configuration.

Dependencies:
- a defined execution model;
- security boundaries around arbitrary Gradle execution;
- deterministic fixtures and environment control.

### Cluster D — Play policy evolution

Includes:
- versioned policy profiles;
- expanded automated policy checks;
- policy source/version traceability;
- explicit manual-review fallbacks.

Dependencies:
- stable rule-version architecture;
- current official Play policy sources;
- avoidance of false claims about Play Console state.

### Cluster E — Reporting formats

Includes:
- HTML;
- SARIF;
- stable machine-readable finding serialization if needed as a shared foundation.

Dependencies:
- stable finding/severity schema.

### Cluster F — Permission risk

Includes:
- permission categorization;
- risk evidence and rationale;
- configurable/versioned classification rules.

Dependencies:
- an explicit risk taxonomy;
- evidence-based rules rather than generic permission counts.

---

## 8. Current external reference points

The transition audit used current official documentation as an external specification check:

- Android APK Signature Scheme v2:
  https://source.android.com/docs/security/features/apksigning/v2
- Android APK Signature Scheme v3:
  https://source.android.com/docs/security/features/apksigning/v3
- Android APK Signature Scheme v3.1:
  https://developer.android.com/about/versions/13/features
- Android APK Signature Scheme v3.2:
  https://source.android.com/docs/security/features/apksigning/v3-2
- Google Play target API requirements:
  https://support.google.com/googleplay/android-developer/answer/11926878
- Android App Bundle:
  https://developer.android.com/guide/app-bundle

The current Google Play target-API profile in the repository matches the published requirements effective August 31, 2026.

---

## 9. Transition conclusion

M0.6 leaves the repository in a stable state with a clear separation between verified evidence and deliberate manual-review boundaries.

The principal transition facts are:

1. The core artifact-audit foundation is implemented and validated.
2. The current APK cryptographic verifier is useful but intentionally not a complete implementation of every Android v2/v3 algorithm/key combination.
3. v3.1 and v3.2 are detected but not cryptographically verified.
4. Proof-of-rotation verification is cryptographically meaningful but does not model lineage capability flags as an independent semantic layer.
5. AAB verification requires a separate definition of what is being proven.
6. Gradle/variant evaluation is a materially different execution problem from the current static parser.
7. Play policy automation is naturally versioned and extensible, but the current implementation is intentionally narrow.
8. Reporting and permission classification are independent capability layers.

This audit therefore does **not** assign all remaining items automatically to M0.7. It establishes the evidence, boundaries, and dependencies that the next stage must use to define the actual M0.7 scope.

## Stage status

**Etapa 2 — Auditoría de transición M0.6 → M0.7: EN CURSO**

No production-code correction is required by this audit at this point.

The next step is the second audit of this transition document, followed by CI validation and the checkpoint that will determine whether Etapa 2 can be marked FINALIZADA.

