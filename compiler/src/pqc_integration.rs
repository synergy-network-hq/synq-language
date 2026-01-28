//! Integration with aegis-pqsynq for PQC operations
//! This module provides helpers for generating PQC-related bytecode and runtime verification
//! 
//! This is the core integration layer that connects SynQ smart contracts with post-quantum cryptography.

use pqsynq::{DigitalSignature, KeyEncapsulation, Kem, KemAlgorithm, PqcError, Sign, SignAlgorithm};

/// PQC operation helpers for code generation and runtime
pub struct PqcIntegration;

impl PqcIntegration {
    /// Generate bytecode for ML-DSA signature verification
    pub fn mldsa_verify_bytecode() -> Vec<u8> {
        vec![0x80] // MLDSAVerify opcode
    }

    /// Generate bytecode for FN-DSA signature verification
    pub fn fndsa_verify_bytecode() -> Vec<u8> {
        vec![0x82] // FNDSAVerify opcode
    }

    /// Generate bytecode for ML-KEM key exchange
    pub fn mlkem_key_exchange_bytecode() -> Vec<u8> {
        vec![0x81] // MLKEMKeyExchange opcode
    }

    /// Check if a function name is a PQC operation
    pub fn is_pqc_function(name: &str) -> bool {
        name.starts_with("verify_mldsa")
            || name.starts_with("verify_fndsa")
            || name.starts_with("verify_slhdsa")
            || name.starts_with("mlkem_")
            || name.starts_with("mldsa_")
            || name.starts_with("fndsa_")
            || name.starts_with("slhdsa_")
    }

    /// Get the algorithm variant from a function name
    pub fn get_sign_algorithm(name: &str) -> Option<SignAlgorithm> {
        if name.contains("mldsa44") {
            Some(SignAlgorithm::Mldsa44)
        } else if name.contains("mldsa65") {
            Some(SignAlgorithm::Mldsa65)
        } else if name.contains("mldsa87") {
            Some(SignAlgorithm::Mldsa87)
        } else if name.contains("fndsa512") {
            Some(SignAlgorithm::Fndsa512)
        } else if name.contains("fndsa1024") {
            Some(SignAlgorithm::Fndsa1024)
        } else {
            None
        }
    }

    /// Get the KEM algorithm variant from a function name
    pub fn get_kem_algorithm(name: &str) -> Option<KemAlgorithm> {
        if name.contains("mlkem512") {
            Some(KemAlgorithm::Mlkem512)
        } else if name.contains("mlkem768") {
            Some(KemAlgorithm::Mlkem768)
        } else if name.contains("mlkem1024") {
            Some(KemAlgorithm::Mlkem1024)
        } else {
            None
        }
    }
}

impl PqcIntegration {
    /// Verify an ML-DSA signature using pqsynq
    pub fn verify_mldsa_signature(
        algorithm: SignAlgorithm,
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, PqcError> {
        let signer = Sign::new(algorithm);
        signer.verify(message, signature, public_key)
    }

    /// Verify an FN-DSA signature using pqsynq
    pub fn verify_fndsa_signature(
        algorithm: SignAlgorithm,
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, PqcError> {
        let signer = Sign::new(algorithm);
        signer.verify(message, signature, public_key)
    }

    /// Perform ML-KEM key exchange using pqsynq
    pub fn mlkem_key_exchange(
        algorithm: KemAlgorithm,
        public_key: &[u8],
        ciphertext: &[u8],
        secret_key: &[u8],
    ) -> Result<Vec<u8>, PqcError> {
        let kem = Kem::new(algorithm);
        kem.decapsulate(ciphertext, secret_key)
    }

    /// Generate a key pair for signatures
    pub fn generate_signature_keypair(algorithm: SignAlgorithm) -> Result<(Vec<u8>, Vec<u8>), PqcError> {
        let signer = Sign::new(algorithm);
        signer.keygen()
    }

    /// Generate a key pair for KEM
    pub fn generate_kem_keypair(algorithm: KemAlgorithm) -> Result<(Vec<u8>, Vec<u8>), PqcError> {
        let kem = Kem::new(algorithm);
        kem.keygen()
    }

    /// Encapsulate a shared secret (KEM)
    pub fn kem_encapsulate(algorithm: KemAlgorithm, public_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), PqcError> {
        let kem = Kem::new(algorithm);
        kem.encapsulate(public_key)
    }
}
