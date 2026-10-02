# AUDIT — ERR-096 PRE-AUDIT

Date: 2026-10-02
Repository: `jedaqu/android_release_doctor`
Scope: Android application discovery for Gradle version-catalog plugin aliases

## 1. Current behavior

Application project discovery is implemented in `crates/doctor-core/src/project.rs`.

`discover_application_build_file()`:

- accepts a direct `build.gradle` / `build.gradle.kts` path without application-plugin discovery;
- when given a directory, checks the root build file first;
- then checks immediate child directories for build files;
- uses `looks_like_android_application()` to decide whether a build file is an Android application module.

The current `looks_like_android_application()` implementation strips Gradle comments and then performs only:

```text
sanitized.contains("com.android.application")
```

Therefore discovery recognizes literal plugin IDs but has no path for version-catalog aliases.

## 2. Demonstrated compatibility gap

Gradle officially supports plugin aliases from `gradle/libs.versions.toml` using:

```kotlin
plugins {
    alias(libs.plugins.androidApplication)
}
```

The catalog entry maps the alias to the real plugin ID under `[plugins]`, for example:

```toml
[plugins]
androidApplication = { id = "com.android.application", version.ref = "agp" }
```

The Android documentation also shows module-level Android application aliases such as `alias(libs.plugins.androidApplication)`.

References:

- Gradle Version Catalogs documentation: https://docs.gradle.org/current/userguide/version_catalogs.html
- Android Developers dependency/plugin alias documentation: https://developer.android.com/build/dependencies
- Android Developers Kotlin DSL migration documentation: https://developer.android.com/build/migrate-to-kotlin-dsl

## 3. Exact current gap

A project may therefore have:

1. root `build.gradle.kts` with the Android application alias declared using `apply false`;
2. `app/build.gradle.kts` applying the same alias;
3. `gradle/libs.versions.toml` mapping the alias to `com.android.application`.

The current discovery path does not inspect the catalog and cannot resolve the alias to the Android application plugin.

Once a build file is selected directly, the existing `parse_project()` parser already extracts the `android { ... }` block and related fields. ERR-096 is therefore a discovery problem, not an `android`-block parsing problem.

## 4. Existing public coverage

Current public fixtures cover:

- direct Groovy Android application plugin ID;
- direct Kotlin DSL Android application plugin ID;
- project parsing from a direct build-file path;
- comment stripping;
- project/artifact cross-checks.

The public repository currently contains no `gradle/libs.versions.toml` project fixture and no regression covering version-catalog plugin discovery.

## 5. Risk boundaries

The correction must not:

- treat any `libs.plugins.*` reference as an Android application;
- infer the Android application plugin from a library alias;
- match arbitrary text outside the `plugins { ... }` block;
- treat a top-level `apply false` declaration as an application module;
- alter direct plugin-ID behavior;
- broaden directory recursion beyond the existing root + immediate-child discovery contract.

## 6. Delimited correction

The minimum safe correction is:

1. load only the standard root `gradle/libs.versions.toml` catalog when it exists;
2. inspect only the `plugins { ... }` block of each candidate build file;
3. resolve a referenced `libs.plugins.<alias>` against the catalog's `[plugins]` table;
4. classify the candidate as Android application only when the resolved plugin ID is exactly `com.android.application`;
5. ignore plugin aliases used with `apply false` during application-module discovery;
6. preserve the existing literal `com.android.application` detection path unchanged.

No general TOML dependency or generic Gradle evaluator is required by this scope.

## 7. Required regression coverage

Positive:

- root project with `gradle/libs.versions.toml`;
- root `build.gradle.kts` alias declared with `apply false`;
- immediate `app/build.gradle.kts` applying the Android application alias;
- `parse_project(root)` selects the application module.

Negative:

- catalog alias resolving to `com.android.library` is not treated as an application;
- root-only `apply false` Android application alias is not treated as an application module;
- alias text in comments/strings does not create a false application match.

Compatibility:

- existing direct Groovy plugin ID behavior remains valid;
- existing direct Kotlin DSL plugin ID behavior remains valid.

## 8. Acceptance boundary

ERR-096 is ready for implementation only against this delimited discovery change.

No ERR-093 changes.
No ERR-095 changes.
No publication workflow changes.
No unrelated parser refactor.
