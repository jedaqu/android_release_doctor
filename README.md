# Android Release Doctor

Open-source, local-first tool for auditing Android APK and AAB releases before publication.

> **Status:** early development — M0.1 manifest inspection.

Android Release Doctor inspects the **artifact you are actually going to distribute**, rather than relying only on what a Gradle project declares. The goal is actionable release-readiness diagnostics: what was found, why it matters, and what to fix.

## M0.1 scope

The release artifact is now inspected deeply enough to extract:

- application/package identity;
- versionCode and versionName;
- minSdkVersion and targetSdkVersion;
- effective `android:debuggable` state;
- declared permissions;
- Android components and explicit `android:exported` requirements for supported component types with intent filters.

The parser reads the compiled binary AndroidManifest.xml directly from the APK/AAB. It does not invoke the Android SDK or external shell tools. For AABs, M0.1 inspects the base module manifest.

Still not implemented:

- Gradle project parsing;
- project-vs-artifact comparison;
- cryptographic signature verification;
- Android signing block inspection;
- current Google Play policy rules;
- permission risk classification;
- HTML/SARIF output.

## Usage

```text
android-release-doctor app-release.aab
android-release-doctor app-release.apk
```

The command exits with code `1` when a blocker is detected and `2` for invalid input or invocation errors.

## Development

```bash
cargo check --workspace
cargo test --workspace
cargo run -p doctor-cli -- tests/fixtures/minimal-release.apk
```

## Design principles

- **Local-first:** release artifacts are inspected locally.
- **Evidence before conclusions:** findings are tied to observable artifact evidence.
- **Actionable diagnostics:** warnings explain what to change.
- **Versioned rules:** Android and Play requirements change, so rules are explicit and replaceable.
- **Artifact first:** the final APK/AAB is a source of truth, not just build configuration.

## License

Apache License 2.0.
