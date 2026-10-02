# AUDIT — ERR-093 Second Audit / v0.1.2 Real Publication

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Baseline main: `f55e694c9c9bf491d08d332f50347c65c9c47458`
Validated tag: `v0.1.2`
Distribution workflow run: `37027265016`
Run number: `6`

## Scope

This second audit rechecks the complete acceptance boundary for ERR-093 after the corrected real publication was executed against the production distribution workflow.

The purpose is to confirm that the checksum self-inclusion correction is present and that a real tag-driven publication completes end-to-end without changing the package matrix, checksum contract, or publication behavior.

## Production correction integrity

The checksum-generation correction already present on `main` remains the production implementation:

```bash
find . -maxdepth 1 -type f ! -name SHA256SUMS -print | sed 's#^./##' | sort | xargs sha256sum > SHA256SUMS
```

The correction explicitly excludes `SHA256SUMS` from its own input set.

No additional checksum-generation change was required for the successful v0.1.2 publication.

## Tag and commit integrity

The release tag `v0.1.2` points to:

`f55e694c9c9bf491d08d332f50347c65c9c47458`

The same commit is the current `main` baseline used for this closure audit.

The previously failed remote tag `v0.1.1` remains untouched. Historical `v0.1.0` remains untouched.

## Distribution workflow terminal evidence

GitHub Actions distribution run `37027265016` (run number `6`) completed with overall conclusion **success**.

All five jobs reached terminal success:

| Job | Result |
|---|---|
| `validate` | PASS |
| `package (linux-x86_64, ubuntu-24.04, tar.gz)` | PASS |
| `package (windows-x86_64, windows-2025, zip)` | PASS |
| `package (macos-x86_64, macos-15-intel, tar.gz)` | PASS |
| `publish` | PASS |

The `validate` job completed its workspace-version read, Test, Format, and Clippy steps successfully.

All three package jobs completed release build, archive verification, and package artifact upload successfully.

The `publish` job completed both:

- **Generate and verify checksums** — PASS
- **Publish GitHub Release** — PASS

This is the required terminal evidence for the ERR-093 defect boundary.

## Published release evidence

The successful run published GitHub Release `v0.1.2`.

Release state:

- tag: `v0.1.2`
- release name: `v0.1.2`
- draft: `false`
- prerelease: `false`
- release id: `401939254`

The published release contains exactly four assets:

1. `android-release-doctor-v0.1.2-linux-x86_64.tar.gz`
2. `android-release-doctor-v0.1.2-windows-x86_64.zip`
3. `android-release-doctor-v0.1.2-macos-x86_64.tar.gz`
4. `SHA256SUMS`

The workflow's checksum step verified the generated `SHA256SUMS` successfully before publication.

## Acceptance recheck

ERR-093 required a corrected, real, tag-driven publication demonstrating:

- a valid tag/version release;
- all three platform packages;
- exactly three package checksum entries;
- successful checksum verification;
- a published GitHub Release containing the packages and `SHA256SUMS`.

The v0.1.2 terminal run satisfies that acceptance boundary.

No code, package matrix, checksum algorithm, archive structure, permission model, publication command, or signing behavior required an additional change for this closure.

## Conclusion

**SECOND AUDIT: PASS.**

The production correction for ERR-093 has now been validated by a successful real tag-driven publication of `v0.1.2`.

ERR-093 is ready for formal documentation closure.
