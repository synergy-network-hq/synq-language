# Codex Implementation Report

Date: 2026-05-26

## Summary

Completed a proof-backed pass across SynQ, `aegis-pqsynq`, Synergy-AIVM, and
Project Rosetta. The work focused on safe foundations: current verification
evidence, refreshed audits, canonical payload hashing/tests, Aegis deploy/call
transaction verification proof, and a Rosetta chain-1264 receipt-verifier
interface with deterministic mock tests. A follow-up pass completed old status
file redirects, expanded the Aegis negative/replay verifier matrix, added
ML-DSA-65 deploy vector fixtures, and added a chain-1264 deploy/call verifier
example. Later addenda record `/Users/devpup/Desktop/Testnet-Beta` source
inspection, Rosetta receipt mapping, and the comparative `aegis-pqsynq` versus
`aegis-pqvm` architecture decision.

No commits or pushes were performed.

## Completed Register Items

Exact completed IDs currently marked in
`docs/SynQ-Aegis-AIVM-Canonical-Tasks.md`:

- `CANON-001` through `CANON-004`
- `ARCH-001`, `ARCH-002`, `ARCH-003`, `ARCH-005`, `ARCH-006`,
  `ARCH-007`
- `AUDIT-001` through `AUDIT-007`
- `SPEC-001` through `SPEC-011`
- `AEGIS-001` through `AEGIS-018`, `AEGIS-020`, `AEGIS-BLOCKER-001`
- `AIVM-006`, `AIVM-007`, `AIVM-012`
- `NET-001` through `NET-004`, `NET-006`
- `ROSETTA-001` through `ROSETTA-012`,
  `ROSETTA-015`
- `DEMO-005`, `DEMO-010`

## Partial Or Blocked Items

- `ARCH-004`: left unchecked because the quantumvm/native AIVM executor
  ownership decision is still not proven by tests.
- `SPEC-012`: left unchecked because full bytecode/ABI/manifest/address/receipt
  vector coverage was not added.
- `AEGIS-019`, `AEGIS-021`, `AEGIS-022`: left unchecked because fuzzing,
  expanded CI, and full PQ substrate gate claims were not completed in this pass.
- `NET-005`, `NET-007`, and `NET-008` remain unchecked; `NET-007` still lacks
  a running local RPC listener with captured request/response examples.
- `AIVM-008`, `AIVM-011`, and the remaining AIVM execution/runtime items are
  partial or blocked as noted in the canonical register.
- `ROSETTA-010` is now complete as a definition item; node-level
  `aegis-pqsynq` runtime admission wiring is complete for SynQ carriers, while
  public RPC/indexer exposure remains tracked under the open NET/AIVM items.
- `LANG-*`, remaining `AIVM-*`, remaining `NET-*`, `DX-*`, `SEC-*`, `QA-*`,
  `REL-*`, `DEMO-*`, and
  `FUTURE-*` items remain unchecked unless individually listed above as
  complete.
- Git status: `project-rosetta` exposes local Git metadata and has the Rosetta
  files changed on `main...origin/main`; `synq-language`, `aegis-pqsynq`, and `synergy-aivm`
  still do not expose `.git` metadata in this filesystem snapshot. User-provided
  upstream repos are `synergy-network-hq/synq-language`,
  `synergy-network-hq/aegis-pqsynq`, `synergy-network-hq/synergy-aivm`, and
  `synergy-network-hq/project-rosetta`.

Exact unchecked IDs at finish:

```text
ARCH-001 ARCH-002 ARCH-004 SPEC-012
AEGIS-019 AEGIS-021 AEGIS-022
LANG-001 LANG-002 LANG-003 LANG-004 LANG-005 LANG-006 LANG-007 LANG-008 LANG-009 LANG-010 LANG-011 LANG-012 LANG-013 LANG-014 LANG-015 LANG-016 LANG-017 LANG-018 LANG-019 LANG-020 LANG-021 LANG-022
AIVM-001 AIVM-002 AIVM-003 AIVM-004 AIVM-005 AIVM-006 AIVM-007 AIVM-008 AIVM-009 AIVM-010 AIVM-011 AIVM-012 AIVM-013 AIVM-014 AIVM-015
NET-003 NET-004 NET-005 NET-006 NET-007 NET-008
ROSETTA-013 ROSETTA-014
DX-001 DX-002 DX-003 DX-004 DX-005 DX-006
SEC-001 SEC-002 SEC-003 SEC-004 SEC-005 SEC-006
QA-001 QA-002 QA-003
REL-001 REL-002 REL-003
DEMO-001 DEMO-002 DEMO-003 DEMO-004 DEMO-005 DEMO-006 DEMO-007 DEMO-008 DEMO-009 DEMO-010 DEMO-011 DEMO-012 DEMO-013 DEMO-014
FUTURE-001 FUTURE-002 FUTURE-003 FUTURE-004 FUTURE-005
```

