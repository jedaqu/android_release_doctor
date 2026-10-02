# AUDIT-004 — Trigger Uniformity Delimitation

Date: 2026-10-01  
Repository: `jedaqu/android_release_doctor`  
Baseline: `cb3e99dd13ccf487ee9b2b8b6102bef1fe84bdad`

## Purpose

This document freezes the scope of AUDIT-004 after the pre-audit review of the GitHub Actions trigger configuration.

The finding concerns whether workflow triggers are sufficiently uniform and coherent for their respective contracts. The audit does not require textual identity between workflows.

## Pre-audit finding

The current workflow set uses different triggers:

| Workflow | Push | Pull request | Manual | Tags |
|---|---|---|---|---|
| Rust CI | Development/main branches | Configured target branches | No | No |
| M0.8 Block 3 Action | `m08-block3-github-action` | `m08-block2-cli-output-contract` | Yes | No |
| M0.8 Block 4 Distribution | `m08-block4-distribution-release` | `m08-block3-github-action` | Yes | `v*` |

The differences correspond to the project's stacked branch/workflow architecture:

- Block 2 feeds Block 3.
- Block 3 feeds Block 4.
- Block 4 uses a release-tag trigger for publication.
- Manual execution of Block 3 and Block 4 is available for validation.
- Block 4 publication is additionally guarded so that manual dispatch does not publish a release.

Historical execution evidence reviewed during the pre-audit is consistent with this intended behavior.

## Semantic uniformity

For this audit, trigger uniformity means:

> Each workflow runs on the events required by its documented function and does not permit events that contradict its contract.

It does **not** mean that every workflow must contain identical `on:` declarations.

A workflow may legitimately have a different trigger set when its role, branch contract, validation purpose, or publication boundary differs.

## Publication boundary

The Block 4 workflow distinguishes validation from publication.

Manual `workflow_dispatch` is a validation path. Public release publication requires a push of a version tag matching `v*`, together with the workflow's publication guard.

This distinction is part of the release contract and is not a trigger defect.

## Delimited status for v0.1.0

AUDIT-004 is classified as:

**VALID CONFIGURATION — NO TRIGGER DEFECT CONFIRMED.**

The current trigger matrix is accepted for the v0.1.0 boundary because the pre-audit found functional coherence between:

- workflow purpose;
- branch-to-workflow chain;
- pull-request validation;
- manual validation;
- tag-driven release publication.

No code or workflow correction is required solely by AUDIT-004.

## Explicitly out of scope

The following changes are **not** authorized as part of AUDIT-004:

- making all workflow triggers textually identical;
- adding arbitrary push, pull-request, or manual triggers;
- changing the stacked branch architecture;
- changing branch names solely to achieve trigger symmetry;
- changing the Block 4 publication guard;
- changing release publication semantics;
- redesigning the CI workflow topology;
- changing the CLI, audit engine, Report v1, GitHub Action, or distribution implementation;
- changing the v0.1.0 version boundary.

Any future CI topology or release-trigger redesign requires its own explicit audit and scope.

## Relationship to AUDIT-005

AUDIT-004 validates the coherence of the configured trigger model.

It does **not** prove that a real tag-driven GitHub Release has already been published successfully.

That evidence belongs to **AUDIT-005 — real release publication evidence**.

The findings remain separate:

- **AUDIT-004:** whether configured triggers are coherent with workflow contracts;
- **AUDIT-005:** whether the real publication path has produced the intended public release and assets.

## No error ledger entry

No new ERR entry is created for AUDIT-004 because the pre-audit found no implementation defect or failed trigger validation requiring corrective action.

## Future hardening boundary

Future CI or release hardening may evaluate broader trigger simplification, provenance controls, permissions, or release automation. Such work must first define its own requirements, risks, scope, and validation criteria.

Until separately authorized, the current trigger configuration remains unchanged.

## Delimitation verdict

**AUDIT-004 DELIMITED.**

The trigger differences are understood as functional differences rather than a defect requiring textual uniformity. The scope is bounded, the publication boundary is preserved, and AUDIT-005 remains independent.