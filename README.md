# Android Release Doctor

Open-source, local-first tool for auditing Android APK and AAB releases before publication.

> **Status:** early development — M0.2 project/artifact cross-check.

Android Release Doctor inspects the **artifact you are actually going to distribute** and can now compare it with the Android application Gradle configuration used as the project-side release evidence. The goal is actionable release-readiness diagnostics: what was found, why it matters, and what to fix.

## M0.1 scope

The release artifact is inspected deeply enough to extract:

- application/package identity;
- versionCode and versionName;
- minSdkVersion and targetSdkVersion;
- effective \`android:debuggable\` state;
- declared permissions;
- Android components and explicit \`android:exported\` requirements for supported component types with intent filters;
- DEX payloads, native-library ABIs, and signature metadata presence.

The parser reads the compiled binary AndroidManifest.xml directly from the APK/AAB. It does not invoke the Android SDK or external shell tools. For AABs, M0.1 inspects the base module manifest.

## M0.2 scope

M0.2 adds static project-side evidence from:

- \`build.gradle\`;
- \`build.gradle.kts\`.

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

For project discovery, \`--project\` may point at the Android application module directory, the application build file itself, or a project root containing exactly one discoverable Android application module.

### Project + artifact usage

\`\`\`text
android-release-doctor --project app/ app-release.aab
android-release-doctor --project app/build.gradle.kts app-release.apk
\`\`\`

Running without \`--project\` keeps the M0.1 artifact-only behavior:

\`\`\`text
android-release-doctor app-release.aab
android-release-doctor app-release.apk
\`\`\`

## Still not implemented

- full Gradle/variant evaluation;
- product-flavor-aware expected-value resolution;
- cryptographic signature verification;
- Android signing block inspection;
- current Google Play policy rules;
- permission risk classification;
- HTML/SARIF output.

## Development

\`\`\`bash
cargo check --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p doctor-cli -- tests/fixtures/minimal-release.apk
cargo run -p doctor-cli -- --project tests/fixtures/project-release tests/fixtures/minimal-release.apk
\`\`\`

## Design principles

- **Local-first:** release artifacts and project configuration are inspected locally.
- **Evidence before conclusions:** findings are tied to observable project or artifact evidence.
- **Actionable diagnostics:** warnings explain what to change.
- **Versioned rules:** Android and Play requirements change, so rules are explicit and replaceable.
- **Artifact first:** the final APK/AAB remains the source of truth for what is actually being distributed.
- **Conservative project parsing:** M0.2 never executes Gradle code.

## License

Apache License 2.0.
