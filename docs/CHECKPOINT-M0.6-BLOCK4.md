# Checkpoint — M0.6 Block 4

## State

M0.6 Block 4 is complete on branch m06-block4-signer-error-isolation and has passed the complete Build/Test/Format/Clippy validation gate.

No pull request has been opened for Block 4 yet, and no merge is performed as part of this checkpoint.

## Starting point

Block 4 started from the validated M0.6 Block 3 checkpoint:
238c0403378c09f1175dd89f0cfd5edbfcf53eaa

Base branch:
m06-block3-verification-hardening

## Audit findings

- AUDIT-014 / ERR-016 — signer-local v2/v3 verification failures could abort multi-signer evidence.
- AUDIT-015 / ERR-017 — v3 signer failures could lose already-parsed SDK-range evidence.
- AUDIT-016 / ERR-018 — Block 4 stacked CI trigger coverage required correction.
- AUDIT-017 / ERR-019 — repository status was stale after Block 3.
- ERR-020 — initial Block 4 CI trigger edit duplicated the push entry and omitted PR coverage.
- ERR-021 — rustfmt required a deterministic layout correction in the new regression test.
- ERR-022 — PR trigger audit initially targeted the source branch instead of the stacked base branch.

## Implemented scope

- v2 and v3 signer-local verification failures are isolated into signer-level Invalid/Unsupported results instead of aborting the whole multi-signer pass;
- v3 signer evidence retains the known minSDK/maxSDK range, signer count, algorithms, and certificate fingerprint when those values were available before failure;
- regression tests cover signer-error evidence retention and mixed verified/failed v3 signer aggregation;
- CI push coverage includes the Block 4 source branch;
- CI pull-request target coverage includes the active Block 3 stacked base;
- README status reflects M0.6 Block 4 in progress;
- no new cryptographic algorithms, AAB crypto verification, proof-of-rotation completion, v3.1/v3.2 completion, Play-policy expansion, or unrelated refactoring was introduced.

## Correction sequence

- signer isolation implementation: 135e3df81507ad48d6b03608f282da7154c028af
- regression tests: d6ae40578e2edbc54d2566a9bb774a99fc181cc1
- initial CI edit: 927ed9b809bb491c688869ea1024ce82c9f55e78
- README status: 0b43ac43a74b1bee8df6c9d86dd68036bac9a1ec
- second-audit documentation: 9d91f90be156c289474b1d53b6628265da330b23
- corrected CI trigger edit: d7a68a2d77e1779a3be715982c79d1afbe8888f4
- rustfmt correction: af70dcb1175c092de319eee9c601ff1830b42a1e
- CI follow-up audit: 94cc6a8a293478e78216350fb2114d0dd177c762
- final stacked PR-target correction: f71c16ae43cd9f57748ad494c10ada9f4a7f6dd4
- ledger resolution: 06b9ce39a5e41bf1fb18433ddfd80f63ccefd0e5

## Final validation

Actions run #200 / 36746725415 completed successfully:

- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

The run validated the final implementation and current workflow configuration before this checkpoint document was added.

## Acceptance

M0.6 Block 4 is checkpoint-ready. The next milestone work must begin from the validated Block 4 checkpoint and must first review docs/ERRORS-AND-FIXES.md.