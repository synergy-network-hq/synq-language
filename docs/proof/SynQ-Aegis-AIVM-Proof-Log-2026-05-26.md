# SynQ / aegis-pqsynq / Synergy-AIVM / Rosetta Proof Log

Date: 2026-05-26
Workspace root inspected: `/Volumes/xcode/Synergy-Network-Projects`

## Canonical Register

Command:

```sh
rg -n "SynQ \+ aegis-pqsynq \+ Synergy-AIVM Canonical Task Register" \
  /Volumes/xcode/Synergy-Network-Projects/synq-language \
  /Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq \
  /Volumes/xcode/Synergy-Network-Projects/synergy-aivm \
  /Volumes/xcode/Synergy-Network-Projects/project-rosetta
```

Output excerpt:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-Aegis-AIVM-Canonical-Tasks.md:1:# SynQ + aegis-pqsynq + Synergy-AIVM Canonical Task Register
```

## Git Status

User-provided upstream GitHub repos:

- `synq-language`: `https://github.com/synergy-network-hq/synq-language`
- `aegis-pqsynq`: `https://github.com/synergy-network-hq/aegis-pqsynq`
- `synergy-aivm`: `https://github.com/synergy-network-hq/synergy-aivm`
- `project-rosetta`: `https://github.com/synergy-network-hq/project-rosetta`

Current local filesystem finding: no `.git` directory or `.git` file is present
within the four audited paths, and `git status` cannot read checkout metadata
from these local snapshots. This is recorded as a local metadata availability
gap, not as a claim that the upstream projects are not GitHub-backed.

Commands:

```sh
git -C /Volumes/xcode/Synergy-Network-Projects/synq-language status --short --branch
git -C /Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq status --short --branch
git -C /Volumes/xcode/Synergy-Network-Projects/synergy-aivm status --short --branch
git -C /Volumes/xcode/Synergy-Network-Projects/project-rosetta status --short --branch
```

Output for each path:

```text
fatal: not a git repository (or any parent up to mount point /Volumes)
Stopping at filesystem boundary (GIT_DISCOVERY_ACROSS_FILESYSTEM not set).
```

Current blocker: these four local paths do not expose `.git` metadata in this filesystem snapshot.

## SynQ Language Workspace

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon cargo fmt --check
```

Result: PASS, exit code 0.

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon cargo check --workspace --all-features --all-targets --locked
```

Output excerpt:

```text
Checking aegis-pqsynq v0.1.0 (.../synq-language/aegis-pqsynq/pqsynq)
Checking quantumvm v1.0.0 (.../synq-language/vm)
Checking compiler v0.1.0 (.../synq-language/compiler)
Checking cli v0.1.0 (.../synq-language/cli)
Finished `dev` profile [unoptimized + debuginfo] target(s)
```

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon cargo test --workspace --all-features --all-targets --locked
```

Output excerpts:

```text
Running tests/synq_verifier_tests.rs
running 8 tests
test wrong_chain_fails ... ok
test valid_mldsa65_call_verifies ... ok
test valid_mldsa65_deploy_verifies ... ok
test wrong_domain_fails ... ok
test transaction_dispatch_preserves_security_errors ... ok
test invalid_signature_fails ... ok
test malformed_public_key_fails ... ok
test transaction_dispatch_verifies_deploy ... ok

Running tests/canonical_payload_tests.rs
running 3 tests
test canonical_payload_bytes_are_deterministic_and_versioned ... ok
test canonical_payload_rejects_oversized_length_prefixed_fields ... ok
test signing_payload_hash_changes_with_domain_and_chain_binding ... ok

Running tests/integration_test.rs
running 6 tests
test test_compile_all_documented_examples ... ok
test test_verify_accepts_matching_bytecode_and_executes ... ok

Running tests/semantic_test.rs
running 15 tests
test semantic_accepts_mldsa_builtin_with_expected_types ... ok
test semantic_rejects_de_scoped_slhdsa_builtin ... ok

Running tests/integration_test.rs
running 7 tests
test test_mldsa_verify ... ok
test test_truncated_header_returns_error_instead_of_panicking ... ok
```

Result: PASS. The workspace test command completed with all reported tests passing.

Command:

```sh
npm run typecheck
```

Workdir: `/Volumes/xcode/Synergy-Network-Projects/synq-language/sdk`

Output excerpt:

```text
> typecheck
> tsc --noEmit
```

Result: PASS, exit code 0.

Command:

```sh
npm run test:integration
```

Workdir: `/Volumes/xcode/Synergy-Network-Projects/synq-language/sdk`

Output excerpt:

```text
✔ Contract deploy/call emit expected JSON-RPC envelopes
✔ QuantumVMSDK tx/balance/block RPC calls are end-to-end well-formed
✔ QuantumVMClient rejects JSON-RPC and HTTP failures
✔ PQC key APIs fail explicitly until backend is wired
ℹ tests 4
ℹ pass 4
```

Result: PASS.

## Addendum: Model B pqsynq / pqvm Node Boundary

Changed source paths:

- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/Cargo.toml`
  added a path dependency on active SynQ workspace `aegis-pqsynq` as `pqsynq`.
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml`
  now shares the SynQ workspace `pqrust-internals` and `pqrust-traits` paths so
  `aegis-pqvm` and `aegis-pqsynq` use one `pqrust_internals` native link target.
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/src/utils.rs`
  was updated for the shared pqrust RNG symbol name
  `pqrust_RUST_randombytes`.
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/synq_admission.rs`
  defines `SynQAdmissionEnvelope`, `SynQAdmissionKind`,
  `SynQVerificationSummary`, `SynQAdmissionError`, chain-1264 network alias
  normalization, and deploy/call pqsynq verification adapters.
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/aegis_tx_tool.rs`
  calls `verify_transaction_payload_for_chain_admission` before existing
  `AegisPqvmVerifier` outer transaction admission.
- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/execution.rs`
  carries optional SynQ verification summaries and structured error codes in
  execution state and receipts.

Command:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration CARGO_BUILD_JOBS=4 cargo test -p synergy-testnet synq_admission::tests -- --nocapture
```

Workdir: `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Result: FAIL before tests because `/Users` ran out of space while building
RocksDB into the requested target directory.

Output excerpt:

```text
write64() failed, returned -1 expected 452650976, errno=28
ranlib: error writing file '/Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration/debug/build/librocksdb-sys-490d2293a6b777c6/out/librocksdb.a'
error: failed to run custom build command for `librocksdb-sys v0.17.3+10.4.2`
```

Disk evidence:

```text
df -h /Users/devpup/Desktop/Testnet-Beta /Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration /Volumes/xcode
/dev/disk3s1   228Gi   181Gi   1.0Gi   100%   /System/Volumes/Data
/dev/disk7s1   237Gi   234Gi   2.8Gi    99%   /Volumes/xcode

du -sh /Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration
2.6G

du -sh /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
7.3G

du -sh /Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration/debug/build/librocksdb-sys-490d2293a6b777c6
1.2G
```

Command:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test -p synergy-testnet synq_admission::tests -- --nocapture
```

Workdir: `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Result: FAIL after compiling the library because binary linking ran out of
space.

Output excerpt:

```text
Compiling synergy-testnet v1.2.0 (/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src)
ld: write() failed, errno=28 (No space left on device)
error: could not compile `synergy-testnet` (bin "synergy-node") due to 1 previous error
```

Command:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test -p synergy-testnet --lib synq_admission::tests -- --nocapture
```

Workdir: `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Result: PASS.

Output excerpt:

```text
running 10 tests
test synq_admission::tests::network_alias_normalization_accepts_testnet_names_for_chain_1264 ... ok
test synq_admission::tests::network_alias_normalization_rejects_wrong_chain_and_unrelated_network ... ok
test synq_admission::tests::malformed_carrier_preserves_canonicalization_code ... ok
test synq_admission::tests::wrong_chain_preserves_aegis_chain_code ... ok
test synq_admission::tests::valid_deploy_carrier_verifies_with_pqsynq ... ok
test synq_admission::tests::invalid_signature_preserves_aegis_sig_code ... ok
test synq_admission::tests::wrong_domain_preserves_aegis_domain_code ... ok
test synq_admission::tests::valid_call_carrier_verifies_with_pqsynq ... ok
test synq_admission::tests::valid_deploy_carrier_passes_pqsynq_then_existing_pqvm_admission ... ok
test synq_admission::tests::valid_call_carrier_passes_pqsynq_then_existing_pqvm_admission ... ok
test result: ok. 10 passed; 0 failed; 0 ignored; 432 filtered out
```

Command:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test -p synergy-testnet --lib execution::tests::receipt_preserves_synq_verification_summary -- --nocapture
```

Workdir: `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Result: PASS.

Output excerpt:

```text
running 1 test
test execution::tests::receipt_preserves_synq_verification_summary ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 441 filtered out
```

Command:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test -p aegis-pqsynq --test synq_verifier_tests --locked
```

Workdir: `/Volumes/xcode/Synergy-Network-Projects/synq-language`

Result: FAIL before tests because `/Users` had only about 120 MiB free after the
node build attempts.

Output excerpt:

```text
error: failed to build archive at `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target/debug/deps/liblibc-4bcd655ccfb5170a.rlib`: No space left on device (os error 28)
error: failed to write to `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target/debug/deps/rmetahYrjvo/full.rmeta`: No space left on device (os error 28)
```

Command:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target cargo fmt --all --check
```

Workdir: `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Result: FAIL: rustfmt reported formatting diffs in touched files.

Follow-up command:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target cargo fmt --all
```

Result: FAIL because `/Users` had no writable space for rustfmt temp/output
files.

Output excerpt:

```text
cannot create temp file for here document: No space left on device
Error writing files: io error: /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/aegis_tx_tool.rs: No space left on device (os error 28)
```

Later retry after transient build files were released:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target cargo fmt --all
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target cargo fmt --all --check
```

Workdir: `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Result: PASS, exit code 0 for both commands.

Command:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test -p aegis-pqsynq --test synq_verifier_tests --locked
```

Workdir: `/Volumes/xcode/Synergy-Network-Projects/synq-language`

Result: PASS after retry.

Output excerpt:

```text
running 17 tests
test malformed_signature_fails ... ok
test oversized_signature_fails ... ok
test call_payload_cannot_be_replayed_as_wallet_auth ... ok
test expired_payload_fails ... ok
test oversized_public_key_fails ... ok
test invalid_signature_fails ... ok
test testnet_payload_cannot_be_replayed_as_mainnet ... ok
test deploy_domain_cannot_be_replayed_as_call ... ok
test transaction_dispatch_preserves_security_errors ... ok
test malformed_public_key_fails ... ok
test wrong_chain_fails ... ok
test transaction_dispatch_verifies_deploy ... ok
test wrong_algorithm_fails ... ok
test wrong_domain_fails ... ok
test valid_mldsa65_deploy_verifies ... ok
test valid_mldsa65_call_verifies ... ok
test wrong_signer_fails ... ok
test result: ok. 17 passed
```

Command:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test --manifest-path /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml --test integrations_dispatch --test vm_validation --test security_smoke --locked
```

Result: FAIL before tests with an undefined RNG compatibility symbol after
switching `aegis-pqvm` to shared SynQ pqrust internals.

Output excerpt:

```text
Undefined symbols for architecture arm64:
  "_PQRUST_RUST_randombytes", referenced from:
      _PQCLEAN_MLDSA44_CLEAN_crypto_sign_keypair
      _PQCLEAN_FALCON512_CLEAN_crypto_sign_keypair
ld: symbol(s) not found for architecture arm64
error: could not compile `aegis-pqvm` (test "vm_validation") due to 1 previous error
```

Fix added:

- `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/src/utils.rs`
  now exports `PQRUST_RUST_randombytes` as a compatibility shim that delegates to
  `pqrust_internals::pqrust_RUST_randombytes`.

Final node retest attempt after narrowing the pqsynq dependency to launch-scope
features `["std", "mldsa"]`:

```sh
CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target CARGO_BUILD_JOBS=2 cargo test -p synergy-testnet --lib synq_admission::tests -- --nocapture
```

Result: FAIL before tests because `/Users` was exhausted again while writing the
incremental pqsynq query cache.

Output excerpt:

```text
error: failed to write query cache to `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target/debug/incremental/pqsynq-0148guwpdpj37/s-hiw7ukqcfh-1d56yyj-working/query-cache.bin`: No space left on device (os error 28)
```

Current disk evidence:

```text
df -h /Users/devpup/Desktop/Testnet-Beta /Volumes/xcode
/dev/disk3s1   228Gi   182Gi   116Mi   100%   /System/Volumes/Data
/dev/disk7s1   237Gi   173Gi    64Gi    73%   /Volumes/xcode

du -sh /Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration
2.6G

du -sh /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
8.6G
```

## Addendum: Approved Cleanup And Final Verification

Approved cleanup command:

```sh
rm -rf /Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration
test ! -e /Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration
df -h /Users/devpup/Desktop/Testnet-Beta /Volumes/xcode
```

Result: PASS. Only the approved integration-attempt build cache was removed.

Output excerpt:

```text
/dev/disk3s1   228Gi   178Gi   3.8Gi    98%   /System/Volumes/Data
/dev/disk7s1   237Gi   194Gi    43Gi    82%   /Volumes/xcode
```

Environment used for final verification:

```sh
export CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
export CARGO_BUILD_JOBS=2
```

Command:

```sh
cargo test --manifest-path /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml --test integrations_dispatch --test vm_validation --test security_smoke --locked
```

Result: PASS.

Output excerpt:

```text
Running tests/integrations_dispatch.rs
running 6 tests
test mldsa_verify_detached_returns_true_then_false_on_tamper ... ok
test fndsa_verify_detached_returns_true ... ok
test result: ok. 6 passed

Running tests/security_smoke.rs
running 3 tests
test mldsa_smoke_tamper_message_or_signature_fails ... ok
test result: ok. 3 passed

Running tests/vm_validation.rs
running 15 tests
test evm_precompile_accepts_valid_decode_then_fails_on_bad_args ... ok
test cosmwasm_call_accepts_bound_payload ... ok
test result: ok. 15 passed
```

The `PQRUST_RUST_randombytes` compatibility shim in
`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/src/utils.rs`
is validated by this pass.

Command:

```sh
cargo test -p synergy-testnet --lib synq_admission::tests -- --nocapture
```

Workdir: `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Result: PASS.

Output excerpt:

```text
running 10 tests
test synq_admission::tests::network_alias_normalization_accepts_testnet_names_for_chain_1264 ... ok
test synq_admission::tests::network_alias_normalization_rejects_wrong_chain_and_unrelated_network ... ok
test synq_admission::tests::malformed_carrier_preserves_canonicalization_code ... ok
test synq_admission::tests::wrong_domain_preserves_aegis_domain_code ... ok
test synq_admission::tests::valid_deploy_carrier_verifies_with_pqsynq ... ok
test synq_admission::tests::wrong_chain_preserves_aegis_chain_code ... ok
test synq_admission::tests::invalid_signature_preserves_aegis_sig_code ... ok
test synq_admission::tests::valid_call_carrier_verifies_with_pqsynq ... ok
test synq_admission::tests::valid_deploy_carrier_passes_pqsynq_then_existing_pqvm_admission ... ok
test synq_admission::tests::valid_call_carrier_passes_pqsynq_then_existing_pqvm_admission ... ok
test result: ok. 10 passed
```

Command:

```sh
cargo test -p synergy-testnet --lib execution::tests::receipt_preserves_synq_verification_summary -- --nocapture
```

Workdir: `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Result: PASS.

Output excerpt:

```text
running 1 test
test execution::tests::receipt_preserves_synq_verification_summary ... ok
test result: ok. 1 passed
```

Command:

```sh
cargo test -p aegis-pqsynq --test synq_verifier_tests --locked
```

Workdir: `/Volumes/xcode/Synergy-Network-Projects/synq-language`

Result: PASS.

Output excerpt:

```text
running 17 tests
test valid_mldsa65_call_verifies ... ok
test valid_mldsa65_deploy_verifies ... ok
test wrong_chain_fails ... ok
test wrong_domain_fails ... ok
test invalid_signature_fails ... ok
test result: ok. 17 passed
```

Command:

```sh
cargo fmt --all --check
```

Workdir: `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Result: PASS, exit code 0.

## Comparative audit: aegis-pqsynq vs aegis-pqvm

Status date: 2026-05-26

Command:

```sh
find /Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq -name Cargo.toml -o -name package.json
```

