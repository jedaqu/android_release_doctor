# ERR-095 — Delimitation of the corrective change

## Status

**Root cause:** DEMONSTRATED  
**Implementation:** APPLIED — focused correction and regression coverage on the ERR-095 branch  
**ERR-096:** OUT OF SCOPE  
**Production code changes in this document:** NONE

This document freezes the correction boundary after byte-level reproduction of the real-world v2 false negative.

## 1. Confirmed defect boundary

The current v2 parser performs:

1. read the length-prefixed digests sequence;
2. read the length-prefixed certificates sequence;
3. read the length-prefixed additional-attributes sequence;
4. require the `signedData` reader to be fully consumed.

The reproduced compatible APK contains, after those three known fields, exactly one additional length-prefixed element whose length is zero:

```text
00 00 00 00
```

Those four bytes are inside the signed v2 `signedData` byte sequence and are therefore covered by the cryptographic signature. The reference Android signing implementation can emit that fourth empty element, while the reference verifier accepts the signed data without requiring exhaustive consumption after the three known fields.

The defect is therefore **not** a general requirement to ignore trailing bytes. It is a specific compatibility case involving one valid, empty, length-prefixed fourth element.

## 2. Correction boundary

The future correction may:

- preserve the existing parsing of digests, certificates, and additional attributes;
- after those three fields, recognize exactly one additional length-prefixed element;
- accept that fourth element **only when its encoded length is exactly zero**;
- then require complete consumption of `signedData`;
- continue verifying the cryptographic signature over the complete original `signedData` bytes, including the fourth empty element.

The future correction must **not**:

- delete or globally weaken `LengthReader::finish()`;
- accept arbitrary non-zero residual bytes;
- accept more than one residual element;
- accept a residual element whose length is non-zero;
- ignore truncated four-byte length prefixes;
- ignore inconsistent or overflowing length fields;
- alter verification of the outer v2 signer structure;
- alter certificate/public-key binding;
- alter APK content-digest verification;
- alter v3, v3.1, v3.2, or v4 behavior;
- modify SIGNING-001/002 semantics;
- touch ERR-096;
- introduce an unrelated parser refactor.

## 3. Required malformed-input behavior

The corrected parser must continue rejecting all of the following:

### A. Incomplete optional fourth field

Residual bytes after the third field are fewer than four bytes.

Expected result: **Invalid**.

### B. Non-empty optional fourth field

A complete fourth length prefix is present and its encoded length is greater than zero.

Expected result: **Invalid**.

### C. Multiple residual fields

The first optional fourth field is empty, but bytes remain afterward.

Expected result: **Invalid**.

### D. Malformed length

The fourth field length cannot be read safely or exceeds the containing `signedData`.

Expected result: **Invalid**.

### E. Unrelated trailing bytes

Any remaining bytes that do not form exactly one encoded empty fourth element.

Expected result: **Invalid**.

This preserves strict structural validation while adding the demonstrated compatibility case.

## 4. Required regression coverage before implementation closure

The implementation block must contain focused coverage for all of these cases:

1. **Existing conventional v2 fixture**
   - three known fields only;
   - no residual bytes;
   - result remains `Verified`.

2. **Compatible fourth-empty-element fixture**
   - three known fields;
   - exactly one additional four-byte zero length prefix;
   - valid cryptographic signature over the complete `signedData`;
   - result is `Verified`.

3. **Short residual negative case**
   - fewer than four residual bytes;
   - result is `Invalid`.

4. **Non-empty fourth-element negative case**
   - a fourth length-prefixed element with non-zero payload;
   - result is `Invalid`.

5. **Multiple-element negative case**
   - one empty fourth element followed by additional bytes;
   - result is `Invalid`.

6. **Truncated/overflowing length negative case**
   - malformed length structure;
   - result is `Invalid`.

7. **Cryptographic integrity**
   - the compatible fixture must verify the signature over the complete original `signedData`;
   - the correction must not bypass or move cryptographic verification.

## 5. Regression-compatibility rule

The real-world artifact that demonstrated ERR-095 remains private evidence.

The public repository must not add:

- the external application identity;
- the real third-party APK/AAB;
- private workflow reports;
- private artifact URLs or internal evidence paths.

The public regression should reproduce the **proven structural compatibility contract**, not publish the private artifact.

## 6. Exact implementation scope

The anticipated production change is confined to the v2 `signedData` parser in:

`crates/doctor-core/src/signature_verify.rs`

The implemented semantic change is limited to the transition immediately after parsing:

- `v2 digests`;
- `v2 certificates`;
- `v2 additional attributes`.

The parser now accepts exactly one additional length-prefixed element only when its payload is empty, then requires complete consumption. No public evidence-model change was introduced.

## 7. Acceptance criteria

ERR-095 implementation can proceed to second audit only when:

- the demonstrated empty fourth element is accepted;
- conventional v2 signed data remains accepted;
- malformed/trailing cases above remain rejected;
- the cryptographic signature is still verified over the complete signed data;
- no v3/v3.1/v3.2 behavior changes are introduced;
- focused tests pass;
- the second audit confirms the diff is limited to the defined scope.

Final closure still requires the repository workflow's four terminal gates:

**Build → Test → Format → Clippy**

followed by documentation update and checkpoint.

## 8. References

Android APK Signature Scheme v2:
https://source.android.com/docs/security/features/apksigning/v2

AOSP `V2SchemeSigner`:
https://android.googlesource.com/platform/tools/apksig/+/master/src/main/java/com/android/apksig/internal/apk/v2/V2SchemeSigner.java

AOSP `V2SchemeVerifier`:
https://android.googlesource.com/platform/tools/apksig/+/master/src/main/java/com/android/apksig/internal/apk/v2/V2SchemeVerifier.java
