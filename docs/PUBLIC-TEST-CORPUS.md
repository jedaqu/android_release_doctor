# Public Test Corpus — Android Release Doctor

## Purpose

This document inventories the Android artifact fixtures and test-support material intentionally kept in the public repository.

The corpus is designed to exercise Android Release Doctor without publishing real third-party release artifacts or private real-world validation evidence.

## APK and AAB fixtures

| Fixture | Kind | Primary purpose |
|---|---|---|
| `tests/fixtures/minimal-release.apk` | APK | General manifest, inventory, project cross-check, Play-readiness, and unsigned/signing-absence controls |
| `tests/fixtures/minimal-release.aab` | AAB | Minimal App Bundle artifact control |
| `tests/fixtures/crypto-v2-release.apk` | APK | Conventional valid APK Signature Scheme v2 control |
| `tests/fixtures/crypto-v3-release.apk` | APK | Conventional valid APK Signature Scheme v3 control |
| `tests/fixtures/crypto-v31-release.apk` | APK | Valid v3.1 and proof-of-rotation control |
| `tests/fixtures/crypto-v31-no-v3.apk` | APK | Invalid v3.1 without a v3.0 base block |
| `tests/fixtures/crypto-v31-no-v3-attr.apk` | APK | Invalid v3.1 missing the required v3 stripping-protection attribute |
| `tests/fixtures/crypto-v31-wrong-rotation-min-sdk.apk` | APK | Invalid v3.1 SDK-range / rotation-min-sdk mismatch |
| `tests/fixtures/crypto-v31-lineage-mismatch.apk` | APK | Invalid v3.1 proof-of-rotation lineage control |
| `tests/fixtures/m07-crypto-v2-ecdsa-sha512-p384.apk` | APK | v2 ECDSA/SHA-512 P-384 cryptographic control |
| `tests/fixtures/m07-crypto-v3-ecdsa-sha512-p384.apk` | APK | v3 ECDSA/SHA-512 P-384 cryptographic control |
| `tests/fixtures/proof-rotation-valid.bin` | Binary fixture | Deterministic proof-of-rotation parsing and validation control |

## Test-support material

### Core artifact helpers

`crates/doctor-core/tests/fixtures.rs` contains focused support used to exercise the public corpus, including:

- temporary APK/AAB construction for malformed and protocol-specific cases;
- deterministic AAB protobuf-manifest construction;
- temporary native-library ZIP fixtures for 16 KiB alignment checks;
- tampering helpers for cryptographic regression tests;
- project fixtures for Groovy and Kotlin DSL cross-checks.

### Signature verification controls

The signature verification test module contains support for:

- length-prefixed sequence construction;
- v2/v3 signature-entry construction;
- certificate-chain parsing controls;
- proof-of-rotation fixture decoding and mutation;
- cryptographic tampering controls;
- direct parser-level malformed-input tests.

## ERR-095 public compatibility fixture

The ERR-095 implementation adds a public, generic v2 compatibility fixture for the demonstrated structure in which valid v2 `signedData` contains:

1. digests;
2. certificates;
3. additional attributes;
4. exactly one additional empty length-prefixed element encoded as `00 00 00 00`.

The fixture is intentionally synthetic and self-contained. It does not identify or contain the real-world third-party artifact that originally exposed ERR-095.

## Public/private boundary

The public corpus may contain generic, purpose-built fixtures needed to reproduce a technical contract.

The public repository must not contain:

- real third-party APK/AAB files used during private validation;
- private workflow reports from real-world artifacts;
- private artifact URLs;
- third-party application names when they are not required by the public technical contract;
- private continuity material.

The private repository remains the location for real-world artifact validation and associated evidence.

## Coverage principle

The public corpus is organized around reproducible technical behaviors, not around any particular external application.

A new fixture should be public only when it is:

- necessary to reproduce a documented technical contract;
- generic or purpose-built for testing;
- free of private third-party release material;
- covered by a focused regression test.
