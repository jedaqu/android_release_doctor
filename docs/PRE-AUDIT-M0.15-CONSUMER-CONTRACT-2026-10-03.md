# M0.15 — Consumer Contract Validation Pre-Audit — 2026-10-03

## Baseline

- Repository: `jedaqu/android_release_doctor`
- Visibility: public
- Default branch: `main`
- Base main: `d1d681e46391c95fd77152779ff4e3656995837f`
- Branch: `test/m015-consumer-contract-validation-2026-10-03`
- Previous closed milestone: M0.14 — Product Assurance Expansion

## Objective

Prove the published GitHub Action contract from the perspective of a genuinely separate consumer repository.

The validation target is not the repository-local Action path. The consumer must invoke the published Action by immutable commit SHA and validate the observable contract exposed to a caller.

## Authorized scope

1. Use a separate consumer repository to invoke:
   `jedaqu/android_release_doctor/.github/actions/android-release-doctor@d1d681e46391c95fd77152779ff4e3656995837f`.
2. Exercise the consumer-visible Action inputs and outputs.
3. Validate at least:
   - successful audit path (exit 0);
   - blocking audit path (exit 1);
   - invalid/usage path (exit 2).
4. Validate the generated Report v1 against the published schema from the same immutable product revision.
5. Record evidence and disposition without exposing private consumer-repository identity or private project material in the public product repository.
6. Preserve all production behavior and existing product contracts.

## Non-goals

- No Rust production logic changes.
- No CLI contract changes.
- No Action input/output redesign.
- No Report v1 schema changes.
- No new release/distribution machinery.
- No private material in the public repository.
- No monetization or commercial strategy material.

## Pre-audit findings

- The current Action already exposes `artifact`, `project`, `play`, `play-platform`, `format`, `output`, `exit-code`, and `report-path`.
- M0.14 established immutable-SHA invocation inside the product repository, but not from a separate consumer repository.
- M0.14 also established runtime Report v1 schema validation and a 12-case deterministic corpus.
- The current public main Action implementation is pinned by this milestone to the immutable current head shown above for consumer testing.

## Required classification rule

Any failure must first be classified as:
1. consumer-test harness issue;
2. environment/dependency issue;
3. documentation/schema mismatch;
4. product defect.

No production change is authorized solely because a consumer-test harness assumption is wrong.

## Expected disposition

This milestone should remain test/assurance-only unless the consumer evidence exposes a concrete, reproducible defect in the published Action contract.
