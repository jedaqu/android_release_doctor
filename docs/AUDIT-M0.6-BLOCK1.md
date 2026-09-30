# M0.6 Block 1 — Pre-change audit

Date: 2026-09-30
Validated baseline: `2e76c460d8e1ed190e7594e12e9bb9f98f0c8b42`
Branch: `m06-block1-state-hardening`

## Audit method

This audit was performed against the validated M0.5 Block 2 implementation baseline before any M0.6 source changes.

Reviewed:
- `doctor-core` audit/report model in `crates/doctor-core/src/lib.rs`
- APK signing structure and cryptographic verification in `signing.rs` and `signature_verify.rs`
- native/ZIP evidence paths
- rule registry
- Rust CI workflow
- existing unit/integration coverage
- validated CI run `36730441553` (#147)

No production source changes are part of this audit commit.

## Findings

### AUDIT-001 — Manual-review state is not first-class

The public `Finding` model has only `Pass`, `Warning`, and `Blocker`.

The M0.5 cryptographic verifier already distinguishes `Verified`, `Invalid`, and `Unsupported`, but `evaluate()` collapses unsupported/incomplete verification into a normal warning for `SIGNING-003`.

The rule registry itself describes unsupported/incomplete cryptographic verification as a manual-review condition.

Consequences:
- manual review cannot be counted separately;
- the rendered report cannot distinguish advisory warnings from required manual verification;
- downstream automation cannot reliably tell "not proven" from "warning";
- the semantic distinction introduced by M0.5 is lost at the reporting boundary.

Required change: introduce a first-class manual-review finding state while keeping blocker exit semantics unchanged.

### AUDIT-002 — v2/v3 certificate sequences are parsed as exactly one certificate

`parse_signed_data_v2()` and `parse_signed_data_v3()` read one length-prefixed certificate and immediately call `finish()` on the complete certificate sequence.

This means a valid signer certificate chain containing additional certificates is treated as trailing data and can be reported as cryptographic failure.

Required change: parse the complete certificate sequence, retain the first certificate as the signer certificate used for public-key binding/fingerprint evidence, and accept additional chain certificates instead of rejecting the sequence for containing them.

This is a correctness hardening change, not a relaxation of cryptographic verification of the signer itself.

### AUDIT-003 — CI trigger coverage does not include the next stacked milestone

`.github/workflows/rust.yml` currently includes `m05-crypto-signatures` in push triggers, but the pull-request branch filter does not include `m05-crypto-signatures`.

The workflow therefore does not provide the same automatic PR validation path for a PR whose base is the validated M0.5 Block 2 branch.

Required change: extend the CI branch trigger coverage for the M0.6 development branch and the validated M0.5 Block 2 base used by the stacked workflow.

### AUDIT-004 — Regression coverage is missing for the two boundaries above

Existing tests cover:
- valid real v2/v3 fixtures;
- tampering;
- algorithm selection;
- SDK-range consistency;
- unsupported key-size boundaries.

They do not directly cover:
- report-level preservation of an explicit manual-review state;
- acceptance of additional certificates in a v2/v3 certificate sequence.

Required change: add focused regression tests for both boundaries.

## M0.6 Block 1 scope

Only the four findings above are in scope for the first implementation block.

Out of scope for this block:
- proof-of-rotation cryptographic completion;
- v3.1 cryptographic verification;
- v3.2 hybrid/PQC verification;
- AAB cryptographic signing verification;
- broad refactoring of the audit architecture;
- new Play policy rules.

## Acceptance criteria

1. Manual-review findings are represented explicitly in the report model.
2. `SIGNING-003` uses the explicit manual-review state for unsupported/incomplete verification.
3. Existing blockers remain blockers and CLI exit code semantics remain unchanged.
4. v2/v3 certificate sequences with additional chain certificates no longer fail solely because trailing certificates are present.
5. Focused regression tests prove both behaviors.
6. CI workflow triggers cover the stacked M0.6 validation path.
7. Existing M0.1-M0.5 behavior remains covered.
