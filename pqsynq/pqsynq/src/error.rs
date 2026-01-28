//! Error types for PQSynQ

use core::fmt;
use alloc::string::String;

/// Errors that can occur during PQC operations
#[derive(Debug, Clone, PartialEq)]
pub enum PqcError {
    /// Invalid key size
    InvalidKeySize,
    /// Invalid ciphertext size
    InvalidCiphertextSize,
    /// Invalid signature size
    InvalidSignatureSize,
    /// Invalid message size
    InvalidMessageSize,
    /// Key generation failed
    KeyGenerationFailed,
    /// Encryption/encapsulation failed
    EncryptionFailed,
    /// Decryption/decapsulation failed
    DecryptionFailed,
    /// Signature generation failed
    SignatureFailed,
    /// Signature verification failed
    VerificationFailed,
    /// Invalid algorithm
    InvalidAlgorithm,
    /// Buffer too small
    BufferTooSmall,
    /// Internal error
    InternalError,
    /// Not implemented
    NotImplemented,
    /// Cryptographic error with message
    CryptoError(String),
}

impl fmt::Display for PqcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PqcError::InvalidKeySize => write!(f, "Invalid key size"),
            PqcError::InvalidCiphertextSize => write!(f, "Invalid ciphertext size"),
            PqcError::InvalidSignatureSize => write!(f, "Invalid signature size"),
            PqcError::InvalidMessageSize => write!(f, "Invalid message size"),
            PqcError::KeyGenerationFailed => write!(f, "Key generation failed"),
            PqcError::EncryptionFailed => write!(f, "Encryption/encapsulation failed"),
            PqcError::DecryptionFailed => write!(f, "Decryption/decapsulation failed"),
            PqcError::SignatureFailed => write!(f, "Signature generation failed"),
            PqcError::VerificationFailed => write!(f, "Signature verification failed"),
            PqcError::InvalidAlgorithm => write!(f, "Invalid algorithm"),
            PqcError::BufferTooSmall => write!(f, "Buffer too small"),
            PqcError::InternalError => write!(f, "Internal error"),
            PqcError::NotImplemented => write!(f, "Not implemented"),
            PqcError::CryptoError(msg) => write!(f, "Cryptographic error: {}", msg),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PqcError {}
