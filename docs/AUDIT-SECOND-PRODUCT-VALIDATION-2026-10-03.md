# AUDIT — Second Product Validation Campaign — 2026-10-03

Repository: `jedaqu/android_release_doctor`

## Status

**CAMPAIGN VALIDATED — PRODUCT CODE UNCHANGED**

Final validation run: **37080224270**  
Validation workflow commit: `72bd1801cad06e33bed48af5fbc01955aa68bdde`  
Workflow syntax lint: **SUCCESS** (final lint run for the campaign branch)

Main baseline under audit:
`0e5b46ae287f476e47bcf1f2c0082d12c04b5c38`

No production source file was modified by this campaign.

## Scope

This campaign was designed to validate the behavior exposed by the GitHub Action and CLI, with explicit semantic oracles rather than accepting report generation alone as evidence.

Coverage:

1. Rust formatting, workspace tests, Clippy and CLI build.
2. APK signing and cryptographic findings:
   - v2
   - v3
   - v3.1
   - invalid v3.1 lineage
   - invalid rotation minimum SDK
   - ECDSA/SHA-512/P-384 v2 and v3
   - existing unit coverage for RSA boundary cases, ECDSA P-521, tamper detection and v3.1 lineage semantics.
3. Google Play profile selection:
   - mobile
   - wear
   - automotive
   - tv
   - xr
4. GitHub Action execution contract.
5. Controlled native-library matrix:
   - arm64-v8a + x86_64 at 16 KiB
   - 64-bit ELF misalignment
   - correct 64-bit plus 32-bit armeabi-v7a at 4 KiB
6. AAB native boundary and manual-review behavior.
7. Gradle cross-check mismatch matrix:
   - applicationId
   - targetSdk
   - minSdk
   - versionCode
   - versionName
   - release debuggable
8. Malformed artifacts:
   - missing manifest
   - unreadable manifest
9. Scale/path stress:
   - 25,005 ZIP entries
   - unusual paths
   - spaces
   - Unicode
   - long path names
10. CLI usage/output errors.
11. Public real-artifact corpus:
   - Session Android 1.33.5 APK
   - Session Android 1.33.5 AAB
   - Termux 0.119.0-beta.3 universal debug APK
12. Checksum verification for the pinned Session artifacts before audit.

## Final result

All four campaign jobs completed successfully:

- **Baseline, crypto and Play — SUCCESS**
- **Native ABI and AAB boundary — SUCCESS**
- **Malformed, scale and public corpus — SUCCESS**
- **Gradle cross-check semantic matrix — SUCCESS**

The workflow syntax was independently checked with actionlint and reached **SUCCESS**.

## Important observed evidence

### Cryptography

The explicit semantic oracle produced:

- v2 — SIGNING-003 PASS
- v3 — SIGNING-003 PASS
- v3.1 — SIGNING-003 PASS
- v3.1 lineage mismatch — SIGNING-003 BLOCKER
- v3.1 rotation-min-SDK mismatch — SIGNING-003 BLOCKER
- ECDSA/SHA-512/P-384 v2 — SIGNING-003 PASS
- ECDSA/SHA-512/P-384 v3 — SIGNING-003 PASS

The existing Rust test suite also exercised additional boundaries including RSA 1024 PKCS#1/SHA-256 and SHA-512, RSA-PSS/SHA-256, ECDSA P-521, tamper detection, proof-of-rotation semantics, and v3.1 consistency.

### Play profiles

All five platform contracts passed:

- mobile
- wear
- automotive
- tv
- xr

### Gradle cross-checks

All six intentional mismatches produced the expected rule and severity:

- CROSSCHECK-001 — BLOCKER
- CROSSCHECK-002 — BLOCKER
- CROSSCHECK-003 — WARNING
- CROSSCHECK-004 — WARNING
- CROSSCHECK-005 — WARNING
- CROSSCHECK-006 — BLOCKER

### Malformed and scale corpus

The final campaign confirmed:

- missing manifest → MANIFEST-001 BLOCKER
- unreadable manifest → MANIFEST-002 BLOCKER
- 25,005-entry ZIP was audited successfully
- unusual-path corpus was audited successfully
- CLI invalid-format, invalid-output-path and missing-input contracts returned exit code 2

