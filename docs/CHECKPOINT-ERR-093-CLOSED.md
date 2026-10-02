# CHECKPOINT — ERR-093 CLOSED

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Scope: M0.9 / AUDIT-005 publication checksum self-inclusion defect

## Closure state

ERR-093 completed:

**PRE-AUDIT → production correction → validation → second audit → real tag-driven publication → documentation closure → PR CI → merge → post-merge CI → checkpoint.**

## Defect

The distribution workflow generated `SHA256SUMS` by enumerating regular files while creating the checksum output file through shell redirection. Because the output file was not excluded from the input set, it could be included in its own checksum input, producing a nondeterministic self-reference.

The observed failure was a real publication-workflow defect.

## Production correction

The checksum input set was corrected to exclude the output file explicitly:

```bash
find . -maxdepth 1 -type f ! -name SHA256SUMS -print | sed 's#^./##' | sort | xargs sha256sum > SHA256SUMS
```

The correction preserves the existing SHA-256 algorithm, three-package contract, checksum verification command, archive/package matrix, filename assertions, publication command, permissions, and trigger design.

## Real publication validation

Corrected publication target: `v0.1.2`

Tag and commit:

- tag: `v0.1.2`
- commit: `f55e694c9c9bf491d08d332f50347c65c9c47458`

Distribution workflow:

- run #6
- run id: `37027265016`
- conclusion: **success**
- `validate`: PASS
- Linux package: PASS
- Windows package: PASS
- macOS package: PASS
- `publish`: PASS
- **Generate and verify checksums**: PASS
- **Publish GitHub Release**: PASS

Published GitHub Release:

- release: `v0.1.2`
- release id: `401939254`
- draft: false
- prerelease: false
- assets: exactly the three platform packages plus `SHA256SUMS`

The real publication therefore satisfied the ERR-093 acceptance boundary, including successful checksum generation and verification followed by actual release publication.

## Closure documentation validation

Second audit:

- `docs/AUDIT-ERR-093-SECOND-AUDIT.md`
- result: **PASS**

Closure PR:

- PR #30
- title: `docs: close ERR-093 after v0.1.2 publication`
- head commit: `bbd5963f2c862dc0615ab6f02b8c15d7d3e4d1f8`
- PR Rust CI run #116 / `37028453052`: Build PASS; Test PASS; Format PASS; Clippy PASS
- merge commit: `9edd555689e16f8638a79b3c64048e2a52da59ad`
- post-merge main Rust CI run #117 / `37028705127`: Build PASS; Test PASS; Format PASS; Clippy PASS

The engineering ledger now records ERR-093 as **RESOLVED**.

## Tag integrity

- `v0.1.2` remains the successful corrected publication.
- The previously failed `v0.1.1` tag remains untouched.
- Historical `v0.1.0` remains untouched.
- No force-push or tag movement was used.

## Final conclusion

**ERR-093 CLOSED.**

The production checksum defect was corrected, independently re-audited, validated by a real end-to-end `v0.1.2` publication, documented in the engineering ledger, and followed by successful PR and post-merge CI validation.
