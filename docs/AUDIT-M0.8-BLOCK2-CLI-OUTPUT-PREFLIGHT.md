# M0.8 Block 2 — CLI Output Contract Pre-Flight Audit

Date: 2026-10-01
Branch: `m08-block2-cli-output-contract`
Base checkpoint: `293dd056d74f9107ecc0372e43692576b85cce18`
Prior PR: #17 — open, unmerged, mergeable
Planned PR: Block 2 stacked on `m08-block1-report-contract`

## Result

**PASS — bounded CLI output-contract implementation is authorized after this contract freeze.**

## 1. Current CLI surface

The CLI currently:
- parses `--project`, `--play`, and `--play-platform`;
- accepts exactly one APK/AAB artifact path;
- renders the human-readable report to stdout;
- writes diagnostics to stderr;
- returns exit code 0 with no blockers;
- returns exit code 1 when blockers exist;
- returns exit code 2 for usage/audit errors.

The parser is handwritten and has no external CLI framework.

## 2. Available report infrastructure

Block 1 already provides:
- `ReportV1`;
- `ReportV1Context`;
- compact JSON serialization through `to_json()`;
- pretty JSON serialization through `to_json_pretty()`.

No new JSON implementation belongs in Block 2.

## 3. Frozen CLI scope

Authorized user-facing additions:
- `--format text|json`;
- `--output <path>`;
- `--help`;
- `--version`.

Existing artifact/project/Play flags remain unchanged.

The default format remains `text`.

## 4. Output stream contract

### stdout
Contains report data only:
- text report for `--format text`;
- Report v1 JSON for `--format json`;
- help text for `--help`;
- version text for `--version`.

No diagnostics are written to stdout.

### stderr
Contains diagnostics only:
- usage errors;
- invalid option values;
- audit failures;
- output-file failures;
- serialization failures.

No human-readable diagnostic prefix is mixed into JSON stdout.

## 5. File output

When `--output <path>` is supplied:
- the selected report is written to that file;
- stdout receives no report bytes;
- diagnostics remain on stderr;
- an output write failure returns exit code 2.

The CLI does not add timestamps, metadata, hashes, or wrapper envelopes to the report.

## 6. Format semantics

### `text`
Uses the existing `AuditReport::render_text()` output unchanged.

### `json`
Uses `ReportV1Context` based on the existing Play invocation state and `ReportV1::to_json()`.
The emitted JSON is the frozen Report v1 object, followed by one CLI newline.

No alternative JSON schema is introduced.

## 7. Meta commands

`--help`:
- prints the complete current CLI usage/options;
- exits 0;
- is standalone.

`--version`:
- prints `android-release-doctor <engine-version>`;
- exits 0;
- is standalone.

Combining help/version with report-execution arguments is a usage error and exits 2.

## 8. Exit code contract

| Code | Meaning |
|---|---|
| 0 | command completed and no BLOCKER findings exist |
| 1 | audit completed and at least one BLOCKER finding exists |
| 2 | usage error, audit/input error, output-file error, or serialization/internal CLI error |

MANUAL-REVIEW findings do not alter the exit code.

This preserves the existing blocker semantics while making them explicit and stable for CI consumers.

## 9. Argument validation

The parser will reject:
- unknown options;
- missing values after `--project`, `--play-platform`, `--format`, or `--output`;
- duplicate `--project`, `--play`, `--play-platform`, `--format`, or `--output`;
- invalid format values;
- `--play-platform` without `--play`;
- zero or multiple artifact paths;
- report-execution arguments combined with `--help` or `--version`.

Option order remains unrestricted.

## 10. Tests authorized

New CLI integration tests will use the repository's existing APK fixture and the compiled CLI binary.

Required coverage:
- default text output remains stdout;
- JSON output is valid Report v1 and appears only on stdout;
- JSON output contains Play context when `--play` is supplied;
- output file receives the report and stdout remains empty;
- `--help` exits 0;
- `--version` exits 0;
- usage errors exit 2 and write diagnostics to stderr;
- blocker fixture/error path preserves exit code semantics;
- manual review does not become exit code 1.

No test requires a new external dependency.

## 11. Minimal file set

Expected implementation changes:
- `crates/doctor-cli/src/main.rs`;
- `crates/doctor-cli/tests/cli.rs`;
- optionally `README.md` for the new CLI usage section.

No changes are authorized to:
- cryptographic code;
- audit rules;
- Report v1 schema;
- Report v1 DTO model;
- GitHub Actions workflow;
- packaging/distribution.

## 12. Regression risks

Primary risks:
- accidental change to legacy text output;
- JSON diagnostics leaking into stdout;
- incorrect blocker exit semantics;
- loss of Play context in JSON;
- help/version changing normal artifact execution behavior;
- output-file errors being mistaken for audit results.

Mitigation:
- preserve `render_text()` unchanged;
- integration-test stdout/stderr separately;
- assert exact exit codes;
- validate JSON as `serde_json::Value`;
- test both stdout and file output paths.

## 13. Scope freeze

The CLI output contract is now bounded to the four new user-facing capabilities and the explicit stream/exit semantics above.

Next authorized movement:

**implement → focused CLI tests → second audit → Build → Test → Format → Clippy → correction if necessary → final checkpoint.**

No CLI framework migration, batching, SARIF, HTML, Action integration, or packaging is included.
