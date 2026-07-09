# SynQ Canonical Completion TODO

> Canonical status note (2026-05-26): this backlog is superseded by
> `SynQ-Aegis-AIVM-Canonical-Tasks.md`. Keep this file only as a historical
> reference unless a future task explicitly reconciles it.

**Purpose:** This is the single canonical engineering backlog for taking SynQ from the current checkout to a complete smart contract language and runtime that can be tested on Synergy TESTNET.

**How this list was produced:** Status below is based on direct verification of the repo, build behavior, tests, source files, examples, and SDK. It does not rely on prior checklist claims.

## Verified Current State

### Verified Working

- `cargo metadata --no-deps --format-version 1` succeeds for the workspace.
- `cargo test --workspace --all-features --all-targets` passes after restoring `aegis-pqsynq` and adding the PQClean randombytes compatibility export needed by the bundled HQC backend.
- `compiler`, `vm`, and `cli` have a working baseline for simple parsing, semantic checks, bytecode generation, VM arithmetic/control flow, example compilation, bytecode verification, and CLI compile/run/verify.
- The bundled `pqrust` ML-KEM, ML-DSA, FN-DSA, and HQC-KEM crates compile and pass their own default tests.
- Restored `aegis-pqsynq/pqsynq` is now a workspace member and the real PQC-backed paths pass:
  - `cargo test -p quantumvm --features pqc-aegis`
  - `cargo test -p compiler --features pqc-aegis`
  - `cargo test -p pqsynq-no-std-smoke --features pqc-aegis`
  - `cargo run -p wasm-runtime-smoke --features pqc-aegis`
  - `cargo test -p aegis-pqsynq --all-features --all-targets`
- `sdk` has a working TypeScript verification path:
  - `npx tsc --noEmit` passes.
  - `npm run test:integration` passes.
  - RPC envelope, transaction submission envelope, balance/block calls, JSON-RPC error handling, and explicit unsupported-PQC-key errors are tested.
- `synergy-aivm/runtime/aivm-core` now builds and has a first tested execution adapter that accepts `synq-bytecode-v1` artifacts and executes them through the current SynQ VM path.
- `synergy-aivm/verifier/verifier-core` now has a valid Rust crate target and baseline attestation verification tests.
- `synergy-aivm/runtime/aivm-node` compiles, but it remains a placeholder node binary with no runtime server behavior.

### Verified Blocked

- Official NIST KAT replay files for `aegis-pqsynq` are not present in this checkout. The replay tests now skip missing official files by default and become strict when `PQSYNQ_REQUIRE_NIST_KATS=1`.
- macOS root volume temp space is critically low in the verified environment. Rust builds that need temp files may require `TMPDIR=/Volumes/xcode/tmp` until the root volume is cleaned.
- Synergy TESTNET runtime/RPC contract deployment methods have not yet been verified live.
- AIVM is not yet the production execution environment for TESTNET contracts. It now has a baseline SynQ handoff, but no deploy/call/state/account/storage/receipt lifecycle.

### Verified Incomplete

- SynQ is not yet a complete smart contract platform.
- The current language frontend can parse and compile useful examples, but it is not a frozen language spec with exhaustive conformance tests.
- The current `quantumvm` path is an interim SynQ bytecode executor. SynQ readiness depends on integrating that path into Synergy AIVM or replacing it with a native AIVM SynQ runtime.
- AIVM does not yet implement a full contract lifecycle model with deploy/call/state/account/storage/receipt semantics suitable for TESTNET execution.
- The CLI does not yet implement testnet deploy, test, estimate, trace, artifact packaging, or source verification workflows.
- The SDK does not yet implement real PQC key management, transaction signing, ABI/client generation, or Synergy TESTNET deployment flows.
- Generated Solidity is a compatibility artifact, not a production target with implemented PQC library bindings.

## Completion Definition

SynQ is complete only when all of these are true:

- The language spec is frozen for v1 and every documented syntax form has parser, semantic, codegen, and negative tests.
- A real PQC backend is restored or replaced, tested, and wired through compiler, VM, CLI, SDK, no-std smoke, and wasm smoke paths.
- Contract deploy/call/state behavior is deterministic and implemented in Synergy AIVM and the Synergy TESTNET RPC/transaction layer.
- Compiled SynQ artifacts execute in Synergy AIVM. A local `quantumvm` pass alone is not sufficient for completion.
- At least one canonical SynQ contract can be compiled, packaged, deployed to Synergy TESTNET, called, and verified from CLI and SDK.
- Security, gas, ABI, storage, and source-verification rules are documented and enforced by tests.
- CI runs the full default suite plus PQC-enabled suite when backend files are present.
- Public docs and examples describe only implemented behavior.

