# SECOND AUDIT — M0.8 Manual Workflow Dispatch

Date: 2026-10-01
Base: `main` at `7b076e4518f05171d6c1751e99f6738a53fa4024`
Implementation branch: `ci/m08-manual-workflow-dispatch`
Implementation head under audit: `3b49d5a07735c5215fe779d594e3b9a789ff6884`

## Result

**PASS — the bounded CI ergonomics change preserves the existing M0.8 behavior and adds manual validation without opening a publication path.**

## Exact diff

The branch is 3 commits ahead of `main` and changes exactly three files:

- `.github/workflows/m08-block3-action.yml`
  - adds `workflow_dispatch`;
  - preserves the existing push trigger for `m08-block3-github-action`;
  - preserves the existing pull-request target `m08-block2-cli-output-contract`.
- `.github/workflows/m08-block4-distribution.yml`
  - adds `workflow_dispatch`;
  - preserves the existing branch and tag push triggers;
  - preserves the existing pull-request target `m08-block3-github-action`;
  - preserves the publication guard.
- `docs/M0.8-MANUAL-WORKFLOW-EXECUTION.md`
  - documents the manual validation path and the tag-driven publication boundary.

No audit engine, CLI implementation, Action metadata, package contents, dependency graph, version, or release publication logic was changed.

## Manual execution semantics

### Block 3

A manual run executes the existing self-test unchanged:

- success path;
- blocker path;
- operational-error path;
- Report v1 JSON validation;
- exit-code validation.

### Block 4

A manual run executes the existing distribution validation unchanged:

- workspace version read;
- locked test;
- format;
- Clippy;
- Linux x86_64 package;
- Windows x86_64 package;
- macOS x86_64 package;
- packaged binary `--version` and `--help`;
- archive-content verification;
- package artifact upload.

## Publication safety

The publish job remains:

```yaml
if: github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v')
```

Therefore `workflow_dispatch` cannot reach the Release publication job.

For tag publication, the pre-existing version equality check between `GITHUB_REF_NAME` and the workspace `Cargo.toml` version remains unchanged.

## Required CI gate

Rust CI run `36927997846` on implementation head `3b49d5a07735c5215fe779d594e3b9a789ff6884` completed successfully:

- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

The CI gate is terminal; no unresolved job remained.

## Error ledger

No new ERR-NNN entry was required. The implementation introduced no observed failure.

## Public/private separation

PASS.

The change contains only public workflow configuration and public operational documentation. No private prompt, conversation, credential, commercial strategy, private roadmap, or internal-only artifact was introduced.

## Scope conclusion

The intended operating rule is now explicit:

**manual run = validation**

**version tag = publication**

The existing M0.8 delivery contracts remain intact and can receive future incremental improvements without changing the publication safety boundary.
