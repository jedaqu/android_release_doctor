# Android Release Doctor — v0.1.3 Candidate Notes

## What is prepared

This candidate represents the current maintained Android Release Doctor product surface after the M0.16 final readiness gate.

It provides:
- APK/AAB artifact inspection;
- application identity, SDK and debuggable evidence;
- component and permission inventory;
- static Groovy/Kotlin Gradle cross-checks;
- versioned Google Play readiness checks;
- ELF native inspection;
- native/ZIP 16 KiB evidence within the documented ABI boundary;
- APK v2/v3/v3.1 cryptographic verification for the supported classical algorithm matrix;
- supported v3 proof-of-rotation verification;
- explicit unsupported/manual-review boundaries;
- Report v1 JSON;
- stable CLI exit codes;
- reusable GitHub Action consumption.

## Validation

The candidate enters preparation with:
- 147 automated Rust/CLI tests passing;
- permanent Action Validation passing;
- Report v1 schema validation passing;
- external consumer immutable-SHA validation passing across success, blocker and invalid-input paths;
- independent public APK/AAB evidence from multiple projects.

## Important boundaries

Android Release Doctor is an evidence-producing audit tool. It does not execute arbitrary Gradle logic, simulate Android runtime trust state, prove all Play Console declarations, or cryptographically verify unsupported signing schemes.

For AABs, bundle contents are inspected but raw bundle offsets do not prove the final generated APK's native packaging alignment.

## Publication state

These notes are candidate release documentation only. No GitHub Release/tag publication is asserted here.
