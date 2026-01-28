// Conditional wasm_bindgen macro
#[cfg(feature = "wasm")]
macro_rules! wasm_bindgen {
    ($($tt:tt)*) => {
        #[wasm_bindgen]
        $($tt)*
    };
}

#[cfg(not(feature = "wasm"))]
macro_rules! wasm_bindgen {
    ($($tt:tt)*) => {
        $($tt)*
    };
}

// Include Node.js initialization for WASM builds
#[cfg(feature = "wasm-nodejs")]
pub mod nodejs_init;

#[cfg(feature = "kyber")]
pub mod mlkem;
#[cfg(feature = "dilithium")]
pub mod mldsa;
#[cfg(feature = "falcon")]
pub mod fndsa;
#[cfg(feature = "sphincsplus")]
pub mod slhdsa;
#[cfg(feature = "hqc")]
pub mod hqckem;
#[cfg(feature = "classicmceliece")]
pub mod cmce;

// Pure Rust implementations from rustpqc folder
// #[cfg(feature = "rustpqc-kyber")]
// pub mod rustpqc_mlkem;
// #[cfg(feature = "rustpqc-dilithium")]
// pub mod rustpqc_mldsa;

// NIST Reference WASM implementations
#[cfg(feature = "nist-wasm")]
pub mod nist_wasm_mlkem;
#[cfg(feature = "nist-wasm")]
pub mod nist_wasm_mldsa;
#[cfg(feature = "nist-wasm")]
pub mod wasm_loader;

/// Trait definitions for unified algorithm interfaces.
pub mod traits;

pub mod hash;
pub mod utils;
pub mod performance;
pub mod blockchain;

// Security modules
pub mod secure_key_management;
pub mod security_monitoring;
pub mod audit_logging;

// The `js_bindings` module exposes a JavaScript‑friendly API on top of the
// low‑level functions.  It is compiled unconditionally when building the
// WebAssembly target so that its exports are available via `wasm-pack`.
pub mod js_bindings;

// The Python bindings are conditionally compiled when the
// `python-bindings` feature is enabled.  See `Cargo.toml` for more
// details.  The module contains PyO3 wrappers that expose the
// algorithms to Python as a native extension.
// #[cfg(feature = "python-bindings")]
// pub mod python_bindings;

#[cfg(feature = "kyber")]
pub use mlkem::*;
#[cfg(feature = "dilithium")]
pub use mldsa::*;
#[cfg(feature = "falcon")]
pub use fndsa::*;
#[cfg(feature = "sphincsplus")]
pub use slhdsa::*;
#[cfg(feature = "hqc")]
pub use hqckem::*;
#[cfg(feature = "classicmceliece")]
pub use cmce::*;

// Re-export pure Rust implementations
// #[cfg(feature = "rustpqc-kyber")]
// pub use rustpqc_mlkem::*;
// #[cfg(feature = "rustpqc-dilithium")]
// pub use rustpqc_mldsa::*;

// Re-export NIST Reference WASM implementations
#[cfg(feature = "nist-wasm")]
pub use nist_wasm_mlkem::*;
#[cfg(feature = "nist-wasm")]
pub use nist_wasm_mldsa::*;
#[cfg(feature = "nist-wasm")]
pub use wasm_loader::*;

pub mod nist;

#[cfg(feature = "nist-ref")]
pub use nist::{nist_ml_kem_512_keypair, /* all functions */};
