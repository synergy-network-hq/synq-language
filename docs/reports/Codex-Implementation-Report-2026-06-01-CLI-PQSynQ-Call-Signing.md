# Codex Implementation Report - 2026-06-01 CLI PQSynQ Call Signing

## Summary

Added pqsynq-backed local call-envelope commands to the SynQ CLI:

- `sign-call`
- `verify-call`

The new commands create and verify `SYNQ_CONTRACT_CALL_V1`
`ContractCallEnvelope` JSON for the current no-argument Counter subset
(`increment()` and `get()`). They use `aegis-pqsynq` for ML-DSA-65 signing,
canonical payload hashing, chain-1264 binding, network normalization, SynQ
address derivation, and structured verifier errors. They do not submit calls
to public TESTNET.

## Files Changed

- `/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/src/main.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/tests/integration_test.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/scripts/ama_demo_synq_counter.sh`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-User-Manual.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/demo/SynQ-Counter-AMA-Demo-2026-06-06.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/demo/SynQ-Counter-AMA-Demo-Visual.html`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/demo/SynQ-Counter-AMA-Expected-Output-2026-06-06.txt`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/proof/SynQ-Aegis-AIVM-Proof-Log-2026-05-26.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-Aegis-AIVM-Canonical-Tasks.md`

## Commands Run

```sh
cd /Volumes/xcode/Synergy-Network-Projects/synq-language
export CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
export CARGO_BUILD_JOBS=2
cargo test -p cli --test integration_test --locked
cargo test -p compiler --test artifact_test --locked
cargo test -p aegis-pqsynq --test synq_verifier_tests --locked
cargo fmt --all --check
./scripts/ama_demo_synq_counter.sh --negative-tests
```

## Test Status

Passed:

- CLI integration tests: `22/22`
- Compiler artifact tests: `6/6`
- pqsynq verifier tests: `17/17`
- Formatting: `cargo fmt --all --check`
- AMA demo script: passed and regenerated the visual board

## What Is Proven

- `sign-call` reads ABI/manifest artifacts, resolves the no-argument method
  selector, computes the empty args hash, creates a `SYNQ_CONTRACT_CALL_V1`
  payload, signs with ML-DSA-65 through `aegis-pqsynq`, and self-verifies.
- `verify-call` verifies the call envelope through
  `AegisSynQVerifier::testnet_1264()`.
- Negative tests preserve structured errors for wrong domain, invalid
  signature, and method-selector/payload mismatch.
- The AMA demo now shows CLI keygen, deploy envelope verification, and
  `increment()`/`get()` call envelope verification before Model B admission and
  local AIVM execution.

## Not Claimed

- No public RPC listener was started.
- No live TESTNET deployment or call was submitted.
- `synq call Counter.increment` and `synq call Counter.get` live submission
  commands remain pending.
- Public AIVM RPC handlers remain disabled.
- Explorer/indexer display of SynQ verification summaries remains pending.

## Register Updates

Updated proof notes:

- `LANG-015`: partial CLI surface now includes `sign-call` and `verify-call`.
- `LANG-016`: completed item now includes call signing and verification as
  pqsynq-backed crypto-specific CLI paths.
- `LANG-017`: partial testing proof now includes 22 CLI integration tests and
  call-envelope negative tests.
- `DEMO-003`: proof strengthened with the current 22-test CLI run.
- `DEMO-009`: left unchecked with partial proof for `sign-call`; no live
  `synq call` command exists yet.
- `DEMO-010`: proof strengthened with CLI `verify-call` and current demo run.

## Recommended Next Prompt

Implement the live local `synq call` command surface without public TESTNET
claims: consume a verified `ContractCallEnvelope`, wrap it in the
`synq-admission-v1` carrier, run pqsynq inner verification, keep pqvm outer
admission, execute the direct AIVM Counter handoff, and return the deterministic
local receipt. Add tests for `Counter.increment`, `Counter.get`, wrong chain,
wrong domain, invalid signature, missing method, and receipt hash stability.
