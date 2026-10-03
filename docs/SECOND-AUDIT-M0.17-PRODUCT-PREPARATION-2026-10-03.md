# M0.17 — Product Preparation / v0.1.3 Second Audit — 2026-10-03

## Baseline

- Prepared against main: `34d9c732c9f115e9c7478545fc22ea02a52d4a0f`.
- Cargo workspace version: `0.1.3`.
- M0.16 post-merge Action Validation: Run `37142095746` — SUCCESS.
- M0.16 post-merge Rust CI: Run `37142095776` — SUCCESS.

## User-facing contract audit

### CLI

The README and product manifest agree on:
- human-readable default output;
- Report v1 JSON via `--format json`;
- optional `--output`;
- optional `--project`;
- optional `--play` and supported platform selection;
- exit codes 0/1/2.

### Reusable Action

The README, product manifest, and actual `.github/actions/android-release-doctor/action.yml` agree on:
- inputs: artifact, project, play, play-platform, format, output;
- outputs: exit-code, report-path;
- Cargo/Rust runner requirement;
- stable exit semantics.

The README uses an explicit reviewed reference placeholder and recommends immutable commit-SHA pinning rather than a moving `@main` reference.

### Report v1

The product manifest identifies `docs/report-schema-v1.0.json` as the machine-readable contract. Runtime schema validation is already exercised by permanent CI and the external consumer evidence.

## Publication preparation audit

Confirmed:
- no public GitHub release is claimed as already published;
- no tag is claimed as already created;
- publication steps are documented as manual owner actions;
- no release-build GitHub Action was added;
- no new binary-distribution contract was introduced.

## Product code audit

Comparison against the M0.17 base shows documentation-only additions. No Rust source, CLI implementation, Action implementation, schema, fixture binary, or permanent workflow changed.

## Public/private audit

M0.17 adds only public product-preparation material:
- product manifest;
- candidate release notes;
- preparation checklist;
- milestone checkpoints/audits.

No private continuity, credentials, commercial strategy, pricing, or monetization content is introduced.

## Disposition

M0.17 preparation objective is satisfied.

The public product is prepared as a v0.1.3 candidate. Publication itself remains a separate manual owner action at a future exact main commit.
