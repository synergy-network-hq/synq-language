# aegis-pqsynq Integration Status

## ✅ Completed Integration

aegis-pqsynq has been **fully moved into SynQ** as an internal dependency:

### Structure

- **Location**: `SynQ/aegis-pqsynq/pqsynq/` (moved from `aegis-pqsynq/`)
- **Status**: Internal dependency of SynQ workspace
- **Integration**: Fully integrated into VM and Compiler

### VM Integration

- ✅ Replaced `pqc-shims` placeholder with `pqsynq` in `vm/Cargo.toml`
- ✅ Updated VM to use `pqsynq::Sign` and `pqsynq::Kem` for actual PQC operations
- ✅ DilithiumVerify opcode now uses `Sign::mldsa65().verify()`
- ✅ FalconVerify opcode now uses `Sign::fndsa512().verify()`
- ✅ KyberKeyExchange opcode now uses `Kem::mlkem768().decapsulate()`

### Compiler Integration

- ✅ Added `pqsynq` dependency to `compiler/Cargo.toml`
- ✅ Created `pqc_integration.rs` module with full PQC function support
- ✅ Codegen detects PQC functions and generates appropriate bytecode
- ✅ Integration layer provides runtime verification functions

### Workspace Configuration

- ✅ Added `aegis-pqsynq/pqsynq` to SynQ workspace members
- ✅ Added pqrust crates to workspace for dependency resolution
- ✅ Defined workspace.dependencies for all PQC crates

## Current Status

The integration is **structurally complete but not functionally complete** for final SynQ smart-contract scope.
The SynQ workspace builds/tests cleanly for the current PQC subset used by VM/compiler:

- **ML-DSA** (mldsa44/65/87)
- **FN-DSA (Falcon)** (fndsa512/fndsa1024)
- **ML-KEM** (mlkem512/768/1024)
- **HQC-KEM** (hqckem128/hqckem192/hqckem256) via dedicated VM opcodes and compiler PQC dispatch

HQC-KEM is now available in `aegis-pqsynq` via the `hqckem` feature. CMCE and SLH-DSA remain intentionally de-scoped for the current SynQ smart-contract profile due key-size/footprint tradeoffs.

Portability status is now explicit:
- `wasm32-unknown-unknown` no-default baseline compiles.
- `wasm32-wasip1` full-feature compile works with explicit WASI SDK tooling (`WASI_SDK_DIR` + `CC_wasm32_wasi`).
- Deterministic CI/bootstrap for that WASI toolchain is now wired (`bootstrap_wasi_sdk.sh` + workflow cache/export + wasm compile gates).
- Contract-level HQC fixture coverage now includes both positive and negative (mismatched-key) execution paths through parser -> codegen -> VM.
- KEM FFI failure handling is now hardened for ML-KEM and HQC-KEM:
  - `pqrust` wrappers return structured errors (`FfiFailure`) instead of panicking on non-zero backend status.
  - `aegis-pqsynq` propagates these as `PqcError::CryptoError`, and mismatch/tamper misuse paths are covered by tests.
- Signature FFI failure handling is now also hardened for shipped signature families:
  - ML-DSA (`mldsa44/65/87`) and FN-DSA (`fndsa512/1024`) wrappers now return structured `Result` errors on backend failure paths.
  - `aegis-pqsynq` signature facade now propagates those failures as `PqcError::CryptoError`.
- Deterministic replay fixtures are now integrated for shipped SynQ profile algorithms:
  - Pinned fixture set: `aegis-pqsynq/pqsynq/tests/vectors/pinned_vectors.json`
  - Integrity/provenance manifest with SHA-256 fail-fast gate: `aegis-pqsynq/pqsynq/tests/vectors/manifest.json`
  - Replay tests: `vector_replay_tests.rs` + `vector_manifest_tests.rs`
