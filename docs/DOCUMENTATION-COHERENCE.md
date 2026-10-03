# Documentation Coherence Principle

## Purpose

Android Release Doctor treats documentation as part of the engineering system, not as decoration.

A capability is not considered complete when code merely exists. It is complete only when the implementation, tests, CI evidence, public documentation, schemas/contracts, and active checkpoint describe the same state.

## Mandatory rule

For every validated product change:

**implement → validate → update evidence → update documentation/contracts → second audit → close the block**

A block must not be closed while the public documentation materially contradicts the validated implementation.

## Historical records

Historical pre-audits, implementation audits, second audits, checkpoints, and ledger entries preserve their original chronology and observations.

They are not silently rewritten to make history look cleaner.

When later work changes the practical status of an old finding, use one of these mechanisms:

- an append-only reconciliation in the ledger;
- a clearly labeled post-integration/current-state section;
- a newer current-state document that supersedes the old snapshot.

The purpose is to preserve both facts:

1. what was observed at the time;
2. what is true now.

## Current-state source of truth

The repository must maintain a clear current-state document when milestone history alone is no longer sufficient to describe the present product.

The current-state document must identify the current baseline and explicitly distinguish:

- demonstrated capabilities;
- partial/manual capabilities;
- unsupported boundaries;
- open hardening gaps;
- historical material.

## Contract and example consistency

When a public contract changes, update together as applicable:

- README usage examples;
- JSON schemas or other machine-readable contracts;
- Action metadata/examples;
- CLI exit semantics;
- tests and fixtures;
- milestone/checkpoint documentation.

Examples must reference the current public repository and current interfaces.

## Release/history hygiene

Removed product surfaces remain recoverable through Git history, but must not continue to be presented as current product capabilities.

Historical references are acceptable when clearly identified as historical.

## Closure criterion

The documentation state itself is part of the closure audit:

**no unresolved documentation contradiction → no closure.**
