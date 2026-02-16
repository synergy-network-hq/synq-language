# SynQ Project Status and Execution Backlog

**Document Purpose:** Single source of truth for SynQ delivery status, active blockers, and completion tasks required to reach production-ready smart contract authoring and execution.

**Last Audited:** 2026-02-16

**Audit Scope:**

- Workspace crates: `compiler`, `vm`, `cli`, `aegis-pqsynq/pqsynq`, `sdk`
- Documentation and specs under `docs/`
- Build/test execution under default features and `--all-targets`
- Feature-complete claims validation (`aegis-pqsynq --all-features`)

**Build Baseline (Verified):**

- `cargo test --workspace --all-targets` -> **PASS**
- `cargo test -p aegis-pqsynq --all-features --all-targets` -> **PASS**
- `cargo test -p cli --test integration_test --locked` -> **PASS** (compile/run + deterministic verify + tamper rejection + `.synq` source overwrite guard)
- `cargo test -p compiler --test semantic_test --locked` -> **PASS** (symbol/return semantic validation suite)
- `cargo clippy -p aegis-pqsynq --all-targets --all-features --no-deps -- -D warnings` -> **PASS**
- `bash aegis-pqsynq/pqsynq/scripts/run_tests.sh` -> **PASS**
- `cargo audit` -> **PASS** (no CVEs or in-scope unresolved advisory findings)
- `cargo run -p cli -- compile --path docs/examples/{1..6}-*.synq` -> **PASS** (all six canonical examples now compile and emit `.compiled.synq` + `.sol`)

**Current PQ Foundation Status (2026-02-09 Update):**

- Module naming standardized to `aegis-pqsynq` (package) while preserving `pqsynq` import compatibility.
- `full` feature now maps to the implemented algorithm set (`mlkem`, `mldsa`, `fndsa`, `hqckem`, `std`).
- Detached signature helpers are implemented and exercised in full-mode tests.
- Contextual signing/verification is implemented for ML-DSA and explicitly returns `NotImplemented` for FN-DSA.
- Legacy “full” tests/CI/scripts that referenced unsupported CMCE/SLH APIs have been replaced with implementation-aligned coverage.
- HQC support is now implemented in `aegis-pqsynq`; CMCE/SLH remain de-scoped for current smart-contract profile.
- Signature FFI wrappers (ML-DSA/FN-DSA) now propagate structured errors instead of assert/unchecked failure paths.
- Deterministic replay fixtures are now present with integrity manifest checks for all currently shipped SynQ profile algorithms.
- Official NIST replay harness now validates shipped algorithms (ML-KEM/ML-DSA/FN-DSA strict replay, HQC compatibility replay path).
- Deterministic compliance evidence now emits `pqsynq-compliance-report.md` with source SHA-256 inventory + per-check logs.
- WASI toolchain bootstrap is now deterministic in both local and CI flows (`bootstrap_wasi_sdk.sh` + workflow cache/env wiring).
- Key lifecycle controls are now documented and implemented via `SecretBytes` + zeroization helpers in `pqsynq`.
- CLI deterministic verification is now available (`cli verify --source --bytecode [--run]`) and tested for both match and tamper mismatch behavior.
- CLI compile output now protects `.synq` source files from overwrite by emitting `.compiled.synq` when input already ends with `.synq`.
- Compiler parser compatibility now supports Solidity-style SynQ contract fixtures used in `docs/examples`:
  - Solidity-style state declarations (`Type public name;`)
  - Solidity-style params/events (`Type name`, `Type indexed name`)
  - mapping types (`mapping(K => V)`), tuple return signatures (`-> (T1, T2, ...)`)
  - contract-local `struct`/`enum` declarations
  - typed local declarations (`UInt256 x = ...;`) and richer assignment lvalues
  - `msg.value`, object/array literals, and `else if` chains
- Compiler semantic analysis baseline is now integrated into CLI compile flow:
  - undefined symbol detection for assignments/references
  - duplicate state variable and duplicate parameter rejection
  - return arity enforcement (void vs non-void)
  - constructor/function/block scope symbol handling baseline
  - semantic regression suite + CLI failure-path test coverage

---

## 1. Executive Status (No Spin)

