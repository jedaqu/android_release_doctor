# AUDIT-005 — Real Release Publication Evidence Delimitation

Date: 2026-10-01
Repository: `jedaqu/android_release_doctor`
Baseline: `bc063a8777167fcd71c50271104f472dd86c1078`
Version boundary: `v0.1.0`

## Purpose

This document delimits AUDIT-005 after the pre-audit review of the real release-publication path.

The audit question is not whether the distribution workflow is configured to publish a release. The question is whether there is objective evidence that the configured publication path has actually produced the intended public release and its release assets.

## Pre-audit finding

The current distribution workflow contains a publication path that:

1. validates the workspace version;
2. validates a version tag when running from a tag;
3. builds Linux x86_64, Windows x86_64, and macOS x86_64 packages;
4. verifies package contents;
5. uploads package artifacts;
6. generates and verifies `SHA256SUMS`;
7. invokes `gh release create` only for a tag push.

The publication job is guarded by:

`github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v')`

Historical distribution run `36926742627` successfully completed validation and all three package jobs, but its `publish` job was skipped. Therefore that run demonstrates distribution validation, not a real public release publication.

Current pre-audit evidence also shows no `v0.1.0` tag reference and no GitHub Release for the repository.

## What counts as publication evidence

For AUDIT-005, a real publication should be evidenced by all of the following:

- a real version tag exists in the repository;
- the tag identifies the intended release version;
- a GitHub Actions distribution run was triggered by the tag push;
- the distribution run reaches the `publish` job;
- the `publish` job completes successfully;
- the resulting GitHub Release exists publicly;
- the release contains the expected platform packages;
- the release contains `SHA256SUMS`;
- the release assets correspond to the version validated by the workflow.

Evidence may be collected from GitHub's public repository state and the corresponding workflow run.

## Important distinction

The following are **not**, by themselves, proof of real publication:

- the existence of a workflow containing `gh release create`;
- a successful branch-based distribution validation;
- successful package artifacts from a non-tag run;
- a skipped `publish` job;
- documentation claiming that a release can be published;
- a local build;
- an unreleased tag candidate;
- a workflow configuration that has never been exercised through the tag publication path.

## Delimited status for v0.1.0

AUDIT-005 is currently classified as:

**PUBLICATION EVIDENCE NOT YET DEMONSTRATED.**

This is an evidence-status finding, not yet a confirmed implementation defect.

No ERR entry is created at the delimitation stage.

## Explicitly out of scope

AUDIT-005 does not authorize:

- modifying the distribution workflow;
- changing publication guards;
- changing package formats;
- changing the supported platform matrix;
- changing checksum algorithms;
- adding signing or provenance systems;
- changing GitHub permissions;
- redesigning release automation;
- changing the CLI or audit engine;
- changing the v0.1.0 product contract.

If the real publication exercise reveals an implementation failure, that failure must be classified separately and corrected through the normal ERR → focused fix → second audit → CI process.

## Publication exercise boundary

A controlled real publication exercise is the appropriate next validation step if the project is ready to publish `v0.1.0`.

The exercise must use the existing release contract without changing it merely to obtain a successful result.

If publication succeeds, the evidence becomes the basis for the second audit and formal closure.

If publication fails, the exact failing workflow, job, step, event, commit/tag, and error must be recorded before any correction is attempted.

## Relationship to earlier audits

- AUDIT-003 established the checksum limitation and deliberately separated integrity verification from publisher/provenance authenticity.
- AUDIT-004 established that the workflow trigger differences are semantically coherent.
- AUDIT-005 now tests the separate proposition that the configured publication path has actually produced a public release.

## Delimitation verdict

**AUDIT-005 DELIMITED.**

The scope is restricted to demonstrating real release publication for the defined release boundary. Configuration correctness, checksum limitations, and future release hardening remain separate concerns.