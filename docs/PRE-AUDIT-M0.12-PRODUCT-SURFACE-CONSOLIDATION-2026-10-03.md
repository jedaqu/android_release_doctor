# M0.12 — Product Surface Consolidation Pre-Audit — 2026-10-03

## Baseline

- Repository: `jedaqu/android_release_doctor`
- Visibility: public
- Default branch: `main`
- Audited baseline HEAD: `325154034f965de475d4214959d9608c60f82de8`
- Work branch: `product/m012-surface-consolidation`
- Previous closed milestone: M0.11-A — GitHub Action Hardening

## Mandatory pre-audit scope

This block consolidates the technical product surface without introducing new audit capabilities.

The permanent user-facing technical surfaces are:

1. `crates/doctor-core` — Rust audit engine.
2. `crates/doctor-cli` — command-line interface.
3. `.github/actions/android-release-doctor/action.yml` — reusable GitHub composite Action.
4. Report v1 and `docs/report-schema-v1.0.json` — machine-readable output contract.
5. Fixtures and regression tests supporting the above surfaces.
6. Permanent CI that validates the maintained product surface.

Historical campaign/stress/temporary workflows are development evidence, not product interfaces.

## Findings

### CONSOL-001 — Current product-state SHA was stale

The repository's actual `main` HEAD is `325154034f965de475d4214959d9608c60f82de8`, while the current-state/checkpoint documents still referenced `57c32a9b21b76e4229462ff0c7ddd6dd47a1f199` as the current HEAD.

This is documentation drift only; it does not alter product behavior.

### CONSOL-002 — Rust CI retains historical branch triggers

`.github/workflows/rust.yml` still lists numerous milestone/development branches in push and pull-request triggers.

The maintained product CI only needs the supported `main` integration path for the consolidated public surface. Historical branch names do not represent current product interfaces.

### CONSOL-003 — Action validation workflow retains historical milestone naming

`.github/workflows/m08-block3-action.yml` validates the permanent reusable Action, but its workflow display name still identifies the historical M0.8 milestone.

The validation behavior is current and useful; the historical display naming is no longer an accurate description of its role.

### CONSOL-004 — No current temporary workflow files remain in `.github/workflows`

The current workflow directory contains only:

- `.github/workflows/rust.yml`
- `.github/workflows/m08-block3-action.yml`

The stress/campaign/fixture-harvest/lint workflows visible in Actions history are not current files on `main`.

They must not be recreated merely to make historical Actions entries disappear.

## Non-goals

- No new audit capability.
- No change to APK/AAB parsing semantics.
- No change to cryptographic verification.
- No change to Report v1 schema.
- No change to CLI exit semantics.
- No change to the reusable Action inputs/outputs.
- No release/distribution machinery.
- No private material or private planning is to be added to the public repository.

## Proposed consolidation

1. Restrict permanent Rust CI triggers to the `main` integration path.
2. Keep the existing Action self-test, but rename its workflow display title from the historical milestone name to a product-facing technical name.
3. Preserve the existing Action path, CLI, Core, Report v1/schema, fixtures, and validation behavior unchanged.
4. Reconcile current-state/checkpoint documentation with the actual `main` HEAD after integration.
5. Perform a second audit and CI validation.
6. Only after the consolidated surface is closed, perform repository/history cleanup of obsolete development artifacts where technically possible.

## Audit disposition before implementation

The proposed changes are limited to CI trigger hygiene and product-surface documentation/naming. No production audit logic is changed.
