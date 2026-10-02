# AUDIT-ERR-094 — Second Audit

## Scope

Second audit of the ERR-094 correction before public merge.

## Pre-audit finding

The AAB path selected the correct manifest entry but sent its protobuf-encoded contents to the APK binary-XML parser. Android's AAPT2 resource proto defines the manifest representation as `XmlNode`, `XmlElement`, and `XmlAttribute`, and bundletool consumes that representation for App Bundle manifests. citeturn786512search0turn865944search0

## Correction audited

- Added a dedicated protobuf decoder for the AAB manifest representation.
- Kept the existing APK binary-XML parser unchanged.
- Routed APK and AAB manifests through their corresponding parser.
- Reused the existing `ManifestInfo` model.
- Preserved strict malformed/truncated input handling.
- Added focused parser unit tests.
- Added an end-to-end AAB audit regression using a generated protobuf manifest.

## Regression surface

- Existing APK manifest tests pass.
- Existing AAB audit coverage was moved to the protobuf representation.
- The generated AAB regression extracts package identity, version code/name, minSdk, targetSdk, debuggable state, permission evidence, and activity/intent-filter evidence.
- Rust CI run `36960802366` completed Build, Test, Format, and Clippy successfully.

## Scope audit

Changed files:
- `crates/doctor-core/src/lib.rs`
- `crates/doctor-core/src/manifest_proto.rs`
- `crates/doctor-core/tests/fixtures.rs`

No changes were made to:
- APK AXML parsing behavior
- Play policy logic
- project parsing
- APK signing verification
- distribution workflows

## Final real-world validation

A real-world AAB regression check against the corrected public code passed. The generated Report v1 output contains an AAB application section with targetSdk data, `manifest_error` is null, and no `MANIFEST-002` finding is emitted.

This final evidence closes the original compatibility finding without identifying the external artifact source.

## Conclusion

**SECOND AUDIT — PASS. ERR-094 RESOLVED.**

The implementation, repository-level regression suite, and real-world AAB validation all pass. ERR-094 can now be removed from the active remediation set only at the historical-error level; its ledger entry is retained permanently.

The remediation roadmap remains active because ERR-095 and ERR-096 are still open.
