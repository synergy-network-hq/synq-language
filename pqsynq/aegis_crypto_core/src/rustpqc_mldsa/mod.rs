//! This module provides the ML-DSA-65 post-quantum digital signature algorithm
//! implementation using the pure Rust implementation from the rustpqc folder.
//! This is conditionally compiled when the `rustpqc-dilithium` feature is enabled.

use ml_dsa::sign::{ crypto_sign_keypair, crypto_sign_signature, crypto_sign_verify };
use ml_dsa::params::{ CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES, CRYPTO_BYTES };
use wasm_bindgen::prelude::*;

/// Represents a ML-DSA-65 key pair, containing both the public and secret keys.
/// These keys are essential for performing cryptographic operations such as
/// signing and verifying messages.
#[wasm_bindgen]
pub struct RustPqcDilithiumKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl RustPqcDilithiumKeyPair {
    /// Returns the public key component of the ML-DSA-65 key pair.
    /// The public key is used for verifying signatures.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the ML-DSA-65 key pair.
    /// The secret key is used for signing messages.
    /// It should be kept confidential.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

// ML-DSA-65 Functions using pure Rust implementation
#[wasm_bindgen]
pub fn rustpqc_dilithium65_keygen() -> RustPqcDilithiumKeyPair {
    let mut pk = [0u8; CRYPTO_PUBLICKEYBYTES];
    let mut sk = [0u8; CRYPTO_SECRETKEYBYTES];

    crypto_sign_keypair(&mut pk, &mut sk);

    RustPqcDilithiumKeyPair {
        pk: pk.to_vec(),
        sk: sk.to_vec(),
    }
}

#[wasm_bindgen]
pub fn rustpqc_dilithium65_sign(secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
    if secret_key.len() != CRYPTO_SECRETKEYBYTES {
        return Err(
            format!(
                "Invalid secret key length: expected {}, got {}",
                CRYPTO_SECRETKEYBYTES,
                secret_key.len()
            ).into()
        );
    }

    let mut sk = [0u8; CRYPTO_SECRETKEYBYTES];
    sk.copy_from_slice(secret_key);

    let mut sig = [0u8; CRYPTO_BYTES];
    let mut siglen = 0;

    let result = crypto_sign_signature(&mut sig, &mut siglen, message, message.len(), &sk);
    if result != 0 {
        return Err("Signing failed".into());
    }

    Ok(sig[..siglen].to_vec())
}

#[wasm_bindgen]
pub fn rustpqc_dilithium65_verify(
    public_key: &[u8],
    signature: &[u8],
    message: &[u8]
) -> Result<bool, JsValue> {
    if public_key.len() != CRYPTO_PUBLICKEYBYTES {
        return Err(
            format!(
                "Invalid public key length: expected {}, got {}",
                CRYPTO_PUBLICKEYBYTES,
                public_key.len()
            ).into()
        );
    }

    let mut pk = [0u8; CRYPTO_PUBLICKEYBYTES];
    pk.copy_from_slice(public_key);

    let result = crypto_sign_verify(signature, signature.len(), message, message.len(), &pk);

    Ok(result == 0)
}

// Non-WASM functions for Rust usage
#[cfg(not(target_arch = "wasm32"))]
pub fn rustpqc_dilithium65_keygen_rust() -> (Vec<u8>, Vec<u8>) {
    let mut pk = [0u8; CRYPTO_PUBLICKEYBYTES];
    let mut sk = [0u8; CRYPTO_SECRETKEYBYTES];

    crypto_sign_keypair(&mut pk, &mut sk);

    (pk.to_vec(), sk.to_vec())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn rustpqc_dilithium65_sign_rust(secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, String> {
    if secret_key.len() != CRYPTO_SECRETKEYBYTES {
        return Err(
            format!(
                "Invalid secret key length: expected {}, got {}",
                CRYPTO_SECRETKEYBYTES,
                secret_key.len()
            )
        );
    }

    let mut sk = [0u8; CRYPTO_SECRETKEYBYTES];
    sk.copy_from_slice(secret_key);

    let mut sig = [0u8; CRYPTO_BYTES];
    let mut siglen = 0;

    let result = crypto_sign_signature(&mut sig, &mut siglen, message, message.len(), &sk);
    if result != 0 {
        return Err("Signing failed".to_string());
    }

    Ok(sig[..siglen].to_vec())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn rustpqc_dilithium65_verify_rust(
    public_key: &[u8],
    signature: &[u8],
    message: &[u8]
) -> Result<bool, String> {
    if public_key.len() != CRYPTO_PUBLICKEYBYTES {
        return Err(
            format!(
                "Invalid public key length: expected {}, got {}",
                CRYPTO_PUBLICKEYBYTES,
                public_key.len()
            )
        );
    }

    let mut pk = [0u8; CRYPTO_PUBLICKEYBYTES];
    pk.copy_from_slice(public_key);

    let result = crypto_sign_verify(signature, signature.len(), message, message.len(), &pk);

    Ok(result == 0)
}
