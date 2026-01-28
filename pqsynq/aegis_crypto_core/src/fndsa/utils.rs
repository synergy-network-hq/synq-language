//! Utility functions for FN-DSA (FFT over NTRU-Lattice-Based Digital Signature Algorithm) operations.

use alloc::format;
use pqrust_fndsa::fndsa512::{public_key_bytes, secret_key_bytes};
use alloc::string::String;
use alloc::string::ToString;


/// Returns the expected public key length for Falcon.
pub fn public_key_length() -> usize {
    public_key_bytes()
}

/// Returns the expected secret key length for Falcon.
pub fn secret_key_length() -> usize {
    secret_key_bytes()
}

/// Validates that a public key has the correct length.
pub fn validate_public_key_length(key: &[u8]) -> Result<(), String> {
    let expected_len = public_key_length();
    if key.len() != expected_len {
        Err(format!("Invalid public key length. Expected {}, got {}", expected_len, key.len()))
    } else {
        Ok(())
    }
}

/// Validates that a secret key has the correct length.
pub fn validate_secret_key_length(key: &[u8]) -> Result<(), String> {
    let expected_len = secret_key_length();
    if key.len() != expected_len {
        Err(format!("Invalid secret key length. Expected {}, got {}", expected_len, key.len()))
    } else {
        Ok(())
    }
}

/// Validates that a signature is not empty.
/// Note: Falcon signatures have variable length, so we only check they're not empty.
pub fn validate_signature_not_empty(signature: &[u8]) -> Result<(), String> {
    if signature.is_empty() {
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
        assert!(public_key_length() > 0);
        assert!(secret_key_length() > 0);
    }

    #[test]
    fn test_validation_functions() {
        let valid_pk = vec![0u8; public_key_length()];
        let invalid_pk = vec![0u8; public_key_length() + 1];

        assert!(validate_public_key_length(&valid_pk).is_ok());
        assert!(validate_public_key_length(&invalid_pk).is_err());

        let valid_sk = vec![0u8; secret_key_length()];
        let invalid_sk = vec![0u8; secret_key_length() - 1];

        assert!(validate_secret_key_length(&valid_sk).is_ok());
        assert!(validate_secret_key_length(&invalid_sk).is_err());

        let valid_sig = vec![0u8; 100];
        let empty_sig = vec![];

        assert!(validate_signature_not_empty(&valid_sig).is_ok());
        assert!(validate_signature_not_empty(&empty_sig).is_err());
    }
}
