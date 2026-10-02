# AUDIT — ERR-093 v0.1.2 Rebaseline Second Audit

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Baseline: `main` / `0042d933af33c2229cfacdf3c97917f40afb3c94`
Head: `25315a60b175e8cf0bc4fe5f59d0609fbaf06da5`
PR: #27

## Scope recheck

The release rebaseline contains only the metadata/documentation required after the failed `v0.1.1` validation.

Changed files:

- `Cargo.toml`: workspace version `0.1.1` → `0.1.2`.
- `Cargo.lock`: `doctor-cli` and `doctor-core` package versions `0.1.1` → `0.1.2`.
- `CHANGELOG.md`: current release entry rebaselined to `0.1.2`; unpublished `0.1.1` entry removed.
- `README.md`: current release/download/version references rebaselined to `0.1.2` without changing the general example `v0.1.0` tag wording.
- `docs/AUDIT-ERR-093-REBASELINE-V0.1.2.md`: records the release-state rebaseline.

No Rust production code, distribution workflow, checksum logic, package matrix, publication command, signing logic, or Action interface was changed.

## ERR-093 correction integrity

The checksum correction already present on `main` remains unchanged:

```bash
find . -maxdepth 1 -type f ! -name SHA256SUMS -print | sed 's#^./##' | sort | xargs sha256sum > SHA256SUMS
```

## Version propagation

The ERR-104 correction is inherited unchanged from merged `main`.

`ENGINE_VERSION` derives from `CARGO_PKG_VERSION`, so the new workspace version `0.1.2` will propagate to the binary `--version` output.

## Public documentation consistency

The current README no longer claims that `v0.1.1` is a published GitHub Release.

The new release target is consistently documented as `v0.1.2`.

## CI evidence

PR #27 Rust CI:

- Run `37024937547`
- Run number `108`
- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

## Tag integrity

The existing remote `v0.1.1` tag remains untouched.

Historical `v0.1.0` remains untouched.

No force-push or tag movement is part of this rebaseline.

## Conclusion

**SECOND AUDIT: PASS.**

The v0.1.2 release rebaseline is scoped correctly and passes all four Rust CI gates. The next validation boundary is the actual tag-driven distribution workflow for `v0.1.2`.
