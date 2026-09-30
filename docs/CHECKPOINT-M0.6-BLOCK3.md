# Checkpoint — M0.6 Block 3

## State

M0.6 Block 3 is complete and validated on branch `m06-block3-verification-hardening`.

PR #10 remains open/draft and has not been merged.

## Starting point

Block 3 started from M0.6 Block 2 checkpoint:

`c99ff30c28022d70ebc2ce20e73becf99f97475e`

Base branch:

`m06-block2-signing-evidence`

## Block 3 audit findings

- AUDIT-009 / ERR-009 — v3 verification incorrectly rejected targeted multi-signer configurations.
- AUDIT-010 / ERR-010 — v3 signer SDK-range evidence was discarded from `CryptoSchemeInfo`.
- AUDIT-011 / ERR-011 — stacked M0.6 CI trigger coverage was incomplete.
- AUDIT-012 / ERR-012 — README repository status was stale.
- ERR-013 — compile failure after the v3 verification change.
- ERR-014 / AUDIT-013 — rustfmt failure caused by one extra blank line.
- ERR-015 — Clippy `needless_question_mark`.

The incremental ledger `docs/ERRORS-AND-FIXES.md` was reviewed before the correction cycle and updated throughout it.

## Implemented scope

Block 3:

- verifies each declared v3 signer independently;
- retains each verified signer minSDK/maxSDK range in `CryptoSchemeInfo.sdk_ranges`;
- merges multi-signer v3 evidence without claiming runtime-platform-specific signer selection;
- preserves v2 behavior with empty SDK-range evidence;
- updates stacked M0.6 CI trigger coverage;
- updates repository status documentation;
- records and corrects the validation-cycle failures without unrelated refactoring.

## Correction sequence

- ERR-013 correction: `1e7f1b6d4f306759558a0489a5acbde4bd3d288d`
- ERR-014 correction: `2962984babeea8f6ab86ee0df9da99bdbef44782`
- ERR-015 correction: `9126fd8d20fc49d410f640c345da15f3529a9f49`

Second audits:

- `docs/SECOND-AUDIT-M0.6-BLOCK3.md`
- `docs/SECOND-AUDIT-M0.6-BLOCK3-FORMAT.md`
- `docs/SECOND-AUDIT-M0.6-BLOCK3-CLIPPY.md`

## Final validation before checkpoint

Actions run #184 / `36745400537`:

- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

Validated head at the start of checkpoint documentation update:

`8cb575652a09d703d2af2d6c2c27df2486e4908e`

Ledger resolution update:

`aa337e4385ab29e32cb6b83d5e4540c12f0c588c`

## Acceptance

M0.6 Block 3 is checkpoint-ready. No merge is performed as part of this checkpoint.