## Files Changed

- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/proof/SynQ-Aegis-AIVM-Proof-Log-2026-05-26.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/reports/Codex-Implementation-Report-2026-05-26.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-Aegis-AIVM-Canonical-Tasks.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/checklist.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/INTEGRATION_STATUS.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/PQSynQ-Completion-Execution-Plan.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-Project-Status-and-Execution-Backlog.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-Security-Audit-Manual.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/management/Definition-of-Done.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/management/SynQ-Release-Milestones.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/management/Weekly-Status-Template.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/audits/synq-language-audit.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/audits/aegis-pqsynq-audit.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/audits/synergy-aivm-audit.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/audits/project-rosetta-audit.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/architecture/repo-dependency-map.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/architecture/aegis-pqsynq-vs-aegis-pqvm-integration.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/integration/synq-aegis-aivm-contract.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/lib.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/serialization.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/signature.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/examples/generate_synq_verifier_vectors.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/examples/synq_deploy_call_verifier.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/canonical_payload_tests.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/synq_verifier_tests.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/synq_deploy_vector_tests.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_valid_deploy.json`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_invalid_signature.json`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_wrong_chain.json`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_wrong_domain.json`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/src/lib.rs`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/src/serialization.rs`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/src/signature.rs`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/examples/generate_synq_verifier_vectors.rs`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/examples/synq_deploy_call_verifier.rs`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/tests/canonical_payload_tests.rs`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/tests/synq_verifier_tests.rs`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/tests/synq_deploy_vector_tests.rs`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_valid_deploy.json`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_invalid_signature.json`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_wrong_chain.json`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_wrong_domain.json`
- `/Volumes/xcode/Synergy-Network-Projects/project-rosetta/crates/rosetta-verification/src/lib.rs`
- `/Volumes/xcode/Synergy-Network-Projects/project-rosetta/crates/rosetta-verification/src/synergy.rs`
- `/Volumes/xcode/Synergy-Network-Projects/project-rosetta/docs/synergy-testnet-integration.md`

## Commands Run

- `find ... -name .git` across the four audited paths and `/Volumes/xcode`
- `git -C <repo> status --short --branch` for all four audited paths
- `cargo fmt --check` in SynQ and Rosetta
- `cargo check --workspace --all-features --all-targets --locked` in SynQ
- `cargo test --workspace --all-features --all-targets --locked` in SynQ
- `cargo test -p aegis-pqsynq --all-features --all-targets --locked` in SynQ
- `cargo test -p aegis-pqsynq --test canonical_payload_tests --locked`
- `cargo test -p aegis-pqsynq --test synq_verifier_tests --locked`
- `cargo test -p aegis-pqsynq --test synq_verifier_tests --test synq_deploy_vector_tests --locked`
- `cargo run -p aegis-pqsynq --example synq_deploy_call_verifier --locked`
- `rg -n "secret|private|sk|seed" aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_*.json || true`
- `rg -n "Canonical status|canonical register|SynQ-Aegis-AIVM-Canonical" ...`
- `npm run typecheck` in `synq-language/sdk`
- `npm run test:integration` in `synq-language/sdk`
- `cargo test` in AIVM `runtime/aivm-core`
- `cargo test` in AIVM `runtime/aivm-node`
- `cargo test` in AIVM `verifier/verifier-core`
- `cargo test --workspace` in Project Rosetta
- `cargo clippy --workspace --all-targets -- -D warnings` in Project Rosetta
- `corepack pnpm --filter @synergy/rosetta-sdk typecheck`
- `corepack pnpm --filter @synergy/rosetta-sdk test`
- `cargo test --manifest-path .../aegis-pqsynq/aegis-pqsynq/pqsynq/Cargo.toml --test canonical_payload_tests --locked`

## Test Status

Passed:

- SynQ Rust formatting, check, and full workspace tests.
- Embedded `aegis-pqsynq` full all-features/all-targets test gate.
- Aegis canonical payload tests: 3 passed.
- Aegis SynQ verifier tests: 17 passed.
- Aegis ML-DSA-65 deploy vector replay: 1 test passed across 4 JSON fixtures.
- Aegis chain-1264 deploy/call example: ran successfully and printed
  `deploy=verified` and `call=verified`.
