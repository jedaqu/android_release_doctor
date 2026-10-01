# CHECKPOINT — M0.8 Manual Workflow Dispatch

Date: 2026-10-01
Base: main
Implementation branch: ci/m08-manual-workflow-dispatch
Pre-checkpoint implementation/audit head: f59904a8664e9d7b53b7f081c38a703595b34f48

## Scope

This bounded infrastructure movement removes the need to create temporary branches solely to invoke the M0.8 Block 3 and Block 4 validation workflows.

Implemented:

- workflow_dispatch on M0.8 Block 3;
- workflow_dispatch on M0.8 Block 4;
- public operational documentation for manual validation;
- second-audit documentation;
- explicit preservation of the tag-only Release publication boundary.

The existing push, pull-request, packaging, checksum, and publication logic is unchanged.

## Operating contract

**Manual workflow run = validation**

**Version tag push = publication**

Block 4 publication remains guarded by:

github.event_name == 'push'
AND
refs/tags/v*

A manual run therefore cannot publish a GitHub Release.

## Validation before checkpoint documentation

Rust CI run 36928231056 on implementation/audit head f59904a8664e9d7b53b7f081c38a703595b34f48 completed successfully:

- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

No new ERR-NNN entry was required.

## Second audit

Second audit result: **PASS**.

The audit confirmed:

- exact bounded diff;
- preservation of existing M0.8 triggers;
- no change to audit, CLI, Action, packaging, or release logic;
- manual execution remains validation-only;
- public/private separation remains clean.

The audit document is:

docs/SECOND-AUDIT-M0.8-MANUAL-WORKFLOW-DISPATCH.md

## Public/private boundary

PASS.

Only public workflow configuration and public operational documentation were added. No private prompts, conversations, credentials, commercial strategy, private roadmap, or internal-only material was introduced.

## Closure rule

This checkpoint documentation commit is documentation-only relative to the already audited implementation. The checkpoint closes only after terminal Rust CI on this exact commit reports:

- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

No merge is part of this checkpoint.

## Future evolution

Both M0.8 workflow surfaces remain incremental and may receive later improvements. Any future change must preserve the explicit distinction between validation execution and tag-driven publication unless a separately audited release-process change intentionally revises that contract.
