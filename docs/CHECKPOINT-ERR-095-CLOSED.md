# CHECKPOINT — ERR-095 CLOSED

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Scope: APK v2 signature verification compatibility

## Closure state

ERR-095 completed its full lifecycle:

**PRE-AUDIT → real-world byte-level reproduction → root-cause demonstration → correction delimitation → minimal implementation → focused regression coverage → second audit → public-boundary audit → Build/Test/Format/Clippy → documentation closure.**

## Root cause

The real-world v2 `signedData` structure contains the three known length-prefixed fields followed by exactly one additional empty length-prefixed element:

`00 00 00 00`

ADR previously required exhaustive consumption immediately after the third field and therefore classified this valid signed structure as trailing bytes.

## Final correction

`parse_signed_data_v2()` now:

1. parses the existing digests, certificates, and additional attributes;
2. accepts at most one additional length-prefixed element;
3. accepts that element only when its payload is empty;
4. retains the final exhaustive-consumption check.

Cryptographic verification continues to operate on the complete original `signedData`.

## Public regression

The public corpus contains:

`tests/fixtures/crypto-v2-empty-element-release.apk.b64`

The text fixture decodes to the canonical 1,588-byte APK with SHA-256:

`596db1d5efe23cf8a5227e430123595464c319f673cfc37f275f04c09efabdc5`

Regression coverage includes:

- conventional three-field v2 control remains Verified;
- empty fourth-element compatibility case is Verified end-to-end through SIGNING-003;
- truncated fourth element is rejected;
- non-empty fourth element is rejected;
- multiple residual elements are rejected.

## Final validation evidence

- Actions run #85 / `37015424258`: Build PASS; Test PASS; Format PASS; Clippy PASS.
- Actions run #88 / `37015630862`: documentation/second-audit state validated with Build PASS; Test PASS; Format PASS; Clippy PASS.
- 92 `doctor-core` unit tests passed.
- 17 `doctor-core` fixture integration tests passed, including the ERR-095 end-to-end case.

## Public/private boundary

The real third-party artifact and its private validation evidence remain private. The public repository contains only the generic, purpose-built compatibility contract and regression material.

No third-party application name or private artifact identifier is present in the public repository.

## Scope closure

ERR-096 was not touched.

ERR-093 remains separately OPEN until its corrected real publication is demonstrated end-to-end.

## Checkpoint conclusion

**ERR-095 CLOSED — APK v2 compatibility correction validated.**

The next work may begin only from this validated repository state.
