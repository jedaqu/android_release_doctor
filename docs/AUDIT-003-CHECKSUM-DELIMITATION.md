# AUDIT-003 — Checksum Limitation Delimitation

Date: 2026-10-01  
Repository: `jedaqu/android_release_doctor`  
Baseline: `e227df59af2d6f8bc2fba4a2cedf91a2fd23faee`

## Purpose

This document freezes the scope of AUDIT-003 after the pre-audit review of the current distribution and release-packaging implementation.

The finding concerns the security meaning of the published `SHA256SUMS` manifest. It is a limitation of the integrity mechanism, not a demonstrated defect in the current checksum implementation.

## Pre-audit finding

The current distribution workflow:

1. builds the Linux x86_64, Windows x86_64, and macOS x86_64 packages;
2. generates `SHA256SUMS` from the final package files;
3. verifies that manifest with `sha256sum -c SHA256SUMS`;
4. publishes the manifest together with the release assets when a valid release tag triggers the publication job.

This provides hash-based integrity checking of the released files.

A checksum manifest does not, by itself, establish publisher authenticity or artifact provenance. If an attacker could replace both an artifact and its checksum manifest, the checksum could still validate the substituted artifact.

That limitation is a property of the chosen verification model. No failure of SHA-256 generation, manifest coverage, or manifest self-verification was found in this audit.

## Delimited status for v0.1.0

AUDIT-003 is classified as:

**VALID LIMITATION — NOT A PRODUCT BUG.**

For the v0.1.0 release boundary, the following are accepted as the defined checksum contract:

- every published distribution archive has a SHA-256 entry;
- the manifest is generated from the final release archives;
- the release workflow verifies the generated manifest before publication;
- users can independently recompute the SHA-256 digest of a downloaded archive and compare it with the published manifest;
- checksum verification is presented as an integrity check, not as an independent proof of publisher identity or build provenance.

No code or workflow correction is required solely by this finding.

## Explicitly out of scope

The following changes are **not** authorized as part of AUDIT-003:

- replacing SHA-256 with another hash algorithm;
- changing archive generation;
- changing the package matrix;
- adding binary signing;
- adding release signing;
- adding Sigstore, GitHub Artifact Attestations, SLSA provenance, or another provenance system;
- changing GitHub Actions permissions;
- changing release publication triggers;
- changing the CLI, audit engine, Report v1, or GitHub Action;
- changing the v0.1.0 version boundary.

Those mechanisms may be evaluated later as a separate hardening effort with its own audit and scope.

## Relationship to AUDIT-005

AUDIT-003 does not establish that a real GitHub Release was published successfully.

Evidence that the tag-driven publication path has actually produced a public release, its assets, and its `SHA256SUMS` belongs to **AUDIT-005 — real release publication evidence**.

The two findings must remain separate:

- **AUDIT-003:** what SHA-256 proves and does not prove;
- **AUDIT-005:** whether the publication mechanism has actually been exercised and produced the intended public release.

## No error ledger entry

No new ERR entry is created for AUDIT-003 because the pre-audit found no implementation defect or failed validation event requiring corrective action.

## Future hardening boundary

A future hardening audit may evaluate stronger artifact authenticity and provenance mechanisms. Such work must first define:

- the trust model;
- the signing/provenance mechanism;
- verification UX;
- key or identity lifecycle;
- CI permission requirements;
- migration and backward-compatibility expectations;
- release documentation changes.

Until separately authorized, the current SHA-256 integrity model remains unchanged.

## Delimitation verdict

**AUDIT-003 DELIMITED.**

The limitation is understood, bounded, documented, and separated from AUDIT-005. No implementation correction is warranted by AUDIT-003 at this stage.
