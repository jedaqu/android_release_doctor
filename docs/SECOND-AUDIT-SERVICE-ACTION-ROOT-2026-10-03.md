# Second Audit — Repository-Root Android Release Doctor Action — 2026-10-03

## Baseline

- Branch: `service/action-root-metadata-2026-10-03`
- Base main: `1b1446b80cfd66d9333c3d51c9b5ec0bc2a8d206`
- Final validated implementation head for this audit: `7365c47b111f4f4ff62c044c42d57796b6596136`

## Scope

Verify the authorized service Action packaging correction after implementation.

### In scope

- repository-root `action.yml`;
- preserved Action inputs and outputs;
- preserved composite execution semantics;
- repository-root Cargo manifest resolution;
- Marketplace metadata placement and branding;
- removal of the obsolete nested Action metadata file;
- permanent Action Validation consumption path;
- immutable consumer-style Action invocation;
- public README/current-state service references.

### Out of scope

- Rust audit logic;
- Report v1 schema;
- fixtures;
- Play semantics;
- cryptographic logic;
- hosted service architecture;
- commercial pricing.

## Findings

### Root metadata

PASS.

The canonical Action metadata is now `action.yml` at repository root. The previous nested metadata file is removed from the current tree.

GitHub Marketplace documentation requires the action repository to contain a single root `action.yml` or `action.yaml` for automatic Marketplace listing.

### Action contract

PASS.

The following inputs remain unchanged:

- `artifact`
- `project`
- `play`
- `play-platform`
- `format`
- `output`

The following outputs remain unchanged:

- `exit-code`
- `report-path`

Exit semantics remain 0/1/2.

### Execution model

PASS.

The Action continues to invoke:

`cargo run --locked --quiet --manifest-path "$GITHUB_ACTION_PATH/Cargo.toml" -p doctor-cli -- ...`

The only execution-path change is the manifest location moving from the nested Action directory to the repository-root Action directory.

### Marketplace metadata

PASS.

The root metadata includes:

- unique Action name: `Android Release Doctor`;
- author: `JEDAQU`;
- description;
- branding icon;
- branding color.

No Marketplace publication was performed.

### Validation

PASS.

- Action Validation Run `37149982327`: SUCCESS.
- Rust CI Run `37149982393`: SUCCESS.

The Action Validation workflow exercised the repository-root Action for success, blocker, AAB, project, project+Play, alternative Play platform, invalid inputs, invalid output format/path, Report v1 schema validation, immutable SHA consumption, deterministic corpus, and operational error.

### Corrective validation event

ERR-122 was raised for an initial workflow-ordering error in the new metadata existence check. The check was placed before checkout and therefore failed before Action execution.

The correction moved the check immediately after checkout. The corrected validation passed.

ERR-122 is classified as test/workflow ordering, not a product defect.

## Service productization disposition

SVC-002 — Action metadata placement is RESOLVED for the current repository structure.

The repository is now directly consumable as an Action:

`jedaqu/android_release_doctor@<ref>`

The remaining service-delivery concern SVC-001 remains open by design: the composite Action still requires a usable Rust/Cargo toolchain on the runner because it executes the validated CLI through Cargo.

That concern is a separate architecture block and is not mixed into this metadata correction.

## Conclusion

SECOND AUDIT: PASS.

The repository-root Action packaging correction is bounded, validated, and preserves the established service contract.

No audit-engine behavior was changed.
