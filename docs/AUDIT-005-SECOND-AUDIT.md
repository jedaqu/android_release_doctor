# AUDIT-005 — Second Audit of Publication Correction

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Baseline: `c3a4095838e3030ebe945d91bf417dc5aa84cf8c`
Correction branch: `fix/audit-005-publish-repo-context`

## Scope

The second audit validates only the correction for the reproducible publication failure identified during the controlled AUDIT-005 exercise.

## Evidence reviewed

- Original distribution run `36945562430`, attempt `1`: checksum verification failed.
- Controlled rerun of the same run, attempt `2`: checksum verification passed.
- Controlled rerun, attempt `2`: `Publish GitHub Release` failed with `fatal: not a git repository`.
- Current distribution workflow before correction did not establish a local Git repository in the `publish` job.
- GitHub CLI supports the explicit repository selector `--repo OWNER/REPO` for `gh release create`.

## Correction audited

The publication command was changed from implicit repository discovery to explicit repository binding:

```bash
gh release create "$GITHUB_REF_NAME" --repo "$GITHUB_REPOSITORY" \
  ...
```

The correction does not add checkout and does not alter the artifact, checksum, tag-guard, permission, platform, or version contract.

## Diff-scope result

Comparison against the audited baseline shows exactly three intended files:

1. `.github/workflows/m08-block4-distribution.yml` — one-line command correction.
2. `docs/AUDIT-005-PUBLICATION-EXERCISE-RESULT.md` — publication-exercise evidence.
3. `docs/ERRORS-AND-FIXES.md` — ERR-092 registration.

No product source files, tests, package formats, workflow triggers, publication guard, or permission declarations were changed.

## Semantic audit

The release command still:

- uses the tag from `GITHUB_REF_NAME`;
- uploads the same three platform packages;
- uploads `SHA256SUMS`;
- requires `--verify-tag`;
- uses `--generate-notes`;
- receives `GH_TOKEN: ${{ github.token }}`;
- runs only when the existing tag-push publication guard is satisfied.

The correction only removes dependence on an implicit local Git repository context.

## Second-audit conclusion

**PASS — correction is bounded, semantically aligned with the observed failure, and contains no unrelated implementation changes.**

Required validation after this audit:

- PR validation with Rust CI Build → Test → Format → Clippy;
- merge only after terminal CI PASS;
- re-establish the `v0.1.0` tag on the corrected commit through an explicit controlled tag operation;
- execute the tag-triggered distribution workflow;
- verify successful checksum generation and GitHub Release publication;
- verify the release assets and `SHA256SUMS`.

