# Pre-Audit — NATIVE-003 + Full ABI Matrix

Date: 2026-10-02
Baseline: `main` at `8698171ac203f6e6e4e762189a800f08027e101b`

## Scope

Audit the semantics of NATIVE-003 (native ZIP packaging alignment) and define a deterministic ABI oracle before any production-code change. The campaign must cover:

- `arm64-v8a` aligned/misaligned
- `x86_64` aligned/misaligned
- `armeabi-v7a` with 4 KiB / non-16 KiB alignment
- `x86` with 4 KiB / non-16 KiB alignment
- mixed 32-bit/64-bit APKs
- stored (uncompressed) versus compressed native libraries
- AAB behavior
- alignment-evidence/error paths

## Authoritative basis

Android Developers states that the Google Play 16 KB compatibility requirement applies to 64-bit devices and specifically identifies `arm64-v8a` and `x86_64` shared libraries for the automated alignment check:

https://developer.android.com/guide/practices/page-sizes

The same guidance states that apps shipping uncompressed shared libraries on 16 KB devices need 16 KB ZIP alignment, and recommends:

`zipalign -c -P 16 -v 4 <APK>`

For App Bundles, the guidance states that bundle alignment configuration must be verified separately and that the generated APK must also be checked:

`bundletool dump config --bundle=<AAB>`

AOSP further documents that 16 KB page-size Android targets are `arm64`, with `x86_64` supported for 16 KB user-space simulation in Cuttlefish:

https://source.android.com/docs/core/architecture/16kb-page-size/16kb

## Baseline implementation finding

NATIVE-003 currently evaluates every `.so` ZIP entry uniformly. For APKs it records 16 KiB data-offset alignment only for stored entries; for AAB stored entries it records that final APK ZIP alignment cannot be proven from the AAB entry offset and emits a manual-review condition.

The implementation currently does **not** filter NATIVE-003 by ABI. Therefore a misaligned stored `armeabi-v7a` or `x86` library can influence NATIVE-003 despite the authoritative 16 KB compatibility scope being the 64-bit ABI families.

## Required semantic correction

NATIVE-003 must evaluate 16 KB ZIP packaging compatibility only for:

- `arm64-v8a`
- `x86_64`

32-bit/non-target ABI entries remain visible in inventory but must not cause a 16 KB NATIVE-003 warning or manual-review outcome.

The existing distinction between APK and AAB remains:

- APK + applicable 64-bit stored library + bad offset -> WARNING.
- APK + applicable 64-bit stored libraries all aligned -> PASS.
- APK + applicable 64-bit compressed libraries only -> PASS.
- AAB + applicable 64-bit stored libraries -> MANUAL-REVIEW, because the AAB entry offset does not prove the final generated APK alignment.
- AAB + applicable 64-bit compressed libraries only -> PASS.
- 32-bit/non-target native libraries only -> PASS for NATIVE-003.
- Mixed ABI artifacts are judged only by their applicable 64-bit entries.

## Test oracle

The implementation campaign must add explicit deterministic tests for the above outcomes and for mixed ABI combinations. The tests must assert both finding ID and severity, not merely workflow success.

No production-code change is authorized by this pre-audit document until these semantics are used as the implementation boundary.

## Audit status

PRE-AUDIT COMPLETE — implementation may proceed within the scope above.
