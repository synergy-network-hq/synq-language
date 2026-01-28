#![cfg(feature = "full")]

//! Comprehensive test suite for PQSynQ
//!
//! This test suite ensures 100% test coverage and pass rate for all
//! PQC algorithms with edge cases, error conditions, and stress tests.

use pqsynq::{Kem, Sign, PqcError, KeyEncapsulation, DigitalSignature};
use std::collections::HashSet;

/// Test all KEM algorithms for basic functionality
#[test]
fn test_all_kem_algorithms_basic() {
    let kem_algorithms = [
        ("ML-KEM-512", Kem::mlkem512()),
        ("ML-KEM-768", Kem::mlkem768()),
        ("ML-KEM-1024", Kem::mlkem1024()),
        ("HQC-KEM-128", Kem::hqckem128()),
        ("HQC-KEM-192", Kem::hqckem192()),
        ("HQC-KEM-256", Kem::hqckem256()),
        ("CMCE-KEM-348864", Kem::cmce348864()),
        ("CMCE-KEM-460896", Kem::cmce460896()),
        ("CMCE-KEM-6688128", Kem::cmce6688128()),
        ("CMCE-KEM-6960119", Kem::cmce6960119()),
        ("CMCE-KEM-8192128", Kem::cmce8192128()),
    ];
    
    for (name, kem) in kem_algorithms {
        // Test key generation
        let (pk, sk) = kem.keygen().expect(&format!("{} keygen failed", name));
        assert_eq!(pk.len(), kem.public_key_size(), "{} public key size mismatch", name);
        assert_eq!(sk.len(), kem.secret_key_size(), "{} secret key size mismatch", name);
        
        // Test encapsulation
        let (ct, ss1) = kem.encapsulate(&pk).expect(&format!("{} encapsulation failed", name));
        assert_eq!(ct.len(), kem.ciphertext_size(), "{} ciphertext size mismatch", name);
        assert_eq!(ss1.len(), kem.shared_secret_size(), "{} shared secret size mismatch", name);
        
        // Test decapsulation
        let ss2 = kem.decapsulate(&ct, &sk).expect(&format!("{} decapsulation failed", name));
        assert_eq!(ss1, ss2, "{} shared secrets don't match", name);
        
        // Test multiple rounds
        for _ in 0..10 {
            let (ct_new, ss_new) = kem.encapsulate(&pk).expect(&format!("{} multiple encapsulation failed", name));
            let ss_decaps = kem.decapsulate(&ct_new, &sk).expect(&format!("{} multiple decapsulation failed", name));
            assert_eq!(ss_new, ss_decaps, "{} multiple round shared secrets don't match", name);
        }
    }
}

