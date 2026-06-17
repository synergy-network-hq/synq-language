# Project Rosetta Repo Audit

Audit date: 2026-05-26
Path audited: `/Volumes/xcode/Synergy-Network-Projects/project-rosetta`
Upstream repo: `https://github.com/synergy-network-hq/project-rosetta`

## Summary

Project Rosetta is an internal prototype monorepo for verified cross-chain native
asset settlement. It is explicitly not a pooled bridge, not a wrapped-asset
default, and not production-ready. Current implementation includes Rust protocol
crates, a TypeScript SDK, EVM prototype contracts, docs, mock examples, and test
vectors.

## Toolchain

- Rust workspace: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`
- Rust crates: `rosetta-core`, `rosetta-intent`, `rosetta-verification`,
  `rosetta-settlement`, `rosetta-attestation`, `rosetta-audit`, `rosetta-cli`
- TypeScript SDK: `package.json`, `pnpm-lock.yaml`, `pnpm-workspace.yaml`,
  `sdk/typescript/package.json`, `sdk/typescript/tsconfig.json`
- EVM prototype contracts under `contracts/evm`

## Build And Test Status

- Proof log: `docs/proof/SynQ-Aegis-AIVM-Proof-Log-2026-05-26.md`
- `cargo fmt --check`: PASS.
- `cargo test --workspace`: PASS, including
  `domain::tests::synergy_testnet_domain_is_chain_1264` and 5 new
  `synergy::tests::*` receipt-verifier tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `corepack pnpm --filter @synergy/rosetta-sdk typecheck`: PASS.
- `corepack pnpm --filter @synergy/rosetta-sdk test`: PASS, 4 files and 5 tests
  passed, including `synergy-domain.test.ts`.
- Foundry tests remain unverified in this local run.

## CI

- `.github/workflows/ci.yml` exists and covers Rust, TypeScript, and Solidity
  checks when tools are available.

## Public APIs / Surfaces

- Rust core domain/asset/route/hash types.
- Intent envelope, scope digest, and replay domain helpers.
- Verification adapter interfaces and mock verification.
- Settlement engine with per-intent escrow state machine.
- Audit receipt generation.
- CLI commands for simulation and mock verification.
- TypeScript SDK helpers for wallet/dApp integration.

## Required Surface Check

| Required item | Status | Evidence |
|---|---|---|
| `README.md` | Present | `README.md` |
| `LICENSE` | Present | `LICENSE` |
| `SECURITY.md` | Missing | threat model exists under docs |
| `CONTRIBUTING.md` | Missing | no file found |
| `ARCHITECTURE.md` | Partial | `docs/architecture.md` |
| `ROADMAP.md` | Partial | `docs/phase-0-de-risking.md` |
| `CHANGELOG.md` | Missing | no file found |
| `.github/workflows/` | Present | `.github/workflows/ci.yml` |
| `tests/` | Partial | Rust/TS tests exist in crate and SDK test dirs |
| `examples/` | Present | `examples/escrow-swap-mock/` |
| `docs/` | Present | `docs/` |

## Security-Sensitive Behavior Present

- Scope digest and replay-domain computation.
- Intent validation and consumed-intent checks.
- Mock verification adapters.
- Synergy TESTNET receipt-verifier interface and deterministic mock verifier in
  `crates/rosetta-verification/src/synergy.rs`.
- Per-intent escrow state transitions and refund paths.
- Prototype legal/IP boundaries and production warnings.
- New Synergy TESTNET chain-1264 domain helpers in Rust and TypeScript.

## Security-Sensitive Behavior Missing Or Incomplete

- Live chain-1264 RPC method mapping.
- Production cryptographic provider and audited proof adapters.
- Foundry test verification in the local environment.
- Formal integration with `aegis-pqsynq` for Synergy/SynQ signatures.
- Decision on whether AIVM executes Rosetta verification logic or only indexes
  settlement artifacts.

## End-To-End Blockers

- Current Rosetta flows are mock/prototype only.
- No live network submit path should be enabled until read-only chain-1264
  receipt verification and replay binding are proven.
- Rosetta must remain per-intent escrow only; pooled custody and default wrapped
  assets are out of scope.