- Official NIST replay harness is now integrated for shipped algorithms:
  - `aegis-pqsynq/pqsynq/tests/nist_vector_replay_tests.rs`
  - ML-KEM/ML-DSA/FN-DSA replay official vectors directly
  - HQC executes official-vector compatibility checks plus generated encaps/decaps self-consistency (due upstream KAT framing mismatch)
- Compliance evidence is now emitted as an artifact:
  - `aegis-pqsynq/pqsynq/scripts/generate_compliance_report.sh`
  - `aegis-pqsynq/pqsynq/artifacts/pqsynq-compliance-report.md`
  - includes pass/fail logs and official source SHA-256 hashes
- Key lifecycle controls are now implemented at the crate level:
  - `pqsynq::SecretBytes` zeroizing wrapper
  - `pqsynq::utils::zeroize_bytes`
  - lifecycle policy document in `aegis-pqsynq/pqsynq/docs/KEY_MATERIAL_LIFECYCLE_POLICY.md`
- CLI deterministic verification workflow is now implemented:
  - `cli verify --source <contract.synq> --bytecode <artifact.synq> [--run]`
  - deterministic source->bytecode reproducibility check with SHA-256 mismatch reporting
  - integration tests cover matching and tampered bytecode paths
  - compile flow now avoids source overwrite when input already ends with `.synq` (emits `.compiled.synq`)
- Dependency hygiene and runtime portability gates are now closed:
  - transitive `paste` path was removed from `pqrust-mldsa` (`RUSTSEC-2024-0436` no longer reported in current workspace audit)
  - wasm runtime smoke execution is automated (`scripts/run_wasm_runtime_smoke.sh`) and included in `run_tests.sh`
  - SDK integration tests are automated (`sdk/tests/integration.test.ts`) and included in `run_tests.sh`
- Compiler front-end gaps were reduced with parser/codegen execution work:
  - annotations now parse into contract/function/constructor/event/state-variable AST nodes
  - `for` loop statements now lower into AST and execute through VM in integration tests
  - Solidity-style SynQ fixtures in `docs/examples` now parse/compile end-to-end through official CLI
  - parser compatibility now accepts Solidity-style declarations, mapping types, tuple returns, and contract-local struct/enum declarations
  - Solidity generator now handles `for` and `require_pqc` statements instead of hard-failing on transpile
- Compiler semantic gate now includes baseline typed enforcement:
  - assignment/local-init/return compatibility checks (with parser-fallback guardrails)
  - boolean-condition checks for `require`/`if`/ternary
  - PQC built-in argument/arity checks for ML-DSA/FN-DSA/KEM decapsulation call surfaces
  - compile-time rejection of de-scoped SLH built-ins
  - flow-sensitive control-flow checks for non-void missing-return paths and unreachable statements
  - tuple return expressions are now accepted in grammar so return-path flow checks apply to tuple-returning functions

## Next Steps

1. Advance semantic analysis from baseline to full flow/lvalue rigor (`SEM-002`/`SEM-003`/`SEM-004` remaining depth items), especially flow-sensitive returns and full lvalue typing.
2. Replace SDK Kyber/ML-KEM placeholder crypto path with production-grade implementation wiring (current `sdk/src/crypto/kyber.ts` still contains stubs).
3. Finish first-class developer workflows (`CLI-002`, `CLI-003`, `SC-001`, `SC-002`) so users can test/deploy canonical contracts end-to-end.

## Files Modified

