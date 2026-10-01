# SECOND AUDIT M0.8 Block 3 — GitHub Action Implementation

Date: 2026-10-01
Branch: `m08-block3-github-action`
Baseline: `9072234d09f71e3eaa362335674395fd0fd9f8e9`
Implementation head before CI correction: `ee5fa6123027081926f42ded1e178d187be38723`
Validation commit: `f56eb1448264ef3ce09ccb07b436bceed02388a0`
Documentation head pending final checkpoint: `00f1c015575663d858ac37c5ab8f155e9a38ffa0`

## Result

**PASS — the bounded implementation matches the frozen Block 3 contract; Actions validation remains the final external gate.**

## Implementation review

### Action metadata

Implemented:
- `.github/actions/android-release-doctor/action.yml`;
- required `artifact` input;
- optional `project`, `play`, `play-platform`, `format`, and `output` inputs;
- `exit-code` and `report-path` outputs;
- composite execution through the repository CLI.

The action resolves the workspace Cargo manifest through:
`$GITHUB_ACTION_PATH/../../../Cargo.toml`.

The action explicitly receives `github.action_path` through the environment, matching the nested action location.

### Contract preservation

The action forwards existing CLI controls and does not reimplement audit logic.

Preserved:
- Report v1 JSON selection;
- stdout/file output semantics;
- CLI exit codes 0/1/2;
- existing project and Play options;
- one-artifact input boundary.

No second report model or alternate exit-code scheme was introduced.

### Exit propagation

The CLI status is captured before writing action outputs and then returned unchanged.

Therefore:
- blocker audits remain action failures with exit code 1;
- operational failures remain action failures with exit code 2;
- successful audits remain exit code 0.

When file output is requested, the CLI writes the report before returning its blocker status, so a blocking report remains available for subsequent workflow steps.

### Input handling and security

The implementation:
- constructs a Bash argument array;
- passes inputs as discrete arguments rather than shell-expanded command fragments;
- rejects invalid boolean values for `play`;
- rejects CR/LF in `output` before writing the action output file;
- does not download or execute user-supplied scripts;
- does not upload artifacts or telemetry;
- does not modify the audited artifact.

### Validation workflow

Added:
`.github/workflows/m08-block3-action.yml`

It explicitly validates:
- push coverage on `m08-block3-github-action`;
- pull_request coverage against the actual stacked base `m08-block2-cli-output-contract`;
- successful JSON report generation;
- Report v1 `schema_version = "1.0"`;
- blocker exit code `1` and preserved JSON report;
- operational error exit code `2`.

### Rust CI trigger coverage

Updated:
`.github/workflows/rust.yml`

Only the push branch matrix was extended with `m08-block3-github-action`.
The Build → Test → Format → Clippy job graph remains unchanged.

The pull_request target `m08-block2-cli-output-contract` was already present and therefore required no modification.

## Scope review

### In scope and changed

- GitHub composite action metadata;
- focused action self-test workflow;
- Block 3 contract/preflight/second-audit documentation;
- README action usage;
- Rust CI push trigger coverage for the new stacked branch.

### Unchanged

- audit engine;
- Report v1 schema/model;
- CLI implementation and CLI contract;
- cryptographic verification;
- Gradle evaluation;
- release/distribution packaging.

No unrelated refactor or cleanup was introduced.

## Known prerequisite

The Block 3 action executes the validated CLI through Cargo. A runner with a usable Rust/Cargo toolchain is therefore required.

Prebuilt binary distribution and toolchain packaging remain outside this block.

## CI correction — ERR-082

Actions #10 / run `36887334695` failed before job setup because GitHub could not parse the action metadata at line 134.

The failure was isolated to malformed YAML introduced by the CR/LF output-path validation insertion. No runtime audit behavior executed.

Correction:
- replace only `.github/actions/android-release-doctor/action.yml`;
- preserve the frozen inputs/outputs and Cargo invocation boundary;
- keep the CR/LF output-path guard in valid Bash syntax.

The correction is recorded chronologically as ERR-082 in `docs/ERRORS-AND-FIXES.md`.

## CI correction — ERR-083

Actions #12 / run `36887450382` reached action-manifest validation and found an unclosed GitHub expression caused by the previous bounded file-generation correction.

The failure was isolated to the Bash command invocation line. No audit execution occurred.

Correction:
- replace only the malformed command expansion with the intended Bash array invocation;
- preserve all action inputs, outputs, validation, and CLI execution semantics;
- record the event chronologically as ERR-083.

## Final audit verdict after ERR-083 correction

**PASS — correction is bounded, and the corrected action metadata is semantically aligned.**

## Final validation evidence

- ERR-082: RESOLVED.
- ERR-083: RESOLVED.
- M0.8 Block 3 Action run #18 / `36887625607`: PASS.
  - Success path: PASS
  - Report v1 JSON validation: PASS
  - Blocker path / exit code 1: PASS
  - Blocker report preservation: PASS
  - Operational error / exit code 2: PASS
- Rust CI run #489 / `36887626517`: PASS.
  - Build: PASS
  - Test: PASS
  - Format: PASS
  - Clippy: PASS

These runs validate the corrected implementation commit `f56eb1448264ef3ce09ccb07b436bceed02388a0`.

A final checkpoint documentation commit and terminal Build/Test/Format/Clippy + action self-test run are still required before the block is formally closed.

If any gate fails, the ERR-NNN correction protocol applies.
