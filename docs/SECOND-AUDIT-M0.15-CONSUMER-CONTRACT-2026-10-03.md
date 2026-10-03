# M0.15 — Consumer Contract Validation Second Audit — 2026-10-03

## Baseline and audit head

- Base main: `d1d681e46391c95fd77152779ff4e3656995837f`
- M0.15 public validation branch audit head: `ec60f71f79291b2acb6c518ddaf8f6bdab95b601`
- Scope on public product repository: documentation only
- Production-code changes: none

## External consumer execution

A genuinely separate consumer repository executed the published composite Action from the immutable current product revision:

`jedaqu/android_release_doctor/.github/actions/android-release-doctor@d1d681e46391c95fd77152779ff4e3656995837f`

The consumer workflow was isolated from the product repository and did not copy the Action source into its own tree.

## Validation evidence

### Success path

- Artifact: Meshtastic Android v2.8.1 public release APK.
- SHA-256 observed by the consumer run:
  `7f42735fd1c17c7e6a64d3a48ae1e4baf22cab778e994eb8331d3b993e42eb00`.
- Action result: `exit-code=0`.
- `report-path` matched the supplied output path.
- Generated Report v1 was non-blocking and validated against the published schema.

### Blocking path

- Artifact: maintained public `minimal-release.apk` fixture downloaded from the immutable product revision.
- Action inputs included `play=true` and `play-platform=mobile`.
- Action result: `exit-code=1`.
- `report-path` matched the supplied output path.
- Generated Report v1 contained blockers and validated against the published schema.

### Invalid-input path

- Artifact: maintained public `minimal-release.apk` fixture.
- Invalid Action input: `format=yaml`.
- Action result: `exit-code=2`.
- `report-path` matched the supplied output path.
- No report file was created.

## Runner/toolchain evidence

The external consumer runner exposed:
- Cargo 1.98.1.
- rustc 1.98.1.
- JSON Schema validation through `jsonschema==4.25.1`.

The published Action executed from the immutable SHA without any production-source checkout into the consumer repository.

## Test-harness corrections

Three initial consumer-workflow executions failed before the actual product contract paths were exercised:

- Run `37140931651`: schema download was placed after the success validation step.
- Run `37140968978`: the consumer workflow contained malformed GitHub expression interpolation in temporary paths.
- Run `37140991766`: the workflow retained an extra closing brace in GitHub expressions.

These were consumer-test harness defects. They did not change or expose a product defect.

Final corrected consumer execution:
- Run `37141021253`: SUCCESS.
- All consumer validation steps completed successfully.

## Public-repository diff audit

Comparison of M0.15 public branch against base main shows only documentation changes:
- M0.14 closure checkpoint reconciliation;
- M0.15 start checkpoint;
- M0.15 pre-audit;
- current product-state reconciliation.

No Rust source, Action implementation, CLI behavior, schema, fixture binary, or permanent product workflow was changed.

## Product-defect assessment

No reproducible product defect was exposed by M0.15.

## Disposition

M0.15 objective is satisfied. The published Action contract has now been validated from a genuinely separate consumer repository at an immutable SHA across success, blocker, and invalid-input paths, with Report v1 schema validation.

The private consumer repository is intentionally not named in the public product documentation.