## P0: External Backend And Build Foundation

- [x] **BUILD-001: Make default workspace load without unavailable `aegis-pqsynq`.**
  - Complete.
  - Default workspace metadata and tests run.

- [x] **BUILD-002: Restore `aegis-pqsynq/pqsynq` from external SSD.**
  - Complete.
  - `aegis-pqsynq/pqsynq` is a workspace member.
  - `pqsynq = { package = "aegis-pqsynq", path = "../aegis-pqsynq/pqsynq", ... }` is restored in `vm`, `compiler`, `tooling/no-std-smoke`, and `tooling/wasm-runtime-smoke`.
  - Acceptance verified: `cargo test -p quantumvm --features pqc-aegis`, `cargo test -p compiler --features pqc-aegis`, no-std smoke, wasm smoke, and `cargo test -p aegis-pqsynq --all-features --all-targets` all pass.

- [ ] **BUILD-003: Decide final PQC backend ownership.**
  - Either keep `aegis-pqsynq` as the canonical facade or replace it with direct wrappers over bundled `pqrust` crates.
  - Acceptance: one documented backend abstraction, no duplicate crypto ownership, no dead dependency path.

- [ ] **BUILD-004: Add official NIST KAT artifact policy.**
  - Store or mount the official vector packs consistently.
  - Keep `PQSYNQ_REQUIRE_NIST_KATS=1` strict mode for release/compliance runs.
  - Acceptance: release profile fails when official vector packs are absent; local profile can skip with an explicit message.

- [ ] **BUILD-005: Add CI profiles.**
  - Default profile: no external PQC backend required.
  - PQC profile: real backend required.
  - AIVM profile: builds `synergy-aivm/runtime/aivm-core`, `synergy-aivm/runtime/aivm-node`, and `synergy-aivm/verifier/verifier-core`.
  - Acceptance: CI clearly distinguishes default pass, PQC-backend-required pass, and AIVM integration pass.

## P0: Language Frontend

- [ ] **LANG-001: Freeze SynQ v1 grammar.**
  - Decide which forms are in v1: contracts, constructors, functions, modifiers, annotations, structs, enums, arrays, mappings, events, imports, version pragmas, `require_pqc`, and gas decorators.
  - Acceptance: `specs/synq.dsl` and parser grammar match exactly.

- [ ] **LANG-002: Build parser conformance suite.**
  - Add positive and negative parser fixtures for every grammar feature.
  - Acceptance: every documented syntax form has tests with expected AST shape or expected parse error.

- [ ] **LANG-003: Add source spans to AST and diagnostics.**
  - Track file, line, column, and source range through parser and semantic analysis.
  - Acceptance: CLI errors point to exact source locations for parser and semantic failures.

- [ ] **LANG-004: Implement imports/modules or remove them from v1 docs.**
  - Current docs mention imports, but implementation is not complete.
  - Acceptance: imports resolve deterministically or docs clearly defer them.

- [ ] **LANG-005: Define SynQ ABI types.**
  - Freeze encoding for integers, bools, bytes, addresses, arrays, structs, mappings, events, return values, and PQC key/signature/ciphertext types.
  - Acceptance: ABI spec plus round-trip tests in Rust and TypeScript.

## P0: Semantic Analysis

- [ ] **SEM-001: Complete type checker.**
  - Cover state variables, locals, params, returns, structs, enums, mappings, arrays, member access, index access, calls, casts, literals, and assignment compatibility.
  - Acceptance: invalid programs cannot reach codegen.

- [ ] **SEM-002: Enforce function and contract lifecycle rules.**
  - Constructor constraints, public/private visibility, payable/value behavior if supported, event emission, and return-path correctness.
  - Acceptance: lifecycle misuse has deterministic diagnostics.

- [ ] **SEM-003: Enforce PQC security rules.**
  - `require_pqc` must consume real PQC verification outcomes.
  - Domain separation for messages must be explicit and checked.
  - Unsupported/de-scoped algorithms must fail semantically.
  - Acceptance: replay-prone or ambiguous PQC calls fail unless explicitly allowed.

- [ ] **SEM-004: Enforce gas annotations.**
  - Decide mandatory gas annotation rules for public functions and PQC calls.
  - Acceptance: gas declarations are checked against codegen/VM estimates.

## P0: Code Generation

- [ ] **CG-001: Introduce a formal SynQ IR.**
  - Current direct AST-to-bytecode path is too brittle for a complete language.
  - Acceptance: parser -> AST -> semantic model -> IR -> bytecode is documented and tested.

