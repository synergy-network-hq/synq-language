//! Digital Signature implementations

use alloc::vec::Vec;
use alloc::string::ToString;
use crate::error::PqcError;
use crate::traits::DigitalSignature;
use crate::utils::check_buffer_size;

// Re-export from pqrust crates
#[cfg(feature = "mldsa")]
use pqrust_mldsa::*;
#[cfg(feature = "fndsa")]
use pqrust_fndsa::*;
use pqrust_traits::sign::{PublicKey, SecretKey, DetachedSignature};

/// Enumeration of supported signature algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignAlgorithm {
    #[cfg(feature = "mldsa")]
    Mldsa44,
    #[cfg(feature = "mldsa")]
    Mldsa65,
    #[cfg(feature = "mldsa")]
    Mldsa87,
    #[cfg(feature = "fndsa")]
    Fndsa512,
    #[cfg(feature = "fndsa")]
    Fndsa1024,
}

/// Main Signature interface
pub struct Sign {
    algorithm: SignAlgorithm,
}

impl Sign {
    /// Create a new Signature instance
    pub fn new(algorithm: SignAlgorithm) -> Self {
        Self { algorithm }
    }

    /// Create ML-DSA-44 instance
    #[cfg(feature = "mldsa")]
    pub fn mldsa44() -> Self {
        Self::new(SignAlgorithm::Mldsa44)
    }

    /// Create ML-DSA-65 instance
    #[cfg(feature = "mldsa")]
    pub fn mldsa65() -> Self {
        Self::new(SignAlgorithm::Mldsa65)
    }

    /// Create ML-DSA-87 instance
    #[cfg(feature = "mldsa")]
    pub fn mldsa87() -> Self {
        Self::new(SignAlgorithm::Mldsa87)
    }

    /// Create FN-DSA-512 instance
    #[cfg(feature = "fndsa")]
    pub fn fndsa512() -> Self {
        Self::new(SignAlgorithm::Fndsa512)
    }

    /// Create FN-DSA-1024 instance
    #[cfg(feature = "fndsa")]
    pub fn fndsa1024() -> Self {
        Self::new(SignAlgorithm::Fndsa1024)
    }

}

impl DigitalSignature for Sign {
    fn keygen(&self) -> Result<(Vec<u8>, Vec<u8>), PqcError> {
        match self.algorithm {
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa44 => {
                let (pk, sk) = mldsa44_keypair();
                Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
            }
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa65 => {
                let (pk, sk) = mldsa65_keypair();
                Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
            }
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa87 => {
                let (pk, sk) = mldsa87_keypair();
                Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
            }
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa512 => {
                let (pk, sk) = falcon512_keypair();
                Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
            }
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa1024 => {
                let (pk, sk) = falcon1024_keypair();
                Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
            }
        }
    }

