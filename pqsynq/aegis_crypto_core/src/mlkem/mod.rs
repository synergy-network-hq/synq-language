//! This module provides the Kyber post-quantum key encapsulation mechanism (KEM)
//! implementation. It uses the `pqrust-mlkem` backend for cryptographic
//! operations and exposes key functions as WebAssembly (WASM) bindings for use
//! in JavaScript/TypeScript environments.

pub mod traits;

use pqrust_mlkem::{
    mlkem512::{keypair as keypair512, encapsulate as encapsulate512, decapsulate as decapsulate512, PublicKey as PublicKey512, SecretKey as SecretKey512, Ciphertext as Ciphertext512, SharedSecret as SharedSecret512},
    mlkem768::{keypair as keypair768, encapsulate as encapsulate768, decapsulate as decapsulate768, PublicKey as PublicKey768, SecretKey as SecretKey768, Ciphertext as Ciphertext768, SharedSecret as SharedSecret768},
    mlkem1024::{keypair as keypair1024, encapsulate as encapsulate1024, decapsulate as decapsulate1024, PublicKey as PublicKey1024, SecretKey as SecretKey1024, Ciphertext as Ciphertext1024, SharedSecret as SharedSecret1024},
};
use pqrust_traits::{Error, Result};

pub type Kyber512PublicKey = PublicKey512;
pub type Kyber512SecretKey = SecretKey512;
pub type Kyber512Ciphertext = Ciphertext512;
pub type Kyber512SharedSecret = SharedSecret512;

pub type Kyber768PublicKey = PublicKey768;
pub type Kyber768SecretKey = SecretKey768;
pub type Kyber768Ciphertext = Ciphertext768;
pub type Kyber768SharedSecret = SharedSecret768;

pub type Kyber1024PublicKey = PublicKey1024;
pub type Kyber1024SecretKey = SecretKey1024;
pub type Kyber1024Ciphertext = Ciphertext1024;
pub type Kyber1024SharedSecret = SharedSecret1024;

// Dedicated functions for each level
pub fn kyber512_keypair() -> (Kyber512PublicKey, Kyber512SecretKey) {
    keypair512()
}

pub fn kyber512_encapsulate(pk: &Kyber512PublicKey) -> Result<(Kyber512Ciphertext, Kyber512SharedSecret)> {
    let (ct, ss) = encapsulate512(pk);
    Ok((ct, ss))
}

pub fn kyber512_decapsulate(sk: &Kyber512SecretKey, ct: &Kyber512Ciphertext) -> Result<Kyber512SharedSecret> {
    Ok(decapsulate512(sk, ct))
}

// Similar for 768 and 1024
pub fn kyber768_keypair() -> (Kyber768PublicKey, Kyber768SecretKey) {
    keypair768()
}

pub fn kyber768_encapsulate(pk: &Kyber768PublicKey) -> Result<(Kyber768Ciphertext, Kyber768SharedSecret)> {
    let (ct, ss) = encapsulate768(pk);
    Ok((ct, ss))
}

pub fn kyber768_decapsulate(sk: &Kyber768SecretKey, ct: &Kyber768Ciphertext) -> Result<Kyber768SharedSecret> {
    Ok(decapsulate768(sk, ct))
}

pub fn kyber1024_keypair() -> (Kyber1024PublicKey, Kyber1024SecretKey) {
    keypair1024()
}

pub fn kyber1024_encapsulate(pk: &Kyber1024PublicKey) -> Result<(Kyber1024Ciphertext, Kyber1024SharedSecret)> {
    let (ct, ss) = encapsulate1024(pk);
    Ok((ct, ss))
}

pub fn kyber1024_decapsulate(sk: &Kyber1024SecretKey, ct: &Kyber1024Ciphertext) -> Result<Kyber1024SharedSecret> {
    Ok(decapsulate1024(sk, ct))
}

// Avoid trait-based conversions to prevent dup trait crate issues in benches
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Represents a Kyber key pair, containing both the public and secret keys.
/// These keys are essential for performing cryptographic operations such as
/// encapsulating and decapsulating shared secrets.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct KyberKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl KyberKeyPair {
    /// Returns the public key component of the Kyber key pair.
    /// The public key is used by the sender to encapsulate a shared secret.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the Kyber key pair.
    /// The secret key is used by the recipient to decapsulate the shared secret.
    /// It should be kept confidential.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

/// Represents the output of the Kyber encapsulation process, containing
/// both the ciphertext and the encapsulated shared secret.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct KyberEncapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl KyberEncapsulated {
    /// Returns the ciphertext generated during encapsulation.
    /// This ciphertext is sent to the recipient for decapsulation.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    /// Returns the shared secret derived during encapsulation.
    /// This secret is used for symmetric encryption.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
}

// Legacy functions (for backward compatibility - default to ML-KEM-768)
/// Generates a new Kyber key pair (ML-KEM-768).
///
/// This function uses the `pqrust-mlkem` backend to generate a fresh
/// public and secret key pair for the Kyber KEM scheme.
///
/// # Returns
///
/// A `KyberKeyPair` containing the newly generated public and secret keys.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn kyber_keygen() -> KyberKeyPair {
    kyber768_keygen()
}

/// Encapsulates a shared secret using the provided Kyber public key (ML-KEM-768).
///
/// This function takes a recipient's public key and generates a ciphertext
/// and a shared secret. The ciphertext is sent to the recipient, who can
/// then decapsulate it to recover the same shared secret.
///
/// # Arguments
///
/// * `public_key` - A byte slice representing the recipient's Kyber public key.
///
/// # Returns
///
/// A `Result<KyberEncapsulated, Box<dyn std::error::Error>>` which is:
/// - `Ok(KyberEncapsulated)` containing the generated ciphertext and shared secret.
/// - `Err(JsValue)` if the public key is invalid.
pub fn kyber_encapsulate(public_key: &[u8]) -> Result<KyberEncapsulated, Box<dyn std::error::Error>> {
    kyber768_encapsulate(public_key)
}

