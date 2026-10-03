# M0.17 — Product Preparation / v0.1.3 Pre-Audit — 2026-10-03

## Baseline

- Repository: `jedaqu/android_release_doctor`
- Visibility: public
- Default branch: `main`
- Baseline main HEAD: `34d9c732c9f115e9c7478545fc22ea02a52d4a0f`
- Maintained product version: `0.1.3`
- Previous milestone: M0.16 — Final Product Readiness Gate
- Branch: `prep/m017-product-preparation-v0.1.3-2026-10-03`

## Objective

Prepare the public product for a first reproducible release candidate without changing product behavior and without introducing release-build infrastructure.

The preparation target is a reviewed, documented v0.1.3 product state that can later be published manually by the owner.

## Authorized scope

1. Freeze the maintained public surface and version identity at 0.1.3.
2. Prepare public release notes and a release checklist grounded only in verified repository evidence.
3. Verify the user-facing install/use path for:
   - local CLI;
   - reusable GitHub Action;
   - Report v1 JSON contract.
4. Document immutable commit identity, test evidence, known boundaries, and manual-release steps.
5. Preserve the public/private boundary.
6. Do not add automated release-build workflows or distribution machinery.
7. Do not alter Rust production logic, Action behavior, CLI semantics, or Report v1 schema.

## Product preparation principle

The product is not considered prepared merely because a version string exists. The release candidate must have:
`version identity + public documentation + validated evidence + reproducible installation/usage instructions + explicit release boundaries`.

## Required outputs

- public release notes for v0.1.3;
- product manifest / release checklist;
- final preparation audit;
- closure checkpoint.

## Non-goals

- no monetization/commercial strategy;
- no credentials or private continuity;
- no binary release-build automation;
- no new feature work;
- no moving `@main` Action guidance;
- no claim that GitHub Release/tag publication has already occurred.