- [ ] **CG-002: Implement storage/member/index lowering.**
  - Struct fields, mapping reads/writes, array reads/writes, and state-variable persistence must lower to deterministic bytecode.
  - Acceptance: canonical token, multisig, DAO, NFT, escrow, and staking examples execute meaningful state transitions.

- [ ] **CG-003: Implement function dispatch and returns.**
  - ABI selector, argument decoding, return encoding, internal calls, and external calls.
  - Acceptance: contract call by method name/selector works through VM and SDK.

- [ ] **CG-004: Implement `require_pqc` lowering precisely.**
  - Multiple PQC checks, fallback branches, revert reasons, gas behavior, and result propagation.
  - Acceptance: positive and negative PQC contract fixtures pass with real backend.

- [ ] **CG-005: Decide Solidity backend fate.**
  - Either make Solidity output real and compilable with PQC library bindings, or mark it experimental and keep it out of testnet readiness.
  - Acceptance: no production docs imply unimplemented Solidity deployment support.

## P0: Synergy AIVM Execution Environment

- [x] **AIVM-001: Verify and repair baseline AIVM Rust crate health.**
  - Complete.
  - `runtime/aivm-core` now has valid module wiring and tests.
  - `verifier/verifier-core` now has a valid `lib.rs` target and baseline tests.
  - `runtime/aivm-node` compiles, but it still has no operational server behavior.

- [x] **AIVM-002: Add first SynQ artifact handoff into AIVM.**
  - Complete as an initial bridge.
  - `aivm-core::execution` accepts `synq-bytecode-v1` artifacts and executes them through the current SynQ VM dependency.
  - Acceptance verified by an AIVM unit test that runs SynQ bytecode and returns a receipt.

- [ ] **AIVM-003: Decide final AIVM/SynQ runtime ownership.**
  - Decide whether AIVM embeds `quantumvm` as the canonical SynQ execution core or replaces it with native AIVM instruction execution.
  - Acceptance: one runtime ownership decision documented; no duplicate diverging opcode semantics.

- [ ] **AIVM-004: Freeze the AIVM contract artifact envelope.**
  - Define artifact kind, ABI, bytecode version, compiler version, source hash, metadata hash, gas profile, PQC backend profile, and compatibility rules.
  - Initial draft recorded in `docs/SynQ-AIVM-Execution-Integration.md`; it is not yet a stable wire format.
  - Acceptance: SynQ CLI, SDK, and AIVM agree on the same artifact schema byte-for-byte.

- [ ] **AIVM-005: Implement AIVM deploy/call/state/account lifecycle.**
  - Constructor execution, contract address derivation, call dispatch, persistent storage, sender/value/nonce/chain context, return data, revert data, logs, receipts.
  - Acceptance: AIVM lifecycle tests pass without relying on CLI-only simulation.

- [ ] **AIVM-006: Implement AIVM host ABI and precompile surface for SynQ.**
  - Storage, events, account context, block context, gas metering, PQC precompiles, AI model calls, transcript hooks, and deterministic error codes.
  - Acceptance: SynQ bytecode cannot access host behavior except through documented AIVM imports/precompiles.

- [ ] **AIVM-007: Implement deterministic AI execution calls from SynQ contracts.**
  - Define model IDs, inputs, outputs, transcript hashing, gas accounting, failure behavior, and verifier requirements for AI calls.
  - Acceptance: a SynQ contract calls an AIVM AI function in a deterministic local test with transcript verification.

- [ ] **AIVM-008: Integrate AIVM verifier and transcript flow.**
  - Move from placeholder attestation checks to canonical execution transcripts and real provider/TEE verification policy.
  - Acceptance: contract execution receipts include deterministic transcript commitments that verifier-core can validate.

- [ ] **AIVM-009: Wire AIVM into Synergy TESTNET node/runtime.**
  - Identify the node process that should own AIVM execution and the RPC methods that submit/query contract execution.
  - Acceptance: TESTNET node accepts a SynQ/AIVM artifact, executes it, and exposes a receipt through RPC.

## P0: SynQ VM And Runtime

- [x] **VM-001: Reject truncated bytecode headers without panic.**
  - Done in this session.

- [ ] **VM-002: Formalize deploy/call lifecycle.**
  - Define deployment artifact format, constructor execution, contract address derivation, call entry, call exit, return data, revert data, and logs in terms of AIVM-owned execution.
  - Acceptance: deploy/call lifecycle tests pass through AIVM, with `quantumvm` used only if it is the approved embedded SynQ executor.

- [ ] **VM-003: Implement persistent state storage.**
  - State variables, mappings, arrays, and structs must persist across calls.
  - Acceptance: state transition tests pass for canonical contracts.

