# SynQ User Manual

Version: 1.1 (Full Scope, Implementation-Aligned)
Last Updated: 2026-02-09
Audience: SynQ contract authors, compiler/VM integrators, and engineering auditors

---

## 1. Purpose and Positioning

This manual is the authoritative, implementation-aligned guide for writing and running SynQ smart contracts in the current repository state.

This document deliberately separates three things that are often mixed together in early-stage language projects:

1. The SynQ vision and language surface area
2. What the parser/grammar accepts today
3. What the compiler + VM execute correctly end to end today

That distinction matters. If you treat aspirational syntax as production-ready behavior, you will ship incorrect contracts. This manual is designed to prevent that failure mode.

---

## 2. What SynQ Is

SynQ is a Rust-based smart contract language and toolchain focused on post-quantum cryptography (PQC) as a first-class concern. The current stack is:

- Language frontend: `compiler` crate (grammar, parser, AST, codegen)
- Execution backend: `vm` crate (`QuantumVM`)
- Crypto substrate: `aegis-pqsynq/pqsynq` + `pqrust` wrappers
- CLI wrapper: `cli` crate (compile/run/verify)

Current PQ profile in active scope:

- ML-DSA (44/65/87)
- FN-DSA (512/1024)
- ML-KEM (512/768/1024)
- HQC-KEM (128/192/256)

Current profile intentionally de-scopes CMCE and SLH-DSA from practical SynQ contract usage due footprint and readiness constraints.

---

## 3. Architecture Overview

SynQ source flow:

1. Parse source using Pest grammar (`compiler/src/synq.pest`)
2. Build AST (`compiler/src/ast.rs`)
3. Generate QVM bytecode (`compiler/src/codegen.rs`)
4. Execute bytecode in QuantumVM (`vm/src/vm.rs`)

CLI flow:

- `compile` command:
  - reads SynQ source
  - emits `.synq` bytecode (or `.compiled.synq` when source already ends with `.synq`)
  - emits `.sol` Solidity translation
- `run` command:
  - loads bytecode into VM
  - executes until `HALT` or terminal `RETURN`

---

## 4. Reality Matrix: What Works vs What Is Partial

This section is the most important section in the manual.

### 4.1 End-to-End Working Path (Compile + Execute)

The following are validated in current integration tests:

- Contract parsing for basic contract/function structure
- Bytecode generation and VM execution for:
  - arithmetic primitives
  - ML-DSA verification opcode path
  - ML-KEM decapsulation opcode path
  - HQC decapsulation opcode paths (128/192/256)

Reference tests:

- `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/compiler/tests/integration_test.rs`
- `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/vm/tests/integration_test.rs`

### 4.2 Parsed but Semantically Partial

These constructs exist in grammar/AST but are not fully lowered/executed with production semantics yet:

- Annotation argument semantics are parsed into AST nodes for contracts/functions/constructors/events/state variables; advanced schema validation remains part of semantic-analysis roadmap.
- Rich state/storage semantics (beyond basic stack/memory operations)
- Loop lowering (`for_statement` currently not lowered in parser)
- Full mapping/array semantics used by advanced example contracts
- Comprehensive symbol/type semantic analysis

### 4.3 Treat as Experimental in Current Compiler/VM

The following should be considered unstable unless you validate behavior in your own bytecode-level tests:

- `if` branch lowering and complex conditional control flow
- `require` / `require_pqc` behavior in non-trivial blocks
- complex assignment targets (index/member assignments)
- advanced event-heavy application logic

Bluntly: do not assume Solidity-level maturity. Validate emitted behavior.

---

## 5. Environment Setup

## 5.1 Prerequisites

- Rust stable toolchain
- Cargo
- Git

Optional but recommended:

- `cargo-audit`
- WASI SDK 20.0 for full wasm checks

## 5.2 Workspace Root

Use this workspace root:

`/Users/devpup/Desktop/Synergy/synergy-components/synq-language`

## 5.3 Build and Baseline Verification

Run from workspace root:

```bash
cargo build --workspace
cargo test --workspace --all-targets
cargo clippy -p aegis-pqsynq --all-targets --all-features --no-deps -- -D warnings
```

For full PQ substrate validation:

```bash
bash aegis-pqsynq/pqsynq/scripts/run_tests.sh
```

---

## 6. Project Layout (High Value Directories)

