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
| `NATIVE-002` | Inspect every packaged `.so` ELF PT_LOAD `p_align` value. Report explicit alignment evidence or a manual-review warning when the ELF cannot be parsed. |
| `SIGNING-002` | Inspect APK signing-block structure, including placement, magic, size consistency, and supported v2/v3 IDs. This is structural evidence only; cryptographic signature verification is deferred. |

### M0.5 Block 1 artifact rules

| ID | Purpose |
| --- | --- |
| `NATIVE-003` | Inspect native ZIP packaging. For uncompressed native APK libraries, compare the actual ZIP data-start offset with the 16 KB boundary. Compressed native libraries do not require the offset check. AAB entry offsets are recorded but not treated as proof of final APK alignment. |

### M0.5 Block 2 artifact rules

| ID | Purpose |
| --- | --- |
| `SIGNING-003` | Cryptographically verify supported APK v2/v3 signer data, certificate/public-key binding and the protected APK content digest. Report `PASS` only when all required supported checks for the applicable scheme succeed; cryptographic failure is a blocker; unsupported/incomplete verification is a manual-review warning. |

### M0.5 Block 2 verification boundary

The verifier follows the Android v2/v3 verification model:

- v2 signer signature is verified over signed data;
- v3 signer signature is verified over signed data and its outer/inner SDK ranges must match;
- digest and signature algorithm ID lists must be identical and ordered equally;
- the selected content digest is recomputed using Android's 1 MiB chunk construction over the protected APK sections;
- the first X.509 certificate SubjectPublicKeyInfo must exactly match the signer public key;
- the first certificate SHA-256 fingerprint is retained as signer evidence;
- v3 proof-of-rotation is detected but not fully verified in this block;
- v3.1 is detected structurally but not cryptographically verified in this block;
- v3.2 is detected structurally but not cryptographically verified in this block;
- algorithms, key sizes, curves, or PQC signatures outside the current Rust verifier's supported set remain manual-review evidence rather than being reported as verified.

Android documents the v2/v3 algorithm IDs, digest construction and verification sequence in the AOSP documentation:
https://source.android.com/docs/security/features/apksigning/v2
https://source.android.com/docs/security/features/apksigning/v3
https://source.android.com/docs/security/features/apksigning/v3-1
https://source.android.com/docs/security/features/apksigning/v3-2

### M0.5 Block 1 16 KB semantics

The package-level evidence is intentionally narrower than a full bundle/package validation:

- APK + stored `.so` + 16 KB data offset: verifiable package alignment;
- APK + stored `.so` + non-16 KB data offset: package misalignment;
- APK + compressed `.so`: ZIP data-offset alignment is not applicable;
- AAB + stored `.so`: raw AAB offset is not treated as final APK alignment evidence;
- any unsupported/unverifiable packaging state: warning/manual review, never a guessed pass.

`PLAY-005` combines these package findings with M0.4 ELF PT_LOAD evidence. For API 35+ targets, confirmed native alignment failure is a blocker; missing evidence remains a warning/manual review.

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
