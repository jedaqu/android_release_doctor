# GAP-1 — Cargo Reproducibility Audit

## Scope

This audit covers only Cargo dependency-lock reproducibility for main HEAD:

`8c457a84697a6f571aee8fc5f9b1cb9afa89ebe6`

No source-code, workflow, cryptographic, native/16 KiB, or direct dependency changes were authorized as part of this correction.

## Initial finding

The committed `Cargo.lock` failed strict locked resolution.

Initial lockfile SHA-256:

`bb4e4a69621e7fc83bf3634184615661f283c27aaf0bc367a7722beb749113`

The failure required Cargo to resolve the missing transitive package:

`spin 0.9.9`

CI run `37087228335` had previously resolved `spin 0.9.9` because the build used `cargo build --workspace` without `--locked`.

## Controlled correction

Cargo regenerated the lockfile using:

`cargo test --workspace`

No manual edits were made to `Cargo.lock`.

The generated lockfile SHA-256 was:

`3f967003fc962629edbdc8976cb821febf701b1ab7590275329d206b22910673`

Semantic comparison showed:

- 1 package added: `spin 0.9.9`
- 0 packages removed
- no version changes among packages already present
- no checksum changes
- dependency-entry changes consist primarily of version disambiguation plus feature-driven transitive relationships

The feature tree confirmed the relevant transitive paths, including:

`spin -> lazy_static -> x509-parser / num-bigint-dig -> rsa`

and:

`num-traits feature "libm" -> rsa 0.9.10`

## Reproducibility validation

The regenerated lockfile was validated with:

- `cargo test --workspace --locked` — PASS
- `cargo build --workspace --locked` — PASS
- `cargo fmt --all -- --check` — PASS
- `cargo clippy --workspace --all-targets --locked -- -D warnings` — PASS

After the strict locked validation, the lockfile SHA-256 remained exactly:

`3f967003fc962629edbdc8976cb821febf701b1ab7590275329d206b22910673`

Therefore the strict locked commands did not require any further lockfile modification.

## Repository hygiene

Final local verification:

- HEAD: `8c457a84697a6f571aee8fc5f9b1cb9afa89ebe6`
- modified project file: `Cargo.lock`
- `git diff --check`: PASS

The correction introduced no production source-code changes.

## Classification

GAP-1 is:

**CORRECTED AND VERIFIED LOCALLY**

Integration into GitHub remains a separate controlled step.


## Follow-up controlled minimum-scope experiment — 2026-10-03

A second, isolated experiment was performed to determine whether the PR #52 lockfile contained unnecessary changes beyond the historical minimum-scope hypothesis.

The experiment used a detached worktree at the exact baseline commit:

`8c457a84697a6f571aee8fc5f9b1cb9afa89ebe6`

The baseline `Cargo.lock` SHA-256 in that worktree was:

`bb4e4a69621e7fc83bf3634184615661f283c27aaf0bc367a7722beb749113`

An artificial candidate was constructed **only inside the isolated worktree**, containing the historical minimal proposal:

- add the `lazy_static 1.5.1 → spin` dependency edge;
- add the resolved package `spin 0.9.9`.

No public repository file, PR branch, or GitHub ref was modified by this experiment.

The decisive result was:

`cargo test --workspace --locked` — **FAIL**

Cargo reported that it needed to update the lockfile even though `spin 0.9.9` had been added. Therefore the historical `+spin` candidate alone is insufficient for strict locked resolution.

Next, `cargo update --workspace` was run only in that isolated worktree. Cargo reconstructed the additional transitive resolution and explicit version-qualified dependency references required by the active graph. These included the coexistence of legacy and current versions such as `const-oid 0.9.6` / `0.10.2`, `der 0.7.10` / `0.8.2`, `sha2 0.10.9` / `0.11.0`, `signature 2.2.0` / `3.0.0`, and corresponding `pkcs8`, `spki`, `rand_core`, RSA and transitive packages.

The regenerated experimental lockfile then had SHA-256:

`3f967003fc962629edbdc8976cb821febf701b1ab7590275329d206b22910673`

That hash is exactly the same as the `Cargo.lock` on PR #52 (`8663dcf...`). A direct byte-for-byte comparison returned:

`IDENTICOS`

### Comparative conclusion

The experiment closes the previous ambiguity about the breadth of PR #52's lockfile diff:

- the historical minimal `+spin` candidate is **not sufficient**;
- Cargo itself reconstructs the broader dependency resolution;
- the result is byte-identical to the lockfile committed in PR #52;
- therefore the additional lockfile entries/references in PR #52 are not evidence of an arbitrary manual expansion of scope.

**Conclusion:** the Cargo.lock correction in PR #52 is supported by independent local reproduction of the exact Cargo-generated result. Do not manually reduce the lockfile on the basis of the historical `+spin` proposal.

PR #52 remains open and unmerged. CI `--locked` enforcement remains a separate future block.
