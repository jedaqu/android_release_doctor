# Android Release Doctor

GitHub Actions service for auditing Android APK and AAB artifacts in CI.

> **Status:** The primary service surface is the reusable GitHub Action. The Rust audit engine and CLI provide the service implementation and a local validation interface. Current-state truth is maintained in [CURRENT-PRODUCT-STATE-2026-10-03.md](docs/CURRENT-PRODUCT-STATE-2026-10-03.md); historical milestone documents remain engineering history and are not the current product contract.

> **Validation status (2026-10-03):** The maintained Rust/CLI suite has 147 automated tests passing; the reusable Action is validated through normal, blocker, invalid-input, schema, deterministic-corpus, and external immutable-SHA consumer paths.

Android Release Doctor receives an Android APK or AAB as workflow input, can compare it with the Android application Gradle configuration, and can apply a versioned Google Play submission-readiness profile. The service returns structured evidence through a stable report contract and exit code.

## Current cryptographic coverage — post ERR-038-A / ERR-038-B

The following matrix describes the **current validated implementation state** on `main`. The historical M0.7 sections below remain unchanged as milestone history and are not a current-state matrix.

| Algorithm / variant | Current validated coverage |
|---|---|
| `0x0101` RSA-PSS / SHA-256 | RSA 1024, 2048–8192, and 16384 bits |
| `0x0102` RSA-PSS / SHA-512 | RSA 2048–8192 and 16384 bits; RSA 1024 remains Unsupported because the required 64-byte salt does not fit the modulus boundary |
| `0x0103` RSA PKCS#1 v1.5 / SHA-256 | RSA 1024, 2048–8192, and 16384 bits |
| `0x0104` RSA PKCS#1 v1.5 / SHA-512 | RSA 1024, 2048–8192, and 16384 bits |
| `0x0201` ECDSA / SHA-256 | NIST P-256, P-384, and P-521 |
| `0x0202` ECDSA / SHA-512 | NIST P-384 and P-521; P-256 remains Unsupported |
| `0x0301` DSA / SHA-256 | Cryptographic verification remains Unsupported and is explicitly deferred for a separate security/backend decision |
| APK Signature Scheme v3.2 / PQC | v3.2 block presence is recorded separately; cryptographic PQC verification is not implemented |

For v2/v3/v3.1, Release Doctor selects from the cryptographic algorithms it actually supports rather than treating an Android algorithm identifier alone as proof of verification capability.

ERR-038-A added and validated ECDSA P-521 support. ERR-038-B added and validated the RSA 1024/16384 boundaries without replacing the existing 2048–8192 `ring` path.

Android documents the broader v2/v3 algorithm matrix, including DSA and the larger RSA/EC key boundaries. Google Play App Signing currently uses RSA-based upload/app-signing keys; its quantum-ready option uses a hybrid RSA 4096 + ML-DSA-65 configuration. Those external capabilities do not automatically define Release Doctor's implementation scope.

References:

- https://source.android.com/docs/security/features/apksigning/v2
- https://source.android.com/docs/security/features/apksigning/v3
- https://source.android.com/docs/security/features/apksigning/v3-2
- https://support.google.com/googleplay/android-developer/answer/9842756


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

For APKs, `NATIVE-003` applies to the 64-bit device ABIs `arm64-v8a` and `x86_64`. An uncompressed native library in one of those ABIs with a non-16 KB-aligned data offset is surfaced through `NATIVE-003`. Compressed native libraries do not require this ZIP-offset check. 32-bit native libraries such as `armeabi-v7a` and `x86` remain inventoried but do not trigger this 16 KB packaging rule.

For AABs, a raw entry offset inside the bundle is **not** treated as proof of the final APK's native-library alignment. An uncompressed native library in an applicable 64-bit ABI therefore remains a manual-review case unless the generated APK and bundle alignment configuration are separately verified. AABs containing only 32-bit/non-target native libraries do not trigger `NATIVE-003`.

The Play `PLAY-005` check now combines ELF PT_LOAD evidence from M0.4 with the package-alignment evidence from this block. For API 35+ targets, a confirmed ELF or applicable ZIP misalignment is a blocker; an unverifiable part remains a manual-review warning rather than a guessed pass.

### M0.5 Block 1 usage

The ZIP/package evidence is collected automatically by the existing artifact audit:

```text
android-release-doctor --play app-release.apk
android-release-doctor --play --play-platform mobile app-release.apk
```

No external packaging command is executed by the core engine. Google documents `zipalign -v -c -P 16 4 <APK>` for checking APK alignment and recommends inspecting AAB alignment configuration with bundletool; those checks remain useful corroboration for the release pipeline.

## M0.5 Block 2 scope — APK cryptographic signature verification

M0.5 Block 2 adds cryptographic evidence for APK Signature Scheme v2 and v3 on top of the structural signing-block inspection from M0.4. Current Android documentation also defines v3.1 and, as of Android 17, v3.2; those newer blocks remain explicit manual-review boundaries in this milestone.

The verifier now checks:

- the v2/v3 signer structures and length-prefixed framing;
- the strongest **supported** signature algorithm according to Android's digest-algorithm preference;
- the cryptographic signature over `signed data`;
- the first X.509 certificate's SubjectPublicKeyInfo against the signer public key;
- the APK content digest using Android's 1 MiB chunked digest construction;
- v3 minimum/maximum SDK consistency;
- the required equality and ordering of digest/signature algorithm ID lists;
- v3.1 presence as an explicit manual-review boundary;
- v3.2 presence as an explicit manual-review boundary.

The verifier records a SHA-256 fingerprint of each verified signer's first certificate as additional signer evidence. It does **not** treat the certificate as trusted through a public CA; Android's app-signing model does not require a central certificate authority.

A cryptographic failure is distinct from an unsupported verification capability. Unsupported or incomplete verification is surfaced as a warning/manual-review result; it is never converted into a cryptographic pass.

## M0.6 Block 5 scope — v3 proof-of-rotation verification

M0.6 Block 5 extends the v3 cryptographic evidence boundary to proof-of-rotation when the lineage uses algorithms supported by the current verifier.

The verifier now:

- parses the versioned proof-of-rotation lineage structure;
- validates each lineage certificate and its length-prefixed structure;
- verifies each parent-to-child lineage signature using the previous certificate and declared algorithm;
- verifies the lineage's algorithm linkage and certificate uniqueness;
- verifies that the final lineage certificate matches the current v3 signer certificate;
- records structured proof-of-rotation evidence including lineage level count and verification state.

A malformed or cryptographically invalid lineage is reported as Invalid. A lineage that requires an unsupported verification algorithm remains Unsupported/manual review. The tool does not infer Android runtime certificate trust decisions from the lineage beyond the artifact evidence it actually verifies.

v3.1 and v3.2 remain explicit manual-review boundaries and are not cryptographically verified by this block.

## M0.7 Block 1 scope — ECDSA/SHA-512 P-384 coverage

M0.7 Block 1 originally introduced real verification for signature algorithm `0x0202` using P-384. That historical milestone description is superseded by the later classical cryptographic coverage documented below.

### Current classical cryptographic verification matrix

The current verifier supports and verifies these APK Signature Scheme v2/v3 algorithm families:

| Algorithm | Key/curve | Current verifier state |
|---|---|---|
| `0x0101` RSA-PSS / SHA-256 | RSA 1024, 2048–8192, 16384 | Supported and verified |
| `0x0102` RSA-PSS / SHA-512 | RSA 2048–8192, 16384 | Supported and verified |
| `0x0102` RSA-PSS / SHA-512 | RSA 1024 | Unsupported: 64-byte PSS salt does not fit the 1024-bit modulus |
| `0x0103` RSA-PKCS1-v1.5 / SHA-256 | RSA 1024, 2048–8192, 16384 | Supported and verified |
| `0x0104` RSA-PKCS1-v1.5 / SHA-512 | RSA 1024, 2048–8192, 16384 | Supported and verified |
| `0x0201` ECDSA / SHA-256 | P-256, P-384, P-521 | Supported and verified |
| `0x0202` ECDSA / SHA-512 | P-384, P-521 | Supported and verified |
| `0x0202` ECDSA / SHA-512 | P-256 | Unsupported |
| `0x0301` DSA / SHA-256 | DSA | Unsupported by explicit product/backend boundary |

The current matrix is covered by deterministic positive, tampered-signature, truncated-signature, and key-family-mismatch tests.

The classical signature algorithm IDs and RSA/EC key-size matrix follow the Android v2/v3 signing documentation:

- https://source.android.com/docs/security/features/apksigning/v2
- https://source.android.com/docs/security/features/apksigning/v3

### Current cryptographic scope boundaries

The verifier does **not** cryptographically verify:

- v3.2/PQC signing blocks;
- AAB cryptographic signing, because v2/v3 signatures belong to generated APKs;
- Android runtime trust decisions or PackageManager `SigningInfo` behavior.

Unsupported or incomplete verification is reported explicitly as Unsupported/manual review rather than being guessed into a cryptographic pass.

### M0.7 Block 1 usage

The additional cryptographic evidence is collected automatically when auditing an APK:

```text
android-release-doctor --play app-release.apk
```

## M0.7 Block 2 scope — proof-of-rotation semantic evidence

M0.7 Block 2 extends the validated v3 proof-of-rotation verifier with structured capability evidence for each lineage node.

The audit records:

- the raw lineage capability flags;
- the documented Android capability bits;
- the known-bit mask;
- unknown/reserved bits;
- the decoded capability state for each lineage node.

The verifier does not reject unusual combinations of independent capability bits merely because they are unusual. Unknown/reserved bits remain explicit evidence rather than being guessed into a cryptographic failure.

The existing lineage checks remain unchanged:

- certificate DER validation and uniqueness;
- parent-to-child signature verification;
- lineage algorithm linkage;
- final lineage certificate binding to the current v3 signer;
- signer-local evidence preservation.

v3.1/v3.2 semantics, AAB signing verification, and runtime Android trust-state inference remain outside this block.

## M0.7 Block 3 scope — APK Signature Scheme v3.1 verification

M0.7 Block 3 extends the validated v3 verifier to APK Signature Scheme v3.1 for the supported cryptographic algorithms already present in Release Doctor.

The audit can now verify:

- the v3.1 signing-block presence separately from v3;
- v3.1 signer signatures, certificate/public-key binding, digest and APK content digest;
- v3.1 SDK-range evidence;
- the v3 rotation-min-SDK stripping-protection attribute;
- consistency between the v3 stripping-protection value and the v3.1 rotation target;
- the required v3 base block;
- v3/v3.1 targeted-range ordering and allowed development-era boundary behavior;
- signer-count compatibility;
- proof-of-rotation lineage evidence across v3/v3.1 signers.

The implementation distinguishes malformed, cryptographically invalid, and unsupported cases and preserves the existing structured evidence model.

v3.2/PQC, AAB cryptographic signing verification, runtime Android trust-state simulation, and unrelated algorithm expansion remain outside M0.7 Block 3.

## M0.7 milestone — APK cryptographic verification completeness

M0.7 consolidates the validated cryptographic verification increments delivered by Blocks 1–3:

- **Block 1:** ECDSA/SHA-512 with NIST P-384 (\x600x0202\x60) has real signature verification; unsupported curves remain explicit \x60Unsupported\x60 evidence.
- **Block 2:** proof-of-rotation capability flags are decoded into structured known/unknown-bit evidence without guessing unusual combinations into cryptographic failure.
- **Block 3:** APK Signature Scheme v3.1 is cryptographically verified through the existing supported v2/v3 verification path, with v3/v3.1 rotation-target, stripping-protection, SDK-range, signer-count and lineage consistency checks.
- **Block 4:** integrates those capabilities into the top-level APK audit result and closes the milestone-wide documentation/CI boundary.

### M0.7 supported cryptographic coverage

| Algorithm ID | Algorithm | Current Release Doctor coverage |
|---|---|---|
| \x600x0101\x60 | RSA-PSS / SHA-256 | Supported for the current ring-backed RSA key range |
| \x600x0102\x60 | RSA-PSS / SHA-512 | Supported for the current ring-backed RSA key range |
| \x600x0103\x60 | RSA PKCS#1 v1.5 / SHA-256 | Supported for the current ring-backed RSA key range |
| \x600x0104\x60 | RSA PKCS#1 v1.5 / SHA-512 | Supported for the current ring-backed RSA key range |
| \x600x0201\x60 | ECDSA / SHA-256 | Supported for NIST P-256 and P-384 |
| \x600x0202\x60 | ECDSA / SHA-512 | Supported for NIST P-384 only |

