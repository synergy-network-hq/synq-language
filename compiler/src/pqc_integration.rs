//! Integration with aegis-pqsynq for PQC operations
//! This module provides helpers for generating PQC-related bytecode and runtime verification
//!
//! This is the core integration layer that connects SynQ smart contracts with post-quantum cryptography.

use pqsynq::{
    DigitalSignature, Kem, KemAlgorithm, KeyEncapsulation, PqcError, Sign, SignAlgorithm,
};

/// PQC operation helpers for code generation and runtime
pub struct PqcIntegration;

impl PqcIntegration {
    fn normalize(name: &str) -> String {
        let mut normalized = String::with_capacity(name.len());
        for ch in name.chars() {
            if ch != '_' && ch != '-' {
                normalized.push(ch.to_ascii_lowercase());
            }
        }
        normalized
    }

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

    /// Generate bytecode for HQC-KEM-128 key exchange
    pub fn hqckem128_key_exchange_bytecode() -> Vec<u8> {
        vec![0x84] // HQCKEM128KeyExchange opcode
    }

    /// Generate bytecode for HQC-KEM-192 key exchange
    pub fn hqckem192_key_exchange_bytecode() -> Vec<u8> {
        vec![0x85] // HQCKEM192KeyExchange opcode
    }

    /// Generate bytecode for HQC-KEM-256 key exchange
    pub fn hqckem256_key_exchange_bytecode() -> Vec<u8> {
        vec![0x86] // HQCKEM256KeyExchange opcode
    }

    /// Check if a function name is a PQC operation
    pub fn is_pqc_function(name: &str) -> bool {
        let normalized = Self::normalize(name);
        normalized.starts_with("verifymldsa")
            || normalized.starts_with("verifyfndsa")
            || normalized.starts_with("verifyslhdsa")
            || normalized.starts_with("mlkem")
            || normalized.starts_with("hqckem")
            || normalized.starts_with("mldsa")
            || normalized.starts_with("fndsa")
            || normalized.starts_with("slhdsa")
    }

    pub fn is_mldsa_verify_function(name: &str) -> bool {
        Self::normalize(name).starts_with("verifymldsa")
    }

    pub fn is_fndsa_verify_function(name: &str) -> bool {
        Self::normalize(name).starts_with("verifyfndsa")
    }

    pub fn is_slhdsa_verify_function(name: &str) -> bool {
        Self::normalize(name).starts_with("verifyslhdsa")
    }

    pub fn is_hqckem_family_function(name: &str) -> bool {
        Self::normalize(name).starts_with("hqckem")
    }

    pub fn is_mlkem_family_function(name: &str) -> bool {
        Self::normalize(name).starts_with("mlkem")
    }

    /// Get the algorithm variant from a function name
    pub fn get_sign_algorithm(name: &str) -> Option<SignAlgorithm> {
        let normalized = Self::normalize(name);
        if normalized.contains("mldsa44") {
            Some(SignAlgorithm::Mldsa44)
        } else if normalized.contains("mldsa65") {
            Some(SignAlgorithm::Mldsa65)
        } else if normalized.contains("mldsa87") {
            Some(SignAlgorithm::Mldsa87)
        } else if normalized.contains("fndsa512") {
            Some(SignAlgorithm::Fndsa512)
        } else if normalized.contains("fndsa1024") {
            Some(SignAlgorithm::Fndsa1024)
        } else {
            None
        }
    }

    /// Get the KEM algorithm variant from a function name
    pub fn get_kem_algorithm(name: &str) -> Option<KemAlgorithm> {
        let normalized = Self::normalize(name);
        if normalized.contains("mlkem512") {
            Some(KemAlgorithm::Mlkem512)
        } else if normalized.contains("mlkem768") {
            Some(KemAlgorithm::Mlkem768)
        } else if normalized.contains("mlkem1024") {
            Some(KemAlgorithm::Mlkem1024)
        } else if normalized.contains("hqckem128") {
            Some(KemAlgorithm::Hqckem128)
        } else if normalized.contains("hqckem192") {
            Some(KemAlgorithm::Hqckem192)
        } else if normalized.contains("hqckem256") {
            Some(KemAlgorithm::Hqckem256)
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
        _public_key: &[u8],
        ciphertext: &[u8],
        secret_key: &[u8],
    ) -> Result<Vec<u8>, PqcError> {
        let kem = Kem::new(algorithm);
        kem.decapsulate(ciphertext, secret_key)
    }

    /// Perform HQC-KEM key exchange using pqsynq
    pub fn hqckem_key_exchange(
        algorithm: KemAlgorithm,
        _public_key: &[u8],
        ciphertext: &[u8],
        secret_key: &[u8],
    ) -> Result<Vec<u8>, PqcError> {
        let kem = Kem::new(algorithm);
        kem.decapsulate(ciphertext, secret_key)
    }

    /// Generate a key pair for signatures
    pub fn generate_signature_keypair(
        algorithm: SignAlgorithm,
    ) -> Result<(Vec<u8>, Vec<u8>), PqcError> {
        let signer = Sign::new(algorithm);
        signer.keygen()
    }

    /// Generate a key pair for KEM
    pub fn generate_kem_keypair(algorithm: KemAlgorithm) -> Result<(Vec<u8>, Vec<u8>), PqcError> {
        let kem = Kem::new(algorithm);
        kem.keygen()
    }

    /// Encapsulate a shared secret (KEM)
    pub fn kem_encapsulate(
        algorithm: KemAlgorithm,
        public_key: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>), PqcError> {
        let kem = Kem::new(algorithm);
        kem.encapsulate(public_key)
    }
}

#[cfg(test)]
mod tests {
    use super::PqcIntegration;
    use pqsynq::KemAlgorithm;

    #[test]
    fn detects_hqckem_functions() {
        assert!(PqcIntegration::is_pqc_function(
            "hqckem_hqckem128_decapsulate"
        ));
        assert_eq!(
            PqcIntegration::get_kem_algorithm("hqckem_hqckem256_decapsulate"),
            Some(KemAlgorithm::Hqckem256)
        );
    }

    #[test]
    fn detects_camel_case_verify_aliases() {
        assert!(PqcIntegration::is_pqc_function("verifyMLDSASignature"));
        assert!(PqcIntegration::is_mldsa_verify_function(
            "verifyMLDSASignature"
        ));
        assert!(PqcIntegration::is_fndsa_verify_function(
            "verifyFNDSASignature"
        ));
    }

    #[test]
    fn hqckem_roundtrip_via_integration_helper() {
        let (pk, sk) = PqcIntegration::generate_kem_keypair(KemAlgorithm::Hqckem128)
            .expect("keygen should work");
        let (ct, ss1) = PqcIntegration::kem_encapsulate(KemAlgorithm::Hqckem128, &pk)
            .expect("encapsulation should work");
        let ss2 = PqcIntegration::hqckem_key_exchange(KemAlgorithm::Hqckem128, &pk, &ct, &sk)
            .expect("decapsulation should work");
        assert_eq!(ss1, ss2);
    }
}