/// Decapsulates a shared secret using the provided Kyber secret key and ciphertext (ML-KEM-768).
///
/// This function takes a recipient's secret key and a ciphertext from the sender,
/// and recovers the shared secret that was encapsulated.
///
/// # Arguments
///
/// * `secret_key` - A byte slice representing the recipient's Kyber secret key.
/// * `ciphertext` - A byte slice representing the ciphertext from the sender.
///
/// # Returns
///
/// A `Result<Vec<u8>, Box<dyn std::error::Error>>` which is:
/// - `Ok(Vec<u8>)` containing the recovered shared secret.
/// - `Err(JsValue)` if the secret key or ciphertext is invalid.
pub fn kyber_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    kyber768_decapsulate(secret_key, ciphertext)
}

// Native functions (for testing and non-WASM environments)
#[cfg(not(target_arch = "wasm32"))]
pub fn kyber512_keygen_native() -> KyberKeyPair {
    kyber512_keygen()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber512_encapsulate_native(public_key: &[u8]) -> Result<KyberEncapsulated, String> {
    let pk = Kyber512PublicKey::from_bytes(public_key)
        .map_err(|_| "Invalid public key format".to_string())?;
    let (ss, ct) = kyber512_encapsulate(&pk)
        .map_err(|e| e.to_string())?;
    Ok(KyberEncapsulated {
        ciphertext: ct.into(),
        shared_secret: ss.into(),
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber512_decapsulate_native(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, String> {
    let sk = Kyber512SecretKey::from_bytes(secret_key)
        .map_err(|_| "Invalid secret key format".to_string())?;
    let ct = Kyber512Ciphertext::from_bytes(ciphertext)
        .map_err(|_| "Invalid ciphertext format".to_string())?;
    let ss = kyber512_decapsulate(&sk, &ct)
        .map_err(|e| e.to_string())?;
    Ok(ss.into())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber768_keygen_native() -> KyberKeyPair {
    kyber768_keygen()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber768_encapsulate_native(public_key: &[u8]) -> Result<KyberEncapsulated, String> {
    use pqrust_mlkem::mlkem768 as m768;
    use pqrust_mlkem::ffi as ffi768;

    if public_key.len() != m768::public_key_bytes() {
        return Err(format!(
            "Invalid public key length: got {}, expected {}",
            public_key.len(),
            m768::public_key_bytes()
        ));
    }

    let mut ct = vec![0u8; m768::ciphertext_bytes()];
    let mut ss = vec![0u8; m768::shared_secret_bytes()];

    let rc = unsafe {
        ffi768::PQCLEAN_MLKEM768_CLEAN_crypto_kem_enc(
            ct.as_mut_ptr(),
            ss.as_mut_ptr(),
            public_key.as_ptr(),
        )
    };
    if rc != 0 {
        return Err("encapsulation failed".to_string());
    }

    Ok(KyberEncapsulated { ciphertext: ct, shared_secret: ss })
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber768_decapsulate_native(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, String> {
    use pqrust_mlkem::mlkem768 as m768;
    use pqrust_mlkem::ffi as ffi768;

    if secret_key.len() != m768::secret_key_bytes() {
        return Err(format!(
            "Invalid secret key length: got {}, expected {}",
            secret_key.len(),
            m768::secret_key_bytes()
        ));
    }
    if ciphertext.len() != m768::ciphertext_bytes() {
        return Err(format!(
            "Invalid ciphertext length: got {}, expected {}",
            ciphertext.len(),
            m768::ciphertext_bytes()
        ));
    }

    let mut ss = vec![0u8; m768::shared_secret_bytes()];
    let rc = unsafe {
        ffi768::PQCLEAN_MLKEM768_CLEAN_crypto_kem_dec(
            ss.as_mut_ptr(),
            ciphertext.as_ptr(),
            secret_key.as_ptr(),
        )
    };
    if rc != 0 {
        return Err("decapsulation failed".to_string());
    }
    Ok(ss)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber1024_keygen_native() -> KyberKeyPair {
    kyber1024_keygen()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber1024_encapsulate_native(public_key: &[u8]) -> Result<KyberEncapsulated, String> {
    let pk = Kyber1024PublicKey::from_bytes(public_key)
        .map_err(|_| "Invalid public key format".to_string())?;
    let (ss, ct) = kyber1024_encapsulate(&pk)
        .map_err(|e| e.to_string())?;
    Ok(KyberEncapsulated {
        ciphertext: ct.into(),
        shared_secret: ss.into(),
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber1024_decapsulate_native(
    secret_key: &[u8],
    ciphertext: &[u8]
) -> Result<Vec<u8>, String> {
    let sk = Kyber1024SecretKey::from_bytes(secret_key)
        .map_err(|_| "Invalid secret key format".to_string())?;
    let ct = Kyber1024Ciphertext::from_bytes(ciphertext)
        .map_err(|_| "Invalid ciphertext format".to_string())?;
    let ss = kyber1024_decapsulate(&sk, &ct)
        .map_err(|e| e.to_string())?;
    Ok(ss.into())
}

// Legacy native functions (for backward compatibility)
#[cfg(not(target_arch = "wasm32"))]
pub fn kyber_keygen_native() -> KyberKeyPair {
    kyber768_keygen()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber_encapsulate_native(public_key: &[u8]) -> Result<KyberEncapsulated, String> {
    kyber768_encapsulate_native(public_key)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn kyber_decapsulate_native(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    kyber768_decapsulate_native(secret_key, ciphertext)
}

pub use traits::*;
