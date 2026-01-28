# aegis-pqsynq Integration Status

## ✅ Completed Integration

aegis-pqsynq has been **fully moved into SynQ** as an internal dependency:

### Structure
- **Location**: `SynQ/pqsynq/pqsynq/` (moved from `aegis-pqsynq/`)
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
- ✅ Added `pqsynq/pqsynq` to SynQ workspace members
- ✅ Added pqrust crates to workspace for dependency resolution
- ✅ Defined workspace.dependencies for all PQC crates

## Current Status

The integration is **structurally complete** and the SynQ workspace now builds and tests cleanly as a self-contained workspace for the core PQC subset used by the current VM/compiler:

- **ML-DSA** (mldsa44/65/87)
- **FN-DSA (Falcon)** (fndsa512/fndsa1024)
- **ML-KEM** (mlkem512/768/1024)

Additional algorithm families (SLH-DSA, HQC-KEM, CMCE) are present in the repo but are currently **not enabled in the default SynQ build** while their upstream crate integration is repaired.

## Next Steps

1. Re-enable and validate SLH-DSA, HQC-KEM, and CMCE end-to-end (pqsynq + VM opcodes + compiler intrinsics)
2. Expand end-to-end tests beyond smoke tests (contracts that actually execute meaningful bytecode paths)
3. Finalize compiler↔VM↔SDK interface contracts and gas model conformance

## Files Modified

- `SynQ/vm/Cargo.toml` - Added pqsynq dependency
- `SynQ/vm/src/vm.rs` - Replaced shims with pqsynq implementations
- `SynQ/compiler/Cargo.toml` - Added pqsynq dependency  
- `SynQ/compiler/src/pqc_integration.rs` - Full integration layer
- `SynQ/Cargo.toml` - Added pqsynq to workspace

## Purpose Achieved

✅ **aegis-pqsynq is now fully integrated into SynQ**, giving SynQ smart contracts native post-quantum cryptography capabilities built directly into the language and VM.
