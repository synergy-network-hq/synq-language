#!/usr/bin/env bash
set -euo pipefail

# Run only WASM-targeted tests (those that are conditionally compiled for wasm32
# and generally use wasm-bindgen-test) under a local Wasmtime runner.
#
# Requirements:
# - rustup target add wasm32-wasip1
# - WASI SDK installed and env var WASI_SDK_DIR set (e.g. $HOME/wasi-sdk-22.0)
# - Wasmtime installed and on PATH (e.g. $HOME/.wasmtime/bin)
#
# This script:
# 1) Ensures wasm32-wasip1 target is installed
# 2) Verifies WASI_SDK_DIR and Wasmtime availability
# 3) Builds tests for wasm32-wasip1 with feature "wasm"
# 4) Executes only the *wasm_tests-*.wasm artifacts via wasmtime
#
# Native test artifacts (*_native_tests.wasm) are intentionally skipped because
# they may use threads/stack manipulation not supported by WASI.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[INFO] Using ROOT_DIR: $ROOT_DIR"

if ! rustup target list --installed | grep -q 'wasm32-wasip1'; then
  echo "[INFO] Installing wasm32-wasip1 target..."
  rustup target add wasm32-wasip1
fi

: "${WASI_SDK_DIR:?WASI_SDK_DIR environment variable must be set, e.g. export WASI_SDK_DIR=$HOME/wasi-sdk-22.0}"
WASI_SYSROOT="${WASI_SDK_DIR}/share/wasi-sysroot"
if [ ! -x "${WASI_SDK_DIR}/bin/clang" ]; then
  echo "[ERROR] clang not found at ${WASI_SDK_DIR}/bin/clang"
  exit 1
fi
if [ ! -d "${WASI_SYSROOT}/include" ]; then
  echo "[ERROR] WASI sysroot include not found at ${WASI_SYSROOT}/include"
  exit 1
fi

if ! command -v wasmtime >/dev/null 2>&1; then
  echo "[ERROR] wasmtime not found on PATH. Ensure it is installed, e.g.:"
  echo "  curl -fL https://github.com/bytecodealliance/wasmtime/releases/download/v19.0.2/wasmtime-v19.0.2-x86_64-linux.tar.xz -o /tmp/wasmtime.tar.xz"
  echo "  mkdir -p \$HOME/.wasmtime/bin && tar -xJf /tmp/wasmtime.tar.xz -C /tmp && cp /tmp/wasmtime-*/wasmtime \$HOME/.wasmtime/bin && export PATH=\$HOME/.wasmtime/bin:\$PATH"
  exit 1
fi

echo "[INFO] Building tests for wasm32-wasip1 with feature: wasm"
export CC="${WASI_SDK_DIR}/bin/clang"
export CFLAGS="--target=wasm32-wasip1 --sysroot=${WASI_SYSROOT}"
cargo test -p aegis_crypto_core --no-run --target wasm32-wasip1 --features wasm

echo "[INFO] Running wasm-only test artifacts with wasmtime..."
shopt -s nullglob
ARTS=( target/wasm32-wasip1/debug/deps/*wasm_tests-*.wasm )
if [ ${#ARTS[@]} -eq 0 ]; then
  echo "[WARN] No wasm test artifacts matching *wasm_tests-*.wasm were found."
  echo "       Ensure your wasm test files are named like *_wasm_tests.rs and compiled with #[cfg(target_arch = \"wasm32\")]"
fi

for t in "${ARTS[@]}"; do
  echo "=== Running $t ==="
  wasmtime run --dir=. --env RUST_BACKTRACE=1 "$t"
done

echo "[INFO] Done running wasm tests via Wasmtime."