SynQ is **not** fully built out for production smart-contract development yet. The repository contains a promising core (parser grammar, AST skeleton, bytecode assembler, VM execution loop, and integrated ML-DSA/FN-DSA/ML-KEM primitives), but there is a material gap between:

1. documented capabilities,
2. test coverage/validation guarantees,
3. and implemented language/runtime behavior.

### Current Delivery Position

- **Core cryptographic substrate (ML-KEM/ML-DSA/FN-DSA + optional HQC):** operational.
- **Smart contract language front-end:** partially implemented; parser compatibility has improved substantially, and semantic analysis baseline now rejects common unsafe constructs.
- **Code generation:** functionally incomplete for non-trivial contracts.
- **VM execution model:** usable for small opcode programs and direct PQC opcodes, but not yet aligned with a robust contract lifecycle model.
- **CLI and SDK:** early-stage scaffolding; not yet a complete developer workflow.
- **Production readiness:** not achieved.

### Realistic Completion Scorecard (Evidence-Based)

- Language Spec to Implementation Parity: **40%**
- Compiler Front-End Completeness: **45%**
- Bytecode/IR and Codegen Correctness: **30%**
- VM Contract Runtime Completeness: **45%**
- PQC Coverage vs Claimed Surface: **40%**
- CLI Developer Workflow: **25%**
- SDK Readiness: **20%**
- Testing and Quality Gates: **40%**
- Security/Hardening/Operationalization: **25%**
- Release Readiness: **10%**

**Overall program completeness estimate:** **~40-45%** toward production-ready SynQ smart contract platform.

---

## 2. What Is Implemented Today (Confirmed)

### 2.1 Workspace and Crate Topology

- Rust workspace with active members for:
  - `vm`
  - `compiler`
  - `cli`
  - `aegis-pqsynq/pqsynq`
  - internal PQ crates for ML-KEM/ML-DSA/FN-DSA
- Workspace keeps CMCE and SLH-DSA out of current smart-contract profile; HQC is available via `aegis-pqsynq` feature gating.

### 2.2 VM and Bytecode

- Stack VM with opcode enum and interpreter loop.
- Instruction support includes:
  - stack ops, arithmetic, comparisons
  - basic control flow (`Jump`, `JumpIf`, `Call`, `Return`)
  - memory load/store primitives
  - PQC opcodes:
    - ML-DSA verify
    - FN-DSA verify
    - ML-KEM decapsulation
    - SLH-DSA opcode currently returns explicit runtime error (not enabled)
- Gas meter exists with separate PQC gas accounting.
- Assembler emits a QVM header and code stream.

### 2.3 Compiler

- Pest grammar exists for a broad SynQ syntax surface.
- AST definitions exist for contracts, structs, statements, expressions, and PQC types.
- Parser now covers contract/function/constructor/state/event annotations and lowers `for` loops into executable AST statements.
- Parser supports Solidity-style contract fixtures used across canonical examples.
- Semantic analyzer now executes before code generation in CLI compile path.
- Code generator emits VM opcodes with working variable storage/load lowering and baseline `for` loop execution lowering.
- Solidity generator exists.

### 2.4 PQSynQ

- Implemented and wired algorithms in default path:
  - ML-KEM: 512/768/1024
  - ML-DSA: 44/65/87
  - FN-DSA: 512/1024
- Implemented optional KEM family:
  - HQC-KEM: 128/192/256 (feature: `hqckem`)
- Basic tests pass for current enabled set.
- Benchmarks compile and run in `--all-targets` path (after alignment with enabled algorithms).

### 2.5 CLI

- Current commands:
  - `compile --path <file>`
  - `run --path <bytecode>`
  - `verify --source <file> --bytecode <artifact> [--run]`
- Compile command outputs:
  - `.synq` bytecode artifact for non-`.synq` source names
  - `.compiled.synq` bytecode artifact when source already uses `.synq`
  - `.sol` generated Solidity artifact

### 2.6 Existing Test Evidence

