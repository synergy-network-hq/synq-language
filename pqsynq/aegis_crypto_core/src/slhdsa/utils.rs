//! Utility functions for SLH-DSA (Stateless Hash-Based Digital Signature Algorithm) operations.

use pqrust_slhdsa::slhdsasha2128fsimple::{public_key_bytes, secret_key_bytes, signature_bytes};
use alloc::string::String;
use alloc::format;
use alloc::string::ToString;


/// Returns the expected public key length for SPHINCS+.
pub fn public_key_length() -> usize {
    public_key_bytes()
}

/// Returns the expected secret key length for SPHINCS+.
pub fn secret_key_length() -> usize {
    secret_key_bytes()
}

/// Returns the expected signature length for SPHINCS+.
pub fn signature_length() -> usize {
    signature_bytes()
}

/// Validates that a public key has the correct length.
pub fn validate_public_key_length(key: &[u8]) -> Result<(), String> {
    let expected_len = public_key_bytes();
    if key.len() != expected_len {
        Err(format!("Invalid public key length. Expected {}, got {}", expected_len, key.len()))
    } else {
        Ok(())
    }
}

/// Validates that a secret key has the correct length.
pub fn validate_secret_key_length(key: &[u8]) -> Result<(), String> {
    let expected_len = secret_key_bytes();
    if key.len() != expected_len {
        Err(format!("Invalid secret key length. Expected {}, got {}", expected_len, key.len()))
    } else {
        Ok(())
    }
}

/// Validates that a signature has the correct length.
pub fn validate_signature_length(signature: &[u8]) -> Result<(), String> {
    let expected_len = signature_bytes();
    if signature.len() > expected_len {
        Err(format!("Invalid signature length. Maximum {}, got {}", expected_len, signature.len()))
    } else if signature.is_empty() {
        Err("Signature cannot be empty".to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_constants() {
        assert_eq!(public_key_length(), public_key_bytes());
        assert_eq!(secret_key_length(), secret_key_bytes());
        assert_eq!(signature_length(), signature_bytes());
    }

    #[test]
    fn test_validation_functions() {
        let valid_pk = vec![0u8; public_key_bytes()];
        let invalid_pk = vec![0u8; public_key_bytes() + 1];

        assert!(validate_public_key_length(&valid_pk).is_ok());
        assert!(validate_public_key_length(&invalid_pk).is_err());

        let valid_sk = vec![0u8; secret_key_bytes()];
        let invalid_sk = vec![0u8; secret_key_bytes() - 1];

        assert!(validate_secret_key_length(&valid_sk).is_ok());
        assert!(validate_secret_key_length(&invalid_sk).is_err());

        let valid_sig = vec![0u8; signature_bytes()];
        let invalid_sig = vec![0u8; signature_bytes() + 10];
        let empty_sig = vec![];

        assert!(validate_signature_length(&valid_sig).is_ok());
        assert!(validate_signature_length(&invalid_sig).is_err());
        assert!(validate_signature_length(&empty_sig).is_err());
    }
}
