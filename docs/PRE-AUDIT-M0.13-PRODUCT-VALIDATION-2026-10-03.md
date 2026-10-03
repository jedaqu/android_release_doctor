# M0.13 — Product Validation Expansion Pre-Audit — 2026-10-03

## Baseline

- Repository: `jedaqu/android_release_doctor`
- Base main: `e00bc2b7bd81fed23bfdf62666f279b53607ef69`
- Branch: `test/m013-product-validation`
- Previous closed block: M0.12 Product Surface Consolidation

## Objective

Exercise the reusable GitHub Action against the product paths promised by the public contract that are not currently covered by the permanent Action self-test.

## Existing evidence

The main-branch Rust CI has passed the current automated suite:

- 115 doctor-core unit tests;
- 25 doctor-core integration tests;
- 7 CLI integration tests;
- Build, Test, Format, and Clippy all successful.

The existing Action validation covers:

- normal success;
- Play blocker;
- operational error;
- Report v1 basic structure and Action outputs.

## Validation gaps selected for this block

1. AAB artifact path through the Action.
2. Project input through the Action.
3. Combined project + Play inputs through the Action.
4. Non-default Play platform forwarding.
5. Invalid `play` input handling.
6. Invalid output format propagation.
7. Invalid Play platform propagation.
8. CR/LF output-path rejection at the Action boundary.

These are test-only additions. No audit engine, CLI, Action input/output contract, Report v1 schema, or detection logic is changed.

## Non-goals

- No new product capability.
- No change to production Rust behavior.
- No change to public commercial/private boundaries.
- No replacement of historical engineering records.
