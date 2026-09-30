# M0.6 Block 5 — Second-audit follow-up after individual corrections

Date: 2026-09-30
Working branch: `m06-block5-verification-completeness`
Corrected tree: `dc6d4eb4d241155bb9413e2de38839240901e60b`

## Ledger-first review

The current `docs/ERRORS-AND-FIXES.md` ledger was reviewed before this follow-up audit.

- ERR-001 through ERR-023 remain unchanged and resolved.
- ERR-024 through ERR-026 remain pending awaiting validation.
- ERR-027 is resolved.
- ERR-028 and ERR-029 remain pending awaiting validation.
- No historical error was removed, renumbered, or rewritten.

## AUDIT-027 correction review

**PASS at source-control level.**

The proof-of-rotation fixture is now reachable from the Block 5 branch.

Path:
`tests/fixtures/proof-rotation-valid.bin`

Current blob:
`156c86d85db2467ef3b43f91e1b9572e55b7df33`

Correction commit:
`38dc4991cb0c1249581f16565aa741bb6b8d7bfd`

The branch tree explicitly contains the fixture, so CI can now resolve the test path reproducibly.

The fixture was generated as a version-1 proof-of-rotation lineage with two certificate levels:
- first node: parent algorithm 0, next-signature algorithm 0x0103;
- second node: parent algorithm 0x0103, next-signature algorithm 0;
- second-node signature length: 256 bytes;
- flags: 0 for both nodes.

The structure matches the AOSP v3 lineage layout.

## AUDIT-028 correction review

**PASS at source-inspection level.**

A dedicated `error_to_scheme_info_with_rotation()` helper now carries `ProofOfRotationInfo` across signer-local Invalid/Unsupported conversions.

After `parse_signed_data_v3()` succeeds, later signer failure paths now forward:
- signer count;
- algorithm evidence;
- certificate fingerprint evidence;
- SDK range evidence;
- parsed proof-of-rotation evidence.

Correction commit:
`dc6d4eb4d241155bb9413e2de38839240901e60b`

No corresponding Actions validation has yet been claimed.

## Scope review

**PASS.**

The corrections remain limited to:
- attaching the intended regression fixture to the branch;
- preserving already-established proof-of-rotation evidence.

No new algorithm family, v3.1/v3.2 completion, AAB cryptographic verification, Play policy expansion, broad CLI/report redesign, or unrelated refactoring was introduced.

## Action gate

**OPEN.**

The second-audit failures are corrected at source-control/source-inspection level.

Next validation step:
1. run the Block 5 push Actions gate;
2. validate the Block 4 base workflow after its CI trigger correction;
3. inspect Build/Test/Format/Clippy results;
4. correct only any individual Actions failure;
5. then exercise the stacked PR event before the final Block 5 checkpoint.

No merge is performed.
