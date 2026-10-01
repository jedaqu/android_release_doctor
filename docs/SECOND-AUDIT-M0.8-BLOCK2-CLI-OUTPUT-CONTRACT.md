# SECOND AUDIT M0.8 Block 2 — CLI Output Contract

Date: 2026-10-01
Branch: `m08-block2-cli-output-contract`
Base: `293dd056d74f9107ecc0372e43692576b85cce18`
PR: #18 — open, stacked, unmerged

## Result

**PASS — the CLI output contract is complete, internally consistent, and implementable without widening M0.8 scope.**

## Audit findings

### Contract to implementation mapping

| Contract requirement | Existing/new implementation point |
|---|---|
| `--format text|json` | Handwritten CLI argument parser |
| default `text` | existing renderer path |
| `--output <path>` | CLI report sink |
| `--help` | standalone CLI meta-command |
| `--version` | `doctor_core::ENGINE_VERSION` |
| JSON Report v1 | `ReportV1::from_audit_report` + `to_json()` |
| Play context | `ReportV1Context::with_play` / `without_play` |
| stdout/stderr separation | CLI sink/diagnostic functions |
| exit 0/1/2 | existing blocker/error behavior made explicit |
| MANUAL-REVIEW non-blocking | existing `counts()` semantics |

### Backward compatibility

The default invocation remains unchanged:
- one artifact;
- text report;
- stdout;
- exit 1 only for blockers;
- exit 2 for usage/audit errors.

Existing `--project`, `--play`, and `--play-platform` remain semantically unchanged.

### Report integrity

The CLI will not:
- modify the report schema;
- wrap JSON in another object;
- add timestamps or host metadata;
- recompute findings;
- alter blocker/manual-review semantics.

### Stream integrity

The CLI implementation must ensure JSON mode cannot mix diagnostics with stdout report bytes.

### Testing boundary

CLI tests can rely on existing repository fixtures:
- `tests/fixtures/minimal-release.apk` for successful report and JSON;
- `--play` with the default mobile profile for a deterministic blocker path because the fixture target SDK is below the current mobile requirement;
- temporary output paths for file-sink validation.

The core's existing Report v1 tests already validate JSON structure and validity, so CLI tests do not need a second schema validator.

## Corrections before implementation

None.

**Second audit status: PASS.**

Authorized next action:

**bounded CLI implementation + focused integration tests.**

No merge, GitHub Action, distribution, batching, or schema change is authorized.
