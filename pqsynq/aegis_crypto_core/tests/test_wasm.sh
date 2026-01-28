#!/bin/sh
echo "Running WASM PQC tests..."
echo "Note: Classic McEliece is disabled by default. Use --features cmce to enable it."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    cargo install wasm-pack
fi

# Run WASM tests using wasm-pack (excluding Classic McEliece by default)
wasm-pack test --node --no-default-features --features "mlkem mldsa fndsa slhdsa hqc"
