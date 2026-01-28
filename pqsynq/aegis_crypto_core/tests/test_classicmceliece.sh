#!/bin/sh
echo "Running Classic McEliece tests (experimental algorithm, disabled by default)..."
echo "⚠️  WARNING: Classic McEliece has not been officially selected by NIST for standardization"
echo "This algorithm is experimental and not recommended for production use."
export RUST_MIN_STACK=16777216  # 16MB stack for Classic McEliece
cargo test \
  --features cmce \
  --test cmce_native_tests \
  "$@"