- `cargo test --workspace` passes.
- `cargo test --workspace --all-targets` passes.
- Integration tests are present but minimal:
  - parser smoke test
  - VM arithmetic + PQC operations (ML-DSA, ML-KEM, HQC-KEM variants)
  - CLI compile/run smoke test
  - CLI deterministic verify success + tamper rejection checks
  - semantic analysis positive/negative regression tests
  - CLI compile gate that validates all six documented example contracts

---

## 3. Critical Gaps and Risks (Truth Table)

## 3.1 Compiler Front-End Gaps

- Function-body parsing is materially improved and now handles variable declarations, assignments, `if`, `for`, `require`, `revert`, `emit`, `return`, and `require_pqc` blocks.
- Negative grammar diagnostics and full source-span diagnostics are still incomplete.
- Symbol resolution baseline exists, but full type semantics are still incomplete for robust contract compilation.
- Error reporting lacks comprehensive source-location diagnostics.

**Risk:** Contracts that look valid may compile into behaviorally incorrect or no-op bytecode.

### 3.2 Codegen/VM Semantic Misalignment

- Codegen still has placeholder behavior for member/index semantics, but baseline variable storage/load and loop lowering are now implemented.
- Control-flow and call/return semantics need stricter contract execution model consistency.
- Several AST branches are accepted but not fully lowered with correct runtime behavior.

**Risk:** Semantic drift between source intent and runtime execution.

### 3.3 Documentation vs Implementation Drift

- Language and VM docs claim broader support than what current runtime and feature flags actually provide.
- Opcode tables and semantics in documentation are not fully synchronized with implementation details.

**Risk:** Developer trust erosion and onboarding confusion.

### 3.4 PQC Surface Mismatch

- Default build intentionally enables subset algorithms.
- `aegis-pqsynq --all-features` now passes for the implemented algorithm set (ML-KEM/ML-DSA/FN-DSA); additional families remain out of scope until implemented.

**Risk:** “Full support” messaging is currently non-actionable and blocks roadmap confidence.

### 3.5 SDK Incompleteness

- TypeScript SDK has placeholders and unimplemented Kyber module functions.
- No robust package build/test/release pipeline visible in current repository state.

**Risk:** Even if compiler/VM improves, application developers cannot reliably integrate.

### 3.6 Smart Contract Project Scaffolding Gap

- `smart-contracts/examples` and `smart-contracts/tests` now include HQC-focused parser/codegen/VM fixtures.
- Fixture breadth is still narrow (currently PQC-centric) and needs expansion to core language semantics beyond PQ flows.

**Risk:** Regression safety remains incomplete until non-PQC canonical fixtures are added and CI-gated.

---

## 4. Blockers Required Before “SynQ Usable for Real Contract Development”

A practical minimum bar (MVP-Production Candidate) is:

1. Parser fully materializes function bodies and statements into AST.
2. Compiler semantic baseline is active; complete full type system and PQC-specific semantic rules.
3. Codegen produces deterministic and behaviorally correct bytecode for:
   - variables
   - assignments
   - control flow
   - returns
   - function calls
4. VM execution semantics pass contract-level integration tests.
5. PQC verification and KEM flows are validated through contract-level tests, not just opcode unit tests.
6. CLI provides deploy/test/estimate/trace workflows.
7. SDK supports key management, tx creation/signing/submission, and contract interaction with tests.
8. Docs are synchronized with implementation reality.

---

## 5. Detailed Execution Backlog (Master To-Do)

This section is intentionally exhaustive. Every task includes acceptance criteria so progress can be objectively measured.

## 5.1 Program Management and Delivery Controls

### PM-001: Establish formal release milestones

- Priority: P0
- Status: DONE (2026-02-09)
- Deliverables:
  - Milestone map (M0/M1/M2/M3)
  - Exit criteria per milestone
  - Owners and target dates
- Acceptance Criteria:
  - Document checked into `docs/` and reviewed
  - Each technical workstream mapped to milestone
  - Evidence: `docs/management/SynQ-Release-Milestones.md`

### PM-002: Build a status cadence process

- Priority: P0
- Status: DONE (2026-02-09)
- Deliverables:
  - Weekly status template
  - Risk register template
  - Decision log template
- Acceptance Criteria:
  - Templates committed and used for at least one update cycle
  - Evidence: `docs/management/Weekly-Status-Template.md`

### PM-003: Define and enforce Definition of Done (DoD)

