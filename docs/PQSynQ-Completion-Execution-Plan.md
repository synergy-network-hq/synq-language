# Aegis-PQSynQ Completion Execution Plan

## Objective

Deliver a production-grade `aegis-pqsynq` foundation that SynQ can rely on for secure, verifiable smart-contract cryptography without documentation drift, fake-green CI, or unsupported API claims.

## Scope Definition

This plan covers:

- `aegis-pqsynq/pqsynq` crate completion and hardening
- Evidence-based validation gates
- Integration handoff requirements for `vm`, `compiler`, and later `sdk`

This plan does not claim completion of all SynQ language/runtime work. It isolates the PQ foundation track.

## Current State Snapshot (2026-02-09)

### Completed in this sprint

- [x] Module naming normalized to `aegis-pqsynq` (package) with `pqsynq` import compatibility.
- [x] SynQ manifests updated to depend on `package = "aegis-pqsynq"`.
- [x] `full` feature remapped to implemented set (`mlkem`, `mldsa`, `fndsa`, `hqckem`, `std`).
- [x] `Sign` API now includes:
  - [x] `detached_sign`
  - [x] `verify_detached`
  - [x] `sign_ctx` (ML-DSA)
  - [x] `verify_ctx` (ML-DSA)
  - [x] explicit `NotImplemented` behavior for unsupported FN-DSA context operations
- [x] Legacy failing/full-drift tests replaced with implementation-aligned suites.
- [x] `README`, CI workflow, and local test runner rewritten to match reality.
- [x] Validation evidence captured:
  - [x] `cargo test -p aegis-pqsynq --all-features --all-targets` PASS
  - [x] `cargo clippy -p aegis-pqsynq --all-targets --all-features --no-deps -- -D warnings` PASS
  - [x] `cargo test --workspace --all-targets` PASS
- [x] no-default feature profile cleaned and warning-free (`cargo check -p aegis-pqsynq --no-default-features`)
- [x] `wasm32-unknown-unknown` baseline compile now passes for no-default profile
- [x] `getrandom` wasm backend configuration aligned (`wasm_js` backend + workspace wasm cfg)
- [x] `pqrust` build scripts hardened for WASI targets (non-panicking env handling + `wasm32-wasip1` C-target mapping)
- [x] Signature wrappers hardened (ML-DSA/FN-DSA FFI result checks + structured errors)
- [x] Deterministic replay fixture harness added (pinned vectors + hash manifest + replay tests)
- [x] Key-material lifecycle controls added (`SecretBytes`, `zeroize_bytes`, lifecycle policy doc, tests)
- [x] Validation runner now includes optional `cargo-audit` gate
- [x] Official NIST replay harness added for shipped algorithms (ML-KEM/ML-DSA/FN-DSA strict replay; HQC compatibility replay + generated self-consistency)
- [x] Compliance report artifact generation added (`artifacts/pqsynq-compliance-report.md` + logs + source SHA-256 table)
- [x] WASI toolchain bootstrap is now deterministic in local/CI paths (`bootstrap_wasi_sdk.sh` + CI cache/export wiring)
- [x] Side-channel posture and dependency pinning policies are documented and linked from README
- [x] Deterministic CLI verification command added (`cli verify --source --bytecode [--run]`) with tamper-detection tests
- [x] Target support matrix documented (`aegis-pqsynq/pqsynq/docs/TARGET_SUPPORT_MATRIX.md`)

### Final blockers resolved (2026-02-09)

- [x] Add HQC-KEM support in `aegis-pqsynq` crate (constructors, feature wiring, tests, benchmarks)
- [x] Integrate HQC usage into SynQ VM/compiler contract flows where needed (opcode + intrinsic dispatch for HQC-128/192/256)
- [x] De-scope CMCE-KEM and SLH-DSA for SynQ smart-contract profile due key-size/footprint tradeoffs
- [x] Implement official NIST vector replay harnesses for all shipped algorithms
- [x] Define and enforce key material lifecycle/zeroization policy
- [x] Complete deterministic wasm toolchain bootstrap in CI/local (WASI SDK + `CC_wasm32_wasi` wiring)
- [x] Add contract-level integration tests where compiler emits bytecode that exercises PQ opcodes through VM
- [x] Add wasm runtime smoke tests for wasm targets (execution checks, not only compile checks)
- [x] Eliminate or replace unmaintained `paste` dependency path flagged by `RUSTSEC-2024-0436`
- [x] Add SDK-side integration tests that consume SynQ PQ contract flows end-to-end

## Execution Backlog (Detailed)

## Phase 1: Truth-First API and Feature Surface (Now mostly complete)

- [x] Align crate/package naming and dependency manifests
- [x] Align feature graph and full-mode behavior
- [x] Eliminate non-existent CI/test targets
- [x] Ensure helper APIs exist where tests and docs rely on them
- [x] Rewrite tests to assert real behavior only

Exit Criteria:

- [x] No references to unsupported constructors in active test suites
- [x] `--all-features --all-targets` path is green

## Phase 2: Algorithm Coverage Expansion (Constrained Profile)

Roadmap policy for SynQ smart contracts:

