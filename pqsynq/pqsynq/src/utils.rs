//! Utility functions for PQSynQ

use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;
use crate::error::PqcError;

/// Convert bytes to hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Convert hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, PqcError> {
    hex::decode(hex).map_err(|_| PqcError::InvalidKeySize)
}

/// Check if a buffer has the correct size
pub fn check_buffer_size(buffer: &[u8], expected_size: usize) -> Result<(), PqcError> {
    if buffer.len() != expected_size {
        return Err(PqcError::InvalidKeySize);
    }
    Ok(())
}

/// Check if a buffer is large enough
pub fn check_buffer_min_size(buffer: &[u8], min_size: usize) -> Result<(), PqcError> {
    if buffer.len() < min_size {
        return Err(PqcError::BufferTooSmall);
    }
    Ok(())
}

/// Generate random bytes using cryptographically secure random number generator
#[cfg(feature = "std")]
pub fn random_bytes(size: usize) -> Vec<u8> {
    use getrandom::getrandom;
    
    let mut bytes = vec![0u8; size];
    getrandom(&mut bytes).expect("Failed to generate random bytes");
    bytes
}

#[cfg(not(feature = "std"))]
pub fn random_bytes(_size: usize) -> Vec<u8> {
    // In no-std environments, random bytes must be provided externally
    // This function should not be called in no-std contexts
    panic!("random_bytes requires std feature or external RNG")
}

/// Compare two byte slices in constant time
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Copy bytes with bounds checking
pub fn safe_copy(dst: &mut [u8], src: &[u8]) -> Result<(), PqcError> {
    if src.len() > dst.len() {
        return Err(PqcError::BufferTooSmall);
    }
    dst[..src.len()].copy_from_slice(src);
    Ok(())
}
