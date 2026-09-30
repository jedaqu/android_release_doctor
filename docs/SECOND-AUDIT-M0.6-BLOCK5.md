# M0.6 Block 5 — Second audit before Actions

Date: 2026-09-30
Baseline: `d1dd8cbf596f06572ba9b883528b5133ca28d176`
Working branch: `m06-block5-verification-completeness`

## Audit discipline

This second audit was performed after the scoped Block 5 implementation and before Actions.

The current `docs/ERRORS-AND-FIXES.md` ledger was reviewed first. ERR-001 through ERR-023 remain present and resolved. ERR-024 through ERR-026 remain pending. ERR-027 was corrected and marked resolved after the audit-document citation fix. Newly discovered findings from this second audit are recorded as ERR-028 and ERR-029.

Historical entries were not rewritten or renumbered.

## Audit result

### AUDIT-024 — v3 proof-of-rotation verification

**PASS with scope boundary.**

The implementation now parses the versioned proof-of-rotation structure, validates the parent-to-child signing relationship using the previous certificate and declared algorithm, enforces algorithm linkage, rejects duplicate certificates, requires a terminal next-signature algorithm of zero, and checks that the final lineage certificate matches the current v3 signer certificate.

The implementation follows the current AOSP lineage structure and verification sequence.

Reference:
https://android.googlesource.com/platform/tools/apksig/+/refs/heads/main/src/main/java/com/android/apksig/internal/apk/v3/V3SigningCertificateLineage.java

Unsupported lineage algorithms remain `Unsupported` rather than being guessed as verified.

### AUDIT-025 — structured proof-of-rotation evidence

**PASS with one propagation defect noted below.**

The evidence model now contains `ProofOfRotationInfo` with:
- verification state;
- lineage level count;
- concise verification detail.

The merged scheme result preserves this evidence.

However, AUDIT-028 identifies later signer-failure paths that currently drop already-parsed proof-of-rotation evidence.

### AUDIT-026 — CI stacked validation coverage

**PASS at configuration level; runtime validation still pending.**

The Block 5 branch is present in push coverage.

The Block 4 base workflow now includes `m06-block4-signer-error-isolation` as a pull-request target, which is the required base-workflow condition for a stacked Block 5 PR.

Actions validation is intentionally not performed until the second-audit failures are corrected.

### AUDIT-027 — proof-of-rotation fixture is not reachable from the working branch

**FAIL.**

Commit `f40076f941813f398b6c07441c96377c69741cbd` contains the fixture object, but the current Block 5 branch history does not include that commit and the current branch tree contains no:

`tests/fixtures/proof-rotation-valid.bin`

The Block 5 tests reference that exact path.

This is a reproducibility defect and must be corrected before Actions.

Tracked as ERR-028.

### AUDIT-028 — proof-of-rotation evidence is dropped on later signer failures

**FAIL.**

After `parse_signed_data_v3()` has successfully established proof-of-rotation evidence, later signer-local failures such as unsupported signature selection, certificate/public-key mismatch, signature verification failure, content-digest failure, or algorithm-list inconsistency can be converted through `error_to_scheme_info_with_evidence()` without forwarding the parsed proof evidence.

That breaks the Block 5 evidence-retention requirement for a signer that was already parsed far enough to establish the lineage result.

Tracked as ERR-029.

## Documentation review

**PASS.**

- Block 5 audit source references are now plain stable URLs; no chat/UI citation markers remain.
- README status reflects M0.6 Block 5.
- `rules/README.md` reflects the new proof-of-rotation evidence boundary.
- The ledger remains chronological and historical entries remain intact.

ERR-027 is therefore resolved.

## Scope review

**PASS.**

No new algorithm family, v3.1/v3.2 completion, AAB cryptographic verification, Play policy expansion, CLI redesign, or unrelated refactoring was introduced.

## Action gate

**BLOCKED.**

Do not run the release validation gate yet.

Required individual corrections:
1. connect the proof-of-rotation fixture to the Block 5 branch history;
2. preserve parsed proof-of-rotation evidence across every later signer-local failure conversion;
3. rerun this second audit against the corrected tree;
4. only after the corrected second audit passes, run Actions and validate the Block 4 base workflow;
5. then exercise the stacked PR event and continue toward the Block 5 checkpoint.

No merge is performed.
