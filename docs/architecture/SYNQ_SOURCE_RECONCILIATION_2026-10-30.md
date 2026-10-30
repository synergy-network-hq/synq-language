# SynQ source reconciliation — 2026-10-30

Status: source discovery, GitHub retrieval, canonical SynQ selection, and a local AIVM stateful port completed; production caller and live user deployment remain incomplete.

## Source owner and runtime invariant

The canonical SynQ language directory on the SSD is `04-SynQ/synq-language`. Its GitHub repository is `synergy-network-hq/synq-language`. SynQ emits artifacts for AIVM. AIVM is the only Synergy Network VM and has responsibilities beyond smart contracts. Historical `QuantumVM` and `SynQ/vm` code must be reconciled into that AIVM execution path, not treated as another production VM.

This note records current source state. It does not authorize removal of historical repositories, signed Genesis material, validator state, or deployed releases.

## Remote retrieval

- `synq-language`: all four remote branch refs fetched into the existing SSD directory. `main` is `01125b6470c6b49c586b7a28fe054a62e1c913c6`. The worktree matches it except the pre-existing local `AGENTS.md` user-path repair. The recovery branch differs from main only in `AGENTS.md`; the Aegis-pin branch changes only the exempt submodule pin. The archive branch retains older history.
- `synergy-aivm`: all six remote branch refs fetched into the existing `05-AIVM/synergy-aivm` directory via a temporary Git bundle because `other-mini` lacks GitHub credentials for this private repository. `main` is `9debf33e11fdf4c0c20c3a07d4794e3b37751138`. Existing local differences are `AGENTS.md`, `runtime/aivm-core/Cargo.lock`, and two untracked verifier files; none was replaced.
- `synq-internal`: private repository retrieved for inventory. Its `main` tree `dc8d77ea7dc4df997dc1abad1a42dd828848b29e` contains specifications, audit material, and reports, not a compiler or runtime source tree. Do not import its private records into the public SynQ repository without an access decision.

## SSD source comparison

File counts below exclude generated output, AppleDouble metadata, `pqrust`, and the Aegis PQSynQ module.

| Source compared with canonical SynQ | Shared identical | Shared different | Source-only |
| --- | ---: | ---: | ---: |
| `01-Core-Protocol/testnet-v3/runtime/synq-language` | 117 | 20 | 1 |
| `01-Core-Protocol/networks/testnet-v3/runtime/synq-language` | 118 | 19 | 1 |
| `01-Core-Protocol/.chain1266-faucet-hotfix/runtime/synq-language` | 118 | 19 | 1 |

The sole source-only path in those three copies is a Chain 1264 integration specification, which is historical to a retired network. The differing compiler artifact, CLI, policy, and specification files are older than the canonical SynQ main content. Do not bulk overwrite canonical source with these copies.

Each historical `runtime/SynQ` tree has 17 paths absent from canonical SynQ. They include a standalone HTTP session server with its own `QuantumVM`, old PQC shims, project-tree documents, and scripts. The server uses ephemeral signing and host-local sessions; it is not an acceptable on-chain deployment path. Keep those trees as historical references until consumer migration and provenance review are complete.

`05-AIVM/synergy-aivm` has 110 non-generated local files. Its historical snapshot has no source-only files. The testnet and single-authority AIVM copies each have four source-only files, including `runtime/aivm-core/src/stateful_synq.rs`, and eight files different from the `05-AIVM` tree. Those stateful AIVM changes were imported on local AIVM branch `reconcile/stateful-aivm-2026-10-30`. The remote AIVM fix branches were reviewed as migration inputs; their Chain 1264 compatibility path remains historical. The imported Chain 1266 implementation requires independent source review before release.

## Current execution boundary

The single-authority runtime contains a stateful AIVM deploy/call implementation, persisted contract state, receipts, and signed SynQ admission. The universal `node-platform` implementation currently records a code hash for deploy and executes a bounded legacy instruction VM for calls; its host rejects persistent writes and events. It therefore does not yet provide equivalent stateful contracts. Its SynQ and AIVM crates need a versioned dependency on the sole AIVM execution implementation and an integration test that verifies deploy, state change, view, replay, and receipt consistency.

The AIVM Cargo manifest now resolves the SynQ compiler, legacy bytecode engine, and PQSynQ package from the canonical `04-SynQ/synq-language` checkout. A fresh AIVM checkout requires this SSD sibling layout and a populated Aegis submodule until a portable pinned dependency and CI checkout are provided. The local Core Protocol `synq-language` symlink now resolves to `04-SynQ/synq-language`. Cargo metadata for the single-authority runtime resolves its compiler, `quantumvm`, and PQSynQ package paths through that symlink. This is a local source dependency cutover, not a release or validator deployment.

## Validation performed

- Canonical SynQ compiler artifact test: 6 passed.
- Independent AIVM checkout after stateful import: 42 library tests, 9 governance authorization tests, and 3 canonical address tests passed; `cargo fmt --check` passed. The Wasmtime version was restored to the AIVM main line; the Cargo lock changed to resolve new SynQ/compiler/PQSynQ dependencies.
- Single-authority AIVM stateful mapping write/read/revert test: passed.
- Single-authority node Counter deploy/increment/get/replay test: passed after replacing a silent-skip obsolete fixture path and decoding the current stateful AIVM return format.
- Single-authority RPC receipt replay and compacted receipt-index tests: passed against the same canonical Counter artifact.
- Read-only inspection of the active single-authority validator found `synergy-validator-node` v20.0.0, binary SHA-256 `49369281473320ec27031cb2031360ff21824620bfff4885a8943d95a27d5d57`, active since 2026-10-23 07:32:38 UTC. Its local RPC reports Chain 1266, incarnation 5, and Genesis hash `2272fe2b48c6e2019b27223f61ba8d8a82b58656fe9daab50c81f224a301ca74`.
- The live `synergy_getCode` returned nonempty SynQ IR v2 bytecode for the Genesis Identity address. A read-only `synergy_call` to Staking's `totalVotingPower()` succeeded through `stateful-synq-ir-v2`, returning 0 with matching pre/post state roots. Both direct Aegis and encrypted submission RPC routes rejected empty envelopes at validation; no signed submission was made.
- The live binary hash does not match the available September SSD release build. Its exact source revision has not yet been established, so source-level test success is not release equivalence. No signed user transaction was submitted to the live chain. No validator release or public RPC deployment was changed or verified.

## Remaining completion gates

1. Review the local AIVM stateful port, resolve its dependency lock and portable cross-repository checkout/CI design, then publish a versioned AIVM release. Preserve broader AIVM capabilities and authored history.
2. Replace the standalone `QuantumVM` production owner and the universal node's code-hash-only SynQ path with AIVM-backed deploy/call/state/receipt handling. Remove editable duplicate VM sources only after callers and tests use the canonical dependency.
3. Provide and verify a funded, durable user signing path into the public encrypted transaction submission route. The current `synergy-node tx sign-aegis` command generates a fresh transaction key for each invocation, and `tx submit-aegis` explicitly refuses plaintext submission after ETDAG activation. The local authority RPC accepts the route name but an empty-envelope rejection does not prove a valid user deployment can reach finality. Verify fee/nonce policy, contract artifact retrieval, static calls, and deterministic replay on the single-authority release candidate.
4. Build and deploy the approved release, then verify a signed deploy, state-changing call, view, receipt, and replay on Chain 1266 before claiming live usability.
