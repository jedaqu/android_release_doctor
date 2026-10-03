# M0.11-A — GitHub Action Hardening Second Audit

Date: 2026-10-03  
Base pre-audit: `2a99ea587e5257db8ed64b9bf22bee2b4447f774`  
Validated branch: `audit/m011-action-hardening-preaudit`  
Validation head: `ecddd620a80df455a68f9dca9827dbbb249a32dd`  
PR: #55  
Mode: post-implementation second audit

## 1. Scope authorization

Dani explicitly authorized implementation of Candidates A and B from the M0.11-A pre-audit.

Authorized scope:

- Candidate A — add `--locked` to the Cargo invocation used by the reusable Action.
- Candidate B — exercise the existing bounded Action self-test on the current `main` validation path.

Candidate C remains unimplemented and was not included.

No changes were made to Rust source, Report v1, Play semantics, crypto, native/16 KiB logic, dependency declarations, release machinery, tags, publication, or private/commercial material.

## 2. Implementation audit

### A — Cargo reproducibility hardening

File:

`.github/actions/android-release-doctor/action.yml`

Change:

```text
cargo run
--locked
--quiet
...
```

Only the existing Cargo invocation was changed. Argument-array construction, input validation, report handling, exit-code propagation, and output handling remain unchanged.

Implementation commit:

`8c7d39d124348e33836b626d31fe366b5add80b7`

### B — current-main Action validation

File:

`.github/workflows/m08-block3-action.yml`

Changes:

- add `main` to the existing `push.branches` list;
- add `main` to the existing `pull_request.branches` list.

The existing job graph and the bounded success/blocker/operational-error validation steps remain unchanged.

Implementation commit:

`ecddd620a80df455a68f9dca9827dbbb249a32dd`

## 3. Diff-boundary verification

Comparison:

`2a99ea587e5257db8ed64b9bf22bee2b4447f774` → `ecddd620a80df455a68f9dca9827dbbb249a32dd`

Observed changes beyond the pre-audit document:

- `.github/actions/android-release-doctor/action.yml`: +1 / -0
- `.github/workflows/m08-block3-action.yml`: +2 / -0

No unrelated production or contract files changed in the implementation commits.

## 4. Action validation evidence

Workflow:

`M0.8 Block 3 Action`

Run:

`37127054717`

Result: **SUCCESS**

Validated steps:

- Success path — PASS
- Success Report v1 validation — PASS
- Blocker path — PASS
- Blocker result validation — PASS
- Operational error path — PASS
- Operational error validation — PASS

This demonstrates the existing Action execution contract on the PR branch containing the A+B implementation.

## 5. General Rust CI evidence

Workflow:

`Rust CI`

Run:

`37127054763`

Result: **SUCCESS**

Validated:

- Build — PASS
- Test — PASS
- Format — PASS
- Clippy — PASS

## 6. Contract preservation

The following remain unchanged and validated:

- Action input names and meanings;
- Action output names;
- CLI exit semantics;
- Report v1 schema;
- existing success/blocker/operational-error test design;
- unsupported/manual-review boundaries;
- public product boundary;
- private/public separation.

## 7. Security review disposition

The implementation did not alter the previously audited bounded input-handling model.

The Action still:

- constructs Cargo arguments as an array;
- passes user inputs as discrete arguments;
- validates `play` before execution;
- rejects CR/LF in `output`;
- preserves the CLI exit status.

No new shell-injection or output-file handling issue was introduced by A or B.

## 8. Closure status

The implementation and validation requirements for Candidates A and B are satisfied on PR #55.

PR #55 remains **OPEN** and has **not been merged**.

Therefore:

- A+B implementation: COMPLETE;
- second audit: COMPLETE;
- CI validation: COMPLETE;
- documentation/evidence updates: recorded in this branch;
- main integration: PENDING MERGE;
- milestone closure: NOT YET CLOSED.

Historical pre-audit document remains unchanged. Current-state statements on `main` remain applicable until PR #55 is integrated.

## 9. Conclusion

**M0.11-A A+B — VALIDATED ON PR #55; INTEGRATION PENDING.**

No additional implementation is authorized or required by this second audit.


---

## Post-integration reconciliation — 2026-10-03

M0.11-A Candidates A and B were integrated by PR #55.

- Merge commit: `8e9f639ce71ce06e211718187d3079a5d68458f5`
- PR #55 state: merged.
- Main baseline after integration: `8e9f639ce71ce06e211718187d3079a5d68458f5`.
- Action self-test against the PR targeting `main`: Run `37127173597` — SUCCESS.
- Rust CI validation: Run `37127173629` — SUCCESS.
- ERR-120 / ERR-121: RESOLVED AND INTEGRATED.

This append-only section records current disposition without rewriting the historical second-audit observations.
