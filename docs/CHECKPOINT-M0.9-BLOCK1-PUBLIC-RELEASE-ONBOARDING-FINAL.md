# CHECKPOINT — M0.9 Block 1 Public Release & Onboarding

Date: 2026-10-01
Branch: `m09-block1-public-release-onboarding`
Base capability: M0.8 Block 4
M0.8 base HEAD: `0b8594ffa03294840128169604f890a80a38f295`

## Status

**CLOSED — terminal Rust CI confirms the checkpoint gate.**

## Scope

M0.9 Block 1 prepares the first public release and onboarding surface.

Implemented:

- public-facing README onboarding;
- supported-platform matrix;
- installation and checksum verification guidance;
- first APK/AAB audit examples;
- JSON/CI example;
- GitHub Action example pinned to `v0.1.0`;
- explicit exit-code semantics;
- first public-release CHANGELOG entry;
- documentation of known boundaries.

Validation infrastructure correction:

- Rust CI push coverage now includes the M0.9 implementation branch.

No product engine, CLI, Report v1, Action implementation, or distribution workflow behavior was changed.

## Version boundary

Workspace version:

`0.1.0`

The public release tag remains:

`v0.1.0`

The GitHub Release and tag are intentionally NOT created as part of this checkpoint.

## Corrective chain

- ERR-087 — macOS checksum command portability: RESOLVED.
- ERR-088 — M0.9 branch missing from Rust CI push coverage: RESOLVED.

## Second implementation audit

The second implementation audit was performed against the validated public-release onboarding diff and its requirements. The original internal audit artifact is not part of the public repository and is not treated as public evidence.

Result: **PASS**

## Exact checkpoint CI gate

Closure evidence for the checkpoint commit includes terminal Rust CI Build/Test/Format/Clippy success.

- Build;
- Test;
- Format;
- Clippy.

No merge is part of this checkpoint.

Terminal CI evidence for the current public state is provided by the CI run associated with the final corrected commit. Historical private-run identifiers are intentionally omitted from the public checkpoint.

## Post-checkpoint publication boundary

After this checkpoint is closed, the first public release remains a separate release operation:

1. decide/promote the validated release state;
2. create tag `v0.1.0`;
3. allow the M0.8 Block 4 distribution workflow to validate and publish;
4. verify package assets and `SHA256SUMS`;
5. record the GitHub Release URL and final publication evidence.

The publication step must not be confused with the M0.9 Block 1 implementation checkpoint.

## Working rule

Before any future M0.9 production block:

**ledger review -> pre-audit -> frozen scope -> implementation -> focused validation -> second audit -> CI -> checkpoint**


## Public reconstruction note

This checkpoint was reconstructed from the private development repository into the public repository. Only public-safe product, validation, and user-facing documentation were transferred. Private conversations, internal prompts, commercial strategy, private roadmap material, and other non-public development artifacts remain outside this repository.

The public repository is self-contained for its user-facing release surface; the private repository remains the historical development record.
