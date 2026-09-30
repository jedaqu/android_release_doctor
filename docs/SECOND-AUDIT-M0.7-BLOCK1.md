# SECOND-AUDIT M0.7 Block 1 — ECDSA/SHA-512 P-384

**Initial second-audit target:** implementation head `511ed3844dbf7ed962bc8d973756255d5a95e2f2`  
**Initial validation:** Actions #305 / `36782365269` — Build/Test/Format/Clippy PASS  
**Scope:** review of the completed implementation against the pre-change audit and M0.7 Block 1 acceptance criteria.

## 1. Audit baseline

The second audit re-reviewed the incremental ledger through ERR-042, the pre-change audit, the M0.7 definition, the final implementation, fixtures, dependency graph, workflow, and final push validation.

The implementation remains restricted to the selected increment:

- signature algorithm `0x0202`;
- ECDSA with SHA-512;
- NIST P-384 only;
- both v2 and v3 verification paths.

No v3.1/v3.2, DSA, P-521, RSA key-size expansion, AAB crypto verification, or broad verifier redesign was introduced.

## 2. Implementation verification

The verifier now:

1. recognizes `0x0202` during signature selection;
2. preserves SHA-512 as the digest algorithm;
3. checks that the signer key is EC;
4. checks that the named curve is exactly P-384;
5. uses the certificate SubjectPublicKey BIT STRING as the SEC1 public-key encoding;
6. computes SHA-512 over the actual signed-data bytes;
7. verifies the ASN.1 ECDSA signature using the P-384 backend;
8. returns `Invalid` on cryptographic failure;
9. returns `Unsupported` for ECDSA/SHA-512 curves outside P-384;
10. leaves the existing certificate/public-key binding, APK content digest, algorithm-list equality, signer isolation, SDK evidence, and proof-of-rotation evidence paths intact.

The final implementation is therefore behaviorally aligned with the selected bounded increment.

## 3. Test and fixture verification

The branch contains real v2 and v3 APK fixtures targeting `0x0202` / P-384. Their positive verification tests pass.

A direct deterministic P-384 ECDSA/SHA-512 test also passes, including a tampered-signature negative case.

### Finding AUDIT-M0.7-B1-002

The initial second audit identified two acceptance-coverage gaps in the regression suite:

- the new algorithm had an explicit v3 APK tampering regression but not an equivalent v2 APK tampering regression;
- the unsupported-curve boundary was implemented explicitly but did not yet have a focused regression test demonstrating `Unsupported` for a non-P-384 ECDSA/SHA-512 certificate.

These are test-coverage defects, not production verification defects.

**Required correction:** add the two focused regressions without changing production behavior.

## 4. Dependency/lockfile verification

The implementation introduced direct dependencies on `p384` and `sha2`.

The audit initially found that the tracked `Cargo.lock` had not yet been regenerated for those dependencies. A temporary CI-only lock-generation mechanism was used to generate the lockfile with the repository toolchain; the generated lockfile was committed as:

`3880246bc00ea45d78d04ea6b30449fa84bde54d`

The temporary workflow was then removed. The final branch contains no temporary lock-generation workflow.

The final Rust CI run #305 validates the resulting tracked dependency graph.

## 5. Historical correction validation

ERR-039, ERR-040, ERR-041 and ERR-042 corresponded to implementation/fixture/formatting failures discovered during the first validation passes.

Their corrections are now represented in the branch history and the current implementation passes the complete Rust CI gate.

## 6. Scope and architecture review

No new global verification state was introduced.

The selected algorithm uses a dedicated P-384 verification backend while preserving the existing ring-based verifier for the previously supported algorithms. This keeps the change localized and avoids an unnecessary crypto-backend refactor.

The multi-signer aggregation model remains unchanged.

## 7. Re-audit decision before correction

**SECOND AUDIT — CORRECTIONS REQUIRED**

The production implementation itself is within scope and passes the complete current CI gate, but the Block 1 acceptance matrix requires the two focused regression tests identified in AUDIT-M0.7-B1-002.

After those tests are added, the audit document will be completed with a final PASS and a new validation run.

## 8. Acceptance after correction

The corrected block must demonstrate:

- valid v2 `0x0202` / P-384 → `Verified`;
- valid v3 `0x0202` / P-384 → `Verified`;
- tampered v2 `0x0202` artifact → `Invalid`;
- tampered v3 `0x0202` artifact → `Invalid`;
- non-P-384 `0x0202` → `Unsupported`;
- existing M0.6 cryptographic regressions remain green;
- tracked `Cargo.lock` matches the declared dependencies;
- Build/Test/Format/Clippy all pass.

