# CHECKPOINT M0.7 Block 3 — APK Signature Scheme v3.1 verification

**Status:** FINALIZADO  
**Branch:** `m07-block3-v31-verification`  
**Base checkpoint:** `m07-block2-rotation-semantics`  
**Pre-checkpoint functional validation:** Actions #402 / `36802297492`

## 1. Scope delivered

M0.7 Block 3 delivered the bounded v3.1 capability:

> APK Signature Scheme v3.1 verification for the existing supported cryptographic algorithms, plus deterministic v3/v3.1 rotation semantics.

The implementation now:

- detects and verifies the v3.1 signing block separately;
- reuses the existing cryptographic verification path;
- exposes v3.1 signer evidence and SDK ranges;
- parses rotation-min-SDK stripping protection;
- requires the v3 base block;
- validates rotation-min-SDK consistency;
- validates v3/v3.1 SDK-range relationships, including the documented development-era boundary;
- validates signer-count compatibility;
- preserves structured proof-of-rotation evidence;
- bridges v3 signer certificate identity into v3.1 lineage when v3 does not expose structured lineage;
- rejects malformed and inconsistent lineage evidence deterministically.

## 2. Authoritative fixture coverage

Real signed APK fixtures from the APK Signature Scheme test corpus are included:

- valid v3/v3.1 rotation;
- v3.1 without v3 base block;
- missing v3 stripping-protection attribute;
- rotation-min-SDK mismatch;
- malformed proof-of-rotation lineage.

A separate semantic helper regression exercises a genuine v3/v3.1 lineage-prefix mismatch without bypassing APK cryptographic verification.

## 3. Validation evidence

### Functional Actions validation — #402 / 36802297492

- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

The complete correction chain through ERR-067 was validated.

## 4. Correction history

The Block 3 loop followed the project discipline:

- ERR-057 — scoped v3.1 capability gap;
- ERR-058 — pre-CI source substitution corruption caught and corrected;
- ERR-059 — stacked CI trigger coverage;
- ERR-060 — build label type mismatch;
- ERR-061 — old parser call sites;
- ERR-062 — over-constrained v3.1 SDK minimum;
- ERR-063 — allowed SDK-32 boundary overlap;
- ERR-064 — lineage bridge requirement;
- ERR-065 — malformed-lineage fixture classification;
- ERR-066 — maxSDK test expectation;
- ERR-067 — rustfmt corrections;
- ERR-068 — final correction-chain validation closure.

Historical ledger entries are preserved; ERR-068 records the final validation closure.

## 5. Documentation

- Pre-change audit: `docs/AUDIT-M0.7-BLOCK3.md`
- Second audit: `docs/SECOND-AUDIT-M0.7-BLOCK3.md`
- Incremental ledger: `docs/ERRORS-AND-FIXES.md`
- README: M0.7 Block 3 status synchronized
- Checkpoint: `docs/CHECKPOINT-M0.7-BLOCK3.md`

The final second audit is **PASS**.

## 6. Explicit boundaries retained

This checkpoint does not claim:

- v3.2;
- PQC;
- AAB cryptographic signing verification;
- runtime PackageManager/SigningInfo simulation;
- Play policy automation;
- unrelated algorithm expansion.

Those remain outside Block 3.

## 7. PR and repository state

The Block 3 change is carried by:

**PR #14 — M0.7 Block 3: APK Signature Scheme v3.1 verification**

- Base: `m07-block2-rotation-semantics`
- Head: `m07-block3-v31-verification`
- Merge: **not performed**

No merge is part of the checkpoint process.

## 8. Closure

**M0.7 Block 3 — FINALIZADO.**

The next authorized movement is M0.7 Block 4 / integration and completeness, and it must start from this checkpoint after its final Actions validation has passed.
