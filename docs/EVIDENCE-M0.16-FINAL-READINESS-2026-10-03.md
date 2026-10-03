# M0.16 — Final Product Readiness Evidence Matrix — 2026-10-03

## Readiness baseline

- Main baseline audited: `1298c73d3019799e76cf7ace9706c8f880a98d72`
- Maintained workflows:
  - `.github/workflows/rust.yml`
  - `.github/workflows/android-release-doctor-action.yml`
- No other current workflow files are present in the repository tree.
- Public repository scan for commercial/private-boundary terms returned no matches for monetization, pricing, commercial, revenue, subscription, business model, sales, customer strategy, password, credential, token, or private continuity.

## Automated Rust/CLI evidence

Current main Rust CI Run `37141818410` completed SUCCESS.

Its test job reported:
- 7 CLI integration tests passed;
- 115 doctor-core unit tests passed;
- 25 doctor-core integration tests passed;
- total automated Rust/CLI tests: 147 passed;
- 0 failed.

The same run also completed Build, Format, and Clippy successfully.

## Permanent Action validation

Current main Action Validation Run `37141818392` completed SUCCESS.

The maintained validation workflow completed:
- success path;
- blocker path;
- AAB path;
- project path;
- project + Play path;
- non-default Play platform;
- invalid Play input;
- invalid output format;
- invalid Play platform;
- invalid CR/LF output path;
- Report v1 runtime schema validation;
- consumer-style immutable-SHA Action invocation;
- deterministic APK/AAB corpus;
- operational error path.

All workflow validation steps completed successfully.

## External consumer evidence

Private consumer repository final validation Run `37141021253` completed SUCCESS.

The Action was consumed from an immutable product SHA:
`jedaqu/android_release_doctor/.github/actions/android-release-doctor@d1d681e46391c95fd77152779ff4e3656995837f`

Verified externally:
- real Meshtastic v2.8.1 APK;
- success exit 0;
- blocker exit 1;
- invalid format exit 2;
- success/blocker Report v1 schema validation;
- `report-path` and `exit-code` outputs.

The temporary consumer workflow was removed after evidence collection.

## Independent public APK/AAB evidence

### Meshtastic Android v2.8.1
- APK: 27 PASS, 0 WARNING, 0 BLOCKER.
- AAB: 27 PASS, 0 WARNING, 0 BLOCKER.
- APK SHA-256: `7f42735fd1c17c7e6a64d3a48ae1e4baf22cab778e994eb8331d3b993e42eb00`.
- AAB SHA-256: `c8872f888b326e0b3e50bed1fd97c8f04873e7658589da680e69e7cc2738de8d3`.
- Validation runs: `37139781688`, `37139869383`.

### Reticulum Mobile Emergency Management v1.2.3
- APK: 19 PASS, 1 WARNING, 0 BLOCKER.
- AAB: 19 PASS, 1 WARNING, 0 BLOCKER.
- Warning: `NATIVE-002 — 16 KB ELF alignment` on affected `arm64-v8a` / `x86_64` native libraries.
- APK SHA-256: `113CB66C09BE900A0096FDF6D95F90C86C13FC6F5FCBF1A27F15986196320638`.
- AAB SHA-256: `2E55481D0CBB5E1FC87EAA5DD7BF9907C7071AB0767D29B989FB35CB35529A4E`.
- Validated in the real-public-artifact campaign runs above.

### Ventoid 0.3.1
- APK: 17 PASS, 1 WARNING, 0 BLOCKER.
- AAB: 17 PASS, 1 WARNING, 0 BLOCKER.
- Warning: `NATIVE-002 — 16 KB ELF alignment` affecting four native libraries in `arm64-v8a` / `x86_64`.
- APK SHA-256: `50bd5979ff7bfa5560ce6a52386a43bd3e3b738b40218a897debd9573926d2ca`.
- AAB SHA-256: `c27c7c1dd2a2b3445373dde0f9260af604af3ce087aa8e4e21cd9ddd5a0a10f3`.
- Validation run: `37140437383`.

The real-artifact findings demonstrate that the tool distinguishes clean evidence from warnings rather than treating all real artifacts as uniformly passing.

## Contract/schema evidence

The published Report v1 schema remains `docs/report-schema-v1.0.json` using Draft 2020-12. Current permanent Action validation and the external consumer validation both exercise runtime schema validation on generated success/blocker reports.

## Public documentation coherence

Verified:
- README identifies the reusable Action plus CLI/audit engine as the active product surface.
- Action examples use an explicit reviewed reference placeholder rather than a moving `@main` reference.
- Current product-state documentation preserves historical milestones as history while identifying the maintained surface.
- No stale statement was found claiming that a separate consumer repository or Report v1 runtime validation remains merely future work.
- Current active workflows are limited to the two maintained workflows.

## Final assessment

No reproducible production defect was found in the final readiness gate.

The public product surface is internally coherent and the existing assurance evidence is sufficient to move from engineering validation into product preparation without adding functionality.
