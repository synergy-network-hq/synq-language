//! This module provides the ML-KEM-768 post-quantum key encapsulation mechanism (KEM)
//! implementation using the pure Rust implementation from the rustpqc folder.
//! This is conditionally compiled when the `rustpqc-kyber` feature is enabled.

use ml_kem::kem::{ crypto_kem_keypair, crypto_kem_enc, crypto_kem_dec };
use ml_kem::params::{ PUBLICKEYBYTES, SECRETKEYBYTES, CIPHERTEXTBYTES, SYMBYTES };
use wasm_bindgen::prelude::*;

/// Represents a ML-KEM-768 key pair, containing both the public and secret keys.
/// These keys are essential for performing cryptographic operations such as
/// encapsulating and decapsulating shared secrets.
#[wasm_bindgen]
pub struct RustPqcKyberKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl RustPqcKyberKeyPair {
    /// Returns the public key component of the ML-KEM-768 key pair.
    /// The public key is used by the sender to encapsulate a shared secret.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the ML-KEM-768 key pair.
    /// The secret key is used by the recipient to decapsulate the shared secret.
    /// It should be kept confidential.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

/// Represents the output of the ML-KEM-768 encapsulation process, containing
/// both the ciphertext and the encapsulated shared secret.
#[wasm_bindgen]
pub struct RustPqcKyberEncapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl RustPqcKyberEncapsulated {
    /// Returns the ciphertext generated during encapsulation.
    /// This ciphertext is sent to the recipient for decapsulation.
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    /// Returns the shared secret derived during encapsulation.
    /// This secret is used for symmetric encryption.
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
}

// ML-KEM-768 Functions using pure Rust implementation
#[wasm_bindgen]
pub fn rustpqc_kyber768_keygen() -> RustPqcKyberKeyPair {
    let mut pk = [0u8; PUBLICKEYBYTES];
    let mut sk = [0u8; SECRETKEYBYTES];

    crypto_kem_keypair(&mut pk, &mut sk);

    RustPqcKyberKeyPair {
        pk: pk.to_vec(),
        sk: sk.to_vec(),
    }
}

#[wasm_bindgen]
pub fn rustpqc_kyber768_encapsulate(
    public_key: &[u8]
) -> Result<RustPqcKyberEncapsulated, JsValue> {
    if public_key.len() != PUBLICKEYBYTES {
        return Err(
            format!(
                "Invalid public key length: expected {}, got {}",
                PUBLICKEYBYTES,
                public_key.len()
            ).into()
        );
    }

    let mut pk = [0u8; PUBLICKEYBYTES];
    pk.copy_from_slice(public_key);

    let mut ct = [0u8; CIPHERTEXTBYTES];
    let mut ss = [0u8; SYMBYTES];

    let result = crypto_kem_enc(&mut ct, &mut ss, &pk);
    if result != 0 {
        return Err("Encapsulation failed".into());
    }

    Ok(RustPqcKyberEncapsulated {
        ciphertext: ct.to_vec(),
        shared_secret: ss.to_vec(),
    })
}

#[wasm_bindgen]
pub fn rustpqc_kyber768_decapsulate(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, JsValue> {
    if secret_key.len() != SECRETKEYBYTES {
        return Err(
            format!(
                "Invalid secret key length: expected {}, got {}",
                SECRETKEYBYTES,
                secret_key.len()
            ).into()
        );
    }

    if ciphertext.len() != CIPHERTEXTBYTES {
        return Err(
            format!(
                "Invalid ciphertext length: expected {}, got {}",
                CIPHERTEXTBYTES,
                ciphertext.len()
            ).into()
        );
    }

    let mut sk = [0u8; SECRETKEYBYTES];
    sk.copy_from_slice(secret_key);

    let mut ct = [0u8; CIPHERTEXTBYTES];
    ct.copy_from_slice(ciphertext);

    let mut ss = [0u8; SYMBYTES];

    let result = crypto_kem_dec(&mut ss, &ct, &sk);
    if result != 0 {
        return Err("Decapsulation failed".into());
    }

    Ok(ss.to_vec())
}

// Non-WASM functions for Rust usage
#[cfg(not(target_arch = "wasm32"))]
pub fn rustpqc_kyber768_keygen_rust() -> (Vec<u8>, Vec<u8>) {
    let mut pk = [0u8; PUBLICKEYBYTES];
    let mut sk = [0u8; SECRETKEYBYTES];

    crypto_kem_keypair(&mut pk, &mut sk);

    (pk.to_vec(), sk.to_vec())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn rustpqc_kyber768_encapsulate_rust(public_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
    if public_key.len() != PUBLICKEYBYTES {
        return Err(
            format!(
                "Invalid public key length: expected {}, got {}",
                PUBLICKEYBYTES,
                public_key.len()
            )
        );
    }

    let mut pk = [0u8; PUBLICKEYBYTES];
    pk.copy_from_slice(public_key);

    let mut ct = [0u8; CIPHERTEXTBYTES];
    let mut ss = [0u8; SYMBYTES];

    let result = crypto_kem_enc(&mut ct, &mut ss, &pk);
    if result != 0 {
        return Err("Encapsulation failed".to_string());
    }

    Ok((ct.to_vec(), ss.to_vec()))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn rustpqc_kyber768_decapsulate_rust(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, String> {
    if secret_key.len() != SECRETKEYBYTES {
        return Err(
            format!(
                "Invalid secret key length: expected {}, got {}",
                SECRETKEYBYTES,
                secret_key.len()
            )
        );
    }

    if ciphertext.len() != CIPHERTEXTBYTES {
        return Err(
            format!(
                "Invalid ciphertext length: expected {}, got {}",
                CIPHERTEXTBYTES,
                ciphertext.len()
            )
        );
    }

    let mut sk = [0u8; SECRETKEYBYTES];
    sk.copy_from_slice(secret_key);

    let mut ct = [0u8; CIPHERTEXTBYTES];
    ct.copy_from_slice(ciphertext);

    let mut ss = [0u8; SYMBYTES];

    let result = crypto_kem_dec(&mut ss, &ct, &sk);
    if result != 0 {
        return Err("Decapsulation failed".to_string());
    }

    Ok(ss.to_vec())
}