- `SynQ/vm/Cargo.toml` - Added pqsynq dependency
- `SynQ/vm/src/vm.rs` - Replaced shims with pqsynq implementations
- `SynQ/compiler/Cargo.toml` - Added pqsynq dependency  
- `SynQ/compiler/src/pqc_integration.rs` - Full integration layer
- `SynQ/Cargo.toml` - Added pqsynq to workspace
- `SynQ/compiler/src/parser.rs` - Added executable statement/expression parsing for contract fixtures
- `SynQ/compiler/src/synq.pest` - Extended grammar for Solidity-style contract compatibility, mapping types, tuple returns, and typed local declarations
- `SynQ/compiler/src/codegen.rs` - Added variable storage/load lowering and executable `for` loop lowering
- `SynQ/compiler/src/solidity_gen.rs` - Added compatibility lowering for `for` and `require_pqc` statements in Solidity output
- `SynQ/compiler/tests/integration_test.rs` - Added parser -> codegen -> VM HQC fixture tests
- `SynQ/compiler/tests/integration_test.rs` - Added parser annotation + `for` loop lowering tests and VM execution test
- `SynQ/cli/src/main.rs` - Added `verify` command for deterministic source->bytecode validation (optional execution path)
- `SynQ/cli/tests/integration_test.rs` - Added deterministic verification success/failure integration tests
- `SynQ/smart-contracts/examples/hqckem-contract-flow.synq` - Added HQC contract fixture template
- `SynQ/smart-contracts/tests/hqckem*-decap-fixture.synq` - Added HQC fixture inputs for test harnessing
- `SynQ/pqrust/pqrust-traits/src/lib.rs` - Added `FfiFailure` error variant
- `SynQ/pqrust/pqrust-mlkem/src/mlkem*.rs` - Replaced assertion-based FFI handling with `Result`-based error propagation
- `SynQ/pqrust/pqrust-hqckem/src/hqckem*.rs` - Replaced assertion-based FFI handling with `Result`-based error propagation
- `SynQ/pqrust/pqrust-mldsa/src/mldsa*.rs` - Replaced assertion/unchecked signature FFI handling with `Result`-based error propagation
- `SynQ/pqrust/pqrust-mldsa/Cargo.toml` - Removed `paste` dependency and macro path
- `SynQ/pqrust/pqrust-fndsa/src/fndsa*.rs` - Replaced assertion/unchecked signature FFI handling with `Result`-based error propagation
- `SynQ/aegis-pqsynq/pqsynq/src/kem.rs` - Propagates wrapper failures into `PqcError`
- `SynQ/aegis-pqsynq/pqsynq/src/sign.rs` - Propagates signature wrapper failures into `PqcError`
- `SynQ/aegis-pqsynq/pqsynq/src/utils.rs` - Added zeroization utilities + `SecretBytes` wrapper
- `SynQ/aegis-pqsynq/pqsynq/tests/key_material_tests.rs` - Added key-material lifecycle/zeroization tests
- `SynQ/aegis-pqsynq/pqsynq/tests/vector_replay_tests.rs` - Added deterministic replay fixture tests
- `SynQ/aegis-pqsynq/pqsynq/tests/vector_manifest_tests.rs` - Added fixture hash integrity/provenance test
- `SynQ/aegis-pqsynq/pqsynq/tests/vectors/*` - Added pinned fixtures and manifest
- `SynQ/aegis-pqsynq/pqsynq/examples/generate_pinned_vectors.rs` - Added fixture generation utility
- `SynQ/aegis-pqsynq/pqsynq/scripts/refresh_pinned_vectors.sh` - Added fixture refresh automation
- `SynQ/aegis-pqsynq/pqsynq/scripts/run_wasm_runtime_smoke.sh` - Added wasm runtime execution smoke gate
- `SynQ/aegis-pqsynq/pqsynq/tests/comprehensive_tests.rs` - Added HQC mismatch/tamper misuse tests
- `SynQ/sdk/tests/integration.test.ts` - Added SDK JSON-RPC integration coverage
- `SynQ/sdk/scripts/run_integration_tests.sh` - Added SDK integration test entrypoint

## Purpose Achieved

`aegis-pqsynq` is integrated as the active PQ foundation for SynQ, including HQC wiring in VM/compiler, contract-level parser -> codegen -> VM acceptance tests, hardened KEM/signature error propagation paths (no panic-on-failure for shipped ML-KEM/HQC/ML-DSA/FN-DSA wrappers), deterministic replay-fixture gating, official NIST replay coverage, compliance artifact generation, explicit key lifecycle controls, and deterministic CLI bytecode verification. Remaining work is now mostly SDK/runtime consumption hardening rather than missing core PQ substrate capability.
