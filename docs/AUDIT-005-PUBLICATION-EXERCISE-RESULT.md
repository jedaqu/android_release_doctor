# AUDIT-005 — Controlled Publication Exercise Results

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Baseline exercised: `c3a4095838e3030ebe945d91bf417dc5aa84cf8c`
Version boundary: `v0.1.0`

## Purpose

This document records the controlled publication exercise performed after AUDIT-005 was delimited.

The objective was to distinguish a non-reproducible validation anomaly from a reproducible publication-path defect before changing the release workflow.

## Publication attempt 1

Workflow run: `36945562430`
Attempt: `1`
Trigger: tag push for `v0.1.0`

The validation and three platform package jobs completed successfully.

The `publish` job failed in `Generate and verify checksums`:

```text
SHA256SUMS: FAILED
sha256sum: WARNING: 1 computed checksum did NOT match
Process completed with exit code 1
```

The failure affected the publication exercise, but the available evidence did not establish a deterministic cause from the first run alone.

## Reproducibility rerun

The same workflow run was rerun without changing the commit, tag, package contract, or checksum commands.

Rerun: `36945562430`, attempt `2`

Results:

- validation: PASS
- Linux x86_64 package: PASS
- macOS x86_64 package: PASS
- Windows x86_64 package: PASS
- checksum generation and verification: PASS
- GitHub Release publication: FAIL

The checksum failure from attempt 1 was therefore not reproduced by the controlled rerun.

## Reproducible publication-path failure

The rerun failed in `Publish GitHub Release` with:

```text
failed to run git: fatal: not a git repository
(or any of the parent directories): .git
Process completed with exit code 1
```

The failing command was:

```bash
gh release create "$GITHUB_REF_NAME" \
  dist/... \
  --verify-tag \
  --generate-notes
```

The `publish` job did not perform a repository checkout and did not explicitly provide a repository to GitHub CLI.

GitHub CLI documents `--repo OWNER/REPO` as the repository selector for `gh release create`, and `--verify-tag` verifies that the tag already exists remotely.

Reference: https://cli.github.com/manual/gh_release_create

## Root cause

The publication step relied on implicit local Git repository context that is not present in the artifact-only `publish` job.

This is a workflow implementation defect.

The earlier checksum failure remains classified separately as non-reproducible from the controlled rerun and is not attributed to this root cause.

## Minimal correction boundary

The selected correction is to bind release creation explicitly to the repository already provided by the workflow environment:

```bash
gh release create "$GITHUB_REF_NAME" --repo "$GITHUB_REPOSITORY" \
  ...
```

No checkout is added.

No publication guard, package format, checksum algorithm, permission, platform matrix, or release-version contract is changed.

## Current publication state

At the time of this document:

- `v0.1.0` exists and points to `c3a4095838e3030ebe945d91bf417dc5aa84cf8c`;
- no GitHub Release exists for `v0.1.0`;
- the correction is present on branch `fix/audit-005-publish-repo-context`;
- final validation and real publication remain pending.

## Status

**AUDIT-005 — OPEN: correction implemented, publication validation pending.**