Explicit boundaries remain:

- DSA \x600x0301\x60 is parsed as an Android algorithm ID but remains \x60Unsupported\x60 by the current verifier.
- RSA keys outside the current ring-backed 2048–8192-bit range remain \x60Unsupported\x60.
- ECDSA/SHA-512 curves other than P-384 remain \x60Unsupported\x60.
- v3.2/PQC remains outside M0.7.
- AAB cryptographic signing verification remains outside M0.7.
- Runtime PackageManager/SigningInfo behavior is not simulated.

The supported/unsupported distinction is evidence-first: parsing an algorithm ID does not make it cryptographically supported.


## M0.8 Block 2 scope — CLI output contract

M0.8 Block 2 adds stable command-line control around the Report v1 machine-readable contract while preserving the existing human-readable default.

### Output formats

The default remains text:

```text
android-release-doctor app-release.apk
```

Use Report v1 JSON for CI and tooling:

```text
android-release-doctor --format json app-release.apk
```

The report can be redirected to a file:

```text
android-release-doctor --format json --output report.json app-release.apk
```

Existing project and Play options can be combined with JSON output:

```text
android-release-doctor --project app/ --play --format json app-release.aab
```

### CLI controls

``--help`` prints the complete usage and exits successfully.

``--version`` prints the engine version and exits successfully.

The CLI keeps a stable exit-code contract:

- `0`: audit completed with no blockers;
- `1`: audit completed with one or more blockers;
- `2`: usage, input/audit, output-file, or internal serialization error.

MANUAL-REVIEW findings do not produce exit code 1. In JSON mode, stdout contains only the Report v1 JSON; diagnostics remain on stderr.


## M0.8 Block 3 scope — GitHub Action

M0.8 Block 3 exposes the existing CLI as a reusable GitHub composite action without changing the audit engine or Report v1.

Action path:

`.github/actions/android-release-doctor/action.yml`

Example:

```yaml
steps:
  - uses: actions/checkout@v6

  - name: Audit release
    id: release-doctor
    uses: jedaqu/android_release_doctor/.github/actions/android-release-doctor@<ref>
    with:
      artifact: app/build/outputs/apk/release/app-release.apk
      format: json
      output: android-release-doctor-report.json
```

Inputs:
- `artifact`: required APK/AAB path;
- `project`: optional Android application module path;
- `play`: optional `true|false`, default `false`;
- `play-platform`: optional Play platform, default `mobile`;
- `format`: `text|json`, default `text`;
- `output`: optional report output path.

Outputs:
- `exit-code`: the CLI exit code;
- `report-path`: the requested output path, when one was supplied.

The action preserves the CLI exit semantics:
- `0`: no blockers;
- `1`: one or more blockers;
- `2`: usage, input/audit, output, serialization, or internal error.

When `output` is supplied, the report is written by the existing CLI. A blocking audit therefore still leaves the generated report available for later workflow steps, for example with `continue-on-error: true` and a subsequent upload/inspection step.

The Action executes the validated CLI through Cargo. The runner therefore needs a usable Rust/Cargo toolchain.


## Documentation coherence

The repository treats documentation as part of the product contract. A product change is not considered complete while the public documentation, examples, schemas, tests, and current-state description disagree with the implemented behavior. Historical audit records preserve their original chronology; later corrections are recorded through append-only reconciliation or an explicitly newer current-state document rather than rewriting engineering history.

The working rule is:

**implementation change → evidence update → documentation update → second audit → closure.**

See [DOCUMENTATION-COHERENCE.md](docs/DOCUMENTATION-COHERENCE.md).

## Product boundary

Android Release Doctor is a GitHub Actions service. Its primary consumer interface is the reusable GitHub Action. The Rust audit engine and CLI are the implementation and local validation surfaces behind that service.

The service consumes an APK or AAB plus optional static project configuration, applies the configured audit and Play readiness rules, and returns a stable Report v1 document with an exact exit code.

