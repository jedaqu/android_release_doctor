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
