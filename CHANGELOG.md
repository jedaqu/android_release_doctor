# Changelog

All notable user-facing changes to Android Release Doctor are documented here.

## [0.1.0] — First public release

Status: First public release published as GitHub Release `v0.1.0`. Distribution is tag-driven.

### Highlights
The published [GitHub Release `v0.1.0`](https://github.com/jedaqu/android_release_doctor/releases/tag/v0.1.0) contains the supported first-release packages and `SHA256SUMS`.


- Local-first auditing of Android APK and AAB release artifacts.
- Direct binary AndroidManifest inspection without invoking the Android SDK.
- Static comparison against an Android application Gradle configuration when supplied.
- Versioned Google Play release-readiness profile.
- Native ELF load-segment alignment evidence for 16 KB compatibility review.
- APK package-alignment evidence for applicable uncompressed native libraries.
- APK signing-block inspection and supported APK Signature Scheme v2/v3/v3.1 cryptographic verification.
- Supported v3 proof-of-rotation verification and structured evidence.
- Multi-signer evidence isolation and explicit Verified / Invalid / Unsupported / ManualReview distinctions.
- Stable human-readable CLI output.
- Report v1 JSON output for CI and tooling.
- Stable CLI exit codes for no-blocker, blocker, and operational/usage error states.
- Reusable GitHub Action integration.
- Native x86_64 release packages for Linux, Windows, and Intel macOS.

### Distribution

The release package matrix is:

- Linux x86_64 — `android-release-doctor-v0.1.0-linux-x86_64.tar.gz`
- Windows x86_64 — `android-release-doctor-v0.1.0-windows-x86_64.zip`
- macOS Intel x86_64 — `android-release-doctor-v0.1.0-macos-x86_64.tar.gz`

Published release artifacts are accompanied by `SHA256SUMS`.

### Important boundaries

- ARM64 packages are not included.
- Installers and package-manager integrations are not included.
- OS signing/notarization is not included.
- The GitHub Action currently executes the CLI through Cargo on the runner.
- Gradle configuration is parsed statically; arbitrary Gradle execution and full variant resolution are not performed.
- Play Console declarations that cannot be proven from an APK/AAB remain manual-review items.
- AAB final-generated-APK signing is not inferred from the AAB artifact alone.
- Unsupported cryptographic algorithms remain explicit unsupported/manual-review evidence rather than being guessed into a pass.

### First-use references

See `README.md` for installation, first-audit, JSON/CI, and GitHub Action examples.