Output excerpt:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/Cargo.toml
/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/package.json
/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/aegis_crypto_core/Cargo.toml
```

Command:

```sh
find /Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq -name Cargo.toml -o -name package.json
```

Output excerpt:

```text
/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/Cargo.toml
/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/package.json
/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/aegis_crypto_core/Cargo.toml
```

Command:

```sh
find /Users/devpup/Desktop/Testnet-Beta -path '*/target' -prune -o -name Cargo.toml -print | rg 'aegis-pqvm|pqvm|synergy-testnet'
```

Output excerpt:

```text
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/Cargo.toml
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/Cargo.toml
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/vendor/aegis-pqvm/Cargo.toml
/Users/devpup/Desktop/Testnet-Beta/synergy-address-engine/aegis-pqvm/Cargo.toml
```

Command:

```sh
find /Volumes/xcode/Synergy-Network-Projects -path '*/target' -prune -o -name Cargo.toml -print | rg 'aegis-pqvm|pqvm'
```

Output excerpt:

```text
/Volumes/xcode/Synergy-Network-Projects/aegis-pqvm/Cargo.toml
/Volumes/xcode/Synergy-Network-Projects/aegis-pqvm/fuzz/Cargo.toml
/Volumes/xcode/Synergy-Network-Projects/aegis-pqvm/vendor/pqcrypto-internals/Cargo.toml
/Volumes/xcode/Synergy-Network-Projects/aegis-pqvm/vendor/pqcrypto-traits/Cargo.toml
/Volumes/xcode/Synergy-Network-Projects/synergy-address-engine/aegis-pqvm/Cargo.toml
```

Command:

```sh
rg -n "aegis-pqvm|aegis-pqsynq|pqsynq|\[dependencies\]" \
  /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/Cargo.toml \
  /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/Cargo.toml \
  /Volumes/xcode/Synergy-Network-Projects/synq-language/Cargo.toml \
  /Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/Cargo.toml
```

Output excerpt:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/Cargo.toml:2:name = "aegis-pqsynq"
/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/Cargo.toml:11:name = "pqsynq"
/Volumes/xcode/Synergy-Network-Projects/synq-language/Cargo.toml:8:    "aegis-pqsynq/pqsynq",
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/Cargo.toml:33:aegis-pqvm = { path = "../aegis-pqvm" }
```

Command:

```sh
sed -n '1,260p' /Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/verifier.rs
sed -n '1,260p' /Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/payload.rs
sed -n '1,240p' /Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/domain.rs
sed -n '1,220p' /Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/policy.rs
sed -n '1,180p' /Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/algorithms.rs
sed -n '1,260p' /Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/serialization.rs
sed -n '1,220p' /Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/error.rs
```

Source evidence summary:

```text
`AegisSynQVerifier` exposes verify_synq_transaction, verify_contract_deploy,
verify_contract_call, derive_synq_address, canonicalize_signing_payload,
validate_algorithm_policy, and verify_chain_domain.
`ContractDeployEnvelope` and `ContractCallEnvelope` are SynQ-specific.
`DomainTag` defines SYNQ_TX_V1, SYNQ_CONTRACT_DEPLOY_V1,
SYNQ_CONTRACT_CALL_V1, SYNQ_VALIDATOR_MESSAGE_V1, SYNQ_AIVM_RECEIPT_V1,
SYNQ_STATE_COMMITMENT_V1, SYNQ_WALLET_AUTH_V1, and
SYNQ_CROSS_CHAIN_MESSAGE_V1.
`SynQSecurityPolicy::testnet_1264_policy()` allows ML-DSA-65 for tx/deploy/call
and requires chain 1264 plus network "synergy-testnet".
`canonicalize_signing_payload` uses fixed magic, integer widths, numeric IDs,
and length-prefixed network/signer bytes.
`AegisSynQError` preserves structured codes including AEGIS-CHAIN,
AEGIS-DOMAIN, AEGIS-CANON, AEGIS-KEY, AEGIS-SIG, and AEGIS-ADDRESS.
```

Command:

```sh
sed -n '1,620p' /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/crypto/aegis_pqvm.rs
sed -n '1,620p' /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/aegis_tx_tool.rs
sed -n '1,620p' /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/transaction.rs
sed -n '1,260p' /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/dag_mempool.rs
sed -n '1,300p' /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/execution.rs
```

Source evidence summary:

```text
`src/crypto/aegis_pqvm.rs` defines Synergy domains including SYNERGY_TX_V1,
SYNERGY_BLOCK_V1, SYNERGY_VOTE_V1, SYNERGY_QC_V1, SYNERGY_DAG_NODE_V1,
SYNERGY_STATE_ROOT_V1, and SYNERGY_RECEIPT_ROOT_V1.
`AegisPqvmSigner` and `AegisPqvmVerifier` sign/verify chain transaction,
vote, epoch, P2P, and other Synergy domains.
`aegis_tx_tool.rs` defines `AegisTxSubmissionEnvelope` and verifies the envelope
through `AegisPqvmVerifier::verify_transaction_signature_checked`.
`transaction.rs::validate_for_admission` enforces chain/network and uses
either Aegis carrier verification or embedded PQC signature verification.
`dag_mempool.rs::admit_transaction` verifies canonical bytes and Aegis PQ
signature before DAG insertion.
`execution.rs` defines deterministic `TransactionReceipt`, state roots, and
receipt roots.
```

Command:

```sh
rg -n "SYNERGY_TESTNET_V2|require_testnet_v2|synergy_testnet_v2|CHAIN_ID|NETWORK_ID" \
  /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/synergy_types.rs \
  /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src -g '*.rs'
```

Output excerpt:

```text
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/synergy_types.rs:6:pub const SYNERGY_TESTNET_V2_CHAIN_ID: u64 = 1264;
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/synergy_types.rs:7:pub const SYNERGY_TESTNET_V2_NETWORK_ID: &str = "synergy-testnet-v2";
```

Command:

```sh
rg -n "AIVM|synq_|deploy|call|sendTransaction|submitAegis|Receipt|receipt|estimateGas|estimatePq|verifyPayload|getSecurityPolicy|Contract" \
  /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/rpc/rpc_server.rs \
  /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src -g '*.rs'
```

Output excerpt:

```text
src/rpc/rpc_server.rs:1545:        "synergy_sendTransaction" => {
src/rpc/rpc_server.rs:1612:        "synergy_submitAegisTransaction" | "synergy_submitAegisDagTransaction" => {
src/rpc/rpc_server.rs:2430:        // TEMPORARILY DISABLED:         // AIVM - Artificial Intelligence Virtual Machine Methods
src/rpc/rpc_server.rs:2932:        "synergy_getTransactionReceipt" => {
src/rpc/rpc_server.rs:3119:        "synergy_call" => {
src/rpc/rpc_server.rs:3156:        "synergy_estimateGas" => {
src/rpc/rpc_server.rs:3184:        "synergy_getReceipt" => transaction_receipt_json(&params, chain),
```

Command:

```sh
sed -n '150,270p' /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/synq/compiler.rs
```

Source evidence summary:

```text
The local Testnet-Beta SynQ compiler prototype generates PQC-related signatures
locally and Solidity compatibility text locally. It does not call `aegis-pqsynq`.
This is duplicate/prototype behavior that should not become the canonical
deploy/call security policy.
```

Document created:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/architecture/aegis-pqsynq-vs-aegis-pqvm-integration.md
```

Decision recorded:

```text
Both modules are needed. Current source-proven architecture is Model D:
Testnet-Beta uses aegis-pqvm for chain/node admission and does not yet call
aegis-pqsynq. Recommended target is Model B: node/AIVM calls aegis-pqsynq for
inner SynQ deploy/call payload verification and keeps aegis-pqvm as the outer
blockchain transaction/DAG/VM admission owner.
```

Verification commands:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon cargo fmt --all --check
```

Workdir: `/Volumes/xcode/Synergy-Network-Projects/synq-language`

Result: PASS, exit code 0.

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon cargo test -p aegis-pqsynq --test synq_verifier_tests --locked
```

Output excerpt:

```text
running 17 tests
test valid_mldsa65_deploy_verifies ... ok
test valid_mldsa65_call_verifies ... ok
test wrong_chain_fails ... ok
test wrong_domain_fails ... ok
test invalid_signature_fails ... ok
test transaction_dispatch_preserves_security_errors ... ok
test result: ok. 17 passed; 0 failed
```

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-pqvm-audit cargo test --manifest-path /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml --test integrations_dispatch --test vm_validation --test security_smoke --locked
```

Output excerpt:

