//! WASM bindings for SPHINCS+ operations.

use wasm_bindgen::prelude::*;
use super::utils::*;

/// WASM-exposed function to get the SPHINCS+ public key length.
#[wasm_bindgen]
pub fn sphincsplus_public_key_length() -> usize {
    public_key_length()
}

/// WASM-exposed function to get the SPHINCS+ secret key length.
#[wasm_bindgen]
pub fn sphincsplus_secret_key_length() -> usize {
    secret_key_length()
}

/// WASM-exposed function to get the SPHINCS+ signature length.
#[wasm_bindgen]
pub fn sphincsplus_signature_length() -> usize {
    signature_length()
}

/// WASM-exposed function to validate a SPHINCS+ public key length.
#[wasm_bindgen]
pub fn sphincsplus_validate_public_key(key: &[u8]) -> bool {
    validate_public_key_length(key).is_ok()
}

/// WASM-exposed function to validate a SPHINCS+ secret key length.
#[wasm_bindgen]
pub fn sphincsplus_validate_secret_key(key: &[u8]) -> bool {
    validate_secret_key_length(key).is_ok()
}

/// WASM-exposed function to validate a SPHINCS+ signature length.
#[wasm_bindgen]
pub fn sphincsplus_validate_signature(signature: &[u8]) -> bool {
    validate_signature_length(signature).is_ok()
}
