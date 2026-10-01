# SECOND AUDIT M0.8 Block 2 — CLI Output Contract Implementation

Date: 2026-10-01
Branch: `m08-block2-cli-output-contract`
Base checkpoint: `293dd056d74f9107ecc0372e43692576b85cce18`
Implementation head before documentation: `87acad928d612f237b27f1143c5a46c3d25b4ae9`
PR: #18 — open, stacked, unmerged

## Result

**PASS — implementation matches the frozen CLI output contract and remains bounded.**

## Implementation review

### CLI surface
Implemented:
- `--format text|json`;
- `--output <path>`;
- `--help`;
- `--version`.

Existing:
- `--project`;
- `--play`;
- `--play-platform`;
- one artifact path.

### Report integration

JSON mode uses:
`ReportV1::from_audit_report(report, ReportV1Context)`
followed by `to_json()`.

No audit logic was duplicated into the CLI.

### Stream separation

- report output is stdout by default;
- file output suppresses stdout report bytes;
- errors remain stderr;
- JSON mode does not prepend human-readable text.

### Exit semantics

- 0: no blockers;
- 1: one or more blockers;
- 2: usage, audit/input, output, or serialization/internal error.

Manual-review findings remain non-blocking.

### Backward compatibility

The default invocation still renders `render_text()` and preserves the existing exit semantics.

### Tests

Focused integration coverage now exercises:
- default text output;
- JSON Report v1 output;
- Play context in JSON;
- output-file behavior;
- help;
- version;
- usage error;
- blocker exit code.

The repository's existing Report v1 tests remain the schema/JSON validity boundary.

## Scope review

Changed production area:
- `crates/doctor-cli/src/main.rs`

Validation infrastructure correction:
- `.github/workflows/rust.yml` only; trigger branch coverage was extended for the M0.8 stacked branches.
- The existing Build → Test → Format → Clippy job graph is unchanged.

Changed test area:
- `crates/doctor-cli/tests/cli.rs`

Documentation:
- Block 2 preflight;
- Block 2 contract;
- Block 2 second audit;
- README usage update;
- chronological ERR-079 validation-infrastructure correction.

No changes to:
- audit engine;
- Report v1 schema/model;
- cryptographic verification;
- GitHub workflow;
- packaging;
- batching or alternate report formats.

## Final audit verdict

**PASS with validation-infrastructure correction ERR-079.**

The correction is limited to CI trigger coverage and introduces no product behavior change.

Authorized next step:
**Build → Test → Format → Clippy.**
If a gate fails, apply the ERR-NNN protocol and repeat the second audit and full gate.


## Final validation evidence

- ERR-079: RESOLVED — M0.8 branch trigger coverage added to the existing Rust CI matrix.
- ERR-080: RESOLVED — validated artifact path stored as `PathBuf` after parser validation.
- ERR-081: RESOLVED — rustfmt corrections applied to CLI production/test files.
- Actions #466 / run `36885586680` on corrected commit `c5411b59286617b83ccd8079dce4469c3c64a25a`:
  - Build: PASS
  - Test: PASS
  - Format: PASS
  - Clippy: PASS

**Final second-audit verdict: PASS.**