```text
Running tests/integrations_dispatch.rs
running 6 tests
test mldsa_verify_detached_returns_true_then_false_on_tamper ... ok
test fndsa_verify_detached_returns_true ... ok
test result: ok. 6 passed

Running tests/security_smoke.rs
running 3 tests
test mldsa_smoke_tamper_message_or_signature_fails ... ok
test fndsa_smoke_tamper_message_or_signature_fails ... ok
test result: ok. 3 passed

Running tests/vm_validation.rs
running 15 tests
test evm_precompile_rejects_invalid_magic ... ok
test substrate_dispatch_rejects_invalid_payload ... ok
test cosmwasm_call_accepts_bound_payload ... ok
test result: ok. 15 passed
```

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-testnet-audit cargo test -p synergy-testnet aegis_tx_tool::tests -- --nocapture
```

Result: BLOCKED before tests by local disk capacity, not by a Rust assertion failure.

Output excerpt:

```text
error: failed to write /Volumes/xcode/cargo-target-testnet-audit/debug/deps/libreqwest-...rmeta: No space left on device (os error 28)
warning: pqcrypto-falcon@0.3.0: ar: .../libfalcon-1024_aarch64.a: No space left on device
error: could not compile `reqwest` (lib) due to 1 previous error
error: could not compile `hickory-resolver` (lib) due to 1 previous error
error: could not compile `chrono` (lib) due to 1 previous error
```

Command:

```sh
df -h /Volumes/xcode /tmp /Users/devpup
```

Output excerpt:

```text
Filesystem      Size    Used   Avail Capacity
/dev/disk7s1   237Gi   237Gi    32Mi   100%   /Volumes/xcode
/dev/disk3s1   228Gi   177Gi   5.1Gi    98%   /System/Volumes/Data
```

Final git status command:

```sh
git status --short --branch
```

Output excerpt for `/Volumes/xcode/Synergy-Network-Projects/project-rosetta`:

```text
## main...origin/main
 M crates/rosetta-verification/src/lib.rs
 M crates/rosetta-verification/src/synergy.rs
 M docs/synergy-testnet-integration.md
```

Output excerpt for `/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`:

```text
## main...origin/main
 M src/consensus/consensus_algorithm.rs
?? scripts/testnet/incident-install-runtime-only-v13.sh
?? scripts/testnet/incident-relayer1-build-recovery-plan.sh
?? scripts/testnet/incident-val1-build-recovery-plan.sh
?? scripts/testnet/incident-val5-build-recovery-plan.sh
```

Note: the modified consensus file and untracked incident scripts were not
created or edited during this comparative audit.

## TESTNET-Beta Local Source Inspection

Source root inspected:

```text
/Users/devpup/Desktop/Testnet-Beta
```

Implementation source of truth inspected:

```text
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet
```

Directory inventory:

```text
Git roots found under /Users/devpup/Desktop/Testnet-Beta:
synergy-prism
synergy-horizon
synergy-testnet/node-control-panel
synergy-testnet
synergy-address-engine
synergy-learn
explorer-app
synergy-vault
meme-launchpad
synergy-quest
synergy-forge
synq-language
```

Chain constants discovered:

```text
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/synergy_types.rs
pub const SYNERGY_TESTNET_V2_CHAIN_ID: u64 = 1264;
pub const SYNERGY_TESTNET_V2_NETWORK_ID: &str = "synergy-testnet-v2";
```

RPC methods discovered from
`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/rpc/rpc_server.rs`:

```text
Public read examples:
synergy_chainId
synergy_networkId
synergy_genesisHash
synergy_protocolVersion
synergy_syncing
synergy_getHealth
synergy_getReadiness
synergy_getPeers
synergy_getAegisStatus
synergy_getAegisCapabilities
synergy_verifyAegisTransaction
synergy_getDagStatus
synergy_getDagFrontier
synergy_getDagGraph
synergy_getDagDependencies
synergy_getDagTxOrderRoot
synergy_getTransactionReceipt
synergy_getReceipt
synergy_getBlockReceipts
synergy_getLogs
synergy_getCode
synergy_getStorageAt

Public client examples:
synergy_simulateTransaction
synergy_sendTransaction
synergy_submitAegisTransaction
synergy_submitAegisTransactionBatch
synergy_submitAegisDagTransaction
synergy_submitAegisDagTransactionBatch
synergy_call
synergy_estimateGas
synergy_registerSynID
```

Transaction envelope sources:

```text
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/transaction.rs
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/rpc/rpc_server.rs
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/aegis_tx_tool.rs
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/synergy_types.rs
```

Receipt shapes discovered:

```text
synergy_getReceipt fields:
transactionHash, transactionIndex, blockHash, blockNumber, from, to,
cumulativeGasUsed, gasUsed, effectiveGasPrice, feeCharged, feeCollector,
status, logs, chain

synergy_getTransactionReceipt fields:
transactionHash, transactionIndex, blockHash, blockNumber, from, to,
cumulativeGasUsed, gasUsed, effectiveGasPrice, status, logs, logsBloom,
contractAddress

chain fields:
name, chain_id, chain_id_hex, network_id, genesis_hash
```

Validation flow discovered:

```text
synergy_sendTransaction -> normalize_rpc_transaction -> chainId check ->
Transaction::validate_for_admission -> embedded PQC verification or legacy
Aegis carrier verification -> TX_POOL queue -> optional P2P broadcast

synergy_submitAegisTransaction / synergy_submitAegisDagTransaction ->
AegisTxSubmissionEnvelope -> legacy_transaction_from_aegis_envelope ->
chain check -> canonical transaction bytes -> Aegis PQVM SYNERGY_TX_V1 hash ->
TX_POOL queue -> optional P2P broadcast
```

AIVM status discovered:

```text
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/aivm/runtime.rs
defines AIVMExecutionContext, AIVMExecutionResult, AIVMContract, and AIVMRuntime.

/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/rpc/rpc_server.rs
contains synergy_deployAIVMContract and synergy_executeAIVMContract handlers
commented out as TEMPORARILY DISABLED.
```

Artifact created:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/integration/testnet-source-inventory-2026-05-26.md
```

## Project Rosetta Real Synergy Receipt Mapping

Files changed:

```text
/Users/devpup/Desktop/Testnet-Beta/project-rosetta/crates/rosetta-core/src/domain.rs
/Users/devpup/Desktop/Testnet-Beta/project-rosetta/crates/rosetta-core/src/lib.rs
/Users/devpup/Desktop/Testnet-Beta/project-rosetta/crates/rosetta-verification/src/lib.rs
/Users/devpup/Desktop/Testnet-Beta/project-rosetta/crates/rosetta-verification/src/synergy.rs
/Users/devpup/Desktop/Testnet-Beta/project-rosetta/docs/synergy-testnet-integration.md
/Users/devpup/Desktop/Testnet-Beta/project-rosetta/sdk/typescript/src/route.ts
/Users/devpup/Desktop/Testnet-Beta/project-rosetta/sdk/typescript/test/synergy-domain.test.ts
/Volumes/xcode/Synergy-Network-Projects/project-rosetta/crates/rosetta-verification/src/lib.rs
/Volumes/xcode/Synergy-Network-Projects/project-rosetta/crates/rosetta-verification/src/synergy.rs
/Volumes/xcode/Synergy-Network-Projects/project-rosetta/docs/synergy-testnet-integration.md
```

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-rosetta-canon cargo test -p rosetta-verification synergy::tests -- --nocapture
```

Workdir:
`/Volumes/xcode/Synergy-Network-Projects/project-rosetta`

Output excerpt:

```text
running 9 tests
test synergy::tests::real_synergy_get_receipt_shape_maps_to_rosetta_receipt ... ok
test synergy::tests::rpc_receipt_hash_is_deterministic ... ok
test synergy::tests::mock_synergy_route_scenario_outputs_deterministic_index_record ... ok
test result: ok. 9 passed; 0 failed
```

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-rosetta-testnet-user cargo test -p rosetta-verification synergy::tests -- --nocapture
```

Workdir:
`/Users/devpup/Desktop/Testnet-Beta/project-rosetta`

Output excerpt:

```text
running 9 tests
test synergy::tests::real_synergy_get_receipt_shape_maps_to_rosetta_receipt ... ok
test synergy::tests::rpc_receipt_hash_is_deterministic ... ok
test synergy::tests::mock_synergy_route_scenario_outputs_deterministic_index_record ... ok
test result: ok. 9 passed; 0 failed
```

Command:

```sh
corepack pnpm --filter @synergy/rosetta-sdk test
```

Workdir:
`/Users/devpup/Desktop/Testnet-Beta/project-rosetta`

Output excerpt:

```text
Test Files  4 passed (4)
Tests  5 passed (5)
```

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-rosetta-canon cargo clippy -p rosetta-verification --all-targets -- -D warnings
```

Workdir:
`/Volumes/xcode/Synergy-Network-Projects/project-rosetta`

Result: PASS.

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-rosetta-testnet-user cargo clippy -p rosetta-verification --all-targets -- -D warnings
```

Workdir:
`/Users/devpup/Desktop/Testnet-Beta/project-rosetta`

Result: PASS.

## TESTNET-Beta Targeted Rust Verification

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-testnet-exec cargo test -p synergy-testnet execution::tests -- --nocapture
```

Workdir:
`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Output excerpt:

```text
running 3 tests
test execution::tests::missing_or_altered_authorization_context_fails_closed ... ok
test execution::tests::failed_receipt_is_deterministic_and_conflicts_execute_in_order ... ok
test execution::tests::same_block_executed_repeatedly_produces_same_state_root ... ok
test result: ok. 3 passed; 0 failed
```

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-testnet-exec cargo test -p synergy-testnet launch_ -- --nocapture
```

Workdir:
`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Output excerpt:

```text
running 9 tests
test rpc::rpc_server::tests::public_gateway_allows_launch_aegis_submit_methods ... ok
test rpc::rpc_server::tests::public_gateway_allows_launch_read_rpc_surface ... ok
test rpc::rpc_server::tests::launch_identity_rpc_reports_canonical_testnet_identity ... ok
test result: ok. 9 passed; 0 failed
```

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-testnet-exec cargo test -p synergy-testnet aegis_tx_tool::tests -- --nocapture
```

Workdir:
`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Output excerpt:

```text
running 4 tests
test aegis_tx_tool::tests::altered_transaction_bytes_fail_aegis_verification ... ok
test aegis_tx_tool::tests::real_aegis_transaction_key_signs_verifies_and_admits_to_dag ... ok
test aegis_tx_tool::tests::aegis_submission_envelope_rejects_wrong_chain_and_carrier_tampering ... ok
test aegis_tx_tool::tests::fixture_uses_dependencies_and_no_wallet_cli_path ... ok
test result: ok. 4 passed; 0 failed
```

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-testnet-exec cargo test -p synergy-testnet transaction::tests::admission_requires_testnet_context_and_real_pqc_signature -- --nocapture
```

Workdir:
`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet`

Output excerpt:

```text
running 1 test
test transaction::tests::admission_requires_testnet_context_and_real_pqc_signature ... ok
test result: ok. 1 passed; 0 failed
```

Note: these commands emitted existing dead-code warnings in the
`synergy-testnet` crate. No test failed.

## Local RPC Port Probe

Command:

```sh
lsof -nP -iTCP:5640 -sTCP:LISTEN
```

Result: no listener, exit code 1.

Command:

```sh
lsof -nP -iTCP:5641 -sTCP:LISTEN
```

Result: no listener, exit code 1.

Command:

```sh
curl -sS --max-time 3 -H 'Content-Type: application/json' --data '{"jsonrpc":"2.0","id":1,"method":"synergy_chainId","params":[]}' http://127.0.0.1:5640
```

Output:

```text
curl: (7) Failed to connect to 127.0.0.1 port 5640 after 0 ms: Couldn't connect to server
```

Command:

```sh
curl -sS --max-time 3 -H 'Content-Type: application/json' --data '{"jsonrpc":"2.0","id":1,"method":"synergy_chainId","params":[]}' http://127.0.0.1:5641
```

Output:

```text
curl: (7) Failed to connect to 127.0.0.1 port 5641 after 0 ms: Couldn't connect to server
```

Conclusion: no local RPC endpoint was already listening on qRPC ports 5640 or
5641. I did not start an ad hoc node/validator, so `NET-007` remains partial
with source-backed RPC matrix proof but no active local request/response proof.

## Follow-Up Canonical Register Work

Command:

```sh
rg -n "Canonical status|canonical register|SynQ-Aegis-AIVM-Canonical" checklist.md INTEGRATION_STATUS.md docs/PQSynQ-Completion-Execution-Plan.md docs/SynQ-Project-Status-and-Execution-Backlog.md docs/SynQ-Security-Audit-Manual.md docs/management/Definition-of-Done.md docs/management/SynQ-Release-Milestones.md docs/management/Weekly-Status-Template.md
```

Output excerpt:

```text
checklist.md:3:> Canonical status note (2026-05-26): this file is a historical planning
INTEGRATION_STATUS.md:3:> Canonical status note (2026-05-26): this file is a historical integration
docs/PQSynQ-Completion-Execution-Plan.md:3:> Canonical status note (2026-05-26): this file is a historical execution plan.
docs/SynQ-Project-Status-and-Execution-Backlog.md:3:> Canonical status note (2026-05-26): this backlog is superseded by
docs/SynQ-Security-Audit-Manual.md:3:> Canonical status note (2026-05-26): this manual remains useful security
docs/management/Definition-of-Done.md:3:> Canonical status note (2026-05-26): this DoD remains policy guidance.
docs/management/SynQ-Release-Milestones.md:3:> Canonical status note (2026-05-26): this release plan is a historical planning
docs/management/Weekly-Status-Template.md:3:> Canonical status note (2026-05-26): this template is for reporting cadence
```

Result: PASS. Old checklist/status/planning files now point to the canonical register or explicitly state they are historical/non-authoritative for completion claims.

## Aegis SynQ Verifier Expansion

Changed files:

- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/signature.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/synq_verifier_tests.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/synq_deploy_vector_tests.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_valid_deploy.json`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_invalid_signature.json`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_wrong_chain.json`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_wrong_domain.json`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/examples/generate_synq_verifier_vectors.rs`
- `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/examples/synq_deploy_call_verifier.rs`

Standalone mirror copies were also updated under:

- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/`

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon cargo fmt --check
```

Result: PASS.

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon cargo test -p aegis-pqsynq --test synq_verifier_tests --test synq_deploy_vector_tests --locked
```

Output excerpt:

```text
Running tests/synq_deploy_vector_tests.rs
running 1 test
test ml_dsa_65_deploy_vectors_replay ... ok
test result: ok. 1 passed

Running tests/synq_verifier_tests.rs
running 17 tests
test valid_mldsa65_deploy_verifies ... ok
test valid_mldsa65_call_verifies ... ok
test invalid_signature_fails ... ok
test malformed_public_key_fails ... ok
test malformed_signature_fails ... ok
test wrong_chain_fails ... ok
test wrong_domain_fails ... ok
test wrong_algorithm_fails ... ok
test wrong_signer_fails ... ok
test expired_payload_fails ... ok
test oversized_public_key_fails ... ok
test oversized_signature_fails ... ok
test deploy_domain_cannot_be_replayed_as_call ... ok
test call_payload_cannot_be_replayed_as_wallet_auth ... ok
test testnet_payload_cannot_be_replayed_as_mainnet ... ok
test transaction_dispatch_preserves_security_errors ... ok
test transaction_dispatch_verifies_deploy ... ok
test result: ok. 17 passed
```

Result: PASS.

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon cargo run -p aegis-pqsynq --example synq_deploy_call_verifier --locked
```

Output excerpt:

```text
chain_id=1264
network=synergy-testnet
address=tsynq10104f00102a3a9149fe99438455b9e633aedd3441b835a6901fc89d2b72772b2c4848ac80cd6a1a411
deploy=verified
call=verified
```

Result: PASS.

Command:

```sh
rg -n "secret|private|sk|seed" aegis-pqsynq/pqsynq/tests/vectors/ml_dsa_65_*.json || true
```

Output: no matches.

Result: PASS. The new ML-DSA-65 deploy vector fixtures do not contain private-key or seed fields.

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon cargo test -p aegis-pqsynq --all-features --all-targets --locked
```

Output excerpt:

```text
Running tests/synq_deploy_vector_tests.rs
test ml_dsa_65_deploy_vectors_replay ... ok

Running tests/synq_verifier_tests.rs
running 17 tests
test result: ok. 17 passed

Running unittests examples/synq_deploy_call_verifier.rs
test result: ok. 0 passed
```

Result: PASS. Shell startup emitted two `/tmp` space warnings from `rbenv-init`, but the Cargo command completed successfully with exit code 0 using `TMPDIR=/Volumes/xcode/tmp`.

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon cargo test --workspace --all-features --all-targets --locked
```

Output excerpt:

```text
Running tests/synq_deploy_vector_tests.rs
test ml_dsa_65_deploy_vectors_replay ... ok
Running tests/synq_verifier_tests.rs
test result: ok. 17 passed
Running tests/integration_test.rs
test result: ok. 6 passed
Running tests/semantic_test.rs
test result: ok. 15 passed
Running tests/integration_test.rs
test result: ok. 7 passed
```

Result: PASS.

## Final Git Status Recheck

Commands:

```sh
git rev-parse --show-toplevel 2>&1 || true
git status --short --branch 2>&1 || true
```

Results:

- `/Volumes/xcode/Synergy-Network-Projects/synq-language`: local Git metadata unavailable; command returned `fatal: not a git repository`.
- `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq`: local Git metadata unavailable; command returned `fatal: not a git repository`.
- `/Volumes/xcode/Synergy-Network-Projects/synergy-aivm`: local Git metadata unavailable; command returned `fatal: not a git repository`.
- `/Volumes/xcode/Synergy-Network-Projects/project-rosetta`: Git root `/Volumes/xcode/Synergy-Network-Projects/project-rosetta`; status `## main...origin/main` with no dirty entries.

## aegis-pqsynq

Active buildable crate: `/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq`