- SynQ SDK typecheck and 4 integration tests.
- AIVM core: 3 tests passed.
- AIVM node: 0 tests, command passed.
- AIVM verifier-core: 2 tests passed.
- Project Rosetta Rust workspace tests: passed, including 5 Synergy receipt
  verifier tests.
- Project Rosetta clippy with `-D warnings`: passed.
- Project Rosetta TypeScript SDK typecheck and 5 tests: passed.

Failed or blocked:

- Direct standalone `aegis-pqsynq/aegis-pqsynq/pqsynq` manifest test failed
  before compilation because the crate inherits workspace dependencies but no
  workspace root is present in that local mirror.
- Git status failed for `synq-language`, `aegis-pqsynq`, and `synergy-aivm`
  because no local `.git` metadata was present under those exact filesystem
  paths. `project-rosetta` did expose Git metadata and was clean.
- Foundry/Solidity Rosetta tests were not run.
- Live TESTNET/RPC/validator behavior was not exercised.

## Git Status Summaries

- `/Volumes/xcode/Synergy-Network-Projects/synq-language`
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq`
- `/Volumes/xcode/Synergy-Network-Projects/synergy-aivm`

`git status --short --branch` returned:

```text
fatal: not a git repository (or any parent up to mount point /Volumes)
Stopping at filesystem boundary (GIT_DISCOVERY_ACROSS_FILESYSTEM not set).
```

Follow-up after user correction: a deeper `.git` search found no `.git` files
inside the first three paths. That means those local snapshots do not expose
checkout metadata here; it does not negate the user-provided fact that these
directories map to GitHub-backed projects.

For `/Volumes/xcode/Synergy-Network-Projects/project-rosetta`:

```text
/Volumes/xcode/Synergy-Network-Projects/project-rosetta
## main...origin/main
```

User-provided upstream repositories:

- `https://github.com/synergy-network-hq/synq-language`
- `https://github.com/synergy-network-hq/aegis-pqsynq`
- `https://github.com/synergy-network-hq/synergy-aivm`
- `https://github.com/synergy-network-hq/project-rosetta`

## Blockers

- Decide authoritative source between embedded `synq-language/aegis-pqsynq` and
  standalone `aegis-pqsynq/aegis-pqsynq`; the standalone mirror is not directly
  buildable without a workspace root.
- Active endpoint request/response smoke examples were not run for `NET-007`;
  the RPC/receipt shape is now source-confirmed from the local TESTNET code, and
  local qRPC ports 5640/5641 were not listening during this pass.
- Wire Synergy-AIVM deploy/call gates to `aegis-pqsynq`; current AIVM tests do
  not prove deploy/call security gate integration.
- Reattach or expose local Git checkout metadata for `synq-language`,
  `aegis-pqsynq`, and `synergy-aivm` before producing real branch, diff, or
  remote status summaries from those exact paths against the user-provided
  upstream GitHub repositories.

## Recommended Next Codex Prompt

Add payload-decoder fuzzing for `AEGIS-019`, then inspect CI and decide whether
to expand `AEGIS-021` with `cargo audit`, `cargo deny`, docs, and any TypeScript
binding gates. Keep the active embedded `synq-language/aegis-pqsynq/pqsynq`
crate and the standalone mirror in sync, and update the canonical register only
for cases proven by current commands.

## Addendum: Local TESTNET Source Correction

The correction to inspect `/Users/devpup/Desktop/Testnet-Beta` was completed.
The active node/RPC/runtime source is
`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`.

New source-backed artifact:

- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/integration/testnet-source-inventory-2026-05-26.md`

New or updated Rosetta files:

- `/Volumes/xcode/Synergy-Network-Projects/project-rosetta/crates/rosetta-verification/src/lib.rs`
- `/Volumes/xcode/Synergy-Network-Projects/project-rosetta/crates/rosetta-verification/src/synergy.rs`
- `/Volumes/xcode/Synergy-Network-Projects/project-rosetta/docs/synergy-testnet-integration.md`
- `/Users/devpup/Desktop/Testnet-Beta/project-rosetta/crates/rosetta-core/src/domain.rs`
- `/Users/devpup/Desktop/Testnet-Beta/project-rosetta/crates/rosetta-core/src/lib.rs`
- `/Users/devpup/Desktop/Testnet-Beta/project-rosetta/crates/rosetta-verification/src/lib.rs`
- `/Users/devpup/Desktop/Testnet-Beta/project-rosetta/crates/rosetta-verification/src/synergy.rs`
- `/Users/devpup/Desktop/Testnet-Beta/project-rosetta/docs/synergy-testnet-integration.md`
- `/Users/devpup/Desktop/Testnet-Beta/project-rosetta/sdk/typescript/src/route.ts`
- `/Users/devpup/Desktop/Testnet-Beta/project-rosetta/sdk/typescript/test/synergy-domain.test.ts`

New spec/register/proof updates:

- `/Volumes/xcode/Synergy-Network-Projects/synq-language/specs/synq-chain-1264-integration-spec.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-Aegis-AIVM-Canonical-Tasks.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/proof/SynQ-Aegis-AIVM-Proof-Log-2026-05-26.md`

### TESTNET Findings

- Chain constants are source-backed in
  `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/synergy_types.rs`:
  `chain_id = 1264`, `network_id = "synergy-testnet-v2"`.
- The current local RPC surface is `synergy_*`, not the future `synq_*` surface.
- Current receipt methods are `synergy_getReceipt`,
  `synergy_getTransactionReceipt`, and `synergy_getBlockReceipts`.
- Current transaction admission uses `transaction::Transaction` and
  `aegis_tx_tool::AegisTxSubmissionEnvelope` with `aegis-pqvm`.
- AIVM runtime scaffolding exists in `src/aivm/runtime.rs`, but AIVM deploy and
  execute RPC handlers are commented out as temporarily disabled.

### Additional Commands

- `cargo test -p rosetta-verification synergy::tests -- --nocapture` in
  `/Volumes/xcode/Synergy-Network-Projects/project-rosetta`: 9 passed.
- `cargo test -p rosetta-verification synergy::tests -- --nocapture` in
  `/Users/devpup/Desktop/Testnet-Beta/project-rosetta`: 9 passed.
- `cargo clippy -p rosetta-verification --all-targets -- -D warnings` in both
  Project Rosetta copies: passed.
- `corepack pnpm --filter @synergy/rosetta-sdk typecheck` in
  `/Users/devpup/Desktop/Testnet-Beta/project-rosetta`: passed.
- `corepack pnpm --filter @synergy/rosetta-sdk test` in
  `/Users/devpup/Desktop/Testnet-Beta/project-rosetta`: 5 passed.
- `cargo test -p synergy-testnet execution::tests -- --nocapture`: 3 passed.
- `cargo test -p synergy-testnet launch_ -- --nocapture`: 9 passed.
- `cargo test -p synergy-testnet aegis_tx_tool::tests -- --nocapture`: 4 passed.
- `cargo test -p synergy-testnet transaction::tests::admission_requires_testnet_context_and_real_pqc_signature -- --nocapture`: 1 passed.
- Local qRPC probe on `127.0.0.1:5640` and `127.0.0.1:5641`: no listener, so
  no read-only local request/response RPC examples were captured.

The `synergy-testnet` test commands emitted existing dead-code warnings; no
targeted test failed.

### Register Updates From Addendum

Marked complete with proof:

- `NET-001`
- `NET-002`
- `ROSETTA-008`
- `ROSETTA-009`
- `ROSETTA-011`
- `ROSETTA-012`

Left partial or blocked with notes:

- `NET-003`
- `NET-004`
- `NET-005`
- `NET-006`
- `NET-007`
- `AIVM-006`
- `AIVM-007`
- `AIVM-008`
- `AIVM-011`
- `AIVM-012`
- `ROSETTA-010`

### Final Git Status Addendum

`/Volumes/xcode/Synergy-Network-Projects/synq-language`,
`/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq`, and
`/Volumes/xcode/Synergy-Network-Projects/synergy-aivm` still do not expose local
`.git` metadata.

`/Volumes/xcode/Synergy-Network-Projects/project-rosetta`:

```text
## main...origin/main
 M crates/rosetta-verification/src/lib.rs
 M crates/rosetta-verification/src/synergy.rs
 M docs/synergy-testnet-integration.md
```

`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`:

```text
## main...origin/main
?? scripts/testnet/incident-install-runtime-only-v13.sh
?? scripts/testnet/incident-relayer1-build-recovery-plan.sh
?? scripts/testnet/incident-val1-build-recovery-plan.sh
?? scripts/testnet/incident-val5-build-recovery-plan.sh
```

Those untracked incident scripts pre-existed this addendum and were not
modified.

`/Users/devpup/Desktop/Testnet-Beta/project-rosetta` does not expose local
`.git` metadata in this workspace.

## Addendum: aegis-pqsynq vs aegis-pqvm Comparative Audit

New architecture document:

- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/architecture/aegis-pqsynq-vs-aegis-pqvm-integration.md`

Updated supporting documents:

- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/architecture/repo-dependency-map.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-Aegis-AIVM-Canonical-Tasks.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/proof/SynQ-Aegis-AIVM-Proof-Log-2026-05-26.md`

### Audit Conclusion

Both modules are needed.

- `aegis-pqsynq` owns SynQ-specific deploy/call authorization: SynQ signing
  payloads, `SYNQ_CONTRACT_DEPLOY_V1`, `SYNQ_CONTRACT_CALL_V1`, chain-1264
  binding, ML-DSA-65 launch policy, SynQ address derivation, canonical SynQ
  payload hashes, and structured `AegisSynQError` codes.
- `aegis-pqvm` owns the active Testnet-Beta blockchain/PQVM path: outer
  Synergy transaction signatures, chain/DAG/vote/QC/P2P/archive domains, key
  lifecycle, Aegis transaction carriers, DAG mempool admission, PQ VM/precompile
  ABI dispatch, gas helpers, deterministic state roots, receipt roots, and
  node RPC receipt surfaces.

At the time of this comparative audit, the source-proven model was **Model D**:
Testnet-Beta used `aegis-pqvm` for chain/node admission and did not call
`aegis-pqsynq`. The later Model B implementation addendum below supersedes this
status.

Recommended target model is **Model B**: node/AIVM calls `aegis-pqsynq` for the
inner SynQ deploy/call gate and keeps `aegis-pqvm` as the outer blockchain
transaction/DAG/VM admission owner.

### Source Evidence

- Active pqsynq manifest:
  `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/Cargo.toml`
- Standalone pqsynq mirror:
  `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/Cargo.toml`
- Active pqvm manifest:
  `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml`
- Active node dependency:
  `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/Cargo.toml`
  contains `aegis-pqvm = { path = "../aegis-pqvm" }`.
- Active node pqvm wrapper:
  `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/crypto/aegis_pqvm.rs`
- Aegis transaction carrier/admission:
  `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/aegis_tx_tool.rs`
- DAG admission:
  `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/dag_mempool.rs`
- Chain transaction validation:
  `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/transaction.rs`
- Deterministic receipts/roots:
  `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/execution.rs`
- RPC handlers and disabled AIVM handlers:
  `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/rpc/rpc_server.rs`

### Implemented

- Created the comparative architecture decision document with source inventory,
  API inventory, capability matrix, recommended ownership model, integration
  model, dependency graph, implementation plan, completed changes, and blockers.
- Updated the dependency map to show that the then-current node path used pqvm
  and that pqsynq was not yet wired into TESTNET deploy/call admission. The
  later Model B implementation addendum below supersedes this status.
- Updated the canonical register:
  - `ARCH-003` marked complete.
  - `ROSETTA-010` marked complete.
  - `ARCH-001` left partial because source still contains duplicate/prototype
    behavior and node wiring is incomplete.
  - `ARCH-002`, `AIVM-006`, `AIVM-007`, `AIVM-012`, `NET-003`, and `NET-004`
    left unchecked because node/AIVM deploy/call paths do not yet call
    `aegis-pqsynq`.

No runtime integration code was added during the comparative audit pass. The
later Model B implementation addendum below adds the carrier, node dependency,
and chain-1264 network normalization layer.

### Verification

Passed:

- `cargo fmt --all --check` in `/Volumes/xcode/Synergy-Network-Projects/synq-language`
- `cargo test -p aegis-pqsynq --test synq_verifier_tests --locked`: 17 passed.
- `cargo test --manifest-path /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml --test integrations_dispatch --test vm_validation --test security_smoke --locked`: 6 + 15 + 3 tests passed.

Blocked:

- `cargo test -p synergy-testnet aegis_tx_tool::tests -- --nocapture` did not
  reach tests because `/Volumes/xcode` ran out of space while compiling
  dependencies. `df -h` showed `/Volumes/xcode` at 100% capacity with 32 MiB
  available.

### Current Git Status

`/Volumes/xcode/Synergy-Network-Projects/synq-language`,
`/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq`, and
`/Volumes/xcode/Synergy-Network-Projects/synergy-aivm` still do not expose local
`.git` metadata.

`/Volumes/xcode/Synergy-Network-Projects/project-rosetta`:

```text
## main...origin/main
 M crates/rosetta-verification/src/lib.rs
 M crates/rosetta-verification/src/synergy.rs
 M docs/synergy-testnet-integration.md
```

`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`:

```text
## main...origin/main
 M src/consensus/consensus_algorithm.rs
