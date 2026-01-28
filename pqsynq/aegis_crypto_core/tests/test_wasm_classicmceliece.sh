#!/bin/sh
echo "Running Classic McEliece WASM tests (experimental algorithm, disabled by default)..."
echo "⚠️  WARNING: Classic McEliece has not been officially selected by NIST for standardization"
echo "This algorithm is experimental and not recommended for production use."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    cargo install wasm-pack
fi

# Run WASM tests using wasm-pack with Classic McEliece enabled
wasm-pack test --node --features cmce