- Priority: P0
- Status: DONE (2026-02-09)
- Deliverables:
  - DoD for compiler, VM, CLI, SDK, docs
- Acceptance Criteria:
  - CI gates mapped to DoD checks
  - Evidence: `docs/management/Definition-of-Done.md`

---

## 5.2 Language and Parser Workstream

### LANG-001: Implement full annotation parsing

- Priority: P0
- Status: DONE (2026-02-09)
- Scope:
  - Contract/function/constructor/state variable/event annotations
  - Annotation args with expression parsing
- Acceptance Criteria:
  - Parser no longer has annotation TODO placeholders
  - Unit tests cover nested/invalid annotation scenarios

### LANG-002: Parse function bodies into complete statement AST

- Priority: P0
- Status: IN PROGRESS (2026-02-09)
- Scope:
  - variable declarations
  - assignments
  - if/else
  - for loops
  - require/revert
  - require_pqc blocks
  - emit/return/expression statements
- Acceptance Criteria:
  - AST snapshots for all statement classes
  - Negative tests for malformed syntax

### LANG-003: Parse expressions with precedence-correct AST lowering

- Priority: P0
- Status: IN PROGRESS (2026-02-09)
- Scope:
  - binary precedence
  - unary ops
  - ternary
  - calls/member/index chains
- Acceptance Criteria:
  - golden AST tests for precedence edge cases

### LANG-004: Implement source span tracking in parser

- Priority: P1
- Status: TODO
- Scope:
  - attach line/column ranges to AST nodes
- Acceptance Criteria:
  - parser and compiler errors include exact source location

### LANG-005: Version pragma parser/runtime alignment

- Priority: P1
- Status: TODO
- Scope:
  - parse full comparator sets
  - validate against compiler version
- Acceptance Criteria:
  - compile blocks unsupported pragma versions

### LANG-006: Language conformance test suite

- Priority: P1
- Status: TODO
- Scope:
  - valid/invalid syntax corpus
- Acceptance Criteria:
  - conformance suite integrated in CI

---

## 5.3 Semantic Analysis Workstream

### SEM-001: Implement symbol table and scope resolution

- Priority: P0
- Status: IN PROGRESS (2026-02-16)
- Scope:
  - contract scope
  - function scope
  - block scope
  - shadowing rules
- Acceptance Criteria:
  - undefined symbol and duplicate declaration diagnostics
- Progress Update:
  - implemented baseline contract/function/block scope symbol resolution
  - undefined symbol checks active in CLI compile path
  - duplicate state variable and duplicate parameter diagnostics implemented

### SEM-002: Type checker implementation

- Priority: P0
- Status: IN PROGRESS (2026-02-16)
- Scope:
  - primitive types
  - PQC types
  - assignment compatibility
  - call arg type matching
  - return type enforcement
- Acceptance Criteria:
  - comprehensive type mismatch test suite
- Progress Update:
  - baseline type inference/checks now active for:
    - local variable initialization compatibility
    - assignment compatibility (with parser-lvalue fallback guardrails)
    - return type compatibility
    - boolean condition enforcement for `require` / `if` / ternary
    - numeric bound checks for `for` loops
  - parser call-argument preservation was hardened so semantic call checks use real arity even when expressions are parser-fallback identifiers
  - new semantic regression tests added for initializer mismatch, require-condition mismatch, and call-argument type mismatch

### SEM-003: Control-flow semantic checks

- Priority: P0
- Status: IN PROGRESS (2026-02-16)
- Scope:
  - guaranteed return analysis
  - unreachable code warnings/errors
- Acceptance Criteria:
  - compile-time errors for missing returns in non-void functions
- Progress Update:
  - return-value arity enforcement implemented (void vs non-void return rules)
  - flow-sensitive non-void path analysis implemented:
    - compile-time error when a non-void function may reach end-of-body without returning on all paths
    - compile-time unreachable-statement diagnostics after terminal control-flow (`return` / `revert` or fully terminal `if/else`)
  - tuple return expressions are now accepted in expression grammar, so control-flow return-path analysis applies to tuple-returning functions as well

### SEM-004: PQC semantic rules