?? scripts/testnet/incident-install-runtime-only-v13.sh
?? scripts/testnet/incident-relayer1-build-recovery-plan.sh
?? scripts/testnet/incident-val1-build-recovery-plan.sh
?? scripts/testnet/incident-val5-build-recovery-plan.sh
```

The modified consensus file and untracked incident scripts were not created or
edited during this comparative audit.

### Next Implementation Prompt

Implement the Model B adapter boundary: add a node/AIVM SynQ gate module that
decodes a canonical pqsynq deploy/call envelope from a Synergy transaction
carrier, calls `AegisSynQVerifier::testnet_1264()`, preserves
`AegisSynQError::code()` in a typed `SynQVerificationSummary`, then passes the
outer transaction through the existing `aegis-pqvm` admission path. Resolve the
`synergy-testnet` versus `synergy-testnet-v2` network ID decision before
enabling runtime admission.

## Addendum: Model B Boundary Implementation

### Summary

Implemented the first source-backed Model B boundary in the Testnet-Beta node:

```text
SynQ deploy/call carrier
  -> aegis-pqsynq inner deploy/call verification
  -> SynQ verification summary or structured SynQ admission error
  -> existing AegisTxSubmissionEnvelope / aegis-pqvm outer admission
  -> DAG mempool admission
  -> internal receipt/execution summary fields
```

This does not replace `aegis-pqvm`. It keeps `aegis-pqvm` as the outer
blockchain/DAG/PQVM admission owner and keeps SynQ language policy in
`aegis-pqsynq`.

### Files Changed

Testnet-Beta:

- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/Cargo.toml`
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/lib.rs`
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/synq_admission.rs`
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/aegis_tx_tool.rs`
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/execution.rs`
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml`
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/src/utils.rs`
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/Cargo.lock`

Documentation:

- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/architecture/aegis-pqsynq-vs-aegis-pqvm-integration.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/architecture/repo-dependency-map.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/integration/synq-aegis-aivm-contract.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/proof/SynQ-Aegis-AIVM-Proof-Log-2026-05-26.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-Aegis-AIVM-Canonical-Tasks.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/reports/Codex-Implementation-Report-2026-05-26.md`

### Adapter

`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/synq_admission.rs`
defines:

- `SynQAdmissionEnvelope`
- `SynQAdmissionKind::{Deploy, Call}`
- `SynQVerificationSummary`
- `SynQAdmissionError`
- `normalize_synq_network`
- `verify_synq_deploy_for_chain_admission`
- `verify_synq_call_for_chain_admission`
- `verify_transaction_payload_for_chain_admission`

The carrier prefix is `synq-admission-v1:`. The carrier stores version, kind,
chain ID, network ID, signer, pqsynq payload hash, optional bytecode/manifest/ABI
hashes, and serialized pqsynq deploy/call envelope bytes.

### Network Normalization

The SynQ gate accepts only:

- chain `1264` + `synergy-testnet`
- chain `1264` + `synergy-testnet-v2`

Both normalize to pqsynq network `synergy-testnet`. Wrong chain IDs and unrelated
network IDs are rejected; this does not globally rename the Testnet-Beta node
network.

### Verification Boundary

`aegis-pqsynq` runs in:

- `verify_synq_deploy_for_chain_admission`
- `verify_synq_call_for_chain_admission`
- `verify_transaction_payload_for_chain_admission`

`aegis-pqvm` still runs in:

- `AegisPqvmVerifier::verify_transaction_signature_checked`
- `legacy_transaction_from_aegis_envelope`
- `validate_legacy_aegis_carrier_transaction`
- `DagMempool::admit_transaction`

### Error Preservation

Preserved error codes include:

- `AEGIS-CHAIN`
- `AEGIS-NETWORK`
- `AEGIS-DOMAIN`
- `AEGIS-SIG`
- `AEGIS-CANON`

The internal execution path now has:

- `ExecutionState.synq_verifications`
- `ExecutionState.synq_errors`
- `TransactionReceipt.synq_verification`
- `TransactionReceipt.synq_error_code`
- `TransactionReceipt.synq_error_message`

### Commands Run

Passed:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target cargo fmt --all --check
```

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test -p synergy-testnet --lib synq_admission::tests -- --nocapture
```

Result: 10 passed. Covered network aliases, malformed carrier,
wrong-chain/wrong-domain/invalid-signature errors, valid deploy/call pqsynq
verification, and pqsynq-before-pqvm outer admission.

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test -p synergy-testnet --lib execution::tests::receipt_preserves_synq_verification_summary -- --nocapture
```

Result: 1 passed.

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test -p aegis-pqsynq --test synq_verifier_tests --locked
```

Result: 17 passed.

