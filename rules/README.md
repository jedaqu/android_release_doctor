# Rule registry

Rules are versioned with the audit engine. Each rule should answer:

1. What evidence did Release Doctor inspect?
2. What condition was found?
3. What concrete action should the developer take?

## M0.1 rules

| ID | Purpose |
| --- | --- |
| `ARTIFACT-001` | Recognize APK/AAB input. |
| `ARTIFACT-002` | Verify ZIP structure. |
| `MANIFEST-001` | Verify the Android manifest is present. |
| `MANIFEST-002` | Parse the binary Android manifest. |
| `MANIFEST-003` | Extract the final application/package identity. |
| `SDK-001` | Extract minSdk/targetSdk from the final manifest. |
| `BUILD-001` | Detect an explicitly debuggable final artifact. |
| `VERSION-001` | Extract versionCode. |
| `VERSION-002` | Extract versionName. |
| `PERMISSION-001` | Inventory declared permissions without risk scoring yet. |
| `COMPONENT-001` | Detect components with intent filters that lack explicit android:exported. |
| `DEX-001` | Check for compiled DEX payloads. |
| `NATIVE-001` | Inventory native library ABIs. |
| `SIGNING-001` | Detect signature metadata. Cryptographic verification is deferred. |

M0.1 introduces a small self-contained AXML parser. It intentionally reads the final compiled manifest from the artifact rather than relying on Gradle source files.
