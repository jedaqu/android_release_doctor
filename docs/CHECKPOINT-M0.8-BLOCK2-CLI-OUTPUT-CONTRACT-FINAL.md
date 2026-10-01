# CHECKPOINT M0.8 Block 2 — CLI Output Contract

Date: 2026-10-01
Branch: `m08-block2-cli-output-contract`
Base checkpoint: `293dd056d74f9107ecc0372e43692576b85cce18`
Implementation validation commit: `c5411b59286617b83ccd8079dce4469c3c64a25a`
Final second-audit commit: `3b6f1efbe63f914eaa62ef80560a78aecb7fd50a`
PR: #18 — open, unmerged

## Final status

**FINAL CHECKPOINT — M0.8 Block 2 CLI Output Contract CLOSED.**

## Completed sequence

1. Block 1 final checkpoint verified as the immutable starting point.
2. Ledger reviewed through ERR-076.
3. Block 2 pre-flight audit: PASS.
4. CLI output/stream/exit contract frozen.
5. CLI implementation completed.
6. Focused CLI integration tests added.
7. README usage documentation added.
8. Second implementation audit: PASS.
9. ERR-079 resolved — M0.8 stacked CI trigger coverage.
10. ERR-080 resolved — validated artifact path type.
11. ERR-081 resolved — rustfmt normalization.
12. Actions #466 / run `36885586680`: Build, Test, Format, and Clippy all PASS.

## Frozen CLI contract

New:
- `--format text|json`;
- `--output <path>`;
- `--help`;
- `--version`.

Preserved:
- `--project`;
- `--play`;
- `--play-platform`;
- one APK/AAB artifact path;
- default text output;
- stdout report / stderr diagnostics;
- exit code 0 for no blockers;
- exit code 1 for blockers;
- exit code 2 for usage/audit/output/internal errors.

MANUAL-REVIEW findings remain non-blocking.

## Report integration

JSON mode consumes the already validated Report v1 DTO and serializer.

The CLI does not:
- create another schema;
- alter findings;
- duplicate audit logic;
- add timestamps or host metadata;
- change cryptographic behavior.

## Validation infrastructure

ERR-079 extended the existing Rust CI trigger branch matrix to cover the M0.8 stacked branches:
- `m08-block1-report-contract`;
- `m08-block2-cli-output-contract`.

The existing Build → Test → Format → Clippy job graph was not redesigned.

## Delivered files

- `crates/doctor-cli/src/main.rs`
- `crates/doctor-cli/tests/cli.rs`
- `README.md`
- `.github/workflows/rust.yml`
- `docs/AUDIT-M0.8-BLOCK2-CLI-OUTPUT-PREFLIGHT.md`
- `docs/M0.8-BLOCK2-CLI-OUTPUT-CONTRACT.md`
- `docs/SECOND-AUDIT-M0.8-BLOCK2-CLI-OUTPUT-CONTRACT.md`
- `docs/SECOND-AUDIT-M0.8-BLOCK2-CLI-OUTPUT-IMPLEMENTATION.md`
- `docs/ERRORS-AND-FIXES.md`

## Explicitly out of scope

- GitHub Action product integration;
- distribution/release packaging;
- batch/multi-artifact input;
- SARIF;
- HTML;
- report-schema changes;
- audit-rule changes;
- cryptographic changes;
- AAB final-APK signing verification;
- Gradle execution or variant evaluation.

## CI evidence

Implementation validation:
Actions #466 / run `36885586680`
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

A final Actions run for this checkpoint commit is required before the checkpoint is considered fully closed.

## Next authorized movement

After final checkpoint CI is terminal and successful:

**M0.8 Block 3 — GitHub Action**

That movement starts with its own ledger review and pre-audit. No merge of PR #18 is part of this checkpoint.
