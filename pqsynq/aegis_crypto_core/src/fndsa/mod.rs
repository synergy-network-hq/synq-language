//! This module provides the FN-DSA (FFT over NTRU-Lattice-Based Digital Signature Algorithm)
//! post-quantum digital signature algorithm. It uses the `pqrust-fndsa` backend for cryptographic
//! operations and exposes key functions as WebAssembly (WASM) bindings for use
//! in JavaScript/TypeScript environments.

use pqrust_fndsa::{
    fndsa512::{keypair as keypair512, sign as sign512, open as open512, PublicKey as PublicKey512, SecretKey as SecretKey512, SignedMessage as SignedMessage512},
    fndsa1024::{keypair as keypair1024, sign as sign1024, open as open1024, PublicKey as PublicKey1024, SecretKey as SecretKey1024, SignedMessage as SignedMessage1024},
    // Padded variants if supported
};
use pqrust_traits::sign::{ PublicKey as _, SecretKey as _, SignedMessage as _ };
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Represents a Falcon key pair, containing both the public and secret keys.
/// These keys are essential for signing messages and verifying signatures.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct FalconKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl FalconKeyPair {
    /// Returns the public key component of the Falcon key pair.
    /// The public key is used to verify signatures.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the Falcon key pair.
    /// The secret key is used to sign messages and should be kept confidential.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

pub type Falcon512PublicKey = PublicKey512;
pub type Falcon512SecretKey = SecretKey512;
pub type Falcon512Signature = SignedMessage512;

pub type Falcon1024PublicKey = PublicKey1024;
pub type Falcon1024SecretKey = SecretKey1024;
pub type Falcon1024Signature = SignedMessage1024;

pub fn falcon512_keypair() -> (Falcon512PublicKey, Falcon512SecretKey) {
    keypair512()
}

pub fn falcon512_sign(sk: &Falcon512SecretKey, message: &[u8]) -> Result<Falcon512Signature, Box<dyn std::error::Error>> {
    Ok(sign512(message, sk))
}

pub fn falcon512_verify(pk: &Falcon512PublicKey, message: &[u8], signature: &Falcon512Signature) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(open512(signature, pk).is_ok())
}

pub fn falcon1024_keypair() -> (Falcon1024PublicKey, Falcon1024SecretKey) {
    keypair1024()
}

pub fn falcon1024_sign(sk: &Falcon1024SecretKey, message: &[u8]) -> Result<Falcon1024Signature, Box<dyn std::error::Error>> {
    Ok(sign1024(message, sk))
}

pub fn falcon1024_verify(pk: &Falcon1024PublicKey, message: &[u8], signature: &Falcon1024Signature) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(open1024(signature, pk).is_ok())
}

// Legacy functions for backward compatibility
pub fn falcon_keygen() -> FalconKeyPair {
    let (pk, sk) = falcon512_keypair();
    let mut pk_bytes = [0u8; 32]; // Adjust size as needed
    let mut sk_bytes = [0u8; 64]; // Adjust size as needed
    pk_bytes[..].copy_from_slice(&pk.as_bytes());
    sk_bytes[..].copy_from_slice(&sk.as_bytes());
    FalconKeyPair {
        pk: pk_bytes.to_vec(),
        sk: sk_bytes.to_vec(),
    }
}

pub fn falcon_sign(sk: &[u8], message: &[u8]) -> Vec<u8> {
    // This would need proper implementation
    vec![]
}

pub fn falcon_verify(pk: &[u8], message: &[u8], signature: &[u8]) -> bool {
    // This would need proper implementation
    false
}
