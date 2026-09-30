# Android Release Doctor

Open-source, local-first tool for auditing Android APK and AAB releases before publication.

> **Status:** early development — M0.5 Block 2 (APK v2/v3 cryptographic signature verification).

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

## M0.5 Block 1 scope — ZIP/package alignment

M0.5 Block 1 closes the next part of the native 16 KB evidence chain.

For each packaged native `.so`, the audit now records:

- whether the ZIP entry is stored (uncompressed) or compressed;
- the real ZIP data-start offset exposed by the archive;
- whether an uncompressed APK native library starts on a 16 KB boundary;
- explicit verification errors instead of guessing.

For APKs, an uncompressed native library with a non-16 KB-aligned data offset is surfaced through `NATIVE-003`. Compressed native libraries do not require this ZIP-offset check.

For AABs, a raw entry offset inside the bundle is **not** treated as proof of the final APK's native-library alignment. An uncompressed native library in an AAB therefore remains a manual-review case unless the generated APK and bundle alignment configuration are separately verified.

The Play `PLAY-005` check now combines ELF PT_LOAD evidence from M0.4 with the package-alignment evidence from this block. For API 35+ targets, a confirmed ELF or applicable ZIP misalignment is a blocker; an unverifiable part remains a manual-review warning rather than a guessed pass.

### M0.5 Block 1 usage

The ZIP/package evidence is collected automatically by the existing artifact audit:

```text
android-release-doctor --play app-release.apk
android-release-doctor --play --play-platform mobile app-release.apk
```

No external packaging command is executed by the core engine. Google documents `zipalign -v -c -P 16 4 <APK>` for checking APK alignment and recommends inspecting AAB alignment configuration with bundletool; those checks remain useful corroboration for the release pipeline.

## M0.5 Block 2 scope — APK cryptographic signature verification

M0.5 Block 2 adds cryptographic evidence for APK Signature Scheme v2 and v3 on top of the structural signing-block inspection from M0.4.

The verifier now checks:

- the v2/v3 signer structures and length-prefixed framing;
- the strongest **supported** signature algorithm according to Android's digest-algorithm preference;
- the cryptographic signature over `signed data`;
- the first X.509 certificate's SubjectPublicKeyInfo against the signer public key;
- the APK content digest using Android's 1 MiB chunked digest construction;
- v3 minimum/maximum SDK consistency;
- the required equality and ordering of digest/signature algorithm ID lists;
- v3 proof-of-rotation presence as an explicit manual-review boundary rather than a guessed pass;
- v3.1 presence as an explicit manual-review boundary.

The verifier records a SHA-256 fingerprint of each verified signer's first certificate as additional signer evidence. It does **not** treat the certificate as trusted through a public CA; Android's app-signing model does not require a central certificate authority.

A cryptographic failure is distinct from an unsupported verification capability. Unsupported or incomplete verification is surfaced as a warning/manual-review result; it is never converted into a cryptographic pass.

### M0.5 Block 2 usage

Cryptographic verification is collected automatically when auditing an APK:

```text
android-release-doctor --play app-release.apk
android-release-doctor --play --play-platform mobile app-release.apk
```

AABs are not cryptographically verified in this module because APK v2/v3 signatures live in the generated APK signing block rather than in the AAB artifact itself.

The implementation follows Google's documented v2/v3 verification flow, including signer signature verification, digest verification and certificate/public-key binding. citehttps://source.android.com/docs/security/features/apksigning/v2|AOSP APK Signature Scheme v2 citehttps://source.android.com/docs/security/features/apksigning/v3|AOSP APK Signature Scheme v3 

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
- full cryptographic coverage of every Android-supported v2/v3 signature algorithm and key size;
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