Standalone mirror: `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq`

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-synq-canon cargo test -p aegis-pqsynq --all-features --all-targets --locked
```

Workdir: `/Volumes/xcode/Synergy-Network-Projects/synq-language`

Output excerpts:

```text
Running tests/basic_tests.rs
running 3 tests
test test_mlkem768_basic ... ok
test test_mldsa65_basic ... ok
test test_fndsa512_basic ... ok

Running tests/kat_tests.rs
running 5 tests
test test_mldsa_known_message_matrix ... ok
test test_hqckem_size_and_roundtrip_matrix ... ok

Running tests/nist_vector_replay_tests.rs
running 4 tests
test test_nist_fndsa_replay_vectors ... ok
test test_nist_hqckem_replay_vectors ... ok
test test_nist_mldsa_replay_vectors ... ok
test test_nist_mlkem_replay_vectors ... ok

Running tests/synq_verifier_tests.rs
running 8 tests
test transaction_dispatch_preserves_security_errors ... ok
test transaction_dispatch_verifies_deploy ... ok
test valid_mldsa65_deploy_verifies ... ok
test valid_mldsa65_call_verifies ... ok

Running benches/pqc_benchmarks.rs
Testing ML-DSA Performance/ML-DSA-65_keygen/
Success
Testing ML-DSA Performance/ML-DSA-65_sign/
Success
Testing ML-DSA Performance/ML-DSA-65_verify/
Success
```

Result: PASS.

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-aegis-standalone-canon \
  cargo test --manifest-path /Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/Cargo.toml --test canonical_payload_tests --locked
```

Output:

```text
error: failed to parse manifest at `/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq/Cargo.toml`

Caused by:
  error inheriting `pqrust-fndsa` from workspace root manifest's `workspace.dependencies.pqrust-fndsa`

Caused by:
  failed to find a workspace root
```

Result: BLOCKED for standalone mirror until a real workspace root is restored. Source additions were mirrored; the active embedded workspace is the build-proven copy.

Command:

```sh
diff -qr /Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq \
  /Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq/aegis-pqsynq/pqsynq
```

Output excerpt:

```text
Files .../tests/nist_vector_replay_tests.rs and .../tests/nist_vector_replay_tests.rs differ
```

## Synergy-AIVM

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-aivm-core-canon cargo test
```

Workdir: `/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core`

Output excerpt:

```text
running 3 tests
test execution::tests::executes_synq_bytecode_artifacts ... ok
test vm::wasm_runner::tests::loads_minimal_wasm_module ... ok
test execution::tests::accepts_wasm_artifacts ... ok
test result: ok. 3 passed
```

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-aivm-node-canon cargo test
```

Workdir: `/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-node`

Output excerpt:

```text
running 0 tests
test result: ok. 0 passed
```

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-aivm-verifier-canon cargo test
```

Workdir: `/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/verifier/verifier-core`

Output excerpt:

```text
running 2 tests
test attestation::tests::detects_incomplete_reports ... ok
test verifier::tests::incomplete_attestation_is_not_trusted ... ok
test result: ok. 2 passed
```

## Project Rosetta

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-rosetta-canon cargo fmt --check
```

Result: PASS, exit code 0.

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-rosetta-canon cargo test --workspace
```

Output excerpts:

```text
Running unittests src/lib.rs (.../rosetta_verification-...)
running 10 tests
test synergy::tests::mock_synergy_receipt_verifier_consumes_chain_1264_receipts ... ok
test synergy::tests::receipt_hash_is_deterministic ... ok
test synergy::tests::unfinalized_receipt_is_rejected ... ok
test synergy::tests::bad_receipt_hash_is_rejected ... ok
test synergy::tests::wrong_chain_is_rejected ... ok

Running unittests src/lib.rs (.../rosetta_core-...)
running 3 tests
test domain::tests::synergy_testnet_domain_is_chain_1264 ... ok

Running unittests src/lib.rs (.../rosetta_settlement-...)
running 14 tests
test successful_escrow_swap ... ok
```

Result: PASS.

Command:

```sh
TMPDIR=/Volumes/xcode/tmp CARGO_TARGET_DIR=/Volumes/xcode/cargo-target-rosetta-canon cargo clippy --workspace --all-targets -- -D warnings
```

Output excerpt:

```text
Checking rosetta-core v0.1.0
Checking rosetta-verification v0.1.0
Checking rosetta-cli v0.1.0
Finished `dev` profile [unoptimized + debuginfo]
```

Result: PASS.

Command:

```sh
corepack pnpm --filter @synergy/rosetta-sdk typecheck
```

Output excerpt:

```text
> @synergy/rosetta-sdk@0.1.0 typecheck .../sdk/typescript
> tsc --noEmit
```

Result: PASS.

Command:

```sh
corepack pnpm --filter @synergy/rosetta-sdk test
```

Output excerpt:

```text
Test Files  4 passed (4)
Tests  5 passed (5)
```

Result: PASS.

## Final cleanup and verification rerun after approved cache deletion

Timestamp: 2026-05-26 17:25 CDT.

Cleanup:

```sh
du -sh /Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration
rm -rf -- /Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration
```

Output excerpt:

```text
2.6G    /Users/devpup/Desktop/Testnet-Beta/.cargo-target-synq-integration
deleted approved cache
```

Only the approved integration-attempt build cache was removed. The shared
`/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target` directory and
source/docs/repos/configs/keys/user data were not deleted.

Environment used for the required reruns:

```sh
export CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
export CARGO_BUILD_JOBS=2
```

Command:

```sh
cargo test --manifest-path /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml --test integrations_dispatch --test vm_validation --test security_smoke --locked
```

Output excerpt:

```text
Running tests/integrations_dispatch.rs
running 6 tests
test result: ok. 6 passed

Running tests/security_smoke.rs
running 3 tests
test result: ok. 3 passed

Running tests/vm_validation.rs
running 15 tests
test result: ok. 15 passed
```

Result: PASS. The `_PQRUST_RUST_randombytes` compatibility shim in
`aegis-pqvm/src/utils.rs` is validated by the targeted pqvm dispatch,
validation, and smoke tests.

Command:

```sh
cargo test -p synergy-testnet --lib synq_admission::tests -- --nocapture
```

Output excerpt:

```text
running 10 tests
test synq_admission::tests::valid_deploy_carrier_verifies_with_pqsynq ... ok
test synq_admission::tests::valid_call_carrier_verifies_with_pqsynq ... ok
test synq_admission::tests::valid_deploy_carrier_passes_pqsynq_then_existing_pqvm_admission ... ok
test synq_admission::tests::valid_call_carrier_passes_pqsynq_then_existing_pqvm_admission ... ok
test result: ok. 10 passed; 432 filtered out
```

Result: PASS.

Command:

```sh
cargo test -p synergy-testnet --lib execution::tests::receipt_preserves_synq_verification_summary -- --nocapture
```

Output excerpt:

```text
running 1 test
test execution::tests::receipt_preserves_synq_verification_summary ... ok
test result: ok. 1 passed; 441 filtered out
```

Result: PASS.

Command:

```sh
cargo test -p aegis-pqsynq --test synq_verifier_tests --locked
```

Workdir:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language
```

Output excerpt:

```text
running 17 tests
test valid_mldsa65_deploy_verifies ... ok
test valid_mldsa65_call_verifies ... ok
test transaction_dispatch_verifies_deploy ... ok
test transaction_dispatch_preserves_security_errors ... ok
test result: ok. 17 passed
```

Result: PASS.

Command:

```sh
cargo fmt --all --check
```

Result: PASS.

Register items newly marked complete after this final verification:

- `ARCH-001`
- `AIVM-006`
- `AIVM-007`
- `NET-003`
- `NET-004`
- `NET-006`

Items deliberately left open:

- `NET-007`: no local RPC listener was started and no real request/response
  examples were captured.
- Public RPC/indexer/explorer exposure for the internal SynQ receipt summary
  fields.
- Enabled AIVM deploy/call execution, ABI method lookup, PQ-Gas accounting, and
  trap-code exposure.

Final git status commands:

```sh
git -C /Users/devpup/Desktop/Testnet-Beta/synergy-testnet status --short --branch
git -C /Volumes/xcode/Synergy-Network-Projects/project-rosetta status --short --branch
git -C /Volumes/xcode/Synergy-Network-Projects/synq-language status --short --branch
git -C /Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq status --short --branch
git -C /Volumes/xcode/Synergy-Network-Projects/synergy-aivm status --short --branch
```

Output excerpts:

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

## 2026-05-31 AIVM, SynQ, and aegis-pqsynq Continuation

Environment used for Rust verification:

```sh
export CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
export CARGO_BUILD_JOBS=2
```

### aegis-pqsynq Payload Decoder And Fuzzing