- [ ] **VM-004: Implement account/address model.**
  - Define sender, contract address, value, nonce, chain ID, and address encoding.
  - Acceptance: transaction context is visible to contracts and deterministic.

- [ ] **VM-005: Implement ABI decoder/encoder.**
  - VM must accept call data and emit return data/logs using the SynQ ABI.
  - Acceptance: Rust and SDK ABI tests agree byte-for-byte.

- [ ] **VM-006: Finalize gas model.**
  - Base gas, memory/storage gas, PQC gas, block/tx limits, refunds if any, and failure semantics.
  - Acceptance: gas estimates match VM execution within documented bounds.

- [ ] **VM-007: Wire real PQC precompiles.**
  - ML-DSA, FN-DSA, ML-KEM, HQC-KEM, and explicitly de-scoped algorithms.
  - Acceptance: AIVM executes real contract-level PQC success/failure fixtures through the SynQ runtime path.

- [ ] **VM-008: Define bytecode/artifact versioning.**
  - Magic, version, sections, metadata hash, ABI hash, source hash, and compatibility rules.
  - Acceptance: old/incompatible artifacts fail with clear errors.

## P0: Synergy TESTNET Integration

- [ ] **NET-001: Verify active Synergy TESTNET RPC contract surface.**
  - Identify the actual RPC Gateway and node/runtime methods for AIVM deploy, call, estimate, block, receipt, and source verification.
  - Acceptance: recorded live RPC method matrix with example request/response payloads.

- [ ] **NET-002: Define SynQ transaction payloads for TESTNET.**
  - Compile artifact payload, AIVM artifact envelope, constructor args, call data, gas, nonce, signer, chain ID, and receipts.
  - Acceptance: transaction spec implemented in Rust CLI and TypeScript SDK.

- [ ] **NET-003: Implement first live deploy path.**
  - Choose minimal canonical contract.
  - Compile, package, submit to RPC Gateway, wait for receipt, and verify stored bytecode/source hash.
  - Acceptance: one SynQ contract is deployed and called on Synergy TESTNET.

- [ ] **NET-004: Add TESTNET-safe configuration.**
  - No hardcoded production secrets.
  - Explicit network selection.
  - Chain/network IDs verified from live runtime before defaults are committed.
  - Acceptance: CLI and SDK can target TESTNET predictably.

- [ ] **NET-005: Add node/runtime compatibility tests.**
  - Validate artifact version, AIVM gas limits, RPC payload size, block inclusion, and failure receipts against the live TESTNET runtime.
  - Acceptance: compatibility suite can be run before each TESTNET release.

## P1: CLI

- [ ] **CLI-001: Produce complete build artifacts.**
  - Bytecode, ABI, AIVM artifact envelope, metadata, source hash, compiler version, backend profile, gas estimate, and optional Solidity artifact.
  - Acceptance: `qsc compile` writes deterministic artifact directory that AIVM can load directly.

- [ ] **CLI-002: Implement `qsc test`.**
  - Run contract scenario fixtures with pass/fail summaries.
  - Acceptance: examples and negative tests run from one command.

- [ ] **CLI-003: Implement `qsc deploy`.**
  - Network config, account/signer input, gas, nonce, artifact upload, receipt wait, and source hash verification.
  - Acceptance: deploys canonical SynQ/AIVM artifact to TESTNET.

- [ ] **CLI-004: Implement `qsc estimate`.**
  - Local VM estimate and optional remote RPC estimate.
  - Acceptance: estimates are comparable to actual execution gas.

- [ ] **CLI-005: Implement `qsc trace`.**
  - Instruction trace, stack/memory/state diffs, gas usage, and PQC operation outcomes.
  - Acceptance: failing contract tests produce actionable traces.

- [ ] **CLI-006: Implement source/bytecode verification.**
  - Local deterministic verification already exists; extend it to artifact metadata and remote deployed contracts.
  - Acceptance: remote deployed source verification works.

## P1: SDK

- [x] **SDK-001: Add TypeScript compile gate.**
  - Done in this session.

- [x] **SDK-002: Add tested RPC error handling.**
  - Done in this session.

- [x] **SDK-003: Make unsupported PQC key APIs fail explicitly.**
  - Done in this session.

- [ ] **SDK-004: Implement real PQC key provider.**
  - Wire ML-DSA/FN-DSA/ML-KEM/HQC through a real backend, not placeholders.
  - Acceptance: SDK can generate keys, sign/verify, encapsulate/decapsulate, and match Rust vectors.

