# SECOND AUDIT M0.8 Block 1 — Report v1 Production Implementation

Date: 2026-10-01
Branch: `m08-block1-report-contract`
Frozen contract baseline: `aa055402af1989f86100092b08cbb3e38eaa535e`
Latest implementation commit before this audit documentation: `2a0b42b911c185ca31eb265d1fc3a13b2c5ab165`
PR: #17 — open, unmerged, mergeable

## Result

**PASS — the bounded Report v1 production implementation matches the frozen contract and remains within the authorized scope.**

## 1. Diff scope

The implementation delta contains only:
- `crates/doctor-core/Cargo.toml`
- `crates/doctor-core/src/lib.rs`
- `crates/doctor-core/src/report.rs`
- `docs/AUDIT-M0.8-BLOCK1-REPORT-IMPLEMENTATION-PREFLIGHT.md`
- `docs/ERRORS-AND-FIXES.md`

No CLI source, workflow, audit-rule, project-parser, signing, cryptographic implementation, or frozen schema file was changed.

## 2. DTO boundary

The production path is:

`AuditReport -> ReportV1 DTO -> serde_json serializer`

Internal domain structs do not derive Serialize. The report module owns an explicit DTO graph and explicit conversion functions.

This preserves the frozen architecture requirement and prevents the JSON contract from becoming coupled to internal domain layout.

## 3. Contract coverage

The mapper covers every frozen v1 top-level field:
- `schema_version`
- `engine_version`
- `artifact`
- `application`
- `manifest_error`
- `inventory`
- `project`
- `project_error`
- `play`
- `findings`
- `summary`

The DTO graph explicitly covers:
- manifest application identity and component evidence;
- archive entries and manifest path;
- DEX inventory;
- native ABI/library/ELF evidence;
- native ZIP evidence;
- signature files;
- APK signing-block evidence and errors;
- v2/v3/v3.1 cryptographic evidence;
- v3.1/v3.2 presence boundaries;
- proof-of-rotation;
- known and unknown capability bits;
- project configuration evidence;
- findings and severity vocabulary;
- aggregate counts.

## 4. Semantic preservation

The implementation preserves:
- `null` for optional evidence that was not produced;
- empty arrays for inspected collections with no members;
- explicit boolean `false`;
- inspection/parsing errors in their dedicated error fields.

The focused tests directly exercise these distinctions.

## 5. Play context

Play context is supplied separately through `ReportV1Context`.

When absent, `play` is serialized as `null`.
When present, the DTO emits the selected platform and the existing `PLAY_POLICY_VERSION`.

No Play invocation context was added to `AuditReport`, preserving the frozen domain boundary.

## 6. Determinism

The report mapper copies the repository's existing ordered vectors rather than introducing unordered collections.

The focused deterministic serialization test serializes the same report and context twice and requires byte-for-byte identical JSON output.

## 7. Existing behavior

`AuditReport::render_text()` was not modified.

No CLI flag, exit-code, output-mode, GitHub Action, distribution, cryptographic implementation, or policy-rule behavior was introduced in this movement.

## 8. Focused tests

The new report-module tests cover:
1. all major contract sections;
2. null/empty/false/error semantics;
3. crypto state and v3.1/v3.2 presence boundaries;
4. proof-of-rotation evidence and unknown capability bits;
5. Play context presence/absence;
6. deterministic JSON;
7. valid pretty JSON parsing.

The existing domain tests in `lib.rs` remain unchanged.

## 9. Corrections during implementation

### ERR-077
Observed in Actions #444 / run `36883291509`:
- Build failed because proof-of-rotation domain types were imported from the wrong module path.
- Corrected by importing them from `signature_verify` and scoping `ArtifactKind` to tests.

### ERR-078
Observed in Actions #447 / run `36883538774`:
- Build and Test passed, but Format failed on five rustfmt differences.
- Corrected with formatter-indicated changes only.

Both errors are now marked RESOLVED in `docs/ERRORS-AND-FIXES.md`.

## 10. CI gate

Actions #449 / run `36883817080` on corrected commit `2a0b42b911c185ca31eb265d1fc3a13b2c5ab165`:

- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

All four gates are terminal and successful.

## 11. Scope verdict

The implementation is bounded to Report v1 DTO mapping and JSON serialization plus the dependencies required to serialize that DTO.

No unrelated refactor or new product capability was introduced.

**Second audit status: PASS.**

The next required action is checkpoint documentation and final CI validation for that checkpoint commit. PR merge remains out of scope.
