# M0.17 — Product Preparation Checklist v0.1.3

## Required before manual publication

- [x] Product version metadata is `0.1.3`.
- [x] Public product surface is consolidated.
- [x] 147 automated Rust/CLI tests pass.
- [x] Rust Build/Format/Clippy pass.
- [x] Permanent Action Validation passes.
- [x] Report v1 schema is published and exercised.
- [x] External consumer immutable-SHA validation passes.
- [x] Independent public APK/AAB evidence is recorded.
- [x] README describes current user-facing surfaces.
- [x] Action examples avoid moving `@main` guidance.
- [x] Only two maintained workflows remain.
- [x] Public/private boundary scan is clean.
- [x] No production feature changes are pending.

## Manual publication steps

1. Reconfirm the final main HEAD after product-preparation documentation is merged.
2. Re-run or confirm the required Rust CI and Action Validation results for that exact HEAD.
3. Create the public version tag `v0.1.3` manually at that exact immutable commit.
4. Create the GitHub Release manually using the candidate notes.
5. Record the exact published commit SHA in the private continuity material and in any appropriate public release metadata that does not expose private information.
6. For Action consumers, prefer the exact commit SHA for supply-chain pinning.

## Explicit non-actions

- Do not add release-build GitHub Actions.
- Do not publish private continuity data.
- Do not publish monetization/commercial strategy.
- Do not change product logic during publication.
- Do not replace immutable references with a moving `@main` recommendation.