Historical pre-cleanup failures, superseded by the final verification update
below:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration CARGO_BUILD_JOBS=4 cargo test -p synergy-testnet synq_admission::tests -- --nocapture
```

Failed while building RocksDB with `errno=28` in the requested target directory.

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test -p synergy-testnet synq_admission::tests -- --nocapture
```

Compiled the node library but failed while linking a node binary with `errno=28`.
The `--lib` form passed.

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test --manifest-path /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml --test integrations_dispatch --test vm_validation --test security_smoke --locked
```

Failed before tests with undefined `_PQRUST_RUST_randombytes` after unifying
pqrust internals. A compatibility shim was added in
`aegis-pqvm/src/utils.rs`; at this point in the run, retest was blocked by
local disk exhaustion. The final verification update below records the later
successful retest after approved cache deletion.

Final disk state during this addendum:

```text
/Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration  2.6G
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target      8.6G
/System/Volumes/Data available                                      116Mi
```

No build cache had been deleted at this point because cleanup approval was
still required. The approved cache deletion is recorded in the final
verification update below.

### Register Updates

Marked complete:

- `ARCH-002`
- `AIVM-012`
- `DEMO-005`
- `DEMO-010`

Updated partial notes:

- `ARCH-001`
- `AIVM-006`
- `AIVM-007`
- `NET-003`
- `NET-004`
- `NET-006`

Left unchecked:

- `NET-007`: no local RPC listener/request-response examples were captured.
- Full AIVM deploy/call execution items: public AIVM RPC handlers remain
  disabled and native AIVM bytecode execution was not reached in this pass.

The final verification update below records the later successful pqvm targeted
retest after the approved cache deletion.

### Current Git Status

`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`:

```text
 M Cargo.lock
 M aegis-pqvm/Cargo.toml
 M aegis-pqvm/src/utils.rs
 M release-artifacts/wallet-pqc-cli-testnet-team/README.md
 M src/Cargo.toml
 M src/aegis_tx_tool.rs
 M src/consensus/consensus_algorithm.rs
 M src/execution.rs
 M src/lib.rs
?? scripts/testnet/incident-install-runtime-only-v13.sh
?? scripts/testnet/incident-relayer1-build-recovery-plan.sh
?? scripts/testnet/incident-val1-build-recovery-plan.sh
?? scripts/testnet/incident-val5-build-recovery-plan.sh
?? src/synq_admission.rs
```

`release-artifacts/wallet-pqc-cli-testnet-team/README.md`,
`src/consensus/consensus_algorithm.rs`, and the untracked incident scripts were
already dirty before this implementation work and were not intentionally part of
the Model B changes.

`/Volumes/xcode/Synergy-Network-Projects/project-rosetta`:

```text
 M crates/rosetta-verification/src/lib.rs
 M crates/rosetta-verification/src/synergy.rs
 M docs/synergy-testnet-integration.md
