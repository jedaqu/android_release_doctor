# Rule registry

Rules are versioned with the audit engine. Each rule should answer:

1. What evidence did Release Doctor inspect?
2. What condition was found?
3. What concrete action should the developer take?

## M0.1 artifact rules

| ID | Purpose |
| --- | --- |
| \`ARTIFACT-001\` | Recognize APK/AAB input. |
| \`ARTIFACT-002\` | Verify ZIP structure. |
| \`MANIFEST-001\` | Verify the Android manifest is present. |
| \`MANIFEST-002\` | Report a blocker when the compiled Android manifest cannot be parsed. |
| \`MANIFEST-003\` | Extract the final application/package identity. |
| \`SDK-001\` | Extract minSdk/targetSdk from the final manifest. |
| \`BUILD-001\` | Evaluate the effective \`android:debuggable\` value; an absent attribute is treated as \`false\`. |
| \`VERSION-001\` | Extract versionCode. |
| \`VERSION-002\` | Extract versionName. |
| \`PERMISSION-001\` | Inventory declared permissions without risk scoring yet. |
| \`COMPONENT-001\` | Require explicit \`android:exported\` for activity, activity-alias, service, and receiver declarations with intent filters. |
| \`DEX-001\` | Check for compiled DEX payloads. |
| \`NATIVE-001\` | Inventory native library ABIs. |
| \`SIGNING-001\` | Detect signature metadata. Cryptographic verification is deferred. |

## M0.2 project/artifact rules

| ID | Purpose |
| --- | --- |
| \`PROJECT-001\` | Load the Android application Gradle configuration, or report a blocker when it cannot be discovered/parsed. |
| \`CROSSCHECK-000\` | Explain when project parsing succeeded but artifact manifest evidence is unavailable. |
| \`CROSSCHECK-001\` | Compare Gradle \`applicationId\` with the final artifact package identity. |
| \`CROSSCHECK-002\` | Compare Gradle \`targetSdk\` with the final artifact \`targetSdkVersion\`; a mismatch is a blocker. |
| \`CROSSCHECK-003\` | Compare Gradle \`minSdk\` with the final artifact \`minSdkVersion\`; a mismatch is a warning. |
| \`CROSSCHECK-004\` | Compare Gradle \`versionCode\` with the final artifact versionCode. |
| \`CROSSCHECK-005\` | Compare Gradle \`versionName\` with the final artifact versionName. |
| \`CROSSCHECK-006\` | Compare the release build type's explicit debuggable/isDebuggable setting with the effective artifact value; a mismatch is a blocker. |

### M0.2 parser scope

The project parser reads a single Android application module's \`build.gradle\` or \`build.gradle.kts\` using static lexical inspection. It does not execute Gradle, resolve arbitrary variables, evaluate product flavors, or simulate CI-generated versioning.

For an AAB, the artifact-side evidence remains the M0.1 base-module manifest.