Files:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/serialization.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/algorithms.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/src/domain.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/canonical_payload_tests.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/aegis-pqsynq/pqsynq/tests/payload_decode_fuzz_tests.rs
```

Command:

```sh
cargo test -p aegis-pqsynq --test synq_verifier_tests --test canonical_payload_tests --test payload_decode_fuzz_tests --locked
```

Result: PASS.

Output excerpt:

```text
running 4 tests
test canonical_payload_bytes_are_deterministic_and_versioned ... ok
test canonical_payload_decoder_rejects_trailing_bytes ... ok
test canonical_payload_rejects_oversized_length_prefixed_fields ... ok
test signing_payload_hash_changes_with_domain_and_chain_binding ... ok

running 2 tests
test canonical_payload_roundtrips ... ok
test decoder_never_panics_on_arbitrary_bytes ... ok

running 17 tests
test valid_mldsa65_deploy_verifies ... ok
test valid_mldsa65_call_verifies ... ok
test wrong_chain_fails ... ok
test wrong_domain_fails ... ok
test invalid_signature_fails ... ok
test transaction_dispatch_preserves_security_errors ... ok
test result: ok. 17 passed
```

### SynQ Counter Example And Solidity Compatibility Label

Files:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/examples/Counter.synq
/Volumes/xcode/Synergy-Network-Projects/synq-language/examples/Counter.compiled.synq
/Volumes/xcode/Synergy-Network-Projects/synq-language/examples/Counter.sol
/Volumes/xcode/Synergy-Network-Projects/synq-language/compiler/src/solidity_gen.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/src/main.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/tests/integration_test.rs
```

Generated Solidity compatibility warning:

```text
WARNING: Solidity compatibility preview only; not production deployable.
```

Command:

```sh
cargo test -p cli --test integration_test --locked
```

Result: PASS.

Output excerpt:

```text
running 8 tests
test test_compile_counter_example ... ok
test test_solidity_output_is_labeled_non_production ... ok
test test_compile_all_documented_examples ... ok
test test_compile_and_run ... ok
test test_verify_accepts_matching_bytecode_and_executes ... ok
test result: ok. 8 passed
```

`LANG-014` remains unchecked because Counter ABI and manifest artifacts are not
generated yet.

### AIVM Execution Context, Determinism, Receipts, And Error Taxonomy

Files:

```text
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/Cargo.toml
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/Cargo.lock
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/src/error.rs
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/src/execution.rs
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/src/lib.rs
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/src/vm/wasm_runner.rs
```

Initial command:

```sh
cargo test
```

Result: unit tests passed, then doctest was killed by SIGKILL during rustdoc.
No unit-test failure was observed. The targeted library gate below was rerun
with `--locked` after the lockfile refresh.

Command:

```sh
cargo test --lib --locked
```

Result: PASS.

Output excerpt:

```text
running 8 tests
test execution::tests::execution_context_carries_chain_and_metering_inputs ... ok
test execution::tests::executes_synq_bytecode_artifacts ... ok
test execution::tests::invalid_synq_bytecode_returns_bytecode_error_code ... ok
test execution::tests::receipt_canonical_hash_is_deterministic ... ok
test execution::tests::rejects_wasm_host_imports_with_structured_error_code ... ok
test vm::wasm_runner::tests::rejects_wasm_host_imports ... ok
test result: ok. 8 passed
```

Formatting:

```sh
cargo fmt --all --check
cargo fmt --check
```

Result: PASS for the SynQ workspace and the `aivm-core` crate.

### Register Updates

Marked complete:

- `AEGIS-019`
- `LANG-021`
- `AIVM-004`
- `AIVM-005`
- `AIVM-008`
- `AIVM-011`

Left partial/unchecked:

- `LANG-014`: Counter source, bytecode, and compatibility Solidity exist; ABI
  and manifest artifacts are still missing.
- `AIVM-013`: valid SynQ bytecode, invalid SynQ bytecode, deterministic
  receipt hash, and host-import rejection tests exist; the remaining requested
  negative bytecode, gas/PQ-Gas exhaustion, manifest/signature, Counter
  execution, state rollback, and replay tests are still missing.

Final git status sweep:

```text
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet:
## main...origin/main

/Volumes/xcode/Synergy-Network-Projects/project-rosetta:
## main...origin/main

/Volumes/xcode/Synergy-Network-Projects/synq-language:
fatal: not a git repository at that exact path

/Volumes/xcode/Synergy-Network-Projects/synergy-aivm:
fatal: not a git repository at that exact path

/Volumes/xcode/Synergy-Network-Projects/aegis-pqsynq:
fatal: not a git repository at that exact path
```

## 2026-06-01 SynQ Counter AMA Vertical Slice

Environment:

```sh
export CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
export CARGO_BUILD_JOBS=2
```

### Deterministic SynQ ABI And Manifest Artifacts

Files:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/compiler/src/artifacts.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/compiler/tests/artifact_test.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/src/main.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/contracts/Counter.synq
/Volumes/xcode/Synergy-Network-Projects/synq-language/contracts/Counter.compiled.synq
/Volumes/xcode/Synergy-Network-Projects/synq-language/contracts/Counter.abi.json
/Volumes/xcode/Synergy-Network-Projects/synq-language/contracts/Counter.manifest.json
```

Commands:

```sh
cargo test -p compiler --test artifact_test --offline
cargo test -p cli --test integration_test --locked
cargo fmt --all --check
```

Result: PASS. Compiler artifact tests passed `2/2`; CLI integration tests passed
`8/8`; formatting passed.

Generated hashes:

```text
bytecode_hash=6b8b2d0d1433c0c4941bfc41054a58a004e9cc46e475926f0f70d3d309e92533
abi_hash=ea9c1f48cad5f0d39d299d854ba578f6909a8475093aa8c616b1ee186c599b26
manifest_hash=6334f5a98926f3c5eeb4f9337a9602841e5cc9b77b59f0e648203a296d290332
```

### AIVM State Overlay And Two-Lane Meter

Files:

```text
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/src/state.rs
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/src/metering.rs
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/src/execution.rs
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/examples/counter_state_demo.rs
```

Commands:

```sh
cargo fmt --check
cargo test --lib --locked
cargo run --quiet --example counter_state_demo
```

Result: PASS. AIVM library tests passed `14/14`. Example output:

```text
counter_before=0
counter_increment_return=1
counter_get_return=1
state_root=1bd01d6d3270341d2f6613b4de853e840575be6b82d71fccd8dd7da751fa344c
ordinary_gas_used=31
pq_gas_used=50000
receipt_hash=f5b8e3bde36244ea08aea937abca982db2b78d48ab3ad7162a503686202cb39c
```

## 2026-06-01 CLI PQSynQ Call Envelope Parity

Files:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/src/main.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/tests/integration_test.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/scripts/ama_demo_synq_counter.sh
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-User-Manual.md
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/demo/SynQ-Counter-AMA-Demo-2026-06-06.md
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/demo/SynQ-Counter-AMA-Demo-Visual.html
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/demo/SynQ-Counter-AMA-Expected-Output-2026-06-06.txt
```

Commands:

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

Result: PASS.

```text
CLI integration tests: 22/22
compiler artifact fixture tests: 6/6
pqsynq verifier tests: 17/17
cargo fmt --all --check: PASS
AMA demo script: PASS
```

Key demo output from the current proof run:

```text
[4/9] Exercise pqsynq-backed CLI deploy and call envelopes
synq_cli_keygen: PASS
synq_cli_sign_deploy: PASS
synq_cli_verify_deploy: PASS
synq_cli_sign_call_increment: PASS
synq_cli_verify_call_increment: PASS
synq_cli_sign_call_get: PASS
synq_cli_verify_call_get: PASS
cli_keygen=PASS
cli_deploy_envelope_verified=PASS
cli_increment_call_envelope_verified=PASS
cli_get_call_envelope_verified=PASS
cli_private_key_output=TEMPORARY_LOCAL_FILE_ONLY
```

What this proves:

- The SynQ CLI now signs and verifies local `SYNQ_CONTRACT_CALL_V1`
  `ContractCallEnvelope` JSON through `aegis-pqsynq`.
- The current no-argument Counter call subset covers `increment()` and `get()`.
- `verify-call` preserves structured `aegis-pqsynq` errors for wrong domain,
  invalid signature, and canonical payload mismatch tests.
- The AMA visual board was regenerated at
  `/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/demo/SynQ-Counter-AMA-Demo-Visual.html`.

What this does not prove:

- No public RPC listener or live TESTNET deploy/call was exercised.
- `synq call Counter.increment` and `synq call Counter.get` live submission
  commands are still pending.
- Public AIVM RPC handlers remain disabled.