- Priority: P0
- Status: IN PROGRESS (2026-02-16)
- Scope:
  - enforce correct key/signature pairing
  - enforce algorithm variant compatibility
- Acceptance Criteria:
  - invalid pairing rejected at compile time
- Progress Update:
  - compiler now enforces built-in PQC signature/call contracts for:
    - `verifyMLDSASignature` / `verify_mldsa...`
    - `verifyFNDSASignature` / `verify_fndsa...`
    - `mlkem...decapsulate`
    - `hqckem...decapsulate`
  - de-scoped SLH built-ins are now compile-time rejected in semantic analysis (fail-closed before bytecode generation)
  - codegen PQC builtin detection now supports camelCase + snake_case aliases through normalized dispatch helpers

### SEM-005: Gas annotation semantic validation

- Priority: P1
- Status: TODO
- Scope:
  - annotation schema validation
  - mandatory annotations policy
- Acceptance Criteria:
  - deterministic validation errors with source spans

---

## 5.4 Code Generation Workstream

### CG-001: Implement variable addressing and storage model

- Priority: P0
- Status: IN PROGRESS (2026-02-09)
- Scope:
  - local variable slots
  - function frame layout
- Acceptance Criteria:
  - assignments/reads behave correctly in integration tests

### CG-002: Implement correct function call lowering

- Priority: P0
- Status: TODO
- Scope:
  - function label resolution
  - argument passing convention
  - return handling
- Acceptance Criteria:
  - nested function call tests pass

### CG-003: Complete expression lowering

- Priority: P0
- Status: TODO
- Scope:
  - identifier/member/index semantics
  - logical operators and short-circuiting
  - modulo/shift operations
- Acceptance Criteria:
  - expression semantics match evaluator test oracle

### CG-004: Implement robust control-flow lowering

- Priority: P0
- Status: IN PROGRESS (2026-02-09)
- Scope:
  - branch/loop patching correctness
  - structured label management
- Acceptance Criteria:
  - no invalid jump targets in generated bytecode

### CG-005: Deterministic bytecode generation

- Priority: P1
- Status: TODO
- Scope:
  - stable ordering and deterministic build output
- Acceptance Criteria:
  - identical source generates byte-identical output

### CG-006: Bytecode metadata section

- Priority: P1
- Status: TODO
- Scope:
  - source map/debug metadata
  - contract ABI metadata
- Acceptance Criteria:
  - metadata consumed by trace/debug tooling

---

## 5.5 VM Runtime Workstream

### VM-001: Formalize call/return contract execution model

- Priority: P0
- Status: TODO
- Scope:
  - entrypoint conventions
  - call stack rules
  - return semantics for top-level execution
- Acceptance Criteria:
  - contract entry/exit behavior documented and tested

### VM-002: Harden memory model

- Priority: P0
- Status: TODO
- Scope:
  - local memory and persistent storage separation
  - strict bounds and access checks
- Acceptance Criteria:
  - out-of-bounds and invalid access tests

### VM-003: Implement missing opcodes or remove unsupported syntax

- Priority: P0
- Status: TODO
- Scope:
  - close parser/codegen/VM opcode support gap
- Acceptance Criteria:
  - no parser-accepted construct lowers to unsupported opcode path

### VM-004: Gas accounting conformance harness

- Priority: P0
- Status: TODO
- Scope:
  - per-op gas table
  - PQC gas formulas
  - deterministic gas tests
- Acceptance Criteria:
  - gas snapshots stable across CI runs

### VM-005: VM error taxonomy and diagnostics

- Priority: P1
- Status: TODO
- Scope:
  - structured error codes
  - stack traces / PC tracing
- Acceptance Criteria:
  - debug mode output enables reproducible triage

### VM-006: SLH-DSA support decision

- Priority: P1
- Status: TODO
- Scope:
  - either implement SLH-DSA runtime support or remove language claims
- Acceptance Criteria:
  - docs + runtime behavior are consistent

---

## 5.6 PQC Engine Alignment Workstream

### PQC-001: Decide official algorithm support matrix by release tier

- Priority: P0
- Status: TODO
- Tiers:
  - Tier 1 (required for next release)
  - Tier 2 (experimental)
