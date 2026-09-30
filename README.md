# Android Release Doctor

Open-source, local-first tool for auditing Android APK and AAB releases before publication.

> **Status:** early development — M0 foundation.

Android Release Doctor inspects the **artifact you are actually going to distribute**, rather than relying only on what a Gradle project declares. The long-term goal is actionable release-readiness diagnostics: what was found, why it matters, and what to fix.

## M0 scope

The first engine structurally inspects APK/AAB archives and reports:

- artifact type and ZIP validity;
- Android manifest presence;
- DEX payload presence;
- native library ABI inventory;
- signing metadata presence.

This release does **not** yet validate target/min SDK, `debuggable`, exported components, cryptographic signatures, Play requirements, or project-vs-artifact consistency.

## Usage

```text
android-release-doctor app-release.aab
android-release-doctor app-release.apk
```

The command exits with code `1` when a blocker is detected and `2` for invalid input or invocation errors.

## Development

```bash
cargo test --workspace
cargo run -p doctor-cli -- tests/fixtures/minimal-release.apk
```

## Design principles

- **Local-first:** release artifacts are inspected locally.
- **Evidence before conclusions:** findings are tied to observable artifact evidence.
- **Actionable diagnostics:** warnings explain what to change.
- **Versioned rules:** Android and Play requirements change, so rules are explicit and replaceable.
- **Artifact first:** the final APK/AAB is a source of truth, not just build configuration.

## License

Apache License 2.0.
