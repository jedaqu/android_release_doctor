# AUDIT-005 — Post-Closure Corrective Audit for Checksum Generation

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Baseline main: `65975fd707550257be63b1e0783bc0b915fa5454`
Correction branch: `fix/audit-005-checksum-generation`

## Context

AUDIT-005 was formally closed after successful publication run `36949141163` on commit `e99fcc7a0615b94f8b892434de6f9c71b711ac64`.

A later rerun of the historical distribution run `36945562430`, attempt `2`, completed with failure in `Generate and verify checksums`. This additional evidence reproduces the checksum mismatch that the earlier closure had treated as non-reproducible.

The successful final release run and the later failed rerun are not contradictory: the checksum command has a race in which `SHA256SUMS` may or may not be included in the input set.

## Scope

Correct only the checksum self-inclusion race in the existing publication workflow and register the newly confirmed defect.

Out of scope:

- release command and repository binding;
- publication guard;
- package matrix and formats;
- permissions;
- versioning;
- application/product source;
- release contents;
- existing historical closure documents.

## Evidence reviewed

- Run `36945562430`, attempt `2`: checksum step failed.
- Failed output: `SHA256SUMS: FAILED`.
- Linux, macOS, and Windows package checksum lines each reported `OK`.
- The workflow command redirects output to `SHA256SUMS` while `find` enumerates files.
- Run `36949141163`: terminal success on `e99fcc7...`, proving the affected command can also pass.
- Public Release `v0.1.0` exists with the expected three packages and `SHA256SUMS`.

## Correction audited

Change:

```bash
find . -maxdepth 1 -type f -print | sed 's#^./##' | sort | xargs sha256sum > SHA256SUMS
```

to:

```bash
find . -maxdepth 1 -type f ! -name SHA256SUMS -print | sed 's#^./##' | sort | xargs sha256sum > SHA256SUMS
```

This makes the input set explicitly exclude the output file while preserving the existing three-file count and verification contract.

## Diff-scope result

Comparison against current `main` contains exactly two modified files:

1. `.github/workflows/m08-block4-distribution.yml` — one command-line correction.
2. `docs/ERRORS-AND-FIXES.md` — ERR-093 registration.

No unrelated source, test, release, package, trigger, permission, or publication changes are present.

## Semantic audit

The correction:

- preserves SHA-256;
- preserves `sha256sum -c SHA256SUMS`;
- preserves the three package filename assertions;
- preserves the three-file precondition;
- prevents `SHA256SUMS` from being hashed by itself;
- does not alter the already validated GitHub Release command or publication guard.

## Conclusion

**PASS — the proposed correction is minimal, directly tied to reproducible evidence, and within the declared scope.**

## Required validation

1. PR Rust CI must reach terminal PASS for Build, Test, Format, and Clippy.
2. Merge only after the required CI check passes.
3. Validate the corrected checksum path with a controlled GitHub Actions execution.
4. Record the validation result and post-correction checkpoint before closing the corrective cycle.
