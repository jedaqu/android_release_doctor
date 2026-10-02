# AUDIT — ERR-093 Real Publication Readiness

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Baseline main: `2d138cba8088dadd60626c935e8d48c83531e24a`
Branch: `fix/err-093-real-publication-v0.1.1`

## Scope

Complete the remaining validation required to close ERR-093 by demonstrating a corrected, real tag-driven publication after the checksum self-inclusion correction already merged to `main`.

The historical `v0.1.0` release remains unchanged.

## Established defect and correction

ERR-093 records a reproducible race in the distribution workflow: the checksum command previously enumerated all regular files while redirecting its output to `SHA256SUMS`. The output file could therefore be included in its own checksum input.

The production correction already present on `main` changes the input set to:

```bash
find . -maxdepth 1 -type f ! -name SHA256SUMS -print | sed 's#^./##' | sort | xargs sha256sum > SHA256SUMS
```

The existing checksum algorithm, three-package contract, `sha256sum -c SHA256SUMS`, package filename assertions, release command, publication guard, permissions, and package matrix remain unchanged.

## Validation boundary

A corrected real publication must use a version different from the historical `v0.1.0`.

This validation therefore prepares `v0.1.1`:

- workspace package version becomes `0.1.1`;
- release documentation identifies `v0.1.1` as the corrected publication;
- the tag-driven workflow must validate the tag/version match;
- all three platform packages must be built and checked;
- checksum generation must produce exactly the three package entries;
- `sha256sum -c SHA256SUMS` must pass;
- the GitHub Release must be published with the three packages and `SHA256SUMS`.

## Out of scope

- Rebuilding or modifying historical `v0.1.0`.
- Product/source changes unrelated to distribution.
- Package matrix expansion.
- Permission, trigger, signing, or release-command redesign.
- ERR-094, ERR-095, or ERR-096 changes.

## Acceptance

ERR-093 may be marked RESOLVED only after terminal evidence shows a successful tag-triggered publication of `v0.1.1`, including checksum verification and published release assets.
