# AUDIT — ERR-105 Second Audit

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Baseline main: `f75ae4ee8fc1ab954b3e2441ec27b877d899968d`
Correction commit: `f89294439c02ef0d2e6035eb9392888a03400a48`
PR: #28

## Scope recheck

The correction is limited to the three stale package filenames in `README.md`.

Corrected values:

- `android-release-doctor-v0.1.2-linux-x86_64.tar.gz`
- `android-release-doctor-v0.1.2-windows-x86_64.zip`
- `android-release-doctor-v0.1.2-macos-x86_64.tar.gz`

No source code, workflow, package matrix, checksum, changelog, or tag changes were introduced.

## Regression check

A repository-wide re-read of the README confirms that no `v0.1.1` package filename remains in the current supported-download section.

## CI evidence

Rust CI run `37025923480`, run number `111`:

- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

## Conclusion

**SECOND AUDIT: PASS.**

ERR-105 is ready for merge. Closure requires post-merge Rust CI validation.
