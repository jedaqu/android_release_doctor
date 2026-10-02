# v0.1.3 Distribution — investigation closure

**Date:** 2026-10-02
**Classification:** LIMITACIÓN CONOCIDA — CERRADO SIN CORRECCIÓN

## Closure record

- **Investigation closed:** The M0.8 Block 4 Distribution workflow for v0.1.3 failed at `cargo test --workspace --locked` because Cargo needed to update the lockfile and `--locked` prevented it. The investigation confirmed the `Cargo.lock` reproducibility limitation under strict `--locked` validation.
- **Normal CI:** Build, Test, Format, and Clippy passed on the normal CI path.
- **Isolated investigation:** Cargo 1.98.1 proposed `spin 0.9.9`; complete resolution also introduced transitive changes, including `digest → const-oid` and `num-traits → libm`. Directed, offline, and regeneration methods did not produce a focused lockfile delta. Regeneration produced 255 insertions and 246 deletions.
- **Manual candidate:** The approximately 11-line manual candidate was rejected because it was not validated with the required `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, and `cargo build --workspace --locked` commands. No validated correction exists.
- **Correction:** NOT MADE
- **Correction validated:** NO
- **Original `Cargo.lock` restored:** SÍ
- **Technical product files or `Cargo.lock` modified:** NO
- **Tags or releases modified:** NO
- **Documentation updated:** SÍ; the ERR-112 record and this checkpoint are documentation changes.
- **Known limitation documented:** SÍ
- **State:** INVESTIGACIÓN CERRADA; LIMITACIÓN CONOCIDA; CORRECCIÓN NO REALIZADA

The issue is localized to lockfile reproducibility under Distribution’s strict `--locked` validation. The evidence does not establish a general product, source-code, GitHub Action, or tag failure. No technical correction was made; the original `Cargo.lock` was restored, the technical repository remains unchanged, and only the documentation and checkpoint were updated.
