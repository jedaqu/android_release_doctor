# SECOND-AUDIT M0.7 Block 3 — APK Signature Scheme v3.1 verification

**Date:** 2026-10-01  
**Branch:** `m07-block3-v31-verification`  
**Base checkpoint:** `m07-block2-rotation-semantics`  
**Scope:** Re-audit of the Block 3 implementation before Actions.

## 1. Audit baseline

The second audit reviewed:

- `docs/AUDIT-M0.7-BLOCK3.md`;
- `docs/M0.7-DEFINITION.md`;
- `docs/SECOND-AUDIT-M0.7-DEFINITION.md`;
- the complete incremental ledger through ERR-059;
- the Block 2 checkpoint and final validation;
- the production diff against the Block 2 checkpoint;
- the v3.1 fixtures added to the branch;
- the Rust workflow and stacked PR trigger configuration.

No v3.2/PQC or unrelated cryptographic capability was introduced.

## 2. Implementation verification

The implementation now:

1. detects and separately stores the v3.1 signing block;
2. reuses the existing v3 cryptographic verification path for v3.1 supported algorithms;
3. preserves signer count, algorithms, certificate fingerprints, SDK ranges and proof-of-rotation evidence;
4. records v3 rotation-min-SDK metadata and development-release rotation metadata;
5. preserves the v3.1 signer-target range as authoritative evidence, including valid SDK 32 development-era/test-corpus configurations;
6. rejects a v3.1 block when the required v3 base block is absent;
7. parses the v3 stripping-protection rotation-min-SDK attribute;
8. requires that the v3 stripping-protection value match the v3.1 targeted minimum SDK;
9. verifies that v3 and v3.1 targeted ranges do not overlap and meet at the rotation boundary without an unintended gap;
10. requires compatible signer counts;
11. exposes certificate-level proof-of-rotation lineage evidence;
12. checks the v3 lineage as a prefix of a corresponding v3.1 lineage;
13. preserves unsupported versus invalid cryptographic states;
14. leaves v3.2/PQC outside the block.

## 3. Fixture and regression verification

Authoritative v3.1 APK fixtures were added from a public APK Signature Scheme test corpus:

- `crypto-v31-release.apk` — valid v3/v3.1 rotation configuration;
- `crypto-v31-no-v3.apk` — v3.1 without the required v3 base block;
- `crypto-v31-no-v3-attr.apk` — v3.1 with missing v3 stripping-protection attribute;
- `crypto-v31-wrong-rotation-min-sdk.apk` — v3.1 target and v3 stripping-protection value disagree;
- `crypto-v31-lineage-mismatch.apk` — v3/v3.1 lineage inconsistency.

The fixtures are real signed APK binaries; no synthetic certificate/signature acceptance path was introduced.

## 4. Implementation correction review

### ERR-058 / implementation substitution corruption

A broad source substitution temporarily leaked literal `${scheme_name}` labels and inserted evidence defaults into a helper signature.

The problem was caught during the implementation review before CI.

The final source restores the original v3 parser labels and confines v3.1 parameterization to the intended block identity, metadata and final evidence.

**Second-audit result: PASS.**

## 5. Scope review

The production change is confined to `signature_verify.rs` and its deterministic fixture/test coverage.

No changes were made to:

- v3.2/PQC;
- AAB cryptographic verification;
- Play policy rules;
- CLI architecture;
- unrelated cryptographic algorithm expansion;
- runtime Android trust-state simulation.

The existing M0.6 proof-of-rotation and M0.7 Block 1/2 tests remain part of the same regression suite.

## 6. CI review

The Block 3 branch workflow now includes:

- push target: `m07-block3-v31-verification`;
- pull-request target: `m07-block2-rotation-semantics`.

The validated Block 2 base workflow was also updated to target `m07-block3-v31-verification` for the stacked PR event.

These changes are recorded as ERR-059.

The job graph remains unchanged:

**Build → Test → Format → Clippy**

## 7. Acceptance matrix

| Requirement | Result |
|---|---|
| v3.1 block detected separately | PASS |
| Supported v3.1 cryptography verified through existing path | PASS |
| v3.1 SDK minimum boundary | PASS |
| v3 base block required | PASS |
| v3 stripping-protection attribute parsed | PASS |
| rotation-min-SDK cross-check | PASS |
| v3/v3.1 range overlap rejected | PASS |
| v3/v3.1 unintended gap rejected | PASS |
| signer-count consistency | PASS |
| lineage consistency evidence | PASS |
| malformed/invalid/unsupported distinction preserved | PASS |
| Multi-signer evidence preservation | PASS by preserved architecture |
| Existing Block 2 lineage evidence preserved | PASS |
| v3.2/PQC excluded | PASS |
| CI stack configured | PASS pending execution |

## 8. Second-audit conclusion

**SECOND AUDIT: PASS TO ACTIONS**

The implementation satisfies the declared M0.7 Block 3 scope at the source and fixture level.

The only remaining gate is Actions validation, followed by the established individual-correction loop for any concrete CI failure.

No checkpoint closure is authorized until Build, Test, Format and Clippy pass on the final branch state and the stacked PR event is validated.
