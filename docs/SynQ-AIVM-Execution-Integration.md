# SynQ AIVM Execution Integration

This document records the current SynQ-to-AIVM handoff. It is intentionally narrow: it describes what exists now and what must be finalized before SynQ contracts can be considered TESTNET-ready.

## Current Verified Handoff

- SynQ compiles to `quantumvm` bytecode with the `QVM\0` header.
- `synergy-aivm/runtime/aivm-core` exposes an initial `aivm_core::execution` module.
- The initial AIVM execution adapter accepts `synq-bytecode-v1` artifacts and executes them through the current SynQ `quantumvm` crate.
- AIVM returns an `ExecutionReceipt` with status, gas used, PQC gas used, return data, logs, and error text.
- AIVM `ExecutionRequest` now carries an explicit `ExecutionContext` with chain ID, network ID, block height/timestamp, transaction hash, caller, contract address, gas limit, PQ-Gas limit, and security policy reference.
- AIVM `ExecutionReceipt` now includes a structured error code and exposes a deterministic `canonical_hash()` over versioned ordered receipt fields.
- AIVM now exposes a deterministic contract-key state overlay with explicit commit and rollback behavior plus an ordered state-root hash.
- AIVM now meters ordinary gas and PQ-Gas in separate lanes. `ExecutionContext.admission_pq_gas_used` carries the inner verification cost into SynQ execution receipts.
- AIVM now validates SynQ manifests before `synq-bytecode-v1` execution:
  manifest presence, artifact format, bytecode version, bytecode hash, ABI hash
  when ABI JSON is supplied, chain `1264`, `synergy-testnet` /
  `synergy-testnet-v2` aliasing, and ML-DSA-65 policy.
- AIVM also accepts `wasm-module-v1` artifacts for baseline WASM module loading.
- WASM execution rejects host imports before instantiation. Wall-clock, randomness, network, filesystem, and host-specific behavior must enter through no implicit host APIs.

## Non-Negotiable Completion Rule

SynQ is not complete when bytecode only runs in local compiler or `quantumvm` tests. SynQ is complete only when compiled SynQ artifacts execute through Synergy AIVM and can be deployed, called, traced, and verified through Synergy TESTNET.

## Artifact Envelope Draft

The current Rust draft is:

```text
ContractArtifact {
  format: "synq-bytecode-v1" | "wasm-module-v1",
  bytes: Vec<u8>,
  abi_json: Option<String>,
  manifest_json: Option<String>,
  compiler_version: Option<String>,
  source_hash: Option<String>
}

ExecutionContext {
  chain_id: u64,
  network_id: String,
  block_height: u64,
  block_timestamp_unix: u64,
  tx_hash: [u8; 32],
  caller: Vec<u8>,
  contract_address: Vec<u8>,
  gas_limit: u64,
  pq_gas_limit: u64,
  security_policy: AivmSecurityPolicyRef
}
```

Before TESTNET use, this must become a stable serialized schema with:

- artifact format and version
- SynQ ABI version
- bytecode version
- compiler version
- source hash
- ABI hash
- metadata hash
- PQC backend profile
- gas profile
- supported AIVM host ABI version
- chain/network compatibility rules

## AIVM Runtime Decisions Still Open

- Decide whether `quantumvm` remains the embedded SynQ execution core inside AIVM or whether AIVM implements native SynQ opcode execution.
- Define persistent storage and contract address derivation.
- Define call data, return data, revert data, and event log encoding.
- Define AIVM host calls for storage, events, account context, block context, PQC precompiles, and AI model invocation.
- Define deterministic transcript commitments for contract execution and AI execution.
- Wire the AIVM receipt shape into Synergy TESTNET RPC.

## Manifest Validation Status

`runtime/aivm-core/src/execution.rs::validate_synq_artifact` is the current
direct local gate for SynQ artifact envelopes. The gate is intentionally
deterministic: it reads only the provided request bytes and manifest JSON, and
does not access wall-clock time, filesystem, network, or randomness.

The current test suite covers:

- checked-in Counter bytecode/ABI/manifest validates;
- missing manifest fails closed;
- tampered bytecode hash fails with `AivmErrorCode::Bytecode`;
- tampered ABI hash fails with `AivmErrorCode::Abi`;
- wrong chain and unrelated network fail with `AivmErrorCode::Manifest`;
- `synergy-testnet-v2` normalizes to `synergy-testnet` for chain 1264;
- unsupported artifact format fails with `AivmErrorCode::Manifest`;
- unsupported signature policy fails with `AivmErrorCode::Verification`.

## First TESTNET Smoke Target

The first live smoke should use one minimal SynQ contract that includes:

- one persistent state variable
- one read call
- one write call
- one event
- one PQC-gated path
- deterministic source hash and artifact hash

The smoke is successful only when CLI and SDK can compile the same source, submit the same AIVM artifact envelope, read the same receipt, and verify the deployed source hash.
