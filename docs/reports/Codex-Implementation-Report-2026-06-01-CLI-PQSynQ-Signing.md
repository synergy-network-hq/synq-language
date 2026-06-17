# Codex Implementation Report - 2026-06-01 CLI PQSynQ Signing

## Summary

Added the first pqsynq-backed crypto commands to the SynQ CLI:

- `keygen`
- `sign-deploy`
- `verify-deploy`

These are local-only workflow commands. They create a testnet ML-DSA-65
identity, create a canonical `SYNQ_CONTRACT_DEPLOY_V1` deploy envelope from
existing bytecode/ABI/manifest artifacts, and verify that envelope through
`AegisSynQVerifier::testnet_1264()`. They do not submit a transaction to public
TESTNET.

## Files Changed

- `/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/Cargo.toml`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/src/main.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/tests/integration_test.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/Cargo.lock`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-User-Manual.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/proof/SynQ-Aegis-AIVM-Proof-Log-2026-05-26.md`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-Aegis-AIVM-Canonical-Tasks.md`

## Verification

```text
cargo test -p cli --test integration_test --locked 18/18 PASS
cargo test -p compiler --test artifact_test --locked 6/6 PASS
cargo test -p aegis-pqsynq --test synq_verifier_tests --locked 17/17 PASS
cargo fmt --all --check PASS
./scripts/ama_demo_synq_counter.sh PASS
```

## Register Updates

Newly marked complete:

- `LANG-016`
- `DEMO-003`

Strengthened existing/partial items:

- `LANG-015`: CLI partial proof now includes `keygen`, `sign-deploy`, and
  `verify-deploy`.
- `LANG-017`: CLI smoke/integration proof now covers pqsynq-backed keygen and
  deploy envelope signing.
- `DEMO-005`: CLI signing and self-verification now provide additional
  deployment-payload proof.

Left unchecked:

- `LANG-015`: still missing `test`, `fmt`, live `deploy --chain 1264`, and
  `call`.
- `DEMO-004`: `sign-deploy` creates a local deploy envelope, but no live
  `synq deploy` command exists yet.
- Public RPC/live TESTNET deployment remains unclaimed.

## Security Notes

- Private key bytes are written only to the requested private key JSON file.
- The CLI prints key file paths and the derived address, not private key bytes.
- On Unix platforms, generated private key files are restricted to `0600`.
- All signing and verification is delegated to `aegis-pqsynq`; no duplicate
  SynQ crypto policy was added to the CLI.
