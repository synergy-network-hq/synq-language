//! Core SLH-DSA implementation.
//!
//! This module provides the core SLH-DSA (Stateless Hash-Based Digital Signature Algorithm)
//! post-quantum signature scheme implementation.
//! It integrates with the `pqrust_slhdsa` backend for cryptographic operations
//! and exposes key functions as WebAssembly (WASM) bindings for use in JavaScript/TypeScript environments.

use wasm_bindgen::prelude::*;
use pqrust_slhdsa::slhdsasha2128fsimple::{keypair, detached_sign, verify_detached_signature, PublicKey, SecretKey, DetachedSignature};
use pqrust_traits::sign::{PublicKey as _, SecretKey as _, DetachedSignature as _};
use super::utils::*;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::{String, ToString}};
#[cfg(feature = "std")]
use std::{vec::Vec, string::{String, ToString}};

/// Represents a SPHINCS+ key pair, containing both the public and secret keys.
/// These keys are essential for performing cryptographic operations such as
/// signing messages and verifying signatures.
#[wasm_bindgen]
pub struct SphincsPlusKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl SphincsPlusKeyPair {
    /// Returns the public key component of the SPHINCS+ key pair.
    /// The public key is used to verify signatures created by the corresponding
    /// secret key.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the SPHINCS+ key pair.
    /// The secret key is used to sign messages. It should be kept confidential.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }

    /// Returns the length of the public key in bytes.
    #[wasm_bindgen]
    pub fn public_key_length(&self) -> usize {
        self.pk.len()
    }

    /// Returns the length of the secret key in bytes.
    #[wasm_bindgen]
    pub fn secret_key_length(&self) -> usize {
        self.sk.len()
    }
}

/// Generates a new SPHINCS+ key pair.
///
/// This function uses the `pqc_sphincsplus` backend to generate a fresh
/// public and secret key pair for the SPHINCS+ signature scheme.
///
/// # Returns
///
/// A `Result<SphincsPlusKeyPair, JsValue>` which is:
/// - `Ok(SphincsPlusKeyPair)` containing the newly generated public and secret keys.
/// - `Err(JsValue)` if the key generation process fails.
#[wasm_bindgen]
pub fn sphincsplus_keygen() -> Result<SphincsPlusKeyPair, JsValue> {
    sphincsplus_keygen_native().map_err(|e| JsValue::from_str(&e))
}

/// Signs a message using the provided SPHINCS+ secret key.
///
/// This function takes a secret key and a message, and produces a digital
/// signature using the SPHINCS+ algorithm. The signature can later be
/// verified using the corresponding public key.
///
/// # Arguments
///
/// * `secret_key` - A byte slice representing the SPHINCS+ secret key.
/// * `message` - A byte slice representing the message to be signed.
///
/// # Returns
///
/// A `Result<Vec<u8>, JsValue>` which is:
/// - `Ok(Vec<u8>)` containing the generated digital signature.
/// - `Err(JsValue)` if the secret key has an incorrect length or is invalid,
///   or if the signing process fails.
#[wasm_bindgen]
pub fn sphincsplus_sign(secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
    sphincsplus_sign_native(secret_key, message).map_err(|e| JsValue::from_str(&e))
}

/// Verifies a SPHINCS+ signature against a message and a public key.
///
/// This function checks if a given digital signature is valid for a specific
/// message and public key. It returns `true` if the signature is valid,
/// and `false` otherwise.
///
/// # Arguments
///
/// * `public_key` - A byte slice representing the SPHINCS+ public key.
/// * `message` - A byte slice representing the original message.
/// * `signature` - A byte slice representing the digital signature to verify.
///
/// # Returns
///
/// `true` if the signature is valid, `false` otherwise.
#[wasm_bindgen]
pub fn sphincsplus_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    sphincsplus_verify_native(public_key, message, signature)
}

// Native Functions (without wasm_bindgen attributes)
/// Generates a new SPHINCS+ key pair - Native version.
///
/// # Returns
///
/// A `Result<SphincsPlusKeyPair, String>` which is:
/// - `Ok(SphincsPlusKeyPair)` containing the newly generated public and secret keys.
/// - `Err(String)` if the key generation process fails.
pub fn sphincsplus_keygen_native() -> Result<SphincsPlusKeyPair, String> {
    let (pk, sk) = keypair();
    let keypair = SphincsPlusKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    };
    Ok(keypair)
}

/// Signs a message using the provided SPHINCS+ secret key - Native version.
///
/// # Arguments
///
/// * `secret_key` - A byte slice representing the SPHINCS+ secret key.
/// * `message` - A byte slice representing the message to be signed.
///
/// # Returns
///
/// A `Result<Vec<u8>, String>` which is:
/// - `Ok(Vec<u8>)` containing the generated digital signature.
/// - `Err(String)` if the secret key has an incorrect length or is invalid,
///   or if the signing process fails.
pub fn sphincsplus_sign_native(secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, String> {
    validate_secret_key_length(secret_key)?;

    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| "Invalid secret key".to_string())?;
    let signature = detached_sign(message, &sk);
    Ok(signature.as_bytes().to_vec())
}

/// Verifies a SPHINCS+ signature against a message and a public key - Native version.
///
/// # Arguments
///
/// * `public_key` - A byte slice representing the SPHINCS+ public key.
/// * `message` - A byte slice representing the original message.
/// * `signature` - A byte slice representing the digital signature to verify.
///
/// # Returns
///
/// `true` if the signature is valid, `false` otherwise.
pub fn sphincsplus_verify_native(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    // Validate input lengths
    if validate_public_key_length(public_key).is_err() {
        return false;
    }
    if validate_signature_length(signature).is_err() {
        return false;
    }

    let pk = match PublicKey::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    let sig = match DetachedSignature::from_bytes(signature) {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    verify_detached_signature(&sig, message, &pk).is_ok()
}

// SPHINCS+ implementation is currently paused due to upstream bugs in pqc_sphincsplus. See project documentation for status.