- `compiler/` - grammar, AST, parser, bytecode codegen
- `vm/` - bytecode interpreter and opcode definitions
- `cli/` - command-line wrapper for compile/run/verify
- `aegis-pqsynq/pqsynq/` - PQ crypto facade used by SynQ
- `smart-contracts/` - fixture contracts used by integration tests
- `docs/` - language/runtime documentation
- `audits/` - audit procedure and templates

---

## 7. CLI Usage (Current, Not Aspirational)

## 7.1 Compile Contract

```bash
cargo run -p cli -- compile --path /absolute/path/to/contract.synq
```

What it does:

- Parses SynQ source
- Emits bytecode file:
  - `*.synq` for non-`.synq` source paths
  - `*.compiled.synq` when source already ends with `.synq` (prevents source overwrite)
- Emits Solidity file: same path with `.sol` extension

## 7.2 Run Bytecode

```bash
cargo run -p cli -- run --path /absolute/path/to/contract.compiled.synq
```

Success output ends with:

`Execution finished successfully`

## 7.3 Important Operational Notes

- CLI is intentionally minimal right now (`compile`, `run`, `verify`)
- `verify` enforces deterministic source->bytecode reproducibility and can optionally execute verified bytecode
- No deploy/estimate/trace command surface is implemented in this CLI yet
- Treat this as a compiler/VM harness, not a full blockchain developer console

## 7.4 Verify Deterministic Bytecode

```bash
cargo run -p cli -- verify \
  --source /absolute/path/to/contract.synq \
  --bytecode /absolute/path/to/contract.compiled.synq \
  --run
```

What it does:

- Recompiles source in-memory via the current compiler pipeline
- Compares generated bytecode against the provided `.synq` artifact
- Emits deterministic mismatch diagnostics (first difference + SHA-256 digests)
- Runs VM execution only after verification when `--run` is set

---

## 8. SynQ Language Syntax (Current Grammar)

This section reflects grammar at `compiler/src/synq.pest`.

## 8.1 File Skeleton

```synq
pragma synq ^1.0.0;

contract MyContract {
    owner: Address public;

    constructor(admin: Address) {
        owner = admin;
    }

    @public function ping() {
        revert("halt");
    }
}
```

## 8.2 Version Pragma

Supported comparator tokens:

- `^`
- `>=`
- `<=`
- `>`
- `<`
- `=`

Example:

```synq
pragma synq ^1.0.0;
```

## 8.3 Contracts, Structs, and Events

- `struct` definitions are parsed and represented in AST
- `contract` definitions carry state variables, constructors, functions, events
- Event syntax is available, but end-to-end event runtime semantics are still evolving

## 8.4 State Variables

Current grammar shape:

```synq
name: Type public;
name: Type;
```

Example:

```synq
owner: Address public;
counter: UInt256;
```

## 8.5 Functions

Function shape:

```synq
@public function do_work(arg1: UInt256, arg2: Bytes) -> Bool {
    return true;
}
```

Notes:

- Visibility in grammar is currently driven by `@public`
- Return type is optional
- Global function grammar exists, but contract functions are the normal path

## 8.6 Supported Primitive and PQ Types in Parser

Primitive:

- `Address`
- `UInt8`, `UInt32`, `UInt64`, `UInt128`, `UInt256`
- `Int8`, `Int32`, `Int64`, `Int128`, `Int256`
- `Bool`, `Bytes`, `String`

PQC-named types present in grammar/type parser:

- `MLDSAPublicKey`, `MLDSAKeyPair`, `MLDSASignature`
- `FNDSAPublicKey`, `FNDSAKeyPair`, `FNDSASignature`
- `MLKEMPublicKey`, `MLKEMKeyPair`, `MLKEMCiphertext`
- `SLHDSAPublicKey`, `SLHDSAKeyPair`, `SLHDSASignature`

Important:

- Presence in grammar/type parser does not guarantee full runtime semantics
- SLH-DSA verification opcode currently returns runtime error in active VM profile

## 8.7 Statements

Grammar includes:

- variable declarations
- assignment
- return
- require
- revert
- if/else
- for loops
- emit
- expression statements
- `require_pqc` block

Implementation caveats:

- `for` lowering is currently not implemented in parser-to-AST statement lowering
- complex control-flow should be treated as experimental unless tested at bytecode level

## 8.8 Expressions and Literals

Supported literal forms include:

- numbers: `123`
- strings: `"hello"`
- booleans: `true`, `false`
- address literal: `0x` + 40 hex chars
- bytes literal: `Bytes("deadbeef")`