- [x] Keep ML-KEM / ML-DSA / FN-DSA as baseline profile
- [x] Add HQC-KEM as optional extended KEM family
- [x] Defer CMCE-KEM and SLH-DSA from smart-contract profile unless requirements change

HQC implementation tasks:

- [x] Add `hqckem` feature wiring and dependency integration
- [x] Add `Kem::hqckem128/192/256` constructors and implementations
- [x] Add size/error/roundtrip tests in full suite
- [x] Add HQC benchmark coverage
- [x] Update docs to reflect HQC optional support

Exit Criteria:

- [x] HQC has passing unit + integration + KAT-style coverage in `aegis-pqsynq`
- [x] VM/compiler layers expose and validate HQC flows where product scope requires them

## Phase 3: Verification Rigor and Compliance Evidence

- [x] Add deterministic test harness to replay official NIST vectors from pinned artifacts
- [x] Add vector provenance tracking (source, hash, retrieval date) for pinned replay fixtures
- [x] Add fail-fast check when vectors are missing/stale (fixture hash manifest test)
- [x] Add generated compliance report artifact per CI run

Exit Criteria:

- [x] KAT-style replay is automated and reproducible for pinned regression fixtures
- [x] Official NIST/ACVP replay is automated and reproducible for shipped algorithms
- [x] Report artifact is generated in CI for each release candidate

## Phase 4: Security Hardening

- [x] Add secret key handling policy document
- [x] Introduce key zeroization strategy where practical
- [x] Add misuse-resistance tests (wrong key, wrong context, tampered ciphertext/signature)
- [x] Replace panic/assert FFI failure paths in shipped KEM wrappers with structured error returns
- [x] Replace panic/assert/unchecked FFI failure paths in shipped signature wrappers with structured error returns
- [x] Add side-channel posture statement and assumptions
- [x] Integrate `cargo-audit` gate in local validation runner (optional when tool unavailable)
- [x] Integrate dependency pinning policy for release cadence

Exit Criteria:

- [x] Security controls documented and tested
- [x] No unresolved high-severity findings in PQ foundation scope (latest `cargo audit` has no CVE findings or unresolved dependency advisories in-scope)

## Phase 5: Portability and Runtime Targets

- [x] Define supported target matrix (native + wasm + no_std profile)
- [x] Add no_std-specific tests (where possible)
- [x] Add wasm target checks for baseline profile (`wasm32-unknown-unknown` no-default)
- [x] Add WASI full-feature compile gate (requires WASI SDK + `CC_wasm32_wasi`)
- [x] Add wasm runtime smoke tests
- [x] Gate release on matrix pass with documented toolchain bootstrap

Exit Criteria:

- [ ] Declared target matrix is fully green in CI (local matrix checks are green; CI confirmation remains required)

## Phase 6: SynQ Runtime Integration Gates

- [x] Compiler emits PQ opcodes from representative contract fixtures
- [x] VM executes PQ flows against real contracts (not only direct opcode tests)
- [x] CLI supports deterministic compile+run verification scenarios
- [x] SDK integration tests validate end-to-end contract invocation semantics

Exit Criteria:

- [x] Contract-level PQ tests pass in CI
- [x] SynQ release checklist can treat PQ substrate as production candidate

## Quality Gates (Must Stay Green)

- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy -p aegis-pqsynq --all-targets --all-features --no-deps --locked -- -D warnings`
- [x] `cargo test -p aegis-pqsynq --all-features --all-targets --locked`
- [x] `cargo test --workspace --all-targets --locked`
- [x] `cargo test -p cli --test integration_test --locked` (includes deterministic `verify` + tamper rejection)
- [x] `cargo bench -p aegis-pqsynq --no-run --locked`
- [x] `cargo check -p aegis-pqsynq --no-default-features --locked`
- [x] `cargo check -p aegis-pqsynq --target wasm32-unknown-unknown --no-default-features --locked`
- [x] `WASI_SDK_DIR=... CC_wasm32_wasi=... cargo check -p aegis-pqsynq --target wasm32-wasip1 --no-default-features --features "mlkem,mldsa,fndsa,hqckem" --locked`
- [x] `bash aegis-pqsynq/pqsynq/scripts/run_wasm_runtime_smoke.sh`
- [x] `npm run test:integration` (in `sdk/`)

## Risk Register

- Risk: Re-introducing claim drift as algorithm backlog grows.
  - Mitigation: CI blocks docs/feature changes unless corresponding tests exist.

- Risk: "full" feature grows into a catch-all and becomes unstable.
  - Mitigation: keep `full` explicitly composed and audited every release.

- Risk: Integration passes at library level but fails at contract runtime level.
  - Mitigation: mandatory contract-level PQ acceptance suite before SynQ release candidate.

## Definition of Done for "PQSynQ fully built out"

All criteria must be true:

- [x] Every algorithm family claimed in docs is implemented and tested.
- [x] CI commands and local scripts execute only real test targets.
- [x] KAT compliance evidence exists for every shipped algorithm.
- [x] Security hardening tasks are complete and documented.
- [x] SynQ compiler/vm/cli integration tests validate real contract PQ flows.
- [x] No unresolved high-severity issues in PQ foundation audit scope.
