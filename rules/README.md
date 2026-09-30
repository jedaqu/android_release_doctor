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
| `PLAY-005` | Check whether native payloads require 16 KB page-size compatibility review; detailed ELF alignment inspection is deferred. |

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
