# Android Release Doctor — Product Manifest v0.1.3 Candidate

## Identity

- Product: Android Release Doctor
- Candidate version: `0.1.3`
- Repository: `jedaqu/android_release_doctor`
- License: Apache-2.0
- Candidate status: prepared for manual publication review
- Publication status: not published by this document

## Maintained surfaces

1. Local CLI: `android-release-doctor`.
2. Shared Rust audit engine: `crates/doctor-core`.
3. Reusable GitHub composite Action: `.github/actions/android-release-doctor/action.yml`.
4. Report v1 JSON contract: `docs/report-schema-v1.0.json`.
5. Public fixtures and regression evidence.
6. Permanent validation through Rust CI and Action Validation.

## CLI contract

- Default output: human-readable text.
- Machine-readable output: Report v1 JSON through `--format json`.
- Optional file output: `--output <path>`.
- Optional static project cross-check: `--project <android-module>`.
- Optional Google Play profile: `--play`.
- Play platform: `mobile|wear|automotive|tv|xr`.
- Exit 0: completed audit with no blockers.
- Exit 1: completed audit with one or more blockers.
- Exit 2: usage/input/output/serialization/internal error.

## GitHub Action contract

Action:
`.github/actions/android-release-doctor/action.yml`

Inputs:
- `artifact`
- `project`
- `play`
- `play-platform`
- `format`
- `output`

Outputs:
- `exit-code`
- `report-path`

The Action executes the validated CLI through Cargo on the runner.

## Report contract

Report v1 schema:
`docs/report-schema-v1.0.json`

Schema version:
`1.0`

Validation is exercised by the permanent Action Validation workflow and by the external consumer evidence.

## Engineering evidence entering product preparation

- 147 Rust/CLI automated tests pass.
- Rust CI Build, Test, Format and Clippy pass.
- Permanent Action Validation passes its maintained graph.
- External consumer validation passed at immutable Action SHA.
- Independent public APK/AAB evidence has been exercised across Meshtastic, Reticulum and Ventoid.
- Public/private boundary scan is clean.

## Known boundaries

The candidate does not claim:
- v3.2/PQC cryptographic verification;
- AAB cryptographic signing verification;
- final generated-APK equivalence from raw AAB offsets;
- Android runtime trust-state simulation;
- proof of Play Console declarations that are not encoded in the artifact.

Unsupported/manual-review cases remain explicit.

## Publication rule

A future public GitHub Release/tag, if published, must be created manually from a fully validated final main commit and must record that exact immutable commit SHA.

This document does not publish or imply a release.
