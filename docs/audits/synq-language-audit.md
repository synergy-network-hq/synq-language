# SynQ Language Repo Audit

Audit date: 2026-05-26
Path audited: `/Volumes/xcode/synergy-network-projects/synq-language`
Upstream repo: `https://github.com/synergy-network-hq/synq-language`

## Summary

SynQ Language is a mixed Rust and TypeScript workspace. The Rust workspace owns
the current compiler, VM, CLI, PQC smoke tooling, and embedded `aegis-pqsynq`
dependency. The TypeScript SDK is under `sdk/`.

## Toolchain

- Rust workspace: `Cargo.toml`, `Cargo.lock`
- Rust members: `compiler`, `vm`, `cli`, `tooling/no-std-smoke`,
  `tooling/wasm-runtime-smoke`, embedded `aegis-pqsynq/pqsynq`, and bundled
  `pqrust` crates.
- TypeScript SDK: `sdk/package.json`, `sdk/package-lock.json`,
  `sdk/tsconfig.json`.

## Build And Test Status

- Proof log: `docs/proof/SynQ-Aegis-AIVM-Proof-Log-2026-05-26.md`
- `TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon
  cargo fmt --check`: PASS.
- `TMPDIR=/Volumes/xcode/tmp cargo check --workspace --all-features
  --all-targets --locked`: PASS.
- `TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon
  cargo test --workspace --all-features --all-targets --locked`: PASS.
- `npm run typecheck` in `sdk/`: PASS.
- `npm run test:integration` in `sdk/`: PASS, 4 tests passed.
- `TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon
  cargo test -p aegis-pqsynq --all-features --all-targets --locked`: PASS.
- Temp-space note: large Rust commands should use `TMPDIR=/Volumes/xcode/tmp`.

## CI

- No top-level `.github/workflows/` directory was found.
- Embedded `aegis-pqsynq/.github/workflows/` exists, but it is not a top-level
  SynQ workspace CI surface.

## Public APIs / Surfaces

- Rust compiler API under `compiler/src/lib.rs`.
- VM API under `vm/src/lib.rs`.
- CLI entrypoint under `cli/src/main.rs`.
- SDK package under `sdk/src/`.
- Embedded PQ facade under `aegis-pqsynq/pqsynq/src/lib.rs`.

## Required Surface Check

| Required item | Status | Evidence |
|---|---|---|
| `README.md` | Present | `README.md` |
| `LICENSE` | Missing at root | only embedded `aegis-pqsynq/LICENSE` found |
| `SECURITY.md` | Missing at root | `governance/SECURITY_POLICY.md` exists |
| `CONTRIBUTING.md` | Present | `CONTRIBUTING.md` |
| `ARCHITECTURE.md` | Missing | architecture content is spread across docs |
| `ROADMAP.md` | Missing | roadmap content exists in `checklist.md` and management docs |
| `CHANGELOG.md` | Missing | no root changelog found |
| `.github/workflows/` | Missing at root | only embedded aegis workflows found |
| `tests/` | Partial | crate-level tests exist; no root `tests/` |
| `examples/` | Partial | `smart-contracts/examples`, `docs/examples` |
| `docs/` | Present | `docs/` |

## Security-Sensitive Behavior Present

- PQ signature/KEM facade through embedded `aegis-pqsynq`.
- SynQ-specific deploy/call verifier API through
  `aegis-pqsynq/pqsynq/src/verifier.rs`.
- Canonical signing payload hashing through
  `aegis-pqsynq/pqsynq/src/serialization.rs`.
- Compiler PQC integration in `compiler/src/pqc_integration.rs`.
- VM PQC opcode/precompile behavior in `vm/src/`.
- Deterministic bytecode verification command exists in the CLI.
- Key lifecycle, vector replay, and side-channel docs exist under
  `aegis-pqsynq/pqsynq/docs/`.

## Security-Sensitive Behavior Missing Or Incomplete

- Chain 1264 deploy/call payload verification is not implemented end-to-end.
- AIVM is not yet the production execution environment for deployed contracts.
- Top-level threat model needs refresh against SynQ + AIVM + Rosetta.
- Public docs still include historical/aspirational checklist content.

## Undocumented Integration Assumptions

- AIVM currently depends on SynQ VM through a relative path from
  `/Volumes/xcode/synergy-network-projects/synergy-aivm/runtime/aivm-core` to
  `../../../synq-language/vm`.
- Embedded and standalone `aegis-pqsynq` copies both exist and still differ in
  `tests/nist_vector_replay_tests.rs`.
- Standalone `aegis-pqsynq/aegis-pqsynq/pqsynq` cannot run by direct
  `--manifest-path` because the crate inherits workspace dependencies but no
  workspace root is present in that mirror.
- The canonical chain target is TESTNET chain `1264`.

## End-To-End Blockers

- Shared spec files exist, but fixture-backed cross-implementation test vectors
  are still pending.
- AIVM lacks full deploy/call/state/account/storage/receipt lifecycle.
- TESTNET RPC/node contract deployment methods are not yet verified.