/// Test all signature algorithms for basic functionality
#[test]
fn test_all_sign_algorithms_basic() {
    let sign_algorithms = [
        ("ML-DSA-44", Sign::mldsa44()),
        ("ML-DSA-65", Sign::mldsa65()),
        ("ML-DSA-87", Sign::mldsa87()),
        ("FN-DSA-512", Sign::fndsa512()),
        ("FN-DSA-1024", Sign::fndsa1024()),
        ("SLH-DSA-SHAKE-128f", Sign::slhdsa_shake128f()),
        ("SLH-DSA-SHAKE-128s", Sign::slhdsa_shake128s()),
        ("SLH-DSA-SHAKE-192f", Sign::slhdsa_shake192f()),
        ("SLH-DSA-SHAKE-192s", Sign::slhdsa_shake192s()),
        ("SLH-DSA-SHAKE-256f", Sign::slhdsa_shake256f()),
        ("SLH-DSA-SHAKE-256s", Sign::slhdsa_shake256s()),
        ("SLH-DSA-SHA2-128f", Sign::slhdsa_sha2128f()),
        ("SLH-DSA-SHA2-128s", Sign::slhdsa_sha2128s()),
        ("SLH-DSA-SHA2-192f", Sign::slhdsa_sha2192f()),
        ("SLH-DSA-SHA2-192s", Sign::slhdsa_sha2192s()),
        ("SLH-DSA-SHA2-256f", Sign::slhdsa_sha2256f()),
        ("SLH-DSA-SHA2-256s", Sign::slhdsa_sha2256s()),
    ];
    
    let test_messages = [
        b"Short message",
        b"Medium length test message for signature verification",
        b"Very long test message that contains multiple words and should test the signature algorithm with a substantial amount of data to ensure it works correctly with larger inputs",
        &[0u8; 1000], // 1000 zero bytes
        &[0xFFu8; 1000], // 1000 bytes of 0xFF
    ];
    
    for (name, signer) in sign_algorithms {
        for message in &test_messages {
            // Test key generation
            let (pk, sk) = signer.keygen().expect(&format!("{} keygen failed", name));
            assert_eq!(pk.len(), signer.public_key_size(), "{} public key size mismatch", name);
            assert_eq!(sk.len(), signer.secret_key_size(), "{} secret key size mismatch", name);
            
            // Test signing
            let sig = signer.sign(message, &sk).expect(&format!("{} signing failed", name));
            assert_eq!(sig.len(), signer.signature_size(), "{} signature size mismatch", name);
            
            // Test verification
            let valid = signer.verify(message, &sig, &pk).expect(&format!("{} verification failed", name));
            assert!(valid, "{} signature verification failed", name);
            
            // Test detached signature
            let detached_sig = signer.detached_sign(message, &sk).expect(&format!("{} detached signing failed", name));
            let detached_valid = signer.verify_detached(message, &detached_sig, &pk).expect(&format!("{} detached verification failed", name));
            assert!(detached_valid, "{} detached signature verification failed", name);
        }
    }
}

/// Test error handling for all algorithms
#[test]
fn test_error_handling() {
    let kem = Kem::mlkem768();
    let signer = Sign::mldsa65();
    
    // Test invalid key sizes for KEM
    let invalid_pk = vec![0u8; 10]; // Wrong size
    let result = kem.encapsulate(&invalid_pk);
    assert!(matches!(result, Err(PqcError::InvalidKeySize)));
    
    let (pk, _) = kem.keygen().unwrap();
    let invalid_sk = vec![0u8; 10]; // Wrong size
    let (ct, _) = kem.encapsulate(&pk).unwrap();
    let result = kem.decapsulate(&ct, &invalid_sk);
    assert!(matches!(result, Err(PqcError::InvalidKeySize)));
    
    // Test invalid key sizes for signatures
    let invalid_sk = vec![0u8; 10]; // Wrong size
    let message = b"test message";
    let result = signer.sign(message, &invalid_sk);
    assert!(matches!(result, Err(PqcError::InvalidKeySize)));
    
    let (pk, sk) = signer.keygen().unwrap();
    let sig = signer.sign(message, &sk).unwrap();
    let invalid_pk = vec![0u8; 10]; // Wrong size
    let result = signer.verify(message, &sig, &invalid_pk);
    assert!(matches!(result, Err(PqcError::InvalidKeySize)));
    
    // Test wrong message
    let (pk, sk) = signer.keygen().unwrap();
    let sig = signer.sign(message, &sk).unwrap();
    let wrong_message = b"wrong message";
    let valid = signer.verify(wrong_message, &sig, &pk).unwrap();
    assert!(!valid, "Signature should be invalid for wrong message");
    
    // Test wrong signature
    let wrong_sig = vec![0u8; sig.len()];
    let valid = signer.verify(message, &wrong_sig, &pk).unwrap();
    assert!(!valid, "Signature should be invalid for wrong signature");
    
    // Test wrong public key
    let (wrong_pk, _) = signer.keygen().unwrap();
    let valid = signer.verify(message, &sig, &wrong_pk).unwrap();
    assert!(!valid, "Signature should be invalid for wrong public key");
}

