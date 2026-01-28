//! WASM bindings for Dilithium operations.

use wasm_bindgen::prelude::*;
use super::utils::*;

/// WASM-exposed function to get the Dilithium public key length.
#[wasm_bindgen]
pub fn dilithium_public_key_length() -> usize {
    public_key_length()
}

/// WASM-exposed function to get the Dilithium secret key length.
#[wasm_bindgen]
pub fn dilithium_secret_key_length() -> usize {
    secret_key_length()
}

/// WASM-exposed function to get the Dilithium signature length.
#[wasm_bindgen]
pub fn dilithium_signature_length() -> usize {
    signature_length()
}

/// WASM-exposed function to validate a Dilithium public key length.
#[wasm_bindgen]
pub fn dilithium_validate_public_key(key: &[u8]) -> bool {
    validate_public_key_length(key).is_ok()
}

/// WASM-exposed function to validate a Dilithium secret key length.
#[wasm_bindgen]
pub fn dilithium_validate_secret_key(key: &[u8]) -> bool {
    validate_secret_key_length(key).is_ok()
}

/// WASM-exposed function to validate a Dilithium signature length.
#[wasm_bindgen]
pub fn dilithium_validate_signature(signature: &[u8]) -> bool {
    validate_signature_length(signature).is_ok()
}
