//! WASM bindings for Falcon operations.

use wasm_bindgen::prelude::*;
use super::utils::*;

/// WASM-exposed function to get the Falcon public key length.
#[wasm_bindgen]
pub fn falcon_public_key_length() -> usize {
    public_key_length()
}

/// WASM-exposed function to get the Falcon secret key length.
#[wasm_bindgen]
pub fn falcon_secret_key_length() -> usize {
    secret_key_length()
}

/// WASM-exposed function to validate a Falcon public key length.
#[wasm_bindgen]
pub fn falcon_validate_public_key(key: &[u8]) -> bool {
    validate_public_key_length(key).is_ok()
}

/// WASM-exposed function to validate a Falcon secret key length.
#[wasm_bindgen]
pub fn falcon_validate_secret_key(key: &[u8]) -> bool {
    validate_secret_key_length(key).is_ok()
}

/// WASM-exposed function to validate a Falcon signature is not empty.
#[wasm_bindgen]
pub fn falcon_validate_signature(signature: &[u8]) -> bool {
    validate_signature_not_empty(signature).is_ok()
}
