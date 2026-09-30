# Checkpoint M0.4 — Native ELF and APK signing inspection

Date: 2026-09-30
Branch: `m04-elf-signing`
Validated code baseline: `c6da9e76fb3d3df4e15ffdde42cbc386b98590a9`
Code CI validation: Run #64 (`36662225930`)
Documentation validation before checkpoint: Run #65 (`36662407827`) and Run #66 (`36662474845`)

## Status

M0.4 is a validated implementation baseline. The code baseline passed Build, Test, Format and Clippy. Documentation updates were also validated by CI before the checkpoint commit.

## Validated scope

- ELF32 and ELF64 inspection for packaged native `.so` files.
- Extraction of every ELF `PT_LOAD` `p_align` value.
- Automatic 16 KB alignment evidence per native library.
- Explicit parse-failure evidence instead of guessed compatibility.
- `NATIVE-002` artifact rule for native ELF alignment.
- Evidence-driven `PLAY-005` behavior:
  - no native payload: pass;
  - unreadable native ELF: warning/manual review;
  - API 35+ target with confirmed PT_LOAD alignment below 16 KB: blocker;
  - API 35+ target with inspected PT_LOAD alignment at or above 16 KB: pass;
  - target below API 35 with confirmed sub-16 KB alignment: warning.
- APK signing-block structural inspection.
- Detection of standard v2 (`0x7109871a`) and v3 (`0xf05368c0`) signing-block IDs.
- Validation of signing-block placement, magic, leading/trailing size consistency and ID/value pair bounds.
- Conservative handling of unreadable or unsupported signing-block structures.
- AAB signing semantics remain unchanged; APK signing-block inspection is not applied to AABs.
- Existing M0.1/M0.2/M0.3 audit paths remain available.

## Hardening and debugging

The M0.4 CI cycle exposed and corrected two concrete issues before final validation:

1. Synthetic APK fixtures initially built an invalid 24-byte EOCD record. The fixture was corrected to the ZIP EOCD 22-byte layout, which fixed the signing-block unit tests.
2. Connector-side source serialization around the first ELF file revision produced an unexpected Rust lexical parse failure in CI even though the visible file contents were correct. The ELF source was deleted and recreated from a clean UTF-8 source, then revalidated through Build, Test, Format and Clippy.

No CI failure was left unresolved or bypassed.

## Test coverage

- ELF64 aligned PT_LOAD parsing.
- ELF32 sub-16 KB PT_LOAD parsing.
- Truncated ELF program-header table rejection.
- ELF with no PT_LOAD rejection.
- APK v2/v3 signing-block detection.
- Missing signing block handling.
- Mismatched signing-block size rejection.
- Native-library inventory regression coverage for APK/AAB fixtures.
- Integration coverage for signing-block absence.
- Play `PLAY-005` blocker/pass/manual paths.
- Full existing M0.1/M0.2/M0.3 regression suite.

## Important boundary

M0.4 does not claim complete end-to-end 16 KB packaging compatibility.

The current implementation checks ELF PT_LOAD alignment inside packaged native libraries. It does not yet verify the ZIP entry/file-offset alignment of uncompressed `.so` files, and it does not cryptographically verify APK signatures or signer certificates.

## Deferred

- ZIP alignment verification for uncompressed native libraries.
- Full cryptographic APK signature verification.
- Detailed signer/certificate parsing for v2/v3/v3.1.
- Full Gradle/variant evaluation.
- Product-flavor-aware project expectations.
- CI/generated version resolution.
- Broader Google Play policy automation.
- Permission risk classification.
- HTML/SARIF output.

## Sources

- Google Play target API requirements:
  https://developer.android.com/google/play/requirements/target-sdk
- Google Android 16 KB page-size guidance:
  https://developer.android.com/guide/practices/page-sizes
- AOSP APK Signature Scheme v2:
  https://source.android.com/docs/security/features/apksigning/v2
- AOSP APK Signature Scheme v3:
  https://source.android.com/docs/security/features/apksigning/v3

## Baseline rule

M0.4 extends the existing artifact evidence model. Future milestones must preserve:

- artifact-first evidence;
- explicit manual-review boundaries;
- versioned Play rules;
- deterministic diagnostics;
- existing M0.1/M0.2/M0.3 behavior when newer checks are not requested.

## Recovery

Use `c6da9e76fb3d3df4e15ffdde42cbc386b98590a9` as the validated M0.4 code baseline.

The branch `m04-elf-signing` contains the complete M0.4 implementation plus documentation. The checkpoint commit is documentation-only and should be validated by the final CI run for this milestone.