Call expression shape:

```synq
function_name(arg1, arg2)
```

Current parser call handling is identifier-centric; keep call names simple and explicit.

---

## 9. Post-Quantum Intrinsics and Naming Conventions

This section is source-of-truth for how the current code generator recognizes PQ calls.

Recognition is string-prefix based in:

`/Users/devpup/Desktop/Synergy/synergy-components/synq-language/compiler/src/pqc_integration.rs`

### 9.1 Function Name Prefixes Recognized by Codegen

- `verify_mldsa...`
- `verify_fndsa...`
- `verify_slhdsa...`
- `mlkem_...`
- `hqckem_...`
- `mldsa_...`
- `fndsa_...`
- `slhdsa_...`

### 9.2 Emitted VM Opcode Mapping

- `verify_mldsa...` -> `MLDSAVerify` (`0x80`)
- `verify_fndsa...` -> `FNDSAVerify` (`0x82`)
- `verify_slhdsa...` -> `SLHDSAVerify` (`0x83`)
- `mlkem_...` -> `MLKEMKeyExchange` (`0x81`)
- `hqckem_...` with algorithm tags:
  - `hqckem128` -> `HQCKEM128KeyExchange` (`0x84`)
  - `hqckem192` -> `HQCKEM192KeyExchange` (`0x85`)
  - `hqckem256` -> `HQCKEM256KeyExchange` (`0x86`)

### 9.3 Runtime Algorithm Defaults in VM

- `MLDSAVerify` uses `Sign::mldsa65()`
- `FNDSAVerify` uses `Sign::fndsa512()`
- `MLKEMKeyExchange` uses `Kem::mlkem768()`
- HQC opcodes map to matching HQC variants
- `SLHDSAVerify` currently returns runtime crypto error in this build

### 9.4 Practical Rule

If your contract uses camelCase names like `verifyMLDSASignature`, do not assume automatic PQ opcode lowering. Use naming patterns that codegen currently recognizes, or validate emitted opcodes in tests.

---

## 10. Bytecode and VM Reference

## 10.1 Bytecode Header Layout (QVM)

Assembler emits a 15-byte header:

- magic: `0x51564D00` (`QVM\0`)
- version: `1`
- header length: `15`
- code length: `u32`
- data length: `u32`

Defined in:

- `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/vm/src/assembler.rs`
- `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/vm/src/vm.rs`

## 10.2 Core VM Value Types

Stack values:

- `I32`
- `I64`
- `Bytes`
- `Bool`

## 10.3 Current Opcode Set

Stack:

- `Push` `0x01`
- `Pop` `0x02`
- `Dup` `0x03`
- `Swap` `0x04`

Arithmetic:

- `Add` `0x10`
- `Sub` `0x11`
- `Mul` `0x12`
- `Div` `0x13`

Comparison:

- `Eq` `0x20`
- `Ne` `0x21`
- `Lt` `0x22`
- `Le` `0x23`
- `Gt` `0x24`
- `Ge` `0x25`

Control:

- `Jump` `0x30`
- `JumpIf` `0x31`
- `Call` `0x32`
- `Return` `0x33`

Memory:

- `Load` `0x40`
- `Store` `0x41`
- `LoadImm` `0x42`

PQC:

- `MLDSAVerify` `0x80`
- `MLKEMKeyExchange` `0x81`
- `FNDSAVerify` `0x82`
- `SLHDSAVerify` `0x83`
- `HQCKEM128KeyExchange` `0x84`
- `HQCKEM192KeyExchange` `0x85`
- `HQCKEM256KeyExchange` `0x86`

Utility:

- `Print` `0xF0`
- `Halt` `0xFF`

---

## 11. Gas Model in Current VM

The VM enforces both total gas and PQ-specific gas budgeting.

Default VM configuration:

- total gas: `10,000,000`
- max PQ gas per tx: `300,000`

PQC operations use dynamic model:

`gas = base_cost + data_cost + compute_cost`

Current runtime constants:

- ML-DSA verify: base `6000`, data multiplier `9`, compute `20000`
- FN-DSA verify: base `4000`, data multiplier `6`, compute `10000`
- ML-KEM decap: base `5000`, data multiplier `6`, compute `14000`
- HQC-128 decap: base `6500`, data multiplier `7`, compute `22000`
- HQC-192 decap: base `7000`, data multiplier `7`, compute `26000`
- HQC-256 decap: base `7500`, data multiplier `7`, compute `32000`

