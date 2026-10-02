## [0.1.3] — Cryptographic coverage expansion

### Highlights

This release publishes the validated cryptographic coverage added after v0.1.2:

- ECDSA/SHA-256 with NIST P-521 (0x0201).
- ECDSA/SHA-512 with NIST P-521 (0x0202).
- RSA-PSS/SHA-256 with 1024-bit and 16384-bit keys (0x0101).
- RSA-PSS/SHA-512 with 16384-bit keys (0x0102); 1024-bit RSA/PSS/SHA-512 remains explicitly Unsupported because the required salt does not fit the modulus boundary.
- RSA PKCS#1 v1.5/SHA-256 with 1024-bit and 16384-bit keys (0x0103).
- RSA PKCS#1 v1.5/SHA-512 with 1024-bit and 16384-bit keys (0x0104).
- Deterministic positive/negative cryptographic fixtures and validation for the new boundaries.

### Explicit boundaries

- DSA/SHA-256 (0x0301) remains Unsupported and is deferred for a separate security/backend decision.
- APK Signature Scheme v3.2/PQC remains outside cryptographic verification scope.
- AAB cryptographic signing verification remains outside scope; AABs are audited as bundle artifacts, while APK signing is verified from the APK signing block.

### Distribution

The release package matrix remains Linux x86_64, Windows x86_64, and Intel macOS x86_64. Published release artifacts are accompanied by SHA256SUMS.

v0.1.3 is the first public release containing the validated ERR-038-A and ERR-038-B cryptographic expansion.

# Changelog

## [0.1.2] — Publication integrity correction

### Correction
The distribution workflow excludes the generated `SHA256SUMS` file from its own checksum input set, preventing checksum self-inclusion during publication.

The release binary engine version now derives directly from the Cargo package version, keeping the CLI `--version` output aligned with the published package version.

### Distribution

The release package matrix remains unchanged:

- Linux x86_64 — `android-release-doctor-v0.1.2-linux-x86_64.tar.gz`
- Windows x86_64 — `android-release-doctor-v0.1.2-windows-x86_64.zip`
- macOS Intel x86_64 — `android-release-doctor-v0.1.2-macos-x86_64.tar.gz`

Published release artifacts are accompanied by `SHA256SUMS`.

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
