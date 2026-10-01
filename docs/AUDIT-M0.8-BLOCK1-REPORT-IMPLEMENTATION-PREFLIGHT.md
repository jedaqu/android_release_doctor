# M0.8 Block 1 — Implementation Pre-Flight Audit

Date: 2026-10-01
Branch: `m08-block1-report-contract`
Baseline: `aa055402af1989f86100092b08cbb3e38eaa535e`
PR: #17 — open, unmerged

## Result

**PASS — bounded production implementation is authorized.**

## 1. Domain ownership

The internal audit model is owned by `crates/doctor-core/src/lib.rs`.

Primary source model:
- `AuditReport`
- `ArtifactInventory`
- `ManifestInfo`
- `ProjectInfo`
- `Finding`
- APK signing and cryptographic verification evidence

The existing `render_text()` path remains the legacy text renderer and is not modified by this movement.

## 2. DTO placement

The Report v1 DTO layer will live in:

`crates/doctor-core/src/report.rs`

The DTO layer will be explicit and independent from the internal domain structs. Internal structs will not derive or receive JSON serialization traits.

The public mapping entry point will accept:
- an `AuditReport` reference;
- a separate `ReportV1Context` carrying optional Play invocation context.

This preserves the frozen contract rule that Play context is an envelope and is not stored in `AuditReport`.

## 3. Serialization infrastructure

Current state:
- no direct Serde dependency in `doctor-core`;
- no JSON serializer exists;
- `Cargo.lock` already contains Serde transitively, but `serde_json` is absent;
- CI runs Cargo normally without `--locked`.

Authorized dependency addition:
- `serde` with derive support;
- `serde_json`.

No alternative serialization framework is introduced.

## 4. Concrete mapping surface

The DTO mapper must cover the frozen v1 contract exactly:

- schema and engine version;
- artifact kind/path/size;
- application manifest evidence;
- manifest error;
- archive/native/signing inventory;
- APK signing-block evidence and inspection errors;
- APK cryptographic verification, including v2/v3/v3.1, presence boundaries, proof-of-rotation and unknown capability bits;
- project evidence and project error;
- optional Play context;
- structured findings;
- summary counts.

Semantic requirements:
- `null` means optional evidence was not produced;
- `[]` means the inspected collection is empty;
- `false` remains an explicit negative observation;
- error strings remain distinct from absence.

## 5. Test reuse and new tests

Existing unit tests in `doctor-core/src/lib.rs` remain unchanged and continue covering domain behavior.

New focused tests in `report.rs` will cover:
1. complete DTO-to-JSON mapping;
2. null/empty/false/error semantics;
3. Play context absent vs present;
4. cryptographic/proof-of-rotation evidence preservation;
5. deterministic JSON output for the same domain report.

No end-to-end CLI test is introduced in Block 1 because CLI wiring is explicitly deferred to Block 2.

## 6. Minimal production file set

Authorized implementation changes:
- `crates/doctor-core/Cargo.toml`
- `crates/doctor-core/src/lib.rs`
- `crates/doctor-core/src/report.rs`

Documentation for this pre-flight is recorded separately.

No changes are authorized to:
- `crates/doctor-cli/src/main.rs`;
- GitHub Actions workflows;
- the report schema v1 definition;
- audit rules;
- crypto/signing implementation;
- project parser;
- Play policy rules.

## 7. Regression risks

Primary risks:
- accidental omission of evidence during mapping;
- incorrect null/empty semantics;
- conversion of explicit false values into absent values;
- accidental serialization of internal domain structs;
- accidental modification of existing text output;
- unstable JSON ordering through unordered collections.

Mitigation:
- owned DTOs with explicit field mapping;
- focused semantic tests;
- preservation of existing ordered vectors;
- second audit before Actions.

## 8. Scope freeze

This pre-flight closes the audit phase.

Next authorized movement:

**bounded production Report v1 mapping + JSON serialization + focused tests.**

The next audit sequence remains:

**implement → focused tests → second audit → Build → Test → Format → Clippy → correction only if required → final implementation checkpoint**

No merge is part of this movement.
