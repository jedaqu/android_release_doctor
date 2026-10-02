# AUDIT — ERR-096 SECOND AUDIT

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Scope: Android application discovery for Gradle version-catalog plugin aliases

## 1. Implementation reviewed

Production change is confined to:

`crates/doctor-core/src/project.rs`

The existing discovery contract is preserved:

- direct build-file input still bypasses discovery;
- directory discovery still checks the root build file and immediate child module directories;
- literal `com.android.application` detection remains intact;
- no recursive project-tree expansion was introduced.

The new path only adds:

- optional loading of the root `gradle/libs.versions.toml`;
- inspection of the `plugins { ... }` block;
- resolution of `libs.plugins.*` aliases against the catalog `[plugins]` table;
- exact matching to `com.android.application`;
- suppression of `apply false` aliases during application-module discovery.

## 2. Regression coverage reviewed

Public positive fixture:

`tests/fixtures/project-release-version-catalog/`

It contains:

- root `build.gradle.kts` declaring the Android application alias with `apply false`;
- `app/build.gradle.kts` applying the alias;
- `gradle/libs.versions.toml` mapping both Android application and Android library aliases.

The end-to-end test selects the `app` module and verifies its parsed application ID, target SDK, version code, version name, and release debuggable state.

Unit-level negative coverage verifies:

- non-application aliases are rejected;
- dashed aliases resolve through dotted accessors;
- `apply false` aliases do not count as application modules;
- aliases embedded in quoted strings do not count;
- aliases without a catalog do not count;
- alias text outside the `plugins { ... }` block does not count;
- direct `com.android.application` detection remains valid.

## 3. Public-boundary audit

No third-party real-world application name, release artifact identifier, private checksum, or private continuity material was added.

The added fixture uses only synthetic `com.example.catalogfixture` identifiers.

## 4. Scope audit

No changes were made to:

- ERR-093;
- ERR-095;
- signing verification;
- publication workflows;
- CLI behavior;
- recursive project discovery;
- general Gradle execution;
- general TOML parsing.

## 5. CI evidence

Actions run #94 / `37018187885` completed successfully:

- Build: PASS
- Test: PASS
- Format: PASS
- Clippy: PASS

Observed test totals:

- 100 `doctor-core` unit tests passed;
- 17 fixture integration tests passed.

## 6. Conclusion

The implementation satisfies the ERR-096 acceptance boundary without broadening the project-discovery contract.

ERR-096 is ready for checkpoint closure after the documentation update and post-merge validation.
