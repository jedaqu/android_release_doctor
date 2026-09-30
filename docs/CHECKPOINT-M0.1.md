# Checkpoint M0.1 — Manifest Inspection

Date: 2026-09-29
Branch: `m01-manifest`
Validated baseline commit before checkpoint: `6dba686f377da5a959186de97181586d79337541`
CI validation: Run #36 (`36658969331`)

## Status

M0.1 is a validated baseline. Build, Test, Format and Clippy all passed.

## Validated scope

- APK/AAB binary AndroidManifest.xml inspection.
- Package/application identity.
- versionCode and versionName.
- minSdkVersion and targetSdkVersion.
- Effective android:debuggable state.
- Declared permissions.
- Component and intent-filter inspection.
- Explicit android:exported requirements for supported component types.
- AAB base-module manifest inspection.

## Hardening

- Binary AXML bounds and string-pool validation.
- UTF-8 and UTF-16 decoding.
- Attribute-size/stride validation.
- Parser errors preserved in the audit report.
- MANIFEST-002 blocker for malformed/unparseable manifests.
- Defensive inconsistent-state handling.
- Reproducible Cargo.lock.
- Real APK/AAB fixtures.

## Tests

Coverage includes successful manifest parsing, rule evaluation, exported-component behavior, malformed manifests, and end-to-end malformed-APK reporting.

## Deferred

- Gradle project parsing.
- Project-vs-artifact comparison.
- Cryptographic signature verification and signing-block inspection.
- Current Google Play policy rules.
- Permission risk classification.
- HTML/SARIF output.

## Baseline rule

M0.1 behavior should remain stable while M0.2 begins. New work should build on the existing artifact inventory and manifest evidence model.

## Recovery

Use branch `m01-manifest` at commit `6dba686f377da5a959186de97181586d79337541` as the validated M0.1 code baseline.