    fn sign(&self, message: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, PqcError> {
        check_buffer_size(secret_key, self.secret_key_size())?;
        
        match self.algorithm {
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa44 => {
                let sk = mldsa44::SecretKey::from_bytes(secret_key).map_err(|_| PqcError::CryptoError("Invalid secret key".to_string()))?;
                let sig = mldsa44_detached_sign(message, &sk);
                Ok(sig.as_bytes().to_vec())
            }
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa65 => {
                let sk = mldsa65::SecretKey::from_bytes(secret_key).map_err(|_| PqcError::CryptoError("Invalid secret key".to_string()))?;
                let sig = mldsa65_detached_sign(message, &sk);
                Ok(sig.as_bytes().to_vec())
            }
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa87 => {
                let sk = mldsa87::SecretKey::from_bytes(secret_key).map_err(|_| PqcError::CryptoError("Invalid secret key".to_string()))?;
                let sig = mldsa87_detached_sign(message, &sk);
                Ok(sig.as_bytes().to_vec())
            }
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa512 => {
                let sk = fndsa512::SecretKey::from_bytes(secret_key).map_err(|_| PqcError::CryptoError("Invalid secret key".to_string()))?;
                let sig = falcon512_detached_sign(message, &sk);
                Ok(sig.as_bytes().to_vec())
            }
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa1024 => {
                let sk = fndsa1024::SecretKey::from_bytes(secret_key).map_err(|_| PqcError::CryptoError("Invalid secret key".to_string()))?;
                let sig = falcon1024_detached_sign(message, &sk);
                Ok(sig.as_bytes().to_vec())
            }
        }
    }

    fn verify(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool, PqcError> {
        check_buffer_size(public_key, self.public_key_size())?;
        // Note: signature size check removed because detached signature size can vary
        
        match self.algorithm {
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa44 => {
                let pk = mldsa44::PublicKey::from_bytes(public_key).map_err(|_| PqcError::CryptoError("Invalid public key".to_string()))?;
                let sig = mldsa44::DetachedSignature::from_bytes(signature).map_err(|_| PqcError::CryptoError("Invalid signature".to_string()))?;
                Ok(mldsa44_verify_detached_signature(&sig, message, &pk).is_ok())
            }
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa65 => {
                let pk = mldsa65::PublicKey::from_bytes(public_key).map_err(|_| PqcError::CryptoError("Invalid public key".to_string()))?;
                let sig = mldsa65::DetachedSignature::from_bytes(signature).map_err(|_| PqcError::CryptoError("Invalid signature".to_string()))?;
                Ok(mldsa65_verify_detached_signature(&sig, message, &pk).is_ok())
            }
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa87 => {
                let pk = mldsa87::PublicKey::from_bytes(public_key).map_err(|_| PqcError::CryptoError("Invalid public key".to_string()))?;
                let sig = mldsa87::DetachedSignature::from_bytes(signature).map_err(|_| PqcError::CryptoError("Invalid signature".to_string()))?;
                Ok(mldsa87_verify_detached_signature(&sig, message, &pk).is_ok())
            }
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa512 => {
                let pk = fndsa512::PublicKey::from_bytes(public_key).map_err(|_| PqcError::CryptoError("Invalid public key".to_string()))?;
                let sig = fndsa512::DetachedSignature::from_bytes(signature).map_err(|_| PqcError::CryptoError("Invalid signature".to_string()))?;
                Ok(falcon512_verify_detached_signature(&sig, message, &pk).is_ok())
            }
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa1024 => {
                let pk = fndsa1024::PublicKey::from_bytes(public_key).map_err(|_| PqcError::CryptoError("Invalid public key".to_string()))?;
                let sig = fndsa1024::DetachedSignature::from_bytes(signature).map_err(|_| PqcError::CryptoError("Invalid signature".to_string()))?;
                Ok(falcon1024_verify_detached_signature(&sig, message, &pk).is_ok())
            }
        }
    }

    fn public_key_size(&self) -> usize {
        match self.algorithm {
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa44 => mldsa44_public_key_bytes(),
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa65 => mldsa65_public_key_bytes(),
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa87 => mldsa87_public_key_bytes(),
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa512 => falcon512_public_key_bytes(),
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa1024 => falcon1024_public_key_bytes(),
        }
    }

    fn secret_key_size(&self) -> usize {
        match self.algorithm {
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa44 => mldsa44_secret_key_bytes(),
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa65 => mldsa65_secret_key_bytes(),
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa87 => mldsa87_secret_key_bytes(),
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa512 => falcon512_secret_key_bytes(),
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa1024 => falcon1024_secret_key_bytes(),
        }
    }

    fn signature_size(&self) -> usize {
        match self.algorithm {
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa44 => mldsa44_signature_bytes(),
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa65 => mldsa65_signature_bytes(),
            #[cfg(feature = "mldsa")]
            SignAlgorithm::Mldsa87 => mldsa87_signature_bytes(),
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa512 => falcon512_signature_bytes(),
            #[cfg(feature = "fndsa")]
            SignAlgorithm::Fndsa1024 => falcon1024_signature_bytes(),
        }
    }
}

// Type aliases for specific algorithms
pub type Mldsa44 = Sign;
pub type Mldsa65 = Sign;
pub type Mldsa87 = Sign;
pub type Fndsa512 = Sign;
pub type Fndsa1024 = Sign;