## 2026-06-01 SynQ TOML Config Consumption

Files:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/compiler/src/artifacts.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/compiler/tests/artifact_test.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/Cargo.toml
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/src/main.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/tests/integration_test.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-User-Manual.md
/Volumes/xcode/Synergy-Network-Projects/synq-language/specs/synq-manifest-spec.md
```

Commands:

```sh
cd /Volumes/xcode/Synergy-Network-Projects/synq-language
export CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
export CARGO_BUILD_JOBS=2
cargo test -p compiler --test artifact_test --locked
cargo test -p cli --test integration_test --locked
cargo fmt --all --check
./scripts/ama_demo_synq_counter.sh
```

Result: PASS.

```text
compiler artifact tests: 6/6
CLI integration tests: 15/15
cargo fmt --all --check: PASS
demo script: PASS
```

Config-specific proof:

```text
test_build_consumes_synq_toml_network_alias ... ok
test_build_rejects_unsupported_synq_toml_chain ... ok
test_build_rejects_unsupported_synq_toml_signature_algorithm ... ok
manifest_generation_uses_explicit_artifact_config ... ok
```

The CLI now searches upward from the source file for `synq.toml`, validates
package/compiler/network/security sections, rejects unsupported chain IDs and
signature algorithms, and passes validated config into manifest generation.
No deploy/call/keygen command completion is claimed.

## 2026-06-01 pqsynq-backed CLI Keygen And Deploy Envelope Signing

Files:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/Cargo.toml
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/src/main.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/tests/integration_test.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-User-Manual.md
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/SynQ-Aegis-AIVM-Canonical-Tasks.md
```

Commands:

```sh
cd /Volumes/xcode/Synergy-Network-Projects/synq-language
export CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
export CARGO_BUILD_JOBS=2
cargo test -p cli --test integration_test --locked
cargo test -p compiler --test artifact_test --locked
cargo test -p aegis-pqsynq --test synq_verifier_tests --locked
cargo fmt --all --check
./scripts/ama_demo_synq_counter.sh
```

Result: PASS.

```text
CLI integration tests: 18/18
compiler artifact tests: 6/6
pqsynq verifier tests: 17/17
cargo fmt --all --check: PASS
demo script: PASS
```

CLI crypto proof:

```text
test_keygen_sign_deploy_and_verify_deploy_use_pqsynq ... ok
test_keygen_rejects_non_launch_signature_algorithm ... ok
test_verify_deploy_rejects_wrong_chain_before_pqsynq_context ... ok
```

The CLI now implements:

```text
synq keygen --algorithm ml-dsa-65 --network testnet
synq sign-deploy --bytecode ... --manifest ... --abi ... --private-key ... --output ...
synq verify-deploy <envelope.json>
```

These commands call `aegis-pqsynq` for ML-DSA-65 key generation, SynQ address
derivation, canonical deploy payload construction, signing, and verification.
The keygen command writes local key files but does not print private key bytes.
No live `synq deploy --chain 1264` transaction submission is claimed.

## 2026-06-01 AIVM Manifest-Aware SynQ Artifact Validation

Files:

```text
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/src/execution.rs
/Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core/examples/counter_state_demo.rs
```

Command:

```sh
cd /Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core
export CARGO_TARGET_DIR=/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/target
export CARGO_BUILD_JOBS=2
cargo fmt --check
cargo test --lib --locked
cargo run --quiet --example counter_state_demo
```

Result: PASS.

```text
AIVM library tests: 24/24
checked_in_counter_artifact_envelope_validates ... ok
checked_in_counter_artifact_rejects_tampered_manifest_hash ... ok
wrong_bytecode_hash_fails_with_bytecode_error ... ok
wrong_abi_hash_fails_with_abi_error ... ok
wrong_chain_and_network_fail_with_manifest_error ... ok
unsupported_artifact_format_fails_with_manifest_error ... ok
unsupported_signature_policy_fails_with_verification_error ... ok
node_testnet_v2_alias_is_accepted_for_chain_1264 ... ok
```

The Counter example also passed after adding a local manifest envelope:

```text
counter_before=0
counter_increment_return=1
counter_get_return=1
ordinary_gas_used=31
pq_gas_used=50000
receipt_hash=f5b8e3bde36244ea08aea937abca982db2b78d48ab3ad7162a503686202cb39c
```

The SynQ demo script was rerun and passed with the manifest-aware AIVM path:

```sh
cd /Volumes/xcode/Synergy-Network-Projects/synq-language
./scripts/ama_demo_synq_counter.sh
```

### Generated Counter Hashes Through pqsynq Then pqvm

File:

```text
/Users/devpup/Desktop/Testnet-Beta/synergy-testnet/src/synq_admission.rs
```

Commands:

```sh
bash /Volumes/xcode/Synergy-Network-Projects/synq-language/scripts/ama_demo_synq_counter.sh
cargo test -p synergy-testnet --lib synq_admission::tests -- --nocapture
cargo test -p synergy-testnet --lib execution::tests::receipt_preserves_synq_verification_summary -- --nocapture
cargo test --manifest-path /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml --test integrations_dispatch --test vm_validation --test security_smoke --locked
```

Result: PASS.

```text
Counter artifact-linked admission: 1/1
Testnet SynQ admission suite: 10 passed, 1 opt-in artifact test ignored by default
Testnet receipt summary preservation: 1/1
aegis-pqvm targeted suites: 24/24
pqsynq verifier suite invoked by demo script: 17/17
pqvm_outer_admission_ready=true
```

The script proves a local vertical slice. It does not claim a running public
RPC listener, live TESTNET deployment, persisted on-chain Counter state, or
enabled public AIVM RPC handlers.

## 2026-06-01 AMA Demo Polish, Visual Board, And CLI Workflow

Files:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/src/main.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/cli/tests/integration_test.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/compiler/tests/artifact_test.rs
/Volumes/xcode/Synergy-Network-Projects/synq-language/scripts/ama_demo_synq_counter.sh
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/demo/SynQ-Counter-AMA-Demo-2026-06-06.md
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/demo/SynQ-Counter-AMA-Demo-Visual.html
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/demo/SynQ-Counter-AMA-Expected-Output-2026-06-06.txt
```

Commands:

```sh
cargo fmt --all --check
cargo test -p aegis-pqsynq --test synq_verifier_tests --test canonical_payload_tests --test payload_decode_fuzz_tests --locked
cargo test -p cli --test integration_test --locked
cargo test -p compiler --test artifact_test --locked
```

Result: PASS.

```text
pqsynq tests: 23/23
CLI integration tests: 12/12
compiler artifact fixture tests: 5/5
```

AIVM:

```sh
cd /Volumes/xcode/Synergy-Network-Projects/synergy-aivm/runtime/aivm-core
cargo fmt --check
cargo test --lib --locked
```

Result: PASS, `14/14`.

Testnet-Beta:

```sh
cd /Users/devpup/Desktop/Testnet-Beta/synergy-testnet
cargo fmt --all --check
cargo test -p synergy-testnet --lib synq_admission::tests -- --nocapture
cargo test -p synergy-testnet --lib execution::tests -- --nocapture
cargo test -p synergy-testnet --lib synq_admission::tests::counter_artifacts_pass_pqsynq_then_existing_pqvm_admission -- --ignored --nocapture
```

Result: PASS.

```text
synq_admission default tests: 10 passed, 1 ignored
execution tests: 4/4
artifact-linked Counter admission: 1/1
```

`aegis-pqvm`:

```sh
cargo test --manifest-path /Users/devpup/Desktop/Testnet-Beta/synergy-testnet/aegis-pqvm/Cargo.toml --test integrations_dispatch --test vm_validation --test security_smoke --locked
```

Result: PASS, `24/24`.

Demo script:

```sh
cd /Volumes/xcode/Synergy-Network-Projects/synq-language
./scripts/ama_demo_synq_counter.sh --check
./scripts/ama_demo_synq_counter.sh
```

Result: PASS. The full demo wrote:

```text
/Volumes/xcode/Synergy-Network-Projects/synq-language/docs/demo/SynQ-Counter-AMA-Demo-Visual.html
```

Key output:

```text
chain_id=1264
network_id=synergy-testnet
signature_algorithm=ML-DSA-65
deploy_verification=PASS
call_verification=PASS
wrong_chain_error=AEGIS-CHAIN
wrong_domain_error=AEGIS-DOMAIN
invalid_signature_error=AEGIS-SIG
malformed_carrier_error=AEGIS-CANON
aegis_pqvm_outer_admission=PASS
counter_get_return=1
ordinary_gas_used=31
pq_gas_used=50000
receipt_hash=f5b8e3bde36244ea08aea937abca982db2b78d48ab3ad7162a503686202cb39c
```
