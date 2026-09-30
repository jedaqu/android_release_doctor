# Checkpoint M0.5 Block 1 — ZIP/package alignment

Date: 2026-09-30
Branch: `m05-package-alignment`
Validated code baseline: `43728c2a077a0aa4a4bada209ec3b9acae1571e2`
Code + documentation CI validation before checkpoint: Run #88 (`36664988499`)

## Status

M0.5 Block 1 is a validated implementation baseline on top of the M0.4 code line. The code and documentation state at the validated baseline passed Build, Test, Format and Clippy.

PR #6 remains open as a **draft stacked pull request**:
https://github.com/jedaqu/android-release-doctor/pull/6

Its current base is `m04-elf-signing`, so the review isolates the M0.5 delta instead of repeating the unmerged M0.4 history.

## Validated scope

- Detect every packaged native `.so` entry in the ZIP archive.
- Record whether each native entry is stored (uncompressed) or compressed.
- Record the real ZIP data-start offset from the Rust `zip 2.4.2` reader.
- Verify 16 KB ZIP data alignment for uncompressed native libraries in APKs.
- Treat compressed native libraries as not requiring this ZIP-offset alignment check.
- Record AAB native entry offsets without incorrectly treating the AAB offset as proof of the final generated APK offset.
- Surface explicit packaging verification errors instead of guessing a pass.
- Add `NATIVE-003` for native ZIP packaging evidence.
- Feed ZIP/package evidence into `PLAY-005`.
- Preserve the M0.4 ELF PT_LOAD evidence model.
- Preserve M0.4 APK signing-block structural inspection; no cryptographic verification is introduced in this block.

## PLAY-005 boundary after M0.5 Block 1

For targets at API 35 or higher:

- confirmed ELF PT_LOAD misalignment is a blocker;
- confirmed applicable APK ZIP misalignment is a blocker;
- an unavailable ELF or package-alignment check remains a warning/manual-review item;
- when all applicable evidence is verifiable and aligned, the check passes.

For AABs, raw bundle-entry alignment is not promoted to final APK alignment evidence. The generated APK and bundle alignment configuration remain outside this block.

## Hardening and debugging

The first CI cycle exposed three concrete issues, each corrected before the validated baseline:

1. The implementation initially treated `ZipFile::data_start()` as optional. CI Build on `36664653293` showed that the pinned `zip 2.4.2` API returns `u64`. The model was corrected to store the actual offset directly.
2. CI Format on Run `36664742350` required rustfmt adjustments in the reporting paths and the integration fixture helper. Those changes were applied.
3. CI Clippy on Run `36664808742` rejected a manual modulo check under `-D warnings`. The alignment primitive now uses `is_multiple_of()`.

After those corrections, Build, Test, Format and Clippy all passed on Run #88 (`36664988499`).

## Test coverage

Focused M0.5 coverage includes:

- unit tests for stored/compressed classification;
- unit tests for 16 KB boundary checking;
- APK integration fixture with misaligned stored native library;
- APK integration fixture with 16 KB-aligned stored native library;
- APK integration fixture with compressed native library;
- Play-policy blocker coverage for misaligned stored native payload on API 35+;
- AAB manual-review coverage for uncompressed native payload;
- existing M0.1/M0.2/M0.3/M0.4 regression suite.

CI reported 33 `doctor-core` unit tests and 12 `doctor-core` integration tests passing at the validated baseline.

## Important boundary

M0.5 Block 1 does not execute `zipalign` or bundletool.

Google's Android 16 KB guidance requires 16 KB ZIP alignment for uncompressed native libraries and documents `zipalign -v -c -P 16 4 <APK>` for APK verification. For AABs, the documented verification path includes inspecting bundle alignment configuration and validating the generated APK rather than inferring final APK alignment from a raw AAB entry offset.

Therefore this block claims **artifact-byte evidence**, not complete replacement of the Android packaging toolchain.

## Deferred

- cryptographic APK signature verification;
- v3.1 signing/key-rotation details beyond structural v2/v3 detection;
- full Gradle/variant evaluation;
- product-flavor-aware project expectations;
- CI/generated version resolution;
- broader Google Play policy automation;
- permission risk classification;
- HTML/SARIF output;
- automated bundletool/zipalign execution.

## Sources

- Google Android 16 KB page-size guidance:
  https://developer.android.com/guide/practices/page-sizes
- Google Play target API requirements:
  https://developer.android.com/google/play/requirements/target-sdk
- Rust `zip 2.4.2` crate:
  https://docs.rs/crate/zip/2.4.2
- Rust `ZipFile::data_start()` documentation:
  https://docs.rs/zip/latest/src/zip/read.rs.html

## Baseline rule

Future milestones must preserve:

- artifact-first evidence;
- explicit manual-review boundaries;
- deterministic diagnostics;
- conservative AAB semantics;
- existing M0.1/M0.2/M0.3/M0.4 behavior.

## Recovery

Use `43728c2a077a0aa4a4bada209ec3b9acae1571e2` as the validated M0.5 Block 1 code baseline.

The next M0.5 block is cryptographic APK signature verification. Do not start that work from an unvalidated intermediate commit.