- Acceptance Criteria:
  - published support matrix in docs

### PQC-002: Align `pqsynq` tests with implemented APIs

- Priority: P0
- Status: DONE (2026-02-09)
- Scope:
  - remove or gate tests for non-implemented methods
  - fix failing `--features full` path
- Acceptance Criteria:
  - `cargo test -p aegis-pqsynq --all-features --all-targets` passes

### PQC-003: Add known-answer tests for enabled algorithms

- Priority: P1
- Status: DONE (2026-02-09)
- Scope:
  - ML-KEM/ML-DSA/FN-DSA KAT ingestion and validation
- Acceptance Criteria:
  - KAT tests run in CI and pass deterministically

### PQC-004: Side-channel and constant-time audit preparation

- Priority: P1
- Status: TODO
- Scope:
  - threat model
  - audit checklist
- Acceptance Criteria:
  - external audit package ready

### PQC-005: Batch verification strategy

- Priority: P2
- Status: TODO
- Scope:
  - evaluate need and design for batch verify opcodes
- Acceptance Criteria:
  - architecture decision record committed

---

## 5.7 CLI and Tooling Workstream

### CLI-001: Rename/standardize binary (`qsc`) and command UX

- Priority: P0
- Status: TODO
- Scope:
  - command naming consistency
  - ergonomic help output
- Acceptance Criteria:
  - `qsc --help` reflects official workflow

### CLI-002: Implement `qsc test`

- Priority: P0
- Status: TODO
- Scope:
  - contract test execution harness
- Acceptance Criteria:
  - sample contracts run with pass/fail summaries

### CLI-003: Implement `qsc deploy`

- Priority: P0
- Status: TODO
- Scope:
  - deployment artifact packaging
  - network/RPC integration points
- Acceptance Criteria:
  - can deploy canonical sample contract to target environment

### CLI-004: Implement `qsc estimate` and `qsc trace`

- Priority: P1
- Status: TODO
- Scope:
  - static and dynamic gas estimation
  - execution trace output
- Acceptance Criteria:
  - trace includes opcode sequence and gas deltas

### CLI-005: Implement `qsc lint` and `qsc format`

- Priority: P2
- Status: TODO
- Scope:
  - formatting/lint rules for SynQ source
- Acceptance Criteria:
  - deterministic formatting and lint diagnostics

---

## 5.8 SDK Workstream (TypeScript)

### SDK-001: Implement Kyber module functions currently stubbed

- Priority: P0
- Status: TODO
- Scope:
  - keypair/encapsulate/decapsulate
- Acceptance Criteria:
  - no runtime `Not implemented` in SDK crypto path

### SDK-002: Define package boundaries and build tooling

- Priority: P0
- Status: IN PROGRESS (2026-02-09)
- Scope:
  - package.json/tsconfig/build targets
  - lint/test pipelines
- Acceptance Criteria:
  - SDK installs and builds as standalone package

### SDK-003: Implement transaction signing and verification utilities

- Priority: P0
- Status: TODO
- Scope:
  - deterministic tx serialization
  - signature envelopes
- Acceptance Criteria:
  - roundtrip sign/verify tests pass

### SDK-004: Contract interaction abstraction hardening

- Priority: P1
- Status: TODO
- Scope:
  - ABI encoding/decoding
  - typed call wrappers
- Acceptance Criteria:
  - generated client can invoke sample contracts end-to-end

### SDK-005: Address format and key encoding policy

- Priority: P1
- Status: TODO
- Scope:
  - canonical address encoding (Bech32m or policy-defined alternative)
- Acceptance Criteria:
  - validation + conversion helpers with tests

### SDK-006: Browser and Node compatibility matrix

- Priority: P2
- Status: TODO
- Scope:
  - polyfills
  - WebCrypto compatibility
- Acceptance Criteria:
  - matrix documented with tested environments

---

## 5.9 Smart Contract Fixtures and Acceptance Tests

### SC-001: Populate `smart-contracts/examples` with canonical contracts

- Priority: P0
- Status: TODO
- Scope:
  - minimal token
  - multisig
  - governance
  - escrow
- Acceptance Criteria:
  - examples compile and execute in automated tests

