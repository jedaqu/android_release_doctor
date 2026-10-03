# M0.12 — Product Surface Consolidation Second Audit — 2026-10-03

## Baseline and candidate

- Base `main`: `325154034f965de475d4214959d9608c60f82de8`
- Consolidation head: `092d38f517a0b4d39bfb032eb6e26bb202efbbc3`
- Pull request: #58
- Scope: permanent product/CI surface consolidation only

## Diff boundary

The final comparison contains exactly five changed paths:

1. `.github/workflows/android-release-doctor-action.yml` — renamed from the historical M0.8 workflow name, with the same test graph and product inputs/outputs.
2. `.github/workflows/rust.yml` — removed historical branch triggers; retained `main` push and pull-request validation plus the same Build/Test/Format/Clippy jobs.
3. `README.md` — clarified the CLI and reusable Action as the two primary product entry points and removed the moving `@main` example in favor of a reviewed ref placeholder.
4. `docs/CURRENT-PRODUCT-STATE-2026-10-03.md` — reconciled the baseline and consolidated product surface.
5. `docs/PRE-AUDIT-M0.12-PRODUCT-SURFACE-CONSOLIDATION-2026-10-03.md` — recorded the pre-audit and scope.

No production Rust source, Action implementation, Report v1 schema, fixture content, cryptographic verifier, or CLI contract changed.

## Validation evidence

### Rust CI

Run `37129738589` — SUCCESS.

- Build — PASS
- Test — PASS
- Format — PASS
- Clippy — PASS

### Action validation

Run `37129738604` — SUCCESS.

- Checkout — PASS
- Rust toolchain verification — PASS
- Success path — PASS
- Success report validation — PASS
- Blocker path — PASS
- Blocker result validation — PASS
- Operational error path — PASS
- Operational error validation — PASS

## Contract audit

The consolidation preserves:

- CLI behavior and exit codes;
- Report v1 JSON contract and schema;
- reusable Action inputs and outputs;
- success/blocker/operational-error behavior;
- audit engine capabilities and explicit manual-review boundaries.

The workflow changes alter only validation routing and naming.

## Historical workflow boundary

The current repository contains only the permanent Rust CI and Action validation workflows after this consolidation. Historical campaign, stress-test, fixture-harvest, and lint workflow executions may remain visible in GitHub Actions history because execution history is separate from the current workflow files.

No historical execution is reintroduced into the maintained product surface.

## Public/private boundary

This consolidation contains only public technical product and engineering material. No private continuity data or sensitive operational material is added.

## Audit result

Technical consolidation is validated and ready for integration. Final closure requires the post-merge checkpoint and the append-only engineering-ledger reconciliation for CONSOL-001 through CONSOL-003.
