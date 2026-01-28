//! Kyber-specific trait implementations.

use crate::traits::{ Kem, KemError, Algorithm };
use crate::mlkem::kyber_keygen;
#[cfg(target_arch = "wasm32")]
use crate::mlkem::{ kyber_encapsulate, kyber_decapsulate };
#[cfg(not(target_arch = "wasm32"))]
use crate::mlkem::{ kyber768_encapsulate_native, kyber768_decapsulate_native };
use zeroize::Zeroize;
use std::vec::Vec;

/// Kyber 768 implementation of the KEM trait.
pub struct Kyber768;

/// Kyber public key wrapper.
#[derive(Clone)]
pub struct KyberPublicKey(pub Vec<u8>);

/// Kyber secret key wrapper.
#[derive(Clone)]
pub struct KyberSecretKey(pub Vec<u8>);

/// Kyber ciphertext wrapper.
#[derive(Clone)]
pub struct KyberCiphertext(pub Vec<u8>);

/// Kyber shared secret wrapper.
#[derive(Clone)]
pub struct KyberSharedSecret(pub Vec<u8>);

impl AsRef<[u8]> for KyberPublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for KyberSecretKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for KyberCiphertext {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for KyberSharedSecret {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl KyberSharedSecret {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Zeroize for KyberSecretKey {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl Zeroize for KyberSharedSecret {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl Algorithm for Kyber768 {
    fn name() -> &'static str {
        "Kyber768"
    }

    fn security_level() -> usize {
        192
    }
}

impl Kem for Kyber768 {
    type PublicKey = KyberPublicKey;
    type SecretKey = KyberSecretKey;
    type Ciphertext = KyberCiphertext;
    type SharedSecret = KyberSharedSecret;

    fn keygen() -> Result<(Self::PublicKey, Self::SecretKey), KemError> {
        let keypair = kyber_keygen();
        Ok((KyberPublicKey(keypair.public_key()), KyberSecretKey(keypair.secret_key())))
    }

    fn encapsulate(
        public_key: &Self::PublicKey
    ) -> Result<(Self::Ciphertext, Self::SharedSecret), KemError> {
        #[cfg(target_arch = "wasm32")]
        {
            match kyber_encapsulate(&public_key.0) {
                Ok(encapsulated) =>
                    Ok((
                        KyberCiphertext(encapsulated.ciphertext()),
                        KyberSharedSecret(encapsulated.shared_secret()),
                    )),
                Err(_) => Err(KemError::EncapsulationFailed),
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            match kyber768_encapsulate_native(&public_key.0) {
                Ok(encapsulated) =>
                    Ok((
                        KyberCiphertext(encapsulated.ciphertext()),
                        KyberSharedSecret(encapsulated.shared_secret()),
                    )),
                Err(_) => Err(KemError::EncapsulationFailed),
            }
        }
    }

    fn decapsulate(
        secret_key: &Self::SecretKey,
        ciphertext: &[u8]
    ) -> Result<Self::SharedSecret, KemError> {
        #[cfg(target_arch = "wasm32")]
        {
            match kyber_decapsulate(&secret_key.0, ciphertext) {
                Ok(shared_secret) => Ok(KyberSharedSecret(shared_secret)),
                Err(_) => Err(KemError::DecapsulationFailed),
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            match kyber768_decapsulate_native(&secret_key.0, ciphertext) {
                Ok(shared_secret) => Ok(KyberSharedSecret(shared_secret)),
                Err(_) => Err(KemError::DecapsulationFailed),
            }
        }
    }
}
