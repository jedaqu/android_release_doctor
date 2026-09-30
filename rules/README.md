# Rule registry

Rules are deliberately versioned with the audit engine. Each rule should answer:

1. What evidence did Release Doctor inspect?
2. What condition was found?
3. What concrete action should the developer take?

## M0 rules

| ID | Purpose |
| --- | --- |
| `ARTIFACT-001` | Recognize APK/AAB input. |
| `ARTIFACT-002` | Verify ZIP structure. |
| `MANIFEST-001` | Verify the Android manifest is present in the expected location. |
| `DEX-001` | Check for compiled DEX payloads. |
| `NATIVE-001` | Inventory native library ABIs without claiming compatibility coverage yet. |
| `SIGNING-001` | Detect signing metadata. Cryptographic verification is intentionally deferred. |

The initial rule set is structural. It does not yet claim that an artifact is ready for publication.
