# M0.6 Block 2 — Pre-change audit

Date: 2026-09-30
Validated baseline: `3b50eab248359e1b2b923993c97a2b64e6874158`
Branch: `m06-block2-signing-evidence`

## Audit method

This audit starts from the validated M0.6 Block 1 checkpoint and inspects how the new first-class `ManualReview` state is used across the existing rule set, with special attention to signing evidence and release-readiness diagnostics.

Reviewed:
- `crates/doctor-core/src/lib.rs`
- `crates/doctor-core/src/play.rs`
- `crates/doctor-core/src/signature_verify.rs`
- integration tests in `crates/doctor-core/tests/fixtures.rs`
- `rules/README.md`
- M0.6 Block 1 checkpoint and its Actions validation

No production-source changes are included in this audit commit.

## Findings

### AUDIT-005 — Manual-review semantics are not propagated across existing rules

M0.6 Block 1 introduced `Severity::ManualReview`, but several existing rules still emit ordinary warnings for conditions explicitly described by the rule registry as manual-review boundaries.

Observed examples:
- `PLAY-002` Data Safety declaration;
- `PLAY-003` privacy policy;
- `PLAY-004` Play Console declarations;
- `PLAY-005` when ELF/ZIP evidence is unavailable;
- `NATIVE-002` when ELF parsing is unavailable;
- `NATIVE-003` when ZIP alignment cannot be proven, including the AAB boundary.

These are not confirmed failures of the artifact. They are conditions where the artifact cannot prove the required external property.

Required change: use first-class `ManualReview` for evidence that is explicitly outside artifact proof, while retaining `Warning` for genuine advisory conditions and `Blocker` for confirmed release-breaking conditions.

### AUDIT-006 — SIGNING-001 can produce a misleading warning for valid v2/v3-signed APKs

`SIGNING-001` currently warns whenever no `META-INF` signature files are present and recommends using a signed release artifact.

An APK signed with v2/v3 may legitimately contain no legacy `META-INF` v1 signature files. M0.5 now independently proves v2/v3 cryptographic signatures through `SIGNING-003`.

Therefore the current `SIGNING-001` message can incorrectly imply that a cryptographically verified APK is unsigned.

Required change: make `SIGNING-001` describe legacy/v1 signature material explicitly. When modern v2/v3 evidence is present, absence of `META-INF` files must not be presented as an unsigned-artifact warning. When no signing evidence exists, the warning may remain.

### AUDIT-007 — Existing tests encode the old warning semantics

The integration suite currently expects:
- `PLAY-002`, `PLAY-003`, `PLAY-004` = `Warning`;
- manual-review native evidence states = `Warning`.

Those assertions would permit future regressions back to the old ambiguous semantics.

Required change: update focused tests to assert the new first-class `ManualReview` state and explicitly preserve confirmed blockers and ordinary warnings.

### AUDIT-008 — Rule registry wording and runtime state need to be aligned

The rule registry already describes several conditions as manual-review items, but the runtime severity model historically represented them as warnings.

M0.6 Block 2 should finish that semantic alignment rather than adding another parallel classification.

Required change:
- document the runtime mapping of artifact-proof limits to `MANUAL-REVIEW`;
- document that `WARNING` remains for advisory findings that do not require an external verification step;
- document that `BLOCKER` remains reserved for confirmed release-breaking conditions.

## M0.6 Block 2 scope

Only the four findings above are in scope.

In scope:
- propagate `ManualReview` to existing artifact-proof/external-verification boundaries;
- reconcile `SIGNING-001` with modern v2/v3 cryptographic evidence;
- update focused unit/integration tests;
- update rule registry semantics.

Out of scope:
- new cryptographic algorithms;
- proof-of-rotation completion;
- v3.1/v3.2 cryptographic verification;
- AAB cryptographic verification;
- new Play policy requirements;
- new artifact inspection capabilities;
- broad report/CLI redesign.

## Acceptance criteria

1. Existing manual-review boundaries are represented as `MANUAL-REVIEW` rather than generic warnings.
2. Confirmed release-breaking conditions remain blockers.
3. Ordinary advisory warnings remain warnings.
4. A valid v2/v3-signed APK does not receive an `SIGNING-001` warning merely because legacy v1 `META-INF` files are absent.
5. An artifact with no signing evidence still produces an actionable signing warning.
6. Focused tests prove all changed semantics.
7. The rule registry explicitly documents the runtime severity mapping.
