# M0.15 — Consumer Contract Validation Ledger — 2026-10-03

## Scope

This ledger records the assurance-only evidence and harness corrections for M0.15.

## Entries

### M015-001 — Pre-audit opened
- Base main: `d1d681e46391c95fd77152779ff4e3656995837f`.
- Objective: validate the reusable Action from a separate consumer repository through immutable SHA.
- Production changes authorized: none.

### M015-002 — Consumer harness initial failure
- Run: `37140931651`.
- Cause: schema download was placed after the success-report validation step.
- Classification: test-harness ordering error.
- Product defect: no.
- Correction: schema download moved before report validation.

### M015-003 — Consumer harness expression failure
- Run: `37140968978`.
- Cause: malformed GitHub expression interpolation produced an invalid temporary path.
- Classification: test-harness syntax error.
- Product defect: no.
- Correction: GitHub expression syntax repaired.

### M015-004 — Consumer harness expression brace failure
- Run: `37140991766`.
- Cause: an extra closing brace remained in generated GitHub expressions.
- Classification: test-harness syntax error.
- Product defect: no.
- Correction: extra brace removed.

### M015-005 — Final consumer validation
- Run: `37141021253`.
- Result: SUCCESS.
- Immutable Action reference: `d1d681e46391c95fd77152779ff4e3656995837f`.
- Success path: exit 0, schema PASS.
- Blocker path: exit 1, schema PASS.
- Invalid format path: exit 2, no report file.
- Product defect: none observed.

### M015-006 — Public product diff audit
- Result: documentation-only changes on M0.15 branch.
- Production logic: unchanged.
- Permanent Action workflow: unchanged.
- Report schema: unchanged.

### M015-007 — Final integration and post-merge validation
- PR: #65.
- Final main HEAD: `937fe5059c4535a407c962893646a0d0b64de036`.
- Post-merge Action Validation Run `37141293283`: SUCCESS.
- Post-merge Rust CI Run `37141293281`: SUCCESS.
- M0.15 status: CLOSED.

## Closing rule

M0.15 may close only after:
1. final consumer run is SUCCESS;
2. second audit confirms contract evidence;
3. public current-state documentation is reconciled;
4. final main HEAD is recorded by a closure checkpoint.
