# Checkpoint M0.3 — Google Play Release Readiness

Date: 2026-09-30
Branch: `m03-play-release-readiness`
Validated code baseline before checkpoint: `a9238df0050f070cd6e2631689b7e6e4cbfaecc1`
CI validation: Run #53 (`36661015535`)

## Status

M0.3 is a validated baseline. Build, Test, Format and Clippy all passed on the code baseline above.

## Validated scope

- Versioned Google Play submission-readiness profile `2026-08-31`.
- Platform profiles for mobile, Wear OS, Android Automotive, Android TV and Android XR.
- Final artifact target API validation against the current platform-specific submission matrix.
- Explicit `--play` CLI activation.
- Explicit `--play-platform` selection with mobile as the default.
- Combined project + Play auditing.
- Manual Play Console review items for Data Safety, privacy policy, and App content/declarations.
- 16 KB native-payload review trigger for artifacts with native libraries targeting API 35 or higher.
- Artifact-only and project-only M0.1/M0.2 paths remain available without `--play`.

## Current Play profile

Submission target API requirements encoded by M0.3:

| Platform | Required target API |
| --- | ---: |
| mobile | 36 |
| Wear OS | 35 |
| Android Automotive | 35 |
| Android TV | 34 |
| Android XR | 34 |

The profile uses Google's current target API guidance as of the 2026-08-31 policy effective date.

## 16 KB requirement

Google's current Android guidance requires apps targeting API 35+ to support 16 KB page sizes, with Google Play updates that do not support 16 KB becoming blocked from release starting 2027-02-01.

M0.3 detects native payloads and surfaces a manual verification item. It does not yet inspect native ELF load-segment alignment.

## Manual checks

The binary cannot prove Play Console declarations or the correctness of a privacy-policy URL. M0.3 therefore reports these as explicit review items rather than automatic passes.

## Tests

Coverage includes:

- Play platform target matrix.
- Mobile API 36 pass.
- Mobile API 35 blocker.
- Native payload with API 35+ requiring 16 KB review.
- End-to-end Play readiness on the real APK fixture.
- Existing M0.1 and M0.2 regression coverage.

## Deferred

- Native ELF 16 KB load-segment/alignment inspection.
- Full Gradle/variant evaluation.
- Product-flavor-aware project expectations.
- CI/generated version resolution.
- Cryptographic signature verification and Android signing-block inspection.
- Broader Google Play policy automation.
- Permission risk classification.
- HTML/SARIF output.

## Sources

- Target API requirements:
  https://developer.android.com/google/play/requirements/target-sdk
- 16 KB page-size compatibility:
  https://developer.android.com/guide/practices/page-sizes
- Data Safety:
  https://support.google.com/googleplay/android-developer/answer/10787469
- App review / App content:
  https://support.google.com/googleplay/android-developer/answer/9859455

## Baseline rule

M0.3 behavior should remain stable while the next milestone begins. New work should extend the existing artifact evidence model and preserve the distinction between automatically verifiable evidence and manual Play Console declarations.

## Recovery

Use branch `m03-play-release-readiness` at commit `a9238df0050f070cd6e2631689b7e6e4cbfaecc1` as the validated M0.3 code baseline.
