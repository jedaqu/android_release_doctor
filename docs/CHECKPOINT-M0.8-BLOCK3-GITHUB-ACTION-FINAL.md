# CHECKPOINT M0.8 Block 3 — GitHub Action

Date: 2026-10-01
Branch: `m08-block3-github-action`
Block 2 baseline: `9072234d09f71e3eaa362335674395fd0fd9f8e9`
Current implementation/documentation head before checkpoint commit: `cf180e6a5173a362edf699346e0ced8ee766b3e0`
PR: #19 — open, unmerged
Base: `m08-block2-cli-output-contract`

## Status

**FINAL CHECKPOINT CANDIDATE — M0.8 Block 3 GitHub Action implementation is complete, subject only to terminal CI validation of this checkpoint commit.**

## Completed sequence

1. Block 2 final state verified.
2. M0.8 Block 3 preflight audit: PASS.
3. GitHub Action contract frozen.
4. Composite action implemented at `.github/actions/android-release-doctor/action.yml`.
5. Focused action self-test workflow implemented.
6. README usage documentation added.
7. Rust CI push coverage extended for the Block 3 head branch.
8. Second audit completed.
9. ERR-082 resolved — action metadata YAML parse failure.
10. ERR-083 resolved — malformed GitHub expression in Bash command invocation.
11. Corrected implementation validation:
   - Action run #18 / `36887625607`: PASS;
   - Rust CI run #489 / `36887626517`: Build PASS, Test PASS, Format PASS, Clippy PASS.

## Frozen Block 3 contract

The action:
- accepts one APK/AAB artifact;
- optionally accepts project, Play, Play platform, output format, and report path;
- invokes the already-validated `doctor-cli`;
- exposes `exit-code` and `report-path`;
- preserves CLI exit codes 0/1/2;
- preserves Report v1 JSON semantics;
- does not implement audit rules.

## Validation workflow

`.github/workflows/m08-block3-action.yml` validates:
- successful JSON execution;
- Report v1 `schema_version = "1.0"`;
- blocker exit code 1;
- blocker report preservation;
- operational error exit code 2.

Rust CI remains the unchanged Build → Test → Format → Clippy gate.

## Explicitly out of scope

- audit-engine changes;
- Report v1 schema/model changes;
- CLI contract changes;
- cryptographic changes;
- Gradle execution/variant evaluation;
- SARIF/HTML;
- batching;
- distribution/release packaging;
- prebuilt binary/toolchain packaging;
- Play automation;
- merge.

## Closure condition

This checkpoint becomes formally **CLOSED** only when the GitHub Actions runs associated with this exact checkpoint commit are terminal and successful for:

1. Rust CI — Build;
2. Rust CI — Test;
3. Rust CI — Format;
4. Rust CI — Clippy;
5. M0.8 Block 3 Action self-test.

No implementation or documentation changes are permitted after this checkpoint commit before that terminal validation is evaluated.
