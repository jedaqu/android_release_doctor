# Checkpoint M0.2 — Project / Artifact Cross-Check

Date: 2026-09-30
Branch: `m02-project-artifact-crosscheck`
Validated code baseline before checkpoint: `3410ea19bf4f0b2b2859af5112b6918371d8a242`
CI validation: Run #46 (`36660023761`)

## Status

M0.2 is a validated baseline. Build, Test, Format and Clippy all passed on the code baseline above.

## Validated scope

- Static parsing of Android application `build.gradle`.
- Static parsing of Android application `build.gradle.kts`.
- Project-side namespace and applicationId extraction.
- Project-side compileSdk, minSdk and targetSdk extraction.
- Project-side versionCode and versionName extraction.
- Project-side release debuggable/isDebuggable extraction.
- Project-vs-artifact application identity cross-check.
- Project-vs-artifact targetSdk cross-check.
- Project-vs-artifact minSdk cross-check.
- Project-vs-artifact versionCode and versionName cross-checks.
- Project-vs-artifact effective debuggable cross-check.
- `--project` CLI integration while preserving artifact-only M0.1 behavior.
- Project parsing and comparison failure paths reported inside the audit model.

## Hardening

- Gradle parsing is static; Release Doctor never executes Gradle code.
- Project discovery rejects ambiguous Android application modules.
- Unsupported project paths and parse failures are preserved in the audit report.
- Target SDK and debuggable mismatches are explicit blockers.
- Mismatch and matching fixtures cover both supported Gradle syntaxes.
- The final APK/AAB manifest remains the artifact-side source of truth.

## Tests

Coverage includes:

- Groovy DSL project parsing.
- Kotlin DSL project parsing.
- Matching project/artifact comparison.
- targetSdk mismatch detection.
- invalid project-path handling.
- Existing M0.1 APK/AAB manifest, malformed-manifest, component, and inventory coverage.

## Deferred

- Full Gradle execution and variant resolution.
- Product-flavor-aware project expectations.
- CI/generated version resolution.
- Cryptographic signature verification and Android signing-block inspection.
- Current Google Play policy rules.
- Permission risk classification.
- HTML/SARIF output.

## Baseline rule

M0.2 behavior should remain stable while the next milestone begins. New work should extend the existing artifact evidence model and preserve the static project-side evidence boundary.

## Recovery

Use branch `m02-project-artifact-crosscheck` at commit `3410ea19bf4f0b2b2859af5112b6918371d8a242` as the validated M0.2 code baseline.