### Real public artifacts

Pinned SHA-256 checksums for Session Android 1.33.5 APK and AAB were verified successfully before audit.

The Action successfully audited:

- Session APK — APK, targetSdk 36, 32 native libraries
- Session AAB — AAB, targetSdk 36, 32 native libraries
- Termux universal debug APK — APK, targetSdk 28, 12 native libraries

These results demonstrate successful ingestion and report production on public, independently produced artifacts. A report existing is not treated as proof that every finding is semantically correct; the controlled corpora provide that oracle.

### Native/AAB boundary

Controlled ELF cases produced:

- good64 — PLAY-005 PASS
- bad64 — PLAY-005 BLOCKER
- mixed64-good + armeabi-v7a 4 KiB — PLAY-005 BLOCKER

Controlled AAB behavior produced:

- PLAY-005 MANUAL-REVIEW
- non-base feature manifest entry retained in inventory

The AAB result is consistent with the product's intended boundary that raw bundle entry alignment does not establish final APK alignment.

## Product finding retained for focused review

### FINDING-001 — 16 KiB native ABI scope

The current implementation evaluates native ELF alignment across all detected ABIs. The controlled case containing correctly aligned `arm64-v8a` and `x86_64`, plus a 4 KiB `armeabi-v7a` library, was classified as a PLAY-005 blocker.

This behavior is reproducible and is therefore a real product behavior, not a test-harness artifact.

The current Android Developers guidance states that the Google Play 16 KiB compatibility requirement for apps targeting API 35+ applies to **64-bit devices**, and its concrete alignment procedure specifically calls out `arm64-v8a` and `x86_64`. See:

https://developer.android.com/guide/practices/page-sizes

**Disposition:** candidate product/specification mismatch requiring a focused audit before any implementation change. No fix was made during this campaign.

## Harness corrections made during this campaign

The following were test-only corrections and are not product defects:

1. YAML indentation of embedded Python heredocs.
2. Malformed-manifest oracle originally looked for MANIFEST-002 in the missing-manifest case; corrected to MANIFEST-001.
3. Complex AAB oracle assumed a concrete package name unsupported by the fixture; removed.
4. AAB inventory oracle treated `inventory.entries` as objects; corrected to string membership.

Each correction was followed by a fresh GitHub Actions run.

## Remaining coverage gap

The campaign substantially expanded semantic coverage, but it did not execute every row of the documented cryptographic algorithm/key-size matrix as a full end-to-end APK artifact. Additional unit-level cryptographic coverage exists for several of those boundaries.

This is a **coverage gap**, not a proven product defect.

## Decision

The product remains frozen at the validated implementation.

The next engineering phase is **repository cleanup and product-definition normalization**:

- remove or archive historical `v0.1.x` release/distribution machinery;
- remove release-publication workflow and obsolete release-only documentation from the product surface;
- preserve Action, CLI, audit engine, fixtures and technical evidence that still belong to the product;
- preserve the public/private separation rules;
- then re-audit the cleaned repository and run Actions again.

No release/tag/publication change is part of this validation result.


## Current-state reconciliation — 2026-10-03

This validation campaign is a historical record of the state that existed before the later ABI-scope corrections and product-surface cleanup. Its `FINDING-001 — 16 KiB native ABI scope` is therefore retained as historical evidence, not as a current unresolved finding.

Subsequent validated work resolved that finding through:

- ERR-113 / PLAY-005 ABI restriction to `arm64-v8a` and `x86_64`;
- ERR-114 / NATIVE-002 ABI restriction to `arm64-v8a` and `x86_64`;
- ERR-115 / NATIVE-003 ABI restriction plus the explicit full ABI matrix.

The later cleanup campaign then removed the former release/distribution product surface. Current product truth is maintained separately in `AUDIT-PRODUCT-CLEANUP-2026-10-03.md` and `CURRENT-PRODUCT-STATE-2026-10-03.md`.

This document's earlier campaign results, baseline, and chronology remain unchanged.
