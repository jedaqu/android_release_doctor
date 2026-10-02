# AUDIT-005 — Final Closure

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Release: `v0.1.0`
Validated main/tag commit: `e99fcc7a0615b94f8b892434de6f9c71b711ac64`

## Final publication evidence

The controlled tag publication exercise was executed after the ERR-092 correction and its second audit.

### Repository and tag

- `main` points to `e99fcc7a0615b94f8b892434de6f9c71b711ac64`.
- `v0.1.0` points to the same commit.
- The remote tag reference was independently verified after publication.

### Distribution workflow

Final tag-triggered run: `36949141163`

Terminal result: **success**

The run completed successfully through:

- validate
- Linux x86_64 packaging
- macOS x86_64 packaging
- Windows x86_64 packaging
- checksum generation and verification
- GitHub Release publication

The previously observed checksum failure from run `36945562430`, attempt `1`, did not reproduce in its controlled rerun or in the final corrected publication run.

### Public GitHub Release

GitHub Release ID: `401467112`

Release tag: `v0.1.0`

Release URL: https://github.com/jedaqu/android_release_doctor/releases/tag/v0.1.0

Published: 2026-10-02

The release contains exactly four expected assets:

1. `android-release-doctor-v0.1.0-linux-x86_64.tar.gz`
2. `android-release-doctor-v0.1.0-macos-x86_64.tar.gz`
3. `android-release-doctor-v0.1.0-windows-x86_64.zip`
4. `SHA256SUMS`

The release asset metadata exposes SHA-256 digests for all four uploaded assets.

## Audit conclusion

The AUDIT-005 question was whether the configured publication path had actually produced the intended public release.

That evidence is now present:

- real version tag: verified;
- tag-triggered distribution run: successful;
- publication job: successful;
- GitHub Release: exists;
- expected platform assets: present;
- `SHA256SUMS`: present;
- assets correspond to version `0.1.0`.

**AUDIT-005 — CLOSED.**

This closure does not alter the previously delimited limitation from AUDIT-003: SHA-256 checksums establish artifact integrity for the published files but are not, by themselves, publisher authenticity or provenance.
