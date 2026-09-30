# M0.6 Block 3 — Pre-change audit

Date: 2026-09-30
Validated baseline: `c99ff30c28022d70ebc2ce20e73becf99f97475e`
Branch: `m06-block3-verification-hardening`

## Audit method

This audit starts strictly from the validated M0.6 Block 2 checkpoint. No production-source change is included in this audit document.

Reviewed:
- `crates/doctor-core/src/signature_verify.rs`
- `crates/doctor-core/src/lib.rs`
- `.github/workflows/rust.yml`
- `rules/README.md`
- `README.md`
- M0.6 Block 2 checkpoint and second-audit documents
- existing v2/v3 unit and integration coverage
- current Android/AOSP APK-signing documentation and the Android apksig implementation

The audit is limited to correctness of the existing APK v3 verification boundary and validation workflow. It does not introduce new cryptographic algorithms or complete proof-of-rotation/v3.1/v3.2 verification.

## Findings

### AUDIT-009 — v3 verification rejects targeted multi-signer configurations

Current `verify_v3_block()` requires exactly one parsed v3 signer:

```text
if signer_values.len() != 1 { ... Invalid ... }
```

The Android/apksig model permits v3 signing configurations with multiple signers mapped to different targeted platform versions. The v3 format carries `minSDK` and `maxSDK` for each signer, and Android verification selects the signer whose range applies to the current platform. The apksig tool documentation explicitly describes multiple v3 signers for different targeted platform versions.

The current verifier does not model this boundary and can therefore report a structurally legitimate targeted-signing configuration as cryptographically invalid solely because more than one signer is present.

Required change:
- parse and verify each declared v3 signer instead of rejecting the block solely on signer count;
- retain each signer's minSDK/maxSDK evidence in the verification result;
- preserve the existing proof-of-rotation manual-review boundary;
- do not claim a runtime-platform-specific signer choice that the artifact audit cannot establish.

Reference:
- https://source.android.com/docs/security/features/apksigning/v3
- https://android.googlesource.com/platform/tools/apksig/+/master/src/main/java/com/android/apksig/ApkSigner.java

### AUDIT-010 — v3 signer range evidence is not exposed

The verifier parses v3 minSDK/maxSDK values and checks the outer/inner equality, but the resulting `CryptoSchemeInfo` discards those ranges.

This loses useful artifact evidence and makes it impossible for downstream consumers to distinguish:
- a single unrestricted v3 signer;
- multiple targeted signing configurations;
- a signer range that is valid internally but requires platform-specific interpretation.

Required change:
- expose the verified v3 SDK ranges in the crypto evidence model;
- keep v2 output unchanged;
- keep the report wording conservative: the tool may verify the artifact's declared signer ranges, but it must not pretend to select the signer for a specific runtime platform when that runtime platform is not an audit input.

### AUDIT-011 — CI pull-request trigger coverage is incomplete for stacked M0.6 validation

The current `.github/workflows/rust.yml` push trigger includes the M0.6 branches, but the `pull_request.branches` filter still contains only the older base branches.

The workflow therefore does not guarantee PR-event validation when an M0.6 Block 3 PR targets the validated M0.6 Block 2 branch.

This contradicts the intended stacked validation discipline documented by the earlier M0.6 Block 2 audit.

Required change:
- include the validated M0.6 Block 2 branch in PR triggers;
- retain the older validated base branches;
- include the new Block 3 push branch in push triggers;
- do not redesign the CI job graph.

### AUDIT-012 — Repository status documentation is stale relative to the validated milestone state

`README.md` still labels the repository as M0.5 Block 2 even though M0.6 Blocks 1 and 2 have been validated and Block 3 is now beginning.

This is documentation drift rather than a runtime defect, but it makes the repository's stated maturity inconsistent with the checkpoint history.

Required change:
- update the top-level status to reflect M0.6 Block 3 in progress;
- keep historical checkpoint/audit documents unchanged as records.

## M0.6 Block 3 scope

Only these findings are in scope:

1. harden v3 verification for targeted multi-signer configurations;
2. expose verified v3 SDK-range evidence without making unsupported runtime-platform claims;
3. correct stacked M0.6 CI trigger coverage;
4. align the top-level repository status with the validated milestone.

## Out of scope

- new RSA/EC/DSA/PQC algorithms;
- broader cryptographic backend replacement;
- complete proof-of-rotation verification;
- complete v3.1 cryptographic verification;
- complete v3.2 hybrid/PQC verification;
- AAB cryptographic verification;
- new Google Play policy rules;
- broad report/CLI redesign;
- unrelated refactoring.

## Acceptance criteria

1. A v3 block containing multiple targeted signer entries is not rejected solely because multiple signers exist.
2. Each v3 signer's cryptographic evidence and minSDK/maxSDK range are retained.
3. Existing single-signer v3 fixtures remain verified.
4. Unsupported algorithms, proof-of-rotation, v3.1, and v3.2 retain their existing manual-review boundaries.
5. No current blocker/warning/manual-review semantics are regressed.
6. CI push and PR trigger coverage includes the active stacked M0.6 validation path.
7. The repository status text matches the current milestone.
8. Focused regression tests cover the changed v3 and workflow-facing behavior.
9. No merge is performed as part of this block.
