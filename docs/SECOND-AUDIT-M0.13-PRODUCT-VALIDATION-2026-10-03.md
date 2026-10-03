# M0.13 — Product Validation Expansion Second Audit — 2026-10-03

## Baseline and head

- Base main: `e00bc2b7bd81fed23bfdf62666f279b53607ef69`
- Final test head: `075e707e437cf2a5376d6e76cbd73fc751732596`
- Pull request: #60
- Scope: permanent Action validation expansion only

## Diff audit

The final diff contains exactly:

1. `.github/workflows/android-release-doctor-action.yml`
2. `docs/PRE-AUDIT-M0.13-PRODUCT-VALIDATION-2026-10-03.md`

No Rust production source, CLI source, Action implementation, Report v1 schema, fixtures, or detection logic changed.

## Existing suite

Rust CI on main and on the M0.13 validation branch passes:

- 115 doctor-core unit tests.
- 25 doctor-core integration tests.
- 7 CLI integration tests.
- Build.
- Format.
- Clippy.

Total directly executed Rust/CLI tests: 147 passing.

## Expanded Action validation

The permanent Action validation now exercises:

- APK success path;
- APK Play blocker path;
- AAB artifact path;
- project input path;
- project + Play combined path;
- non-default Play platform;
- operational error path;
- invalid `play` input;
- invalid output format;
- invalid Play platform;
- invalid CR/LF output path;
- Report v1 basic JSON contract;
- Action exit-code and report-path outputs.

### Validation result

PR #60 Action validation completed successfully after correcting two test-harness assumptions:

- The minimal AAB fixture legitimately returns exit code 1 because the artifact contains a blocker; the validation was changed to assert the documented blocker contract while still validating Report v1 and artifact kind.
- The AAB test was marked `continue-on-error` because a nonzero product result is expected and must be inspected through the Action outputs.

These corrections were test-harness corrections, not product defects.

## Product-claim audit

The following promised capabilities are covered by the current automated suite and fixture matrix:

- APK/AAB inventory and manifest inspection;
- package/version/SDK/debuggable data;
- components/exported checks;
- permissions inventory;
- static Groovy and Kotlin Gradle parsing/cross-checks;
- Play target/platform readiness rules;
- ELF and native 16 KiB evidence;
- APK ZIP/native 16 KiB alignment rules;
- v2/v3/v3.1 signature verification within supported algorithms;
- proof-of-rotation verification and capability evidence;
- explicit Unsupported/Manual Review boundaries;
- Report v1 serialization behavior;
- stable CLI exit codes;
- reusable Action execution and output contract.

Explicit non-claims remain outside executable product verification by design: external Play Console declarations, final generated APK equivalence from an AAB, AAB cryptographic signing verification, v3.2/PQC verification, and runtime Android trust-state behavior.

## Remaining validation gaps

The following are not product defects but should be treated as future validation improvements:

- direct JSON Schema validation of generated Report v1 against the published schema file;
- black-box validation against a real consumer repository rather than only an in-repository composite Action invocation;
- additional real-world third-party APK/AAB samples beyond the deterministic project fixtures.

## Disposition

M0.13 is technically validated as a test-only expansion and is ready for integration.
