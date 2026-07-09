# SynQ Example Contracts Remediation Backlog

Backlog ID: `SYNQ-EXAMPLES-REMEDIATION-2026-02-09`
Based On: Audit campaign `SYNQ-EXAMPLES-2026-02-09`
Date: 2026-02-09

---

## Purpose

This backlog converts audit findings into execution work packages with explicit acceptance criteria so the six sample contracts can move from aspirational examples to auditable, deployable references.

---

## P0 - Stop-the-Line Work (Must Complete First)

### P0.1 Parser Compatibility Baseline (All 6 contracts)

Tasks:

1. Define canonical contract syntax profile for current SynQ compiler.
2. Migrate all six contracts to canonical syntax.
3. Add CI job: compile every file in `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/docs/examples`.
4. Fail CI if any example fails parse/compile.

Acceptance criteria:

- `cargo run -p cli -- compile --path <each example>` passes for all six.

---

### P0.2 Signature Authorization Hardening Pattern (All 6 contracts)

Tasks:

1. Add shared helper for signed payload construction including:
   - protocol/version domain separator
   - contract identifier
   - function identifier
   - full argument commitment
   - chain/environment marker
   - nonce
2. Add per-key nonce state and monotonic checks.
3. Reject reused nonce or mismatched payload hash.
4. Add adversarial tests:
   - replay old signature
   - valid signature on wrong function
   - valid signature with altered arguments

Acceptance criteria:

- Replay and context-confusion tests fail closed on all privileged functions.

---

### P0.3 Documentation Integrity Fix

Tasks:

1. Update `/Users/devpup/Desktop/Synergy/synergy-components/synq-language/docs/Examples-Index.md` to remove production-ready claims until P0/P1 gates pass.
2. Add per-example readiness banner: `DRAFT`, `AUDITED`, `CERTIFIED`.
3. Link each example to latest audit report.

Acceptance criteria:

- No claim in example docs contradicts audit evidence.

---

## P1 - Contract-Specific Security Work

### ERC20 (`PQCToken`)

Tasks:

1. Remove duplicate `transfer` definitions and unify pause checks across all balance-moving paths.
2. Harden governance key rotation with two-step activation and key validity checks.
3. Replace raw `approve` overwrite flow with safe allowance update policy.

Acceptance criteria:

- Pause blocks `transfer`, `transferFrom`, and `batchTransfer` consistently.
- Governance actions require nonce-bound signed intent.

---

### MultiSig (`PQCMultiSigWallet`)

Tasks:

1. Enforce unique, non-zero owner set at initialization.
2. Redesign signature collection to explicit signer list with dedup.
3. Implement real execute path (value transfer/call + error handling).
4. Normalize confirmations on owner-set mutation.

Acceptance criteria:

- Threshold logic remains correct across add/remove/replace operations.
- Execution path is functionally complete and test-covered.

---

### DAO (`PQCGovernanceDAO`)

Tasks:

1. Bind voter identity to registered key in `castVoteWithSignature`.
2. Implement actual threshold and token-weighted voting semantics.
3. Replace impossible governance sentinel (`Address(0)`) in cancel path.
4. Implement actual proposal target execution semantics.

Acceptance criteria:

- Forged-voter test fails.
- Weighting and quorum semantics match documentation and tests.

---

### NFT (`PQCNFT`)

Tasks:

1. Introduce monotonic `nextTokenId` counter (no reuse after burn).
2. Implement receiver checks for safe transfer path.
3. Replace stubbed `tokensOfOwner` with real index or remove API.
4. Validate royalty recipient on non-zero percentage.

Acceptance criteria:

- Burn followed by mint cannot collide with existing token IDs.
- Safe transfer to non-compliant contract fails safely.

---

### Escrow (`PQCEscrow`)

Tasks:

1. Implement real custody and settlement transfer logic.
2. Fix expiry/refund state transitions to prevent fund lock.
3. Redesign release authority model to prevent key-mismatch deadlocks.
4. Require non-zero escrow funding at creation.

Acceptance criteria:

- Funds cannot become permanently unreachable through normal state transitions.

---

### Staking (`PQCStaking`)

Tasks:

1. Implement real token transfer in/out and reward payout semantics.
2. Enforce constructor lock policy invariants.
3. Add reward solvency controls and reserve accounting.
4. Introduce active staker indexing strategy to avoid unbounded growth risk.

Acceptance criteria:

- Internal accounting always reconciles with token balances.
- Reward claims fail safely when reserve insufficient.

---

## P2 - Security QA and Certification Readiness

Tasks:

1. Add contract-level negative test suite for each example:
   - replay
   - malformed signature
   - wrong key
   - boundary inputs
   - unauthorized caller
2. Add deterministic compile + run evidence capture scripts for audit artifacts.
3. Re-run full audit campaign and update report statuses to Resolved/Open.

Acceptance criteria:

- Zero unresolved Critical/High findings per contract.
- Each contract has reproducible test artifacts and final certification decision.

---

## Suggested Execution Sequence

1. P0 parser compatibility and signature-hardening pattern.
2. P1 contract-specific logic fixes in this order: Escrow, Staking, DAO, MultiSig, ERC20, NFT.
3. P2 unified adversarial test suite and re-audit.

Reasoning: Escrow and Staking currently carry the largest economic correctness gaps; DAO/MultiSig carry the largest governance integrity gaps.