- [ ] **SDK-005: Implement transaction signing.**
  - SynQ/Synergy transaction format, chain ID, nonce, gas, payload hash, and PQC signature.
  - Acceptance: SDK-signed transaction is accepted by TESTNET RPC.

- [ ] **SDK-006: Generate contract clients from ABI.**
  - Type-safe deploy/call wrappers, event decoding, receipt parsing.
  - Acceptance: generated client can deploy and call canonical contract.

- [ ] **SDK-007: Add wallet integration surface.**
  - Define how wallet-held keys sign SynQ deployments and calls.
  - Acceptance: wallet and SDK produce identical transaction payloads.

## P1: Examples And Contract Test Suite

- [ ] **SC-001: Define canonical minimum TESTNET contract.**
  - Small enough for first live deploy, but includes state, call, event, and one PQC-gated path.
  - Acceptance: this contract is the first TESTNET deployment target.

- [ ] **SC-002: Turn documented examples into executable scenarios.**
  - Token, multisig, DAO, NFT, escrow, staking.
  - Acceptance: each example has compile, deploy, call, state assertion, and failure scenario.

- [ ] **SC-003: Add negative security fixtures.**
  - Bad signatures, wrong domains, wrong keys, replayed messages, bad gas, unsupported algorithms, malformed ABI.
  - Acceptance: each failure has an expected deterministic error.

- [ ] **SC-004: Add bytecode snapshots.**
  - Deterministic outputs for canonical contracts.
  - Acceptance: compiler changes cannot alter bytecode silently.

## P1: Security And Audit

- [ ] **SEC-001: Refresh threat model against implemented runtime.**
  - Include compiler, AIVM runtime, SynQ bytecode executor, CLI, SDK, TESTNET RPC, key lifecycle, source verification, AI execution, transcript verification, and deployment workflow.
  - Acceptance: threat model matches current code.

- [ ] **SEC-002: Enforce domain separation.**
  - All signatures must include contract, function, chain/network, nonce, and intent domain.
  - Acceptance: replay-style negative fixtures fail.

- [ ] **SEC-003: Add fuzzing.**
  - Parser fuzzing, bytecode loader fuzzing, ABI decoder fuzzing, VM execution fuzzing.
  - Acceptance: fuzz targets run in CI/nightly profile.

- [ ] **SEC-004: Add secret handling policy and tests.**
  - No private key logging, deterministic redaction, zeroization where applicable.
  - Acceptance: SDK/CLI logs cannot expose key material.

- [ ] **SEC-005: Independent audit readiness package.**
  - Spec, architecture, threat model, examples, vectors, known limitations, and reproducible test commands.
  - Acceptance: external reviewer can reproduce all evidence.

## P2: Docs, Release, And Governance

- [ ] **DOC-001: Remove unsupported behavior from public docs.**
  - Current README mentions commands and features that are not fully implemented.
  - Acceptance: docs distinguish implemented, experimental, and planned behavior.

- [ ] **DOC-002: Write TESTNET deployment guide.**
  - Install, compile, configure network, deploy, call, verify, troubleshoot.
  - Acceptance: guide works against live TESTNET.

- [ ] **DOC-003: Write language reference from implemented grammar.**
  - No aspirational syntax in the v1 reference.
  - Acceptance: every example in the reference is tested.

- [ ] **REL-001: Version compiler, ABI, bytecode, and SDK together.**
  - Acceptance: release artifacts include compatibility matrix.

- [ ] **REL-002: Package CLI and SDK.**
  - CLI install path, npm package or internal package, reproducible checksums.
  - Acceptance: clean machine can install and run tests.

- [ ] **REL-003: Add release gate.**
  - Default tests, PQC backend tests, SDK compile/tests, examples, docs link check, security fixtures, TESTNET smoke.
  - Acceptance: release cannot ship without all required gates.

## Immediate Next Execution Order

1. Decide whether AIVM embeds `quantumvm` as the SynQ executor or replaces it with native AIVM SynQ execution.
2. Freeze the SynQ/AIVM artifact envelope and ABI/call-data schema.
3. Implement AIVM deploy/call/state/account lifecycle for the minimal canonical contract.
4. Pick the minimal canonical TESTNET contract.
5. Implement CLI artifact packaging, deploy, test, estimate, and trace around AIVM receipts.
6. Wire SDK signing, ABI clients, and deploy/call support.
7. Verify active TESTNET RPC/node AIVM methods and run one live SynQ deployment plus receipt check.
8. Add CI profiles for default, PQC strict, AIVM integration, SDK, and TESTNET smoke gates.
9. Expand examples, negative tests, security gates, and docs until all completion criteria are satisfied.
