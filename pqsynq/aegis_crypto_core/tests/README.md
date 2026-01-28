# Aegis Crypto Core - Test Suite

> **Comprehensive test suite for Aegis Post-Quantum Cryptography**
>
> This directory contains all test suites for the Aegis crypto library, including native, WASM, and browser tests.

## 🧪 Test Overview

The Aegis test suite provides comprehensive coverage for all NIST PQC algorithms across multiple platforms:

- **Native Tests**: Rust unit and integration tests
- **WASM Tests**: WebAssembly compatibility tests
- **Browser Tests**: Browser-based WebAssembly tests
- **Performance Tests**: Benchmarking and performance validation
- **Security Tests**: Cryptographic validation and security checks

## 📁 Test Structure

```
tests/
├── README.md                           # This documentation
├── run_browser_tests.sh               # Browser test runner script
├── run_wasi_tests.sh                  # WASI test runner script
├── *_native_tests.rs                  # Native Rust tests per algorithm
├── *_browser_tests.rs                 # Browser WASM tests per algorithm
├── *_wasi_tests.rs                    # WASI tests per algorithm
├── integration_tests.rs               # Integration test suite
├── performance_tests.rs               # Performance validation tests
├── security_tests.rs                  # Security and validation tests
└── test_vectors/                      # NIST test vectors and KAT files
```

## 🚀 Running Tests

### Native Tests

```bash
# Run all native tests
cargo test

# Run tests for specific algorithm
cargo test mlkem

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

### WASM Tests

#### Browser Tests

```bash
# Run browser tests (recommended)
cargo test-wasm-browser

# Or manually
cargo test -p aegis_crypto_core --target wasm32-unknown-unknown --features wasm,js-bindings -- --test-threads=1

# Using helper script
bash tests/run_browser_tests.sh
```

#### WASI Tests

```bash
# Run WASI tests
cargo test-wasm

# Or manually
cargo test -p aegis_crypto_core --target wasm32-wasi --features wasm

# Using helper script
bash tests/run_wasi_tests.sh
```

### Performance Tests

```bash
# Run performance benchmarks
cargo bench

# Run specific benchmark
cargo bench mlkem

# Run performance tests
cargo test performance
```

## 🔧 Prerequisites

### Native Tests

- Rust 1.70+
- Cargo
- Standard development tools

### WASM Tests

1. **Install WASM target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   rustup target add wasm32-wasi
   ```

2. **Install wasm-bindgen-test**:
   ```bash
   cargo install wasm-bindgen-cli
   ```

3. **Browser requirements** (for browser tests):
   - Chrome/Chromium: `chromium`, `chromium-browser`, or `google-chrome`
   - Firefox: `firefox`
   - Optional: Set `WASM_BINDGEN_TEST_BROWSER=chrome` or `firefox`

4. **WASI requirements** (for WASI tests):
   - `wasmtime` runtime
   - WASI SDK (optional, for advanced testing)

## 📊 Test Coverage

### Algorithm Coverage

| Algorithm | Native | WASM | Browser | Performance | Security |
|-----------|--------|------|---------|-------------|----------|
| **ML-KEM** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **ML-DSA** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **FN-DSA** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **SLH-DSA** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **HQC-KEM** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Classic McEliece** | ✅ | ✅ | ✅ | ✅ | ✅ |

### Test Types

- **Unit Tests**: Individual function testing
- **Integration Tests**: End-to-end workflow testing
- **Property Tests**: Cryptographic property validation
- **Performance Tests**: Speed and memory usage validation
- **Security Tests**: Constant-time and side-channel resistance
- **Compatibility Tests**: Cross-platform compatibility validation

## 🔍 Test Details

### Native Tests

Native tests run in the standard Rust test environment and cover:
- Basic functionality of all algorithms
- Error handling and edge cases
- Memory safety and resource management
- Performance characteristics
- Cryptographic correctness

### WASM Tests

WASM tests ensure compatibility across different WebAssembly environments:

#### Browser Tests
- Run in headless Chrome or Firefox
- Test WebAssembly bindings
- Validate browser compatibility
- Test JavaScript integration

#### WASI Tests
- Run in WASI-compatible runtimes
- Test server-side WASM deployment
- Validate WASI API compatibility
- Test cross-platform deployment

### Performance Tests

Performance tests validate:
- Key generation speed
- Encryption/decryption performance
- Signature generation/verification speed
- Memory usage patterns
- WASM performance characteristics

### Security Tests

Security tests ensure:
- Constant-time implementations
- Proper memory zeroization
- Side-channel resistance
- Cryptographic validation
- NIST test vector compliance

## 🚨 Troubleshooting

### Common Issues

#### Browser Tests

- **No browser found**: Install chromium or firefox and ensure it's on PATH
- **Tests hang**: Ensure your browser supports headless mode
- **Build errors**: Ensure you're using the correct features and target:
  ```bash
  cargo test -p aegis_crypto_core --target wasm32-unknown-unknown --features "wasm,js-bindings" -- --test-threads=1
  ```

#### WASI Tests

- **WASM runtime not found**: Install `wasmtime` or another WASI-compatible runtime
- **WASI API errors**: Ensure WASI SDK is properly installed
- **Target not found**: Install the WASI target:
  ```bash
  rustup target add wasm32-wasi
  ```

#### General Issues

- **Test failures**: Check that all dependencies are installed
- **Performance issues**: Run tests in release mode for accurate performance metrics
- **Memory issues**: Ensure sufficient system memory for WASM tests

### CI/CD Considerations

For continuous integration:

```bash
# Install dependencies
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
cargo install wasm-bindgen-cli

# Install browser for testing
apt-get install chromium-browser  # Ubuntu/Debian
# or
brew install chromium            # macOS

# Run all tests
cargo test
cargo test-wasm-browser
cargo test-wasm
cargo bench
```

## 📈 Test Results

### Expected Results

- **Native Tests**: 100% pass rate
- **WASM Tests**: 100% pass rate
- **Performance Tests**: Within expected performance bounds
- **Security Tests**: All security validations pass

### Performance Benchmarks

Typical performance metrics (on modern hardware):
- **ML-KEM-768**: Keygen ~0.5ms, Encaps ~0.3ms, Decaps ~0.4ms
- **ML-DSA-65**: Keygen ~2ms, Sign ~1ms, Verify ~0.5ms
- **WASM Size**: ~2MB (optimized)

## 🔗 Related Documentation

- [Main Project README](../README.md)
- [Core Library README](../README.md)
- [Package README](../pkg/README.md)
- [Demo Applications](../demos/README.md)
- [Security Update](../../security-update.md)

## 📄 License

This test suite is licensed under the same terms as the main project:
- Apache License, Version 2.0
- MIT License

at your option.