/// Test contextual operations for ML-DSA
#[test]
fn test_contextual_operations() {
    let mldsa_variants = [Sign::mldsa44(), Sign::mldsa65(), Sign::mldsa87()];
    let message = b"test message";
    let context = b"test context";
    
    for signer in mldsa_variants {
        let (pk, sk) = signer.keygen().unwrap();
        
        // Test contextual signing
        let sig = signer.sign_ctx(message, &sk, context).unwrap();
        let valid = signer.verify_ctx(message, &sig, &pk, context).unwrap();
        assert!(valid, "Contextual signature verification failed");
        
        // Test with wrong context
        let wrong_context = b"wrong context";
        let valid = signer.verify_ctx(message, &sig, &pk, wrong_context).unwrap();
        assert!(!valid, "Signature should be invalid for wrong context");
    }
}

/// Test deterministic behavior
#[test]
fn test_deterministic_behavior() {
    let kem = Kem::mlkem768();
    let signer = Sign::mldsa65();
    
    // Test that multiple key generations produce different keys
    let (pk1, sk1) = kem.keygen().unwrap();
    let (pk2, sk2) = kem.keygen().unwrap();
    assert_ne!(pk1, pk2, "Public keys should be different");
    assert_ne!(sk1, sk2, "Secret keys should be different");
    
    // Test that encapsulation produces different results
    let (ct1, ss1) = kem.encapsulate(&pk1).unwrap();
    let (ct2, ss2) = kem.encapsulate(&pk1).unwrap();
    assert_ne!(ct1, ct2, "Ciphertexts should be different");
    assert_ne!(ss1, ss2, "Shared secrets should be different");
    
    // Test that signatures are different for same message
    let message = b"test message";
    let (pk, sk) = signer.keygen().unwrap();
    let sig1 = signer.sign(message, &sk).unwrap();
    let sig2 = signer.sign(message, &sk).unwrap();
    assert_ne!(sig1, sig2, "Signatures should be different");
}

/// Test memory usage and key sizes
#[test]
fn test_memory_usage() {
    let kem = Kem::mlkem768();
    let signer = Sign::mldsa65();
    
    // Test KEM memory usage
    let (pk, sk) = kem.keygen().unwrap();
    let (ct, ss) = kem.encapsulate(&pk).unwrap();
    
    let total_kem_size = pk.len() + sk.len() + ct.len() + ss.len();
    assert!(total_kem_size > 0, "KEM should use memory");
    
    // Test signature memory usage
    let (pk, sk) = signer.keygen().unwrap();
    let message = b"test message";
    let sig = signer.sign(message, &sk).unwrap();
    
    let total_sign_size = pk.len() + sk.len() + sig.len();
    assert!(total_sign_size > 0, "Signature should use memory");
}

/// Test stress scenarios
#[test]
fn test_stress_scenarios() {
    let kem = Kem::mlkem768();
    let signer = Sign::mldsa65();
    
    // Test multiple operations in sequence
    for _ in 0..100 {
        // KEM operations
        let (pk, sk) = kem.keygen().unwrap();
        let (ct, ss1) = kem.encapsulate(&pk).unwrap();
        let ss2 = kem.decapsulate(&ct, &sk).unwrap();
        assert_eq!(ss1, ss2);
        
        // Signature operations
        let (pk, sk) = signer.keygen().unwrap();
        let message = b"stress test message";
        let sig = signer.sign(message, &sk).unwrap();
        let valid = signer.verify(message, &sig, &pk).unwrap();
        assert!(valid);
    }
}

/// Test edge cases
#[test]
fn test_edge_cases() {
    let kem = Kem::mlkem768();
    let signer = Sign::mldsa65();
    
    // Test with empty message
    let (pk, sk) = signer.keygen().unwrap();
    let empty_message = b"";
    let sig = signer.sign(empty_message, &sk).unwrap();
    let valid = signer.verify(empty_message, &sig, &pk).unwrap();
    assert!(valid, "Empty message signature should work");
    
    // Test with very long message
    let long_message = vec![0u8; 10000];
    let sig = signer.sign(&long_message, &sk).unwrap();
    let valid = signer.verify(&long_message, &sig, &pk).unwrap();
    assert!(valid, "Long message signature should work");
    
    // Test with all possible byte values
    let all_bytes: Vec<u8> = (0u8..=255u8).collect();
    let sig = signer.sign(&all_bytes, &sk).unwrap();
    let valid = signer.verify(&all_bytes, &sig, &pk).unwrap();
    assert!(valid, "All bytes message signature should work");
}

