# Product Test & Stress Audit — 2026-10-02

## Scope

This audit exercised Android Release Doctor through GitHub Actions without modifying the product implementation on `main`.

The test surface covered:

- controlled APK/AAB fixtures already present in `tests/fixtures`;
- real public APK/AAB artifacts from Session Android 1.33.5;
- malformed and cryptographically tampered inputs;
- APK signing variants including v2, v3, v3.1 and ECDSA;
- project cross-checks for Groovy, Kotlin DSL and Version Catalog fixtures;
- Google Play profiles for mobile, wear, automotive, TV and XR;
- JSON and text output;
- invalid Action input handling;
- repeatability under five consecutive audits per selected artifact.

## Main integrity

- `main`: `0e5b46ae287f476e47bcf1f2c0082d12c04b5c38`
- No product source was changed during this test campaign.
- All test harnesses were isolated to dedicated test branches.

## Final corpus stress

Workflow: `.github/workflows/product-stress-2026-10-02.yml`

Final successful run:

- Run: `37078021226`
- Commit: `95a1da3529b213b1dd7c80e20b7b77e11640ad2f`
- Result: `success`
- Coverage: 11 artifact cases + Action interface contract + malformed-input stress.

The final malformed-input matrix produced:

| Case | Exit |
| --- | ---: |
| Truncated signed APK | 2 |
| Empty input | 2 |
| Random bytes | 2 |
| One-byte mutation of signed v2 APK | 1 |

The mutated signed APK was reported with `SIGNING-003: APK cryptographic signature invalid`, confirming that a real post-signing modification is detected.

## Controlled fixture observations

The product correctly surfaced the intended cryptographic failure classes:

- v3.1 lineage mismatch: `SIGNING-003 BLOCKER`, with proof-of-rotation verification failure.
- v3.1 rotation min-SDK mismatch: `SIGNING-003 BLOCKER`, with the stripping-protection rotation minimum mismatch.
- ECDSA v2 fixture: cryptographic verification passed.
- ECDSA v3 fixture: cryptographic verification passed.

Fixture-specific unrelated blockers (for example missing manifest/target SDK or incomplete exported component metadata) were preserved and did not prevent the signing checks from executing.

## Real public artifacts

Session Android 1.33.5 APK and AAB were downloaded in Actions from pinned release assets and their expected SHA-256 values were verified before audit.

Both artifacts produced stable Report v1 JSON and completed the audit successfully at the Action layer. Their product-level result was:

- target SDK 36;
- 32 native libraries detected;
- all inspected `arm64-v8a` and `x86_64` libraries were 16 KB aligned;
- four 32-bit libraries (`armeabi-v7a` / `x86`) used 4 KB ELF load alignment;
- ADR emitted `NATIVE-002 WARNING` and `PLAY-005 BLOCKER` because it evaluates the 4 KB-aligned 32-bit libraries as Play 16 KB incompatibilities.

## Candidate product finding: 16 KB ABI scope

This is the most important candidate defect found by the external corpus.

ADR currently checks every native library in `native_libraries` for the 16 KB ELF threshold. Its Play policy logic then treats any unaligned native library as a blocker for targetSdk >= 35.

Current Android guidance says the Google Play 16 KB requirement is for apps on 64-bit devices, and the official `check_elf_alignment.sh` guidance specifically calls out `arm64-v8a` and `x86_64` libraries as the libraries that must be aligned. The current Android platform script itself states that only those two ABI families need to be aligned.

The Session 1.33.5 observation therefore indicates that ADR may be over-detecting 16 KB incompatibility when only 32-bit libraries are below 16 KB.

This is recorded as a **candidate product defect / specification mismatch**, not a source-code correction in this test campaign.

Relevant official sources:

- Android 16 KB page-size guidance: https://developer.android.com/guide/practices/page-sizes
- Android platform `check_elf_alignment.sh`: https://android.googlesource.com/platform/system/extras/+/refs/heads/main/tools/check_elf_alignment.sh

## Repeatability stress

Workflow: `.github/workflows/product-repeatability-2026-10-02.yml`

Final successful run:

- Run: `37078285370`
- Result: `success`
- 5 cases × 5 consecutive audits = 25 repeated audits.

All five cases produced identical canonical JSON report hashes and identical exit codes across all five repetitions.

Observed post-build audit times on the GitHub runner:

| Case | Typical audit time |
| --- | ---: |
| crypto-v2 APK | ~27–29 ms |
| crypto-v31 APK | ~25–26 ms |
| minimal AAB | ~470–472 ms |
| Session APK | ~469–472 ms |
| Session AAB | ~1.59–1.62 s |

These are warm-run audit times after a single binary build and do not include cold compilation.

## Integration stress

Workflow: `.github/workflows/product-integration-2026-10-02.yml`

Final successful run:

- Run: `37078612022`
- Result: `success`

Coverage:

- Groovy Gradle project fixture: passed.
- Kotlin DSL Gradle project fixture: passed.
- Version Catalog Gradle project fixture: passed.
- Mobile Play profile: passed.
- Wear Play profile: passed.
- Automotive Play profile: passed.
- TV Play profile: passed.
- XR Play profile: passed.
- Text output contract: passed.
- Invalid Play platform input: returned usage error `2`.

## Test-harness corrections

Early failures in this campaign were isolated to the test harness, not ADR:

1. Initial JSON assertion expected a non-existent `summary.info` field. Report v1 actually defines `passed`, `warnings`, `blockers`, and `manual_review`.
2. Bash variable name `RANDOM` conflicted with Bash's special variable semantics.
3. A later malformed-test assertion still referenced the old environment names; corrected.
4. Initial repeatability build used `cargo build --locked`, while the production Action uses `cargo run` without `--locked`; corrected to match the real Action runtime.
5. Initial text-output test asserted a header string that is not guaranteed by the text format; reduced to the actual contract: successful execution, valid exit code and non-empty output file.

The final runs passed after these harness-only corrections.

## Audit conclusion

The current implementation is stable under the exercised corpus and repeatability load. The Action interface, JSON report contract, malformed-input handling, signing verification, project cross-checks, Play profile selection and text output all behaved consistently.

The principal product-level follow-up is the 16 KB ABI-scope finding: ADR appears to classify 32-bit `armeabi-v7a` / `x86` ELF alignment as a Play blocker even when the relevant 64-bit `arm64-v8a` / `x86_64` libraries are aligned. This should be resolved against the product specification and official Android guidance before changing implementation.

No other source-level defect is considered proven by this campaign.
