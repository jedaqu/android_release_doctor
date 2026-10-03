# M0.15 — Checkpoint / Closure — 2026-10-03

## State

- Repository: `jedaqu/android_release_doctor`
- Visibility: public
- Default branch: `main`
- Milestone/block: M0.15 — Consumer Contract Validation
- Status: CLOSED
- Final main HEAD: `937fe5059c4535a407c962893646a0d0b64de036`
- Integration PR: #65
- Integration merge commit: `937fe5059c4535a407c962893646a0d0b64de036`
- Production-code changes: none

## Consumer evidence

The reusable GitHub Action was consumed from a genuinely separate consumer repository at immutable product revision:

`jedaqu/android_release_doctor/.github/actions/android-release-doctor@d1d681e46391c95fd77152779ff4e3656995837f`

Final external consumer run:
- Run `37141021253` — SUCCESS.
- Real Meshtastic v2.8.1 APK.
- SHA-256: `7f42735fd1c17c7e6a64d3a48ae1e4baf22cab778e994eb8331d3b993e42eb00`.
- Success path: `exit-code=0`, Report v1 schema PASS.
- Blocking Play path: `exit-code=1`, Report v1 schema PASS.
- Invalid format path: `exit-code=2`, no report file.
- Consumer-visible `report-path` and `exit-code` outputs were verified.

## Product-suite evidence

Post-merge public validation:
- Action Validation Run `37141293283` — SUCCESS.
- Rust CI Run `37141293281` — SUCCESS.
- Rust/CLI suite remains 147 automated tests from the established M0.13/M0.14 evidence base.
- Report v1 schema validation remains exercised.
- No Rust source, CLI behavior, Action implementation, schema, fixture binary, or permanent product workflow changed.

## Harness corrections

Three earlier consumer workflow failures were classified as test-harness corrections:
- `37140931651`
- `37140968978`
- `37140991766`

None exposed a reproducible product defect.

## Closure rule

M0.15 is closed because:
1. separate-consumer immutable-SHA validation succeeded;
2. success, blocker, and invalid-input contracts were exercised;
3. Report v1 schema validation passed for generated success/blocker reports;
4. second audit reconciled the evidence;
5. public current-state documentation and this final checkpoint now record the final main HEAD and post-merge CI evidence.

The temporary external consumer workflow is private and outside the public product repository. Its identity and private project material are intentionally not recorded in this public repository.
