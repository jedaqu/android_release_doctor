# AUDIT — ERR-093 Release Rebaseline for v0.1.2

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Baseline main: `0042d933af33c2229cfacdf3c97917f40afb3c94`
Branch: `fix/err-093-release-rebaseline-v0.1.2-2026-10-02`

## Trigger

ERR-093 real-publication validation used tag `v0.1.1`. The distribution workflow correctly blocked publication because the release binary self-reported `0.1.0` while the workspace/tag version was `0.1.1`.

ERR-104 corrected the independent engine-version source and has passed PR and post-merge Rust CI.

## Release-state evidence

The remote annotated tag `v0.1.1` already exists and points to the failed release commit:

`a00529824f61795e97979d9289ad54b1e504c662`

GitHub reports no Release for `v0.1.1`.

Therefore:

- `v0.1.1` must not be moved, deleted, or reused.
- Historical `v0.1.0` remains unchanged.
- The next release-validation publication must use a new version/tag.

## Rebaseline target

The next release target is **v0.1.2**.

The rebaseline will align:

- workspace package version;
- Cargo.lock package versions for `doctor-core` and `doctor-cli`;
- public README release/download references;
- public CHANGELOG current release entry;
- current release-readiness documentation.

No engine, audit, signing, package, checksum, Action, or publication-workflow logic changes are included in this rebaseline.

## ERR-093 production correction retained

The already-merged checksum correction remains unchanged:

```bash
find . -maxdepth 1 -type f ! -name SHA256SUMS -print | sed 's#^./##' | sort | xargs sha256sum > SHA256SUMS
```

The following release contract remains unchanged:

- three native x86_64 packages;
- SHA-256 checksums;
- `sha256sum -c SHA256SUMS`;
- exact package filename assertions;
- `gh release create` with the three packages and `SHA256SUMS`;
- tag/version equality validation;
- publication guard;
- permissions and trigger design.

## Public-documentation consistency

The current README incorrectly states that `v0.1.1` is published, while GitHub has no Release for that tag.

This rebaseline will correct that inconsistency without exposing unnecessary development-process material.

## Scope

### In scope

1. Rebaseline release metadata from `0.1.1` to `0.1.2`.
2. Correct current public release/download/action documentation.
3. Update current ERR-093 readiness documentation for `v0.1.2`.
4. Preserve the existing production checksum correction.

### Out of scope

- Moving or deleting `v0.1.1`.
- Rebuilding `v0.1.0`.
- Modifying checksum-generation logic again.
- Modifying package matrix or archive structure.
- Modifying CLI or engine-version logic.
- Modifying signing or Action behavior.
- Refactoring unrelated documentation or source.

## Acceptance for this rebaseline

- Workspace and lockfile package versions agree at `0.1.2`.
- No current public documentation claims that `v0.1.1` is published.
- The release-readiness record targets `v0.1.2`.
- Existing ERR-093 workflow correction remains byte-for-byte unchanged.
- Rust CI will validate the release-preparation branch before any tag is created.

This is a release-preparation audit. ERR-093 remains OPEN until a successful tag-driven `v0.1.2` publication is demonstrated end-to-end.
