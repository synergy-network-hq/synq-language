#!/bin/sh
echo "Running native PQC tests with 8MB stack..."
echo "Note: Classic McEliece is disabled by default. Use --features cmce to enable it."
export RUST_MIN_STACK=8388608
cargo test \
  --test mldsa_native_tests \
  --test fndsa_native_tests \
  --test hqc_native_tests \
  --test mlkem_native_tests \
  --test slhdsa_native_tests \
  "$@"
