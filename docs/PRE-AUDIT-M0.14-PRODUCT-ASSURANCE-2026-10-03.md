# M0.14 — Product Assurance Expansion Pre-Audit — 2026-10-03

## Baseline

- Repository: `jedaqu/android_release_doctor`
- Base main: `8668b8defb30caae699e1aa134bc01d65a4737c3`
- Branch: `test/m014-assurance-expansion`
- Previous closed block: M0.13 Product Validation Expansion

## Objective

Move the product validation from repository-internal correctness toward release-assurance evidence.

## Authorized scope

1. Direct runtime validation of Report v1 JSON against `docs/report-schema-v1.0.json`.
2. Consumer-style validation of the published composite Action through an immutable repository SHA reference.
3. Broader deterministic artifact corpus validation using the maintained APK/AAB fixtures already present in the repository, including multiple real cryptographic APK fixtures.
4. Preserve all existing product behavior and contracts.

## Non-goals

- No production audit logic changes.
- No CLI contract changes.
- No Action input/output changes.
- No schema changes unless validation proves an existing contract mismatch.
- No release or distribution machinery.
- No private material.

## Baseline evidence

M0.13 closed with:

- 147 automated Rust/CLI tests passing;
- permanent Action validation covering normal, blocker, operational-error, and invalid-input paths;
- successful post-merge Rust CI and Action Validation.

## Assurance gaps selected for M0.14

- The published JSON Schema is tested structurally by unit tests, but generated CI reports are not yet validated against the actual schema file.
- The Action is exercised from the repository-local path; consumer-style immutable SHA usage is not yet exercised.
- The deterministic fixture corpus can be broadened in CI to cover several independent cryptographic APK fixtures and an AAB fixture through the CLI.

## Expected disposition

Test-only assurance expansion. Any failure found in these tests must first be classified as a test-harness issue, a documentation/schema mismatch, or a product defect before any production change is authorized.
