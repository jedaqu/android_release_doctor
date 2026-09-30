# Rule registry

Rules are versioned with the audit engine. Each rule should answer:

1. What evidence did Release Doctor inspect?
2. What condition was found?
3. What concrete action should the developer take?

## M0.1 artifact rules

| ID | Purpose |
| --- | --- |
| `ARTIFACT-001` | Recognize APK/AAB input. |
| `ARTIFACT-002` | Verify ZIP structure. |
| `MANIFEST-001` | Verify the Android manifest is present. |
| `MANIFEST-002` | Report a blocker when the compiled Android manifest cannot be parsed. |
| `MANIFEST-003` | Extract the final application/package identity. |
| `SDK-001` | Extract minSdk/targetSdk from the final manifest. |
| `BUILD-001` | Evaluate the effective `android:debuggable` value; an absent attribute is treated as `false`. |
| `VERSION-001` | Extract versionCode. |
| `VERSION-002` | Extract versionName. |
| `PERMISSION-001` | Inventory declared permissions without risk scoring yet. |
| `COMPONENT-001` | Require explicit `android:exported` for activity, activity-alias, service, and receiver declarations with intent filters. |
| `DEX-001` | Check for compiled DEX payloads. |
| `NATIVE-001` | Inventory native library ABIs. |
| `SIGNING-001` | Detect signature metadata. Cryptographic verification is deferred. |

## M0.2 project/artifact rules

| ID | Purpose |
| --- | --- |
| `PROJECT-001` | Load the Android application Gradle configuration, or report a blocker when it cannot be discovered/parsed. |
| `CROSSCHECK-000` | Explain when project parsing succeeded but artifact manifest evidence is unavailable. |
| `CROSSCHECK-001` | Compare Gradle `applicationId` with the final artifact package identity. |
| `CROSSCHECK-002` | Compare Gradle `targetSdk` with the final artifact `targetSdkVersion`; a mismatch is a blocker. |
| `CROSSCHECK-003` | Compare Gradle `minSdk` with the final artifact `minSdkVersion`; a mismatch is a warning. |
| `CROSSCHECK-004` | Compare Gradle `versionCode` with the final artifact versionCode. |
| `CROSSCHECK-005` | Compare Gradle `versionName` with the final artifact versionName. |
| `CROSSCHECK-006` | Compare the release build type's explicit debuggable/isDebuggable setting with the effective artifact value; a mismatch is a blocker. |

## M0.3 Google Play rules

Policy profile version: `2026-08-31`

| ID | Purpose |
| --- | --- |
| `PLAY-000` | Identify the versioned Google Play submission profile and platform. |
| `PLAY-001` | Check the final artifact target API against the platform-specific submission requirement. |
| `PLAY-002` | Manual Data Safety review item; the binary cannot prove the Play Console declaration. |
| `PLAY-003` | Manual privacy-policy review item; the binary cannot prove URL validity or policy completeness. |
| `PLAY-004` | Manual Play Console App content/declaration review item, including ads and applicable access/content declarations. |
| `PLAY-005` | Check native ELF 16 KB load-segment alignment against the applicable API requirement; parsing failures remain manual-review warnings. |

### M0.4 artifact rules

| ID | Purpose |
| --- | --- |
| `NATIVE-002` | Inspect every packaged `.so` ELF PT_LOAD `p_align` value. Report a pass when all inspected load segments meet 16 KB alignment, or a warning when the ELF cannot be parsed or an alignment below 16 KB is found outside the API 35+ Play blocker path. |
| `SIGNING-002` | Inspect APK signing-block structure, including placement, magic, size consistency, and supported v2/v3 IDs. This is structural evidence only; cryptographic signature verification is deferred. |

### M0.4 signing coverage

The APK signing-block parser recognizes:

| Scheme | Block ID |
| --- | --- |
| v2 | `0x7109871a` |
| v3 | `0xf05368c0` |

AABs do not use the APK signing block, so `SIGNING-002` is not applied to them. M0.4 also continues to report existing `META-INF` signature metadata through `SIGNING-001`.

### M0.4 16 KB boundary

`PLAY-005` uses the final artifact evidence collected by `NATIVE-002`:

- no native `.so`: pass;
- native ELF cannot be parsed: warning/manual review;
- API 35+ target with confirmed PT_LOAD alignment below 16 KB: blocker;
- API 35+ target with all inspected PT_LOAD alignments at or above 16 KB: pass;
- target below API 35 with a confirmed alignment below 16 KB: warning.

M0.4 does not yet verify ZIP entry alignment for uncompressed native libraries, so it does not claim complete end-to-end 16 KB packaging compatibility.

### M0.3 platform matrix

| Platform | Required target API for Play submission |
| --- | ---: |
| `mobile` | 36 |
| `wear` | 35 |
| `automotive` | 35 |
| `tv` | 34 |
| `xr` | 34 |

### Source policy

The target API matrix follows Google's current Play target API guidance:
https://developer.android.com/google/play/requirements/target-sdk

The 16 KB requirement follows Google's current Android guidance:
https://developer.android.com/guide/practices/page-sizes

Data Safety and Play Console declaration requirements follow Google's current Play Console guidance:
https://support.google.com/googleplay/android-developer/answer/10787469
https://support.google.com/googleplay/android-developer/answer/9859455