```

`/Volumes/xcode/Synergy-Network-Projects/synq-language`,
`/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq`, and
`/Volumes/xcode/Synergy-Network-Projects/synergy-aivm` still do not expose local
`.git` metadata.

### Historical Next Step

This cleanup/retest step is now complete. The final verification results are
recorded below.

## Addendum: Final Verification After Approved Cleanup

### Cleanup Performed

Deleted only the approved integration-attempt build cache:

```text
/Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration
```

No source files, docs, repos, keys, configs, user data, or
`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target` were deleted.

Post-cleanup disk check:

```text
/System/Volumes/Data available: 3.8Gi
/Volumes/xcode available: 43Gi
```

### Final Verification Results

Environment:

```sh
export CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
export CARGO_BUILD_JOBS=2
```

Passed:

```sh
cargo test --manifest-path /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml --test integrations_dispatch --test vm_validation --test security_smoke --locked
```

Result: 6 + 15 + 3 tests passed. This validates the
`PQRUST_RUST_randombytes` compatibility shim in `aegis-pqvm/src/utils.rs`.

```sh
cargo test -p synergy-testnet --lib synq_admission::tests -- --nocapture
```

Result: 10 passed. Proves valid deploy/call, wrong-chain, wrong-domain,
invalid-signature, malformed-carrier, network-alias behavior, and
pqsynq-before-pqvm admission.

```sh
cargo test -p synergy-testnet --lib execution::tests::receipt_preserves_synq_verification_summary -- --nocapture
```

Result: 1 passed.

```sh
cargo test -p aegis-pqsynq --test synq_verifier_tests --locked
```

Result: 17 passed.

```sh
cargo fmt --all --check
```

Result: passed.

### Newly Completed Register Items

After final verification, these additional items are marked complete:

- `ARCH-001`
- `AIVM-006`
- `AIVM-007`
- `NET-003`
- `NET-004`
- `NET-006`

Previously completed in this implementation pass:

- `ARCH-002`
- `AIVM-012`
- `DEMO-005`
- `DEMO-010`

`NET-007` remains unchecked because no local RPC listener was started and no real
request/response examples were captured.

### Remaining Blockers

- Public AIVM deploy/execute RPC handlers remain disabled; direct SynQ carrier
  admission is tested, but public AIVM RPC exposure is not claimed.
- Public RPC/indexer/explorer exposure for `synq_verification`,
  `synq_error_code`, and `synq_error_message` remains pending.
- PQ-Gas and trap-code exposure in public receipts remains pending.
- No live RPC smoke test was captured for `NET-007`.

## Final Verification Update After Approved Cache Cleanup

Timestamp: 2026-05-26 17:25 CDT.

Approved cleanup completed:

- Deleted only `/Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration`
  after confirming it was the 2.6G integration-attempt build cache.
- Did not delete
  `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target`.
- Did not delete source files, docs, repos, keys, configs, or user data.

Required reruns with:

```sh
export CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
export CARGO_BUILD_JOBS=2
```

Results:

- `cargo test --manifest-path /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml --test integrations_dispatch --test vm_validation --test security_smoke --locked`: PASS; 6 integration dispatch tests, 3 security smoke tests, and 15 VM validation tests passed. This validates the `_PQRUST_RUST_randombytes` compatibility shim in `aegis-pqvm/src/utils.rs`.
- `cargo test -p synergy-testnet --lib synq_admission::tests -- --nocapture`: PASS; 10 tests passed, including deploy/call pqsynq verification and deploy/call pqsynq-before-pqvm admission.
- `cargo test -p synergy-testnet --lib execution::tests::receipt_preserves_synq_verification_summary -- --nocapture`: PASS; 1 test passed.
- `cargo test -p aegis-pqsynq --test synq_verifier_tests --locked`: PASS from `/Volumes/xcode/Synergy-Network-Projects/synq-language`; 17 verifier tests passed.
- `cargo fmt --all --check`: PASS.

Register items newly marked complete:

- `ARCH-001`: the implemented node path enforces the intended ownership split:
  `aegis-pqsynq` verifies inner SynQ deploy/call payloads, the node then keeps
  `aegis-pqvm` as outer Synergy transaction/DAG admission, and no duplicate
  downstream SynQ crypto policy was added.
- `AIVM-006`: deploy verification gate is implemented and tested through the
  SynQ carrier/admission adapter.
- `AIVM-007`: call verification gate is implemented and tested through the SynQ
  carrier/admission adapter.
- `NET-003`: deploy node validation now runs pqsynq verification before pqvm
  admission.
- `NET-004`: call node validation now runs pqsynq verification before pqvm
  admission.
- `NET-006`: internal receipt/execution summary fields are implemented and
  tested; public RPC/indexer/explorer exposure is still pending.

Remaining legitimate blockers:

- `NET-007` remains unchecked because no local RPC listener was running and no
  real request/response examples were captured.
- Public RPC/indexer/explorer exposure for internal SynQ receipt summary fields
  is still pending.
- Enabled AIVM deploy/call execution, ABI method lookup, PQ-Gas accounting, and
  trap-code exposure remain pending.

Final git status sweep:

```text
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet:
## main...origin/main [behind 4]
 M Cargo.lock
 M aegis-pqvm/Cargo.toml
 M aegis-pqvm/src/utils.rs
 M release-artifacts/wallet-pqc-cli-testnet-team/README.md
 M src/Cargo.toml
 M src/aegis_tx_tool.rs
 M src/consensus/consensus_algorithm.rs
 M src/execution.rs
 M src/lib.rs
?? scripts/testnet/incident-install-runtime-only-v13.sh
?? scripts/testnet/incident-relayer1-build-recovery-plan.sh
?? scripts/testnet/incident-val1-build-recovery-plan.sh
?? scripts/testnet/incident-val5-build-recovery-plan.sh
?? src/synq_admission.rs

/Volumes/xcode/Synergy-Network-Projects/project-rosetta:
## main...origin/main
 M crates/rosetta-verification/src/lib.rs
 M crates/rosetta-verification/src/synergy.rs
 M docs/synergy-testnet-integration.md

/Volumes/xcode/Synergy-Network-Projects/synq-language:
fatal: not a git repository at that exact path

/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq:
fatal: not a git repository at that exact path

/Volumes/xcode/Synergy-Network-Projects/synergy-aivm:
fatal: not a git repository at that exact path
```
