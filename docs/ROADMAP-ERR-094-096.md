# Temporary Roadmap — ERR-094 / ERR-095 / ERR-096

**Purpose:** Track the controlled resolution of three compatibility findings discovered during post-release real-world artifact validation.

**Public-boundary rule:** This roadmap contains only technical problem statements, scope, acceptance criteria, and engineering order. It intentionally does not identify the external application or expose private test material.

## Current status

- **ERR-094:** RESOLVED — implementation, repository-level validation, and real-world AAB validation completed.
- **ERR-095:** OPEN — APK v2 verification false-negative investigation pending.
- **ERR-096:** OPEN — Android application plugin alias discovery investigation pending.

## Resolution order

### 1. ERR-094 — AAB manifest parsing

**Goal:** Support the protobuf manifest representation used inside Android App Bundles while preserving the existing APK binary-XML path.

**Work boundary**
- Audit the current manifest entry-point and format detection.
- Implement only the minimum decoder/model changes required.
- Preserve existing APK parsing semantics.
- Add focused fixtures for valid and malformed AAB manifest payloads.

**Acceptance**
- AAB manifest-derived fields required by existing rules become available.
- Existing APK manifest tests remain green.
- Malformed AAB manifest payloads remain explicit failures rather than silently accepted data.

### 2. ERR-095 — APK v2 signature false negative

**Goal:** Make valid v2/v3 APKs parse consistently with the Android SDK verifier without weakening cryptographic validation.

**Work boundary**
- Audit length-prefixed v2 signed-data parsing.
- Verify handling of certificate chains and additional attributes.
- Reproduce the trailing-bytes result with a controlled fixture before changing code.
- Add a regression test that captures the real compatibility case.
- Keep malformed/truncated signed data rejection strict.

**Acceptance**
- The previously valid real-world signature case no longer produces a false invalid result.
- Negative cases still reject malformed trailing data.
- v2/v3 verification evidence remains explicit and conservative.

### 3. ERR-096 — Android application plugin alias discovery

**Goal:** Recognize Android application modules that use supported Kotlin DSL/version-catalog plugin aliases.

**Work boundary**
- Audit current application-module discovery.
- Add explicit support for safe alias forms.
- Avoid broad substring heuristics that can produce false positives.
- Add positive and negative fixtures.

**Acceptance**
- Direct plugin-ID Groovy/Kotlin modules continue to work.
- Version-catalog Android application aliases are recognized.
- Non-application modules are not reclassified.

## Execution discipline

Resolve exactly one ERR at a time:

**pre-audit → minimal correction → focused regression tests → second audit → Build/Test/Format/Clippy → documentation update → checkpoint.**

Do not combine the three corrections into a single implementation change.

## Roadmap lifecycle

This document is **temporary**. Keep it while any of ERR-094, ERR-095, or ERR-096 is OPEN.

When all three are RESOLVED:
1. Register each correction and validation result in docs/ERRORS-AND-FIXES.md.
2. Perform a final documentation/public-boundary audit.
3. Delete this roadmap document.
4. Record the final checkpoint without replacing the historical error entries.
