# aegis-pqsynq Repo Audit

Audit date: 2026-05-26
Paths audited:

- Standalone: `/Volumes/xcode/synergy-network-projects/aegis-pqsynq`
- Embedded active workspace copy:
  `/Volumes/xcode/synergy-network-projects/synq-language/aegis-pqsynq`

Upstream repo: `https://github.com/synergy-network-hq/aegis-pqsynq`

## Summary

`aegis-pqsynq/pqsynq` is currently a Rust PQC facade used by SynQ. It has strong
algorithm wrapper/test-vector work and now exposes the required SynQ-specific
policy adapter API for deploy/call authorization, chain binding,
domain-separated signing payloads, payload hashing, transaction dispatch, and
address derivation.

## Toolchain

- Rust crate: `pqsynq/Cargo.toml`, `pqsynq/Cargo.lock`
- Embedded copy builds as a member of the SynQ Rust workspace.
- Standalone package metadata also includes `package.json` and
  `aegis_crypto_core/`, but the standalone `pqsynq` crate relies on workspace
  dependencies when used from SynQ.

## Build And Test Status

- Proof log: `docs/proof/SynQ-Aegis-AIVM-Proof-Log-2026-05-26.md`
- `TMPDIR=/Volumes/xcode/tmp cargo check -p aegis-pqsynq --all-features
  --all-targets --locked`: PASS.
- `TMPDIR=/Volumes/xcode/tmp cargo test -p aegis-pqsynq --all-features
  --all-targets --locked`: PASS in the active embedded workspace.
- `synq_verifier_tests.rs`: PASS, 8 tests covering valid ML-DSA-65 deploy/call,
  wrong chain/domain, malformed key, invalid signature, transaction dispatch,
  and structured error preservation.
- `canonical_payload_tests.rs`: PASS, 3 tests covering deterministic canonical
  bytes, payload hash domain/chain binding, and oversized length-prefixed field
  rejection.
- Direct standalone mirror command with `--manifest-path` is BLOCKED because
  `pqsynq/Cargo.toml` inherits workspace dependencies and no workspace root is
  present under `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq`.
- Standalone copy differs from embedded copy in
  `tests/nist_vector_replay_tests.rs`; embedded copy supports optional skipping
  unless `PQSYNQ_REQUIRE_NIST_KATS=1`.
- New SynQ policy adapter source files were synced from the active embedded copy
  into the standalone copy; current `diff -qr` shows only the
  `nist_vector_replay_tests.rs` strict/skip behavior difference.

## CI

- Embedded/standalone package has `.github/workflows/ci.yml`,
  `security.yml`, and `security-scan.yml` under `aegis-pqsynq/.github`.
- Top-level standalone path is not a git repo in this local filesystem snapshot.

## Public APIs / Surfaces

- Current exposed facade: `Kem`, `Sign`, `DigitalSignature`,
  `KeyEncapsulation`, `PqcError`, `SecretBytes`.
- Supported shipped profile in current docs: ML-KEM, ML-DSA, FN-DSA, and
  optional HQC-KEM.
- New required API scaffold: `AegisSynQVerifier` with deploy/call/transaction
  verification and canonical SynQ signing payload handling exists in the active
  embedded copy and is mirrored into the standalone copy.

## Required Surface Check

| Required item | Status | Evidence |
|---|---|---|
| `README.md` | Present in `pqsynq/` | `pqsynq/README.md` |
| `LICENSE` | Present | `aegis-pqsynq/LICENSE` |
| `SECURITY.md` | Present | `aegis-pqsynq/SECURITY.md` |
| `CONTRIBUTING.md` | Missing | no file found |
| `ARCHITECTURE.md` | Present in active embedded copy | `aegis-pqsynq/ARCHITECTURE.md` |
| `INTEGRATION.md` | Present in active embedded copy | `aegis-pqsynq/INTEGRATION.md` |
| `ROADMAP.md` | Missing | no file found |
| `CHANGELOG.md` | Missing | no file found |
| `.github/workflows/` | Present | `aegis-pqsynq/.github/workflows/` |
| `tests/` | Present | `pqsynq/tests/` |
| `examples/` | Present | `pqsynq/examples/` |
| `docs/` | Present | `pqsynq/docs/` |

## Security-Sensitive Behavior Present

- ML-DSA/FN-DSA signature wrappers.
- ML-KEM/HQC KEM wrappers.
- Size checks and structured `PqcError` paths.
- `SecretBytes` and zeroization utilities.
- KAT/vector replay tests and compliance-report scripts.

## Security-Sensitive Behavior Missing Or Incomplete

- The named vector files requested for SynQ deploy/call envelopes are not yet
  present as `ml_dsa_65_valid_deploy.json`,
  `ml_dsa_65_invalid_signature.json`, `ml_dsa_65_wrong_chain.json`, and
  `ml_dsa_65_wrong_domain.json`.
- The verifier test matrix is strong but not yet complete for every AEGIS-017
  replay/oversize/non-canonical case.

## End-To-End Blockers

- Downstream SynQ CLI/AIVM/node still need direct wiring to consume this as the
  single policy brain.
- Standalone vs embedded copy ownership must be resolved to prevent drift.
