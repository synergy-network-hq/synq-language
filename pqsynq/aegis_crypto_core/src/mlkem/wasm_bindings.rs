//! WASM bindings for Kyber operations.

use wasm_bindgen::prelude::*;
use super::utils::*;

/// WASM-exposed function to get the Kyber public key length.
#[wasm_bindgen]
pub fn kyber_public_key_length() -> usize {
    public_key_length()
}

/// WASM-exposed function to get the Kyber secret key length.
#[wasm_bindgen]
pub fn kyber_secret_key_length() -> usize {
    secret_key_length()
}

/// WASM-exposed function to get the Kyber ciphertext length.
#[wasm_bindgen]
pub fn kyber_ciphertext_length() -> usize {
    ciphertext_length()
}

/// WASM-exposed function to get the Kyber shared secret length.
#[wasm_bindgen]
pub fn kyber_shared_secret_length() -> usize {
    shared_secret_length()
}

/// WASM-exposed function to validate a Kyber public key length.
#[wasm_bindgen]
pub fn kyber_validate_public_key(key: &[u8]) -> bool {
    validate_public_key_length(key).is_ok()
}

/// WASM-exposed function to validate a Kyber secret key length.
#[wasm_bindgen]
pub fn kyber_validate_secret_key(key: &[u8]) -> bool {
    validate_secret_key_length(key).is_ok()
}

/// WASM-exposed function to validate a Kyber ciphertext length.
#[wasm_bindgen]
pub fn kyber_validate_ciphertext(ciphertext: &[u8]) -> bool {
    validate_ciphertext_length(ciphertext).is_ok()
}

/// WASM-exposed function to validate a Kyber shared secret length.
#[wasm_bindgen]
pub fn kyber_validate_shared_secret(shared_secret: &[u8]) -> bool {
    validate_shared_secret_length(shared_secret).is_ok()
}
