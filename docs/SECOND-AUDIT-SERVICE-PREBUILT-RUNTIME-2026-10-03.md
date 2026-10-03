# Second Audit — Service Prebuilt CLI Runtime — 2026-10-03

## Scope

This second audit verifies the implemented prebuilt-runtime architecture after CI correction and before any integration into `main`.

It is intentionally separate from the first architecture audit and does not rewrite historical milestone documents.

## Baseline

- Repository: `jedaqu/android_release_doctor`
- Main baseline: `21e0d16c63ff67c70ed0155d5f2d869a0b3b10c3`
- Branch: `service/prebuilt-cli-runtime-2026-10-03`
- Branch head: `905ff0985bb58a6e26a6f6ea3f9f0459e0e5cfe9`
- PR: #76, draft
- Action Validation: Run `37152271333` — SUCCESS
- Rust CI: Run `37152271324` — SUCCESS

## Verification

### Action packaging

- Repository-root `action.yml` is present.
- The Action uses Node 24 and `dist/index.js`.
- JavaScript Action outputs are declared as descriptions only.
- `src/action.js` and `dist/index.js` are synchronized.
- The runtime Action package contains no Cargo invocation.

### Runtime resolution

- `engine-version` is optional for consumers and defaults from the committed runtime manifest.
- The production path requires a manifest-backed semver engine version.
- The test-only runtime override is isolated from production version resolution.
- Runner platform and architecture are mapped explicitly.
- Unsupported combinations fail instead of falling back silently.

### Integrity

- The runtime manifest is committed to the Action repository.
- Published runtime entries require SHA-256 values.
- Cached binaries are accepted only after SHA-256 comparison.
- Downloaded binaries are verified before execution.
- The runtime executable is invoked directly through `spawn`, without shell interpolation.

### Validation

The latest passing Action Validation run exercised:

- Action metadata;
- Node syntax and packaged-source equality;
- runtime manifest structure;
- runtime manifest JSON Schema;
- prohibition of Cargo inside the Action runtime;
- local prebuilt CLI execution;
- success and blocker paths;
- APK and AAB paths;
- project and Play paths;
- non-default Play platform;
- invalid-input paths;
- Report v1 schema validation;
- immutable-SHA consumer invocation;
- deterministic APK/AAB corpus;
- operational error path.

The latest Rust CI run also passed.

## Historical integrity

The earlier Cargo-based Action implementation remains documented as historical context.

No historical milestone document was rewritten to make the new runtime architecture appear to have existed earlier.

## Product boundary

Android Release Doctor remains a GitHub Actions service.

The prebuilt engine and its technical distribution mechanism are implementation details. They do not change the product identity into an installable application or a product release.

## Remaining integration gate

The architecture is validated, but production runtime distribution is not complete.

Still required before main integration:

1. Build the declared runtime assets for the target platforms.
2. Independently verify the SHA-256 values.
3. Populate the production manifest and move it out of `draft`.
4. Validate a real technical engine distribution and its retrieval path.
5. Reconcile `CURRENT-PRODUCT-STATE-2026-10-03.md` against the integrated main state.
6. Complete the final closure checkpoint.

## Audit result

**PASS — architecture and Action migration are validated on the branch.**

**HOLD — production integration remains gated on the real prebuilt runtime distribution and manifest publication.**