### SC-002: Populate `smart-contracts/tests` with scenario suites

- Priority: P0
- Status: TODO
- Scope:
  - positive/negative behavior tests
  - gas assertions
  - failure mode assertions
- Acceptance Criteria:
  - CI runs contract scenario suite on each PR

### SC-003: Golden bytecode fixtures

- Priority: P1
- Status: TODO
- Scope:
  - expected bytecode snapshots for canonical contracts
- Acceptance Criteria:
  - bytecode diff detection prevents silent regressions

### SC-004: PQC-specific behavior scenarios

- Priority: P1
- Status: TODO
- Scope:
  - valid/invalid signatures
  - key mismatch
  - malformed payloads
- Acceptance Criteria:
  - clear pass/fail matrix for all supported algorithms

---

## 5.10 Documentation and Spec Reconciliation

### DOC-001: Create implementation truth table (spec vs code)

- Priority: P0
- Status: TODO
- Scope:
  - every language feature and opcode mapped to implemented status
- Acceptance Criteria:
  - table checked into docs and reviewed at each milestone

### DOC-002: Correct VM opcode/spec mismatches

- Priority: P0
- Status: TODO
- Scope:
  - opcode values, names, stack effects, gas notes
- Acceptance Criteria:
  - docs reflect exact implementation or clearly mark planned items

### DOC-003: Correct language spec feature claims

- Priority: P0
- Status: TODO
- Scope:
  - remove unsupported claims or mark as roadmap
- Acceptance Criteria:
  - no feature listed as available unless tests prove it

### DOC-004: Publish a “Getting Started (Real)” guide

- Priority: P1
- Status: TODO
- Scope:
  - exact working commands for current supported flow
- Acceptance Criteria:
  - fresh environment validation passes by following guide verbatim

### DOC-005: Add developer migration guide and pitfalls

- Priority: P2
- Status: TODO
- Scope:
  - Solidity-to-SynQ mapping
- Acceptance Criteria:
  - examples cover common mistakes and equivalent patterns

---

## 5.11 QA, CI, and Release Engineering

### QA-001: Expand CI matrix

- Priority: P0
- Status: TODO
- Scope:
  - default + all-targets + feature permutations
- Acceptance Criteria:
  - CI fails on any unsupported/rotting feature path

### QA-002: Enforce lint and formatting gates

- Priority: P0
- Status: TODO
- Scope:
  - `cargo fmt --check`
  - `cargo clippy -D warnings`
- Acceptance Criteria:
  - no warning debt in release branch

### QA-003: Add coverage metrics and quality thresholds

- Priority: P1
- Status: TODO
- Scope:
  - per-crate coverage report
- Acceptance Criteria:
  - threshold policy enforced in CI

### QA-004: Regression test pack for release candidates

- Priority: P1
- Status: TODO
- Scope:
  - compiler + VM + CLI + SDK smoke + contract suites
- Acceptance Criteria:
  - one-command RC validation pipeline

### QA-005: Performance baseline and drift monitoring

- Priority: P2
- Status: TODO
- Scope:
  - benchmark snapshots
- Acceptance Criteria:
  - alert on significant perf regressions

---

## 5.12 Security and Reliability

### SEC-001: Threat model for SynQ contract lifecycle

- Priority: P0
- Status: TODO
- Scope:
  - compile-time, runtime, cryptographic, tooling attack surfaces
- Acceptance Criteria:
  - threat model doc approved and referenced by backlog tasks

### SEC-002: VM safety hardening checklist

- Priority: P0
- Status: TODO
- Scope:
  - stack bounds
  - memory bounds
  - malformed bytecode handling
- Acceptance Criteria:
  - dedicated adversarial test suite passes

### SEC-003: Reproducible build strategy

- Priority: P1
- Status: TODO
- Scope:
  - toolchain pinning
  - deterministic artifacts
- Acceptance Criteria:
  - build reproducibility verified across environments

### SEC-004: External audit readiness package

- Priority: P1
- Status: TODO
- Scope:
  - architecture docs
  - known limitations
  - test evidence
- Acceptance Criteria:
  - package is complete and internally reviewed

### SEC-005: Incident response and rollback plan

- Priority: P2
- Status: TODO
- Scope:
  - severity matrix
  - response runbooks
