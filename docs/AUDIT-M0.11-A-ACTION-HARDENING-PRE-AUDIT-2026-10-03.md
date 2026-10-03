# M0.11-A — GitHub Action Hardening Pre-Audit

Date: 2026-10-03  
Baseline: `1ed9fae706ac03718bddc09d4bc03f6e2edecce9`  
Repository: `jedaqu/android_release_doctor`  
Mode: pre-audit / no implementation change authorized

## 1. Audit objective

This pre-audit covers only the current GitHub composite Action product surface and its validation infrastructure.

The objective is to determine whether the reusable Action has a coherent, reproducible and adequately validated execution contract before any implementation correction is proposed.

No Rust source, Action metadata, workflow, Report v1 schema, Play logic, crypto logic, native/16 KiB logic, or release machinery is authorized to change as part of this pre-audit.

## 2. Governing evidence and ledger

The incremental error ledger was reviewed before beginning this audit.

Relevant historical Action corrections:

- ERR-082 — malformed Action YAML during Block 3 implementation — RESOLVED.
- ERR-083 — malformed GitHub expression during Block 3 implementation — RESOLVED.

Historical M0.8 Block 3 validation established the Action contract and validated:

- success path;
- JSON Report v1 generation;
- blocker path with exit code 1 and report preservation;
- operational-error path with exit code 2.

Those results remain historical evidence and are not automatically treated as current-main validation.

The current documentation-coherence rule requires implementation, evidence, documentation/contracts and checkpoint to remain synchronized before closure.

## 3. Current Action contract

Current metadata defines:

Inputs:

- `artifact` — required APK/AAB path.
- `project` — optional Android application module path.
- `play` — optional boolean.
- `play-platform` — mobile/wear/automotive/tv/xr.
- `format` — text/json.
- `output` — optional report path.

Outputs:

- `exit-code`
- `report-path`

The Action preserves the CLI exit contract:

- 0 — audit completed with no blockers;
- 1 — audit completed with one or more blockers;
- 2 — usage/input/audit/output/serialization/internal error.

## 4. Current implementation observations

### ACTION-001 — current-main Action validation gap

The dedicated Action self-test workflow is currently triggered by:

- push to `m08-block3-github-action`;
- pull request targeting `m08-block2-cli-output-contract`;
- manual `workflow_dispatch`.

It is not currently part of the normal Rust CI path for `main`.

Therefore current Rust CI success on `main` does not itself prove the Action's success/blocker/operational-error contract on the current `main` product state.

Classification: **VALIDATION GAP — CONFIRMED**

No implementation defect is inferred from this finding.

### ACTION-002 — Action Cargo resolution is not locked

The Action currently invokes:

```
cargo run --quiet --manifest-path "$GITHUB_ACTION_PATH/../../../Cargo.toml" -p doctor-cli --
```

The normal Rust CI on current `main` uses `--locked` for Build, Test and Clippy.

Therefore the Action and the repository CI do not currently enforce the same Cargo reproducibility boundary.

Cargo's current documentation states that `--locked` asserts that the exact dependency resolution from `Cargo.lock` is used and exits if Cargo would modify the lockfile. This is the appropriate deterministic-build control for CI-style execution.

Classification: **HARDENING GAP — CONFIRMED**

No implementation change is authorized by this finding alone.

### ACTION-003 — Mutable reference hardening candidate

The README convenience example uses `@main` and separately recommends replacing it with a reviewed commit SHA for supply-chain pinning.

GitHub's current security guidance recommends pinning actions to a full-length commit SHA when immutable action identity is required.

Classification: **HARDENING CANDIDATE — NOT A DEFECT**

The current repository's own Action remains local to the public repository; this finding concerns how consumers reference it and how the repository's documentation examples communicate that trade-off.

### ACTION-004 — Historical self-test dependency on Cargo/toolchain

The Action executes the repository CLI through Cargo on the runner. The runner therefore requires a usable Rust/Cargo toolchain and may need network access for dependencies not already cached.

This is a documented product boundary inherited from M0.8 Block 3.

Classification: **KNOWN PRODUCT BOUNDARY — NOT A DEFECT**

No prebuilt binary distribution is to be reintroduced as an implicit correction.

## 5. Security/input-handling observations

The current Action implementation:

- constructs the Cargo invocation as a Bash argument array;
- passes input values as discrete arguments rather than interpolated shell command fragments;
- validates `play` as `true|false` before execution;
- rejects CR/LF in `output` before writing the GitHub output file;
- captures the CLI exit status and propagates it unchanged;
- writes the report through the existing CLI before returning blocker/error status;
- uses `GITHUB_ACTION_PATH` to locate the repository workspace Cargo manifest.

These observations are consistent with GitHub's documented composite-action mechanism for `github.action_path` / `GITHUB_ACTION_PATH`.

No shell-injection defect is identified in the current bounded inspection.

## 6. Validation evidence boundary

Historical proof exists for the three Action execution classes, but current-main proof is incomplete because the self-test workflow is branch-scoped.

The next validation design must prove, on the current product baseline:

1. success path;
2. blocker path;
3. operational-error path;
4. Report v1 preservation;
5. output-path propagation;
6. exit-code propagation.

The test should run as part of the current-main validation path rather than remaining dependent only on historical development branches.

## 7. Proposed minimal correction boundary

Subject to explicit Primary Operator authorization after this pre-audit:

### Candidate A — lock Action Cargo resolution

Change only the Cargo invocation in `.github/actions/android-release-doctor/action.yml` so that the Action executes Cargo with `--locked`.

No CLI, Rust source, dependency, Report, or audit-engine changes.

### Candidate B — current-main Action validation

Extend the Action validation workflow so the same bounded success/blocker/operational-error self-test is exercised against the current `main` product surface.

The job graph should remain otherwise unchanged.

### Candidate C — documentation/reference hardening

Keep the public example usable while making the SHA-pinning recommendation explicit. No new release/tag contract is implied by this candidate.

Candidates A and B are the primary hardening scope. Candidate C is secondary and documentation-oriented.

## 8. Explicitly out of scope

- new audit capabilities;
- Report v1 schema changes;
- HTML/SARIF;
- Play policy semantic changes;
- AAB cryptographic verification;
- v3.2/PQC;
- native/16 KiB semantic changes;
- dependency upgrades unrelated to Cargo reproducibility;
- prebuilt binary distribution;
- release/publication machinery;
- tag creation or release automation;
- multi-artifact Action inputs;
- private/commercial strategy.

## 9. Audit conclusion

**PRE-AUDIT COMPLETE — IMPLEMENTATION NOT AUTHORIZED**

Two concrete product hardening gaps are confirmed:

- current-main Action validation is incomplete;
- Action Cargo resolution does not currently enforce `--locked`.

The existing implementation's input handling and exit/report propagation are not currently showing a defect.

The next action is a bounded second-pass review of this pre-audit against the exact diff/scope before implementation authorization.
