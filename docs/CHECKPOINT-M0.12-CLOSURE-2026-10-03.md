# M0.12 — Checkpoint / Closure — 2026-10-03

## Current state

- Repository: `jedaqu/android_release_doctor`
- Visibility: public
- Default branch: `main`
- Milestone/block: M0.12 — Product Surface Consolidation
- Status: CLOSED
- Current main HEAD: `508edc22c7111aaada06fe885b93f224a1780853`
- Integration PR: #58
- Integration commit: `508edc22c7111aaada06fe885b93f224a1780853`
- Consolidation validation head: `58449d35ce3e1b81025962f93d0d604cc3cf10ec`

## Final product surface

### Core
- `crates/doctor-core` — shared Rust audit engine.

### CLI
- `crates/doctor-cli` — local `android-release-doctor` command-line interface.

### GitHub Action
- `.github/actions/android-release-doctor/action.yml` — reusable composite Action.

### Report contract
- Report v1 JSON.
- `docs/report-schema-v1.0.json`.

### Permanent validation
- `.github/workflows/rust.yml`.
- `.github/workflows/android-release-doctor-action.yml`.

### Supporting evidence
- Public fixtures and regression tests.
- Current product documentation and append-only engineering evidence.

## Consolidation changes

- Permanent Rust CI push/pull-request triggers are restricted to `main`.
- The reusable Action validation workflow is product-facing and no longer uses the historical M0.8 workflow filename.
- README product-entry wording now treats the CLI and reusable Action as the primary user-facing surfaces.
- No production audit logic or public report contract was changed.

## Validation

- Rust CI Run `37129863362`: SUCCESS.
- Action validation Run `37129863355`: SUCCESS.
- Both validation runs were executed against consolidation head `58449d35ce3e1b81025962f93d0d604cc3cf10ec` before PR #58 integration.

## Historical workflow cleanup

The old workflow files are removed from the current repository tree. GitHub Actions may continue to display historical executions for removed or renamed workflows; execution-history deletion is a separate GitHub operation and is not available through the connected write surface used here.

No historical workflow was recreated solely to alter that history.

## Ledger reconciliation

M0.12 corrections CONSOL-001 through CONSOL-003 are recorded in dedicated `docs/LEDGER-M0.12-CONSOL-*.md` files. The large canonical `docs/ERRORS-AND-FIXES.md` remains untouched rather than risking a replacement of its historical contents through the limited connected write path.

## Public/private boundary

No private continuity data, credentials, passwords, commercial strategy, pricing, private metrics, or internal conversation material was added to the public repository.

## Closure rule

M0.12 is closed on the technical product surface because implementation, current documentation, validation evidence, CI routing, Action validation, and checkpoint are coherent after integration.