- Acceptance Criteria:
  - tabletop simulation completed

---

## 6. Priority Roadmap (Suggested Critical Path)

## Milestone M0: “Truth and Stability” (Immediate)

Objective: stop drift and establish reliable baseline.

- Complete:
  - DOC-001, DOC-002, DOC-003
  - LANG-001 (annotation parsing) baseline
  - PQC-002 (`--features full` decision/fix)
  - QA-001 and QA-002
- Exit Criteria:
  - docs and code are aligned on supported features
  - CI catches unsupported permutations

## Milestone M1: “Compiler and VM Functional Core”

Objective: reliable compilation/execution of non-trivial contracts.

- Complete:
  - LANG-002, LANG-003
  - SEM-001, SEM-002, SEM-003
  - CG-001, CG-002, CG-003, CG-004
  - VM-001, VM-002, VM-003
- Exit Criteria:
  - canonical contracts compile and execute with expected results

## Milestone M2: “Developer Workflow Usability”

Objective: contract developers can build/test/run without internal knowledge.

- Complete:
  - CLI-001, CLI-002, CLI-003
  - SDK-001, SDK-002, SDK-003
  - SC-001, SC-002
  - DOC-004
- Exit Criteria:
  - clean-room developer can follow docs and complete end-to-end flow

## Milestone M3: “Production Candidate”

Objective: release hardening, security posture, quality controls.

- Complete:
  - VM-004, VM-005
  - PQC-003, PQC-004
  - QA-003, QA-004
  - SEC-001, SEC-002, SEC-004
- Exit Criteria:
  - release checklist signed off with quantified risk acceptance

---

## 7. Immediate Next 10 Execution Tasks (Recommended Sprint Start)

1. Finalize support matrix (enabled algorithms and feature flags) and publish it.
2. Build contract-level HQC fixtures that validate parser/codegen/VM end-to-end, now that opcode/intrinsic wiring is in place. (Completed 2026-02-09)
3. Implement annotation parsing end-to-end with tests. (Completed 2026-02-09)
4. Implement function body statement parsing with AST tests. (In progress: parser now supports executable body statements + `for` loop lowering tests.)
5. Implement semantic symbol table and type checks for assignment/calls/returns.
   - In progress: baseline symbol/type/PQC semantic gates are active in CLI compile path; deeper flow analysis and variant-level PQC enforcement remain open.
6. Build variable slot model and deterministic function call lowering in codegen. (In progress: variable storage/load + loop slot updates landed; function-call lowering remains open.)
7. Add contract fixture suite in `smart-contracts/examples` and `smart-contracts/tests`. (Completed 2026-02-09)
8. Implement CLI `qsc test` baseline runner.
9. Replace SDK Kyber stubs and add SDK test harness. (In progress: SDK test harness is done; Kyber crypto implementation is still stubbed.)
10. Publish doc reconciliation update so external messaging matches actual functionality. (Completed 2026-02-09)

---

## 8. Progress Tracking Template (Use for Weekly Updates)

Use this block for each weekly update in this file or a separate status log.

```markdown
### Week of YYYY-MM-DD

- Overall Health: Green | Yellow | Red
- Completed:
  - [ ] <Task IDs>
- In Progress:
  - [ ] <Task IDs>
- Blocked:
  - [ ] <Task IDs + blocker>
- Scope Changes:
  - <approved change requests>
- Risks Added:
  - <new risks>
- Risks Closed:
  - <closed risks>
- Evidence:
  - <test runs, benchmark reports, docs links>
```

---

## 9. Non-Negotiable Quality Gates for “SynQ Ready” Claim

Do not claim SynQ is “fully completed” until all gates below are true:

- Compiler parses and semantically validates real-world contracts reliably.
- Bytecode generation is deterministic and semantically correct for supported language features.
- VM executes supported constructs with validated gas accounting.
- PQC support matrix is explicit, tested, and documented.
- CLI and SDK provide practical end-to-end developer workflow.
- Smart contract fixtures and regression suites are operational in CI.
- Documentation is synchronized with implementation and test evidence.

If these gates are not met, the accurate status is: **“in active buildout, not production-complete.”**