If PQ gas limit is exceeded, VM raises `OutOfGas`.

---

## 12. End-to-End Walkthrough (Recommended First Contract)

Create file: `/tmp/hqckem_demo.synq`

```synq
pragma synq ^1.0.0;

contract HQCDemo {
    function run() {
        hqckem_hqckem128_decapsulate(
            Bytes("001122"),
            Bytes("aabbcc")
        );
        revert("halt");
    }
}
```

Compile:

```bash
cargo run -p cli -- compile --path /tmp/hqckem_demo.synq
```

Run:

```bash
cargo run -p cli -- run --path /tmp/hqckem_demo.compiled.synq
```

Important:

- The example above is syntax-valid, but decapsulation will fail unless key/ciphertext bytes are valid for the selected algorithm.
- For realistic HQC decap fixtures, use the pattern in:
  - `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/smart-contracts/tests/hqckem128-decap-fixture.synq`

---

## 13. Testing and Validation Workflow

## 13.1 Fast Local Contract Pipeline

```bash
cargo run -p cli -- compile --path /absolute/path/to/contract.synq
cargo run -p cli -- run --path /absolute/path/to/contract.compiled.synq
```

## 13.2 Compiler + VM Integration Validation

```bash
cargo test -p compiler --test integration_test
cargo test -p quantumvm --test integration_test
```

## 13.3 PQ Substrate Validation

```bash
bash aegis-pqsynq/pqsynq/scripts/run_tests.sh
```

## 13.4 Compliance Evidence (NIST + Fixture Integrity)

```bash
bash aegis-pqsynq/pqsynq/scripts/generate_compliance_report.sh
```

Artifact:

- `/Users/devpup/Desktop/Synergy/synergy-components/current-focus2/Aegis-PQC/aegis-pqsynq/pqsynq/artifacts/pqsynq-compliance-report.md`

---

## 14. Known Limitations (Read Before Shipping Contracts)

1. Parser and grammar include constructs that are not fully lowered/executed yet.
2. `for` statements are currently not lowered in parser statement conversion.
3. Control-flow and `require` semantics require explicit bytecode-level validation before production use.
4. Many legacy example contracts in `docs/examples/` reflect broader language ambitions and may not compile or execute as-is against current compiler/runtime.
5. `SLHDSAVerify` opcode is currently disabled in active VM runtime profile.
6. CLI is intentionally minimal and does not provide deploy/trace/estimate workflows yet.

Practical engineering rule:

- If a contract matters, write a contract-specific integration test that compiles it, checks emitted opcodes, and executes expected/negative paths in VM.

---

## 15. Migration Guidance for Existing SynQ Documents

Several older docs use aspirational syntax or command surfaces. For implementation-critical decisions, prioritize source + tests over prose docs.

Canonical implementation files:

- `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/compiler/src/synq.pest`
- `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/compiler/src/parser.rs`
- `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/compiler/src/codegen.rs`
- `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/compiler/src/pqc_integration.rs`
- `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/vm/src/opcode.rs`
- `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/vm/src/vm.rs`

---

## 16. Quick Reference

## 16.1 Minimal Contract Skeleton

```synq
pragma synq ^1.0.0;

contract ContractName {
    stateVar: UInt256 public;

    constructor(seed: UInt256) {
        stateVar = seed;
    }

    @public function run(input: UInt256) -> UInt256 {
        let x: UInt256 = input;
        return x;
    }
}
```

## 16.2 Useful Commands

```bash
# Build
cargo build --workspace

# Compile one contract
cargo run -p cli -- compile --path /absolute/path/to/file.synq

# Run one bytecode artifact
cargo run -p cli -- run --path /absolute/path/to/file.compiled.synq

# Integration tests
cargo test -p compiler --test integration_test
cargo test -p quantumvm --test integration_test

# Full PQ validation + compliance artifact
bash aegis-pqsynq/pqsynq/scripts/run_tests.sh
bash aegis-pqsynq/pqsynq/scripts/generate_compliance_report.sh
```

---

## 17. Final Guidance

SynQ is promising, but it is still an engineering system under active buildout. The winning strategy is disciplined verification, not optimism:

- Keep contracts in the tested subset.
- Validate emitted opcodes for every security-critical contract path.
- Treat documentation as guidance, and source/tests as ground truth.

If you follow that discipline, you can build safely while the language matures.
