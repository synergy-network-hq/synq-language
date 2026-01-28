//! Key Encapsulation Mechanisms (KEM) implementations

use alloc::vec::Vec;
use alloc::string::ToString;
use crate::error::PqcError;
use crate::traits::KeyEncapsulation;
use crate::utils::check_buffer_size;

// Re-export from pqrust crates
#[cfg(feature = "mlkem")]
use pqrust_mlkem::*;
use pqrust_traits::kem::{PublicKey, SecretKey, Ciphertext, SharedSecret};

/// Enumeration of supported KEM algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KemAlgorithm {
    #[cfg(feature = "mlkem")]
    Mlkem512,
    #[cfg(feature = "mlkem")]
    Mlkem768,
    #[cfg(feature = "mlkem")]
    Mlkem1024,
}

/// Main KEM interface
pub struct Kem {
    algorithm: KemAlgorithm,
}

impl Kem {
    /// Create a new KEM instance
    pub fn new(algorithm: KemAlgorithm) -> Self {
        Self { algorithm }
    }

    /// Create ML-KEM 512 instance
    #[cfg(feature = "mlkem")]
    pub fn mlkem512() -> Self {
        Self::new(KemAlgorithm::Mlkem512)
    }

    /// Create ML-KEM 768 instance
    #[cfg(feature = "mlkem")]
    pub fn mlkem768() -> Self {
        Self::new(KemAlgorithm::Mlkem768)
    }

    /// Create ML-KEM 1024 instance
    #[cfg(feature = "mlkem")]
    pub fn mlkem1024() -> Self {
        Self::new(KemAlgorithm::Mlkem1024)
    }

}

impl KeyEncapsulation for Kem {
    fn keygen(&self) -> Result<(Vec<u8>, Vec<u8>), PqcError> {
        match self.algorithm {
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem512 => {
                let (pk, sk) = mlkem512_keypair();
                Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
            }
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem768 => {
                let (pk, sk) = mlkem768_keypair();
                Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
            }
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem1024 => {
                let (pk, sk) = mlkem1024_keypair();
                Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
            }
        }
    }

    fn encapsulate(&self, public_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), PqcError> {
        check_buffer_size(public_key, self.public_key_size())?;
        
        match self.algorithm {
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem512 => {
                let pk = mlkem512::PublicKey::from_bytes(public_key).map_err(|_| PqcError::CryptoError("Invalid public key".to_string()))?;
                let (ss, ct) = mlkem512_encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            }
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem768 => {
                let pk = mlkem768::PublicKey::from_bytes(public_key).map_err(|_| PqcError::CryptoError("Invalid public key".to_string()))?;
                let (ss, ct) = mlkem768_encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            }
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem1024 => {
                let pk = mlkem1024::PublicKey::from_bytes(public_key).map_err(|_| PqcError::CryptoError("Invalid public key".to_string()))?;
                let (ss, ct) = mlkem1024_encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            }
        }
    }

    fn decapsulate(&self, ciphertext: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, PqcError> {
        check_buffer_size(ciphertext, self.ciphertext_size())?;
        check_buffer_size(secret_key, self.secret_key_size())?;
        
        match self.algorithm {
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem512 => {
                let ct = mlkem512::Ciphertext::from_bytes(ciphertext).map_err(|_| PqcError::CryptoError("Invalid ciphertext".to_string()))?;
                let sk = mlkem512::SecretKey::from_bytes(secret_key).map_err(|_| PqcError::CryptoError("Invalid secret key".to_string()))?;
                let ss = mlkem512_decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            }
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem768 => {
                let ct = mlkem768::Ciphertext::from_bytes(ciphertext).map_err(|_| PqcError::CryptoError("Invalid ciphertext".to_string()))?;
                let sk = mlkem768::SecretKey::from_bytes(secret_key).map_err(|_| PqcError::CryptoError("Invalid secret key".to_string()))?;
                let ss = mlkem768_decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            }
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem1024 => {
                let ct = mlkem1024::Ciphertext::from_bytes(ciphertext).map_err(|_| PqcError::CryptoError("Invalid ciphertext".to_string()))?;
                let sk = mlkem1024::SecretKey::from_bytes(secret_key).map_err(|_| PqcError::CryptoError("Invalid secret key".to_string()))?;
                let ss = mlkem1024_decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            }
        }
    }

    fn public_key_size(&self) -> usize {
        match self.algorithm {
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem512 => mlkem512_public_key_bytes(),
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem768 => mlkem768_public_key_bytes(),
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem1024 => mlkem1024_public_key_bytes(),
        }
    }

    fn secret_key_size(&self) -> usize {
        match self.algorithm {
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem512 => mlkem512_secret_key_bytes(),
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem768 => mlkem768_secret_key_bytes(),
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem1024 => mlkem1024_secret_key_bytes(),
        }
    }

    fn ciphertext_size(&self) -> usize {
        match self.algorithm {
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem512 => mlkem512_ciphertext_bytes(),
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem768 => mlkem768_ciphertext_bytes(),
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem1024 => mlkem1024_ciphertext_bytes(),
        }
    }

    fn shared_secret_size(&self) -> usize {
        match self.algorithm {
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem512 => mlkem512_shared_secret_bytes(),
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem768 => mlkem768_shared_secret_bytes(),
            #[cfg(feature = "mlkem")]
            KemAlgorithm::Mlkem1024 => mlkem1024_shared_secret_bytes(),
        }
    }
}

// Type aliases for specific algorithms
pub type Mlkem512 = Kem;
pub type Mlkem768 = Kem;
pub type Mlkem1024 = Kem;
pub type Hqckem128 = Kem;
pub type Hqckem192 = Kem;
pub type Hqckem256 = Kem;
pub type Cmce348864 = Kem;
pub type Cmce460896 = Kem;
pub type Cmce6688128 = Kem;
pub type Cmce6960119 = Kem;
pub type Cmce8192128 = Kem;
