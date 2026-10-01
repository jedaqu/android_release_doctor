# M0.8 Block 3 — GitHub Action Preflight Audit

Date: 2026-10-01
Branch: `m08-block3-github-action`
Baseline: `9072234d09f71e3eaa362335674395fd0fd9f8e9`

## Result

**PASS — bounded implementation is authorized for the GitHub Action integration.**

## Baseline verification

- M0.8 Block 2 is closed.
- Block 2 documentary HEAD is `9072234d09f71e3eaa362335674395fd0fd9f8e9`.
- The documentation-only ledger observation is present and recorded as a process observation, not a new ERR entry.
- Actions run `36886699545` for the documentary HEAD is terminal and passed Build, Test, Format, and Clippy.
- PR #18 remains open and unmerged.
- The M0.8 sequence authorizes Block 3 as the next movement.

## Existing contracts carried forward

### Report v1

Block 3 must consume the existing Report v1 serializer through the CLI. It must not create or transform a second report model.

Required invariant:
- JSON output remains Report v1 schema v1.0.
- No timestamps, host metadata, invented artifact hashes, or schema expansion.

### CLI

Block 3 must call the already-frozen CLI contract:

`--format <text|json>`
`--output <path>`
`--project <android-module>`
`--play`
`--play-platform <mobile|wear|automotive|tv|xr>`
one APK/AAB artifact path.

Exit semantics remain:
- 0: audit completed with zero BLOCKER findings;
- 1: audit completed with one or more BLOCKER findings;
- 2: usage/input/audit/output/internal error.

The action must not duplicate or reinterpret these semantics.

## GitHub Action design boundary

The selected mechanism is a GitHub **composite action** located at:

`.github/actions/android-release-doctor/action.yml`

The action will:

1. receive one artifact path and the already-frozen CLI options;
2. invoke the existing `doctor-cli` through Cargo from the action repository source;
3. expose the CLI exit code as an action output;
4. expose the requested report path when file output is requested;
5. preserve the CLI's non-zero exit status so blockers and operational failures fail the action step.

GitHub documents composite actions as metadata-driven step bundles and supports `GITHUB_ACTION_PATH` / `github.action_path` for files in the action repository. The self-repository action reference can resolve the action at the running commit without requiring an application checkout for the action itself.

## Trigger coverage pre-audit

The preventive process observation from Block 2 is applied here before implementation.

The existing Rust workflow currently covers:
- push: M0.8 Block 1 and Block 2 branches;
- pull_request: M0.8 Block 1 and Block 2 target branches.

The new Block 3 branch is not yet in that matrix. This is expected at pre-audit and must be corrected only as part of Block 3 validation infrastructure, with no job-graph redesign.

The new Block 3 action self-test workflow will independently declare:
- push coverage for `m08-block3-github-action`;
- pull_request target coverage for the actual stacked base, `m08-block2-cli-output-contract`.

## Scope

### In scope

- `.github/actions/android-release-doctor/action.yml`;
- action inputs/outputs contract;
- a focused self-test workflow for the action;
- focused action documentation;
- README usage example;
- required CI trigger coverage for the new stacked branch;
- chronological ERR ledger entries only if Actions exposes an actual defect.

### Explicitly out of scope

- audit engine changes;
- Report v1 schema/model changes;
- CLI contract changes;
- new exit codes;
- cryptographic changes;
- distribution/release packaging;
- prebuilt binary distribution;
- Gradle execution/variant evaluation;
- Play automation;
- SARIF/HTML;
- batching/multi-artifact input;
- merge of PR #18;
- unrelated workflow redesign.

## Acceptance boundary

The block can close only after:

1. action metadata and contract match this preflight;
2. success path produces a report;
3. JSON path preserves Report v1;
4. blocker path returns CLI exit code 1 and preserves report output;
5. operational/configuration error path remains exit code 2;
6. second audit passes;
7. Build → Test → Format → Clippy pass;
8. action self-test workflow passes;
9. documentation and ledger are updated;
10. final checkpoint records terminal CI evidence.

No implementation change is authorized outside this boundary.