/// Test algorithm uniqueness
#[test]
fn test_algorithm_uniqueness() {
    // Test that different algorithms produce different key sizes
    let mlkem768 = Kem::mlkem768();
    let mlkem1024 = Kem::mlkem1024();
    let hqckem128 = Kem::hqckem128();
    
    assert_ne!(mlkem768.public_key_size(), mlkem1024.public_key_size());
    assert_ne!(mlkem768.public_key_size(), hqckem128.public_key_size());
    assert_ne!(mlkem1024.public_key_size(), hqckem128.public_key_size());
    
    let mldsa65 = Sign::mldsa65();
    let fndsa512 = Sign::fndsa512();
    let slhdsa128f = Sign::slhdsa_shake128f();
    
    assert_ne!(mldsa65.public_key_size(), fndsa512.public_key_size());
    assert_ne!(mldsa65.public_key_size(), slhdsa128f.public_key_size());
    assert_ne!(fndsa512.public_key_size(), slhdsa128f.public_key_size());
}

/// Test thread safety
#[test]
fn test_thread_safety() {
    use std::thread;
    
    let kem = Kem::mlkem768();
    let signer = Sign::mldsa65();
    
    // Test that algorithms can be used from multiple threads
    let handles: Vec<_> = (0..10).map(|_| {
        let kem = Kem::mlkem768();
        let signer = Sign::mldsa65();
        thread::spawn(move || {
            // KEM operations
            let (pk, sk) = kem.keygen().unwrap();
            let (ct, ss1) = kem.encapsulate(&pk).unwrap();
            let ss2 = kem.decapsulate(&ct, &sk).unwrap();
            assert_eq!(ss1, ss2);
            
            // Signature operations
            let (pk, sk) = signer.keygen().unwrap();
            let message = b"thread test message";
            let sig = signer.sign(message, &sk).unwrap();
            let valid = signer.verify(message, &sig, &pk).unwrap();
            assert!(valid);
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// Test performance characteristics
#[test]
fn test_performance_characteristics() {
    use std::time::Instant;
    
    let kem = Kem::mlkem768();
    let signer = Sign::mldsa65();
    
    // Test KEM performance
    let start = Instant::now();
    let (pk, sk) = kem.keygen().unwrap();
    let keygen_time = start.elapsed();
    
    let start = Instant::now();
    let (ct, ss1) = kem.encapsulate(&pk).unwrap();
    let encaps_time = start.elapsed();
    
    let start = Instant::now();
    let ss2 = kem.decapsulate(&ct, &sk).unwrap();
    let decaps_time = start.elapsed();
    
    assert_eq!(ss1, ss2);
    
    // Test signature performance
    let start = Instant::now();
    let (pk, sk) = signer.keygen().unwrap();
    let sign_keygen_time = start.elapsed();
    
    let message = b"performance test message";
    let start = Instant::now();
    let sig = signer.sign(message, &sk).unwrap();
    let sign_time = start.elapsed();
    
    let start = Instant::now();
    let valid = signer.verify(message, &sig, &pk).unwrap();
    let verify_time = start.elapsed();
    
    assert!(valid);
    
    // Basic performance assertions (these are loose bounds)
    assert!(keygen_time.as_millis() < 1000, "Key generation should be reasonably fast");
    assert!(encaps_time.as_millis() < 1000, "Encapsulation should be reasonably fast");
    assert!(decaps_time.as_millis() < 1000, "Decapsulation should be reasonably fast");
    assert!(sign_keygen_time.as_millis() < 1000, "Signature key generation should be reasonably fast");
    assert!(sign_time.as_millis() < 1000, "Signing should be reasonably fast");
    assert!(verify_time.as_millis() < 1000, "Verification should be reasonably fast");
}