The maintained technical surface is the reusable Action, shared audit engine, CLI, Report v1 contract, public fixtures, and permanent validation workflows.

## Still not implemented

- full Gradle/variant evaluation;
- product-flavor-aware expected-value resolution;
- CI/generated version resolution;
- DSA (algorithm ID `0x0301`) cryptographic verification;
- v3.2/PQC cryptographic verification;
- AAB cryptographic signing verification;
- current Google Play policy automation beyond the M0.3 readiness checks;
- permission risk classification;
- HTML/SARIF output.

## Engineering work principle — consult the incremental error ledger first

Before starting any new audit, code change, correction, refactor, or validation cycle, the current `docs/ERRORS-AND-FIXES.md` ledger must be reviewed first.

The ledger is the project's incremental memory of previously discovered failures and their resolutions. Reviewing it before touching code is intended to:

- avoid repeating previously identified mistakes;
- preserve successful corrective patterns;
- recognize regressions and related failure modes earlier;
- make audits and changes more precise and smaller in scope;
- reduce unnecessary trial-and-error changes;
- connect new failures with their historical context before choosing a correction.

This principle is mandatory for the project's working discipline and complements the sequence: **ledger review → audit → scoped changes → second audit → Actions → follow-up → individual correction → new validation → checkpoint**.

Every newly discovered failure or audit finding that represents an error, defect, CI failure, or corrective event must be appended to that ledger with its cause, correction, validation evidence, and status. Historical entries must not be renumbered or rewritten.

## Development

```bash
cargo check --workspace --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo run -p doctor-cli -- tests/fixtures/minimal-release.apk
cargo run -p doctor-cli -- --project tests/fixtures/project-release tests/fixtures/minimal-release.apk
cargo run -p doctor-cli -- --play tests/fixtures/minimal-release.apk
```

## Design principles

- **Local-first:** artifacts and project configuration are inspected locally.
- **Evidence before conclusions:** findings are tied to observable project or artifact evidence.
- **Actionable diagnostics:** warnings explain what to change.
- **Versioned rules:** Android and Play requirements change, so rules are explicit and replaceable.
- **Artifact first:** the final APK/AAB remains the source of truth for what is actually being distributed.
- **Conservative project parsing:** Gradle code is never executed.
- **Manual means manual:** when Play requires information that the binary cannot prove, Release Doctor reports a concrete review item instead of pretending it can verify it.

## License

Apache License 2.0.

## Getting started

The primary service surface is the reusable GitHub Action. The local CLI is provided for local validation, development, and troubleshooting.

### Local CLI

Run the audit engine from source:

```bash
cargo run -p doctor-cli -- tests/fixtures/minimal-release.apk
cargo run -p doctor-cli -- --project tests/fixtures/project-release tests/fixtures/minimal-release.apk
cargo run -p doctor-cli -- --play tests/fixtures/minimal-release.apk
cargo run -p doctor-cli -- --format json --output report.json tests/fixtures/minimal-release.apk
```

For a normal local development validation:

```bash
cargo check --workspace --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
```

### GitHub Action

The reusable action is available from:

```yaml
- uses: actions/checkout@v6

- name: Audit Android artifact
  id: release-doctor
  uses: jedaqu/android_release_doctor@<reviewed-ref>
  with:
    artifact: app/build/outputs/apk/release/app-release.apk
    format: json
    output: android-release-doctor-report.json
```

For supply-chain control, pin the Action to a reviewed commit SHA.

The Action executes the validated CLI through Cargo on the runner, so the runner must provide a usable Rust/Cargo toolchain. Its inputs are `artifact`, `project`, `play`, `play-platform`, `format`, and `output`; its outputs are `exit-code` and `report-path`.

### What the tool does not claim

Android Release Doctor reports evidence it can actually establish from the artifact and supplied project configuration. It does not pretend to verify information that only exists in external release systems.

Current explicit boundaries include:

- Play Console declarations remain manual review items;
- Gradle is parsed statically and is not executed;
- AAB signing is not treated as equivalent to final generated APK signing verification;
- unsupported cryptographic algorithms remain explicit unsupported/manual-review evidence.

