# Android Release Doctor

Open-source, local-first tool for auditing Android APK and AAB releases before publication.

> **Status:** early development — M0.4 native ELF and APK signing inspection.

Android Release Doctor inspects the **artifact you are actually going to distribute**, can compare it with the Android application Gradle configuration, and can apply a versioned Google Play submission-readiness profile.

## M0.1 scope

The release artifact is inspected deeply enough to extract:

- application/package identity;
- versionCode and versionName;
- minSdkVersion and targetSdkVersion;
- effective `android:debuggable` state;
- declared permissions;
- Android components and explicit `android:exported` requirements for supported component types with intent filters;
- DEX payloads, native-library ABIs, and signature metadata presence.

The parser reads the compiled binary AndroidManifest.xml directly from the APK/AAB. It does not invoke the Android SDK or external shell tools. For AABs, M0.1 inspects the base module manifest.

## M0.2 scope

M0.2 adds static project-side evidence from:

- `build.gradle`;
- `build.gradle.kts`.

The project parser extracts the Android application module's:

- namespace;
- applicationId;
- compileSdk;
- minSdk;
- targetSdk;
- versionCode;
- versionName;
- release debuggable/isDebuggable declaration.

The audit can compare those declarations with the final APK/AAB manifest. This is intentionally a **static cross-check**, not a Gradle evaluator: variant resolution, product flavors, CI-generated versioning, and arbitrary Gradle code are not executed.

## M0.3 scope — Google Play release readiness

M0.3 adds an explicit `--play` readiness profile. The profile is versioned as `2026-08-31` and supports platform-specific target API requirements for:

- `mobile`;
- `wear`;
- `automotive`;
- `tv`;
- `xr`.

The current profile checks the final artifact's target API level and reports the applicable Google Play requirement. It also surfaces manual release checks for Play Console information that cannot be proven from an APK/AAB alone, including Data Safety, privacy-policy readiness, and App content declarations.

For artifacts containing native libraries and targeting API 35 or higher, M0.3 flagged 16 KB page-size compatibility for manual verification. M0.4 now inspects packaged ELF PT_LOAD alignment directly when the native library can be parsed.

### Play readiness usage

Mobile is the default Play platform:

```text
android-release-doctor --play app-release.aab
```

Use an explicit platform when needed:

```text
android-release-doctor --play --play-platform wear app-release.aab
android-release-doctor --play --play-platform tv app-release.aab
```

Project and Play checks can be combined:

```text
android-release-doctor --project app/ --play app-release.aab
```

Without `--play`, the previous M0.1/M0.2 behavior remains unchanged.

## M0.4 scope — native ELF and signing evidence

M0.4 extends the artifact evidence model without invoking Gradle or external Android tooling.

For each packaged native `.so`, the audit can inspect:

- ELF32 and ELF64 program headers;
- every `PT_LOAD` segment's `p_align` value;
- whether all inspected load segments meet the 16 KB alignment threshold;
- parse failures that require manual verification.

This evidence feeds both the artifact audit and `PLAY-005`. For an artifact targeting API 35 or higher, confirmed native ELF load-segment alignment below 16 KB is reported as a blocker. A native ELF parse failure remains a warning/manual-review item rather than a guessed pass or blocker.

M0.4 also inspects the APK signing-block structure. It can detect the standard v2/v3 signing-block IDs and validate the block's placement, magic, size fields, and ID/value pair structure. This is structural evidence only; it does not cryptographically verify signer certificates or the complete APK signature.

M0.4 does **not** yet verify ZIP entry alignment for uncompressed native libraries. Google's 16 KB guidance covers both ELF load-segment alignment and APK/AAB packaging alignment, so final verification of the packaged native payload still requires an appropriate Android packaging check such as `zipalign`/bundletool outside the current parser.

### M0.4 usage

The existing CLI commands remain unchanged:

```text
android-release-doctor --play app-release.aab
android-release-doctor --play app-release.apk
```

The native ELF and APK signing evidence is collected automatically as part of the artifact audit.

## Still not implemented

- full Gradle/variant evaluation;
- product-flavor-aware expected-value resolution;
- CI/generated version resolution;
- cryptographic signature verification;
- ZIP-entry/package alignment verification for uncompressed native libraries;
- current Google Play policy automation beyond the M0.3 readiness checks;
- permission risk classification;
- HTML/SARIF output.

## Development

```bash
cargo check --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p doctor-cli -- tests/fixtures/minimal-release.apk
cargo run -p doctor-cli -- --project tests/fixtures/project-release tests/fixtures/minimal-release.apk
cargo run -p doctor-cli -- --play tests/fixtures/minimal-release.apk
```

## Design principles

- **Local-first:** release artifacts and project configuration are inspected locally.
- **Evidence before conclusions:** findings are tied to observable project or artifact evidence.
- **Actionable diagnostics:** warnings explain what to change.
- **Versioned rules:** Android and Play requirements change, so rules are explicit and replaceable.
- **Artifact first:** the final APK/AAB remains the source of truth for what is actually being distributed.
- **Conservative project parsing:** Gradle code is never executed.
- **Manual means manual:** when Play requires information that the binary cannot prove, Release Doctor reports a concrete review item instead of pretending it can verify it.

## License

Apache License 2.0.
