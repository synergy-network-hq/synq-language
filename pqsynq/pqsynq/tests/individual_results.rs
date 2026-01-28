#![cfg(feature = "full")]

//! Individual test results for each PQC algorithm

use pqsynq::{Kem, Sign, PqcError, KeyEncapsulation, DigitalSignature};

/// Test ML-KEM-512 with individual results
#[test]
fn test_mlkem512_individual() -> Result<(), PqcError> {
    println!("=== ML-KEM-512 Individual Test Results ===");
    
    let kem = Kem::mlkem512();
    println!("Algorithm: ML-KEM-512");
    println!("Public key size: {} bytes", kem.public_key_size());
    println!("Secret key size: {} bytes", kem.secret_key_size());
    println!("Ciphertext size: {} bytes", kem.ciphertext_size());
    println!("Shared secret size: {} bytes", kem.shared_secret_size());
    
    // Key generation test
    let start = std::time::Instant::now();
    let (pk, sk) = kem.keygen()?;
    let keygen_time = start.elapsed();
    println!("Key generation: {}ms", keygen_time.as_millis());
    
    // Encapsulation test
    let start = std::time::Instant::now();
    let (ct, ss1) = kem.encapsulate(&pk)?;
    let encaps_time = start.elapsed();
    println!("Encapsulation: {}ms", encaps_time.as_millis());
    
    // Decapsulation test
    let start = std::time::Instant::now();
    let ss2 = kem.decapsulate(&ct, &sk)?;
    let decaps_time = start.elapsed();
    println!("Decapsulation: {}ms", decaps_time.as_millis());
    
    // Verify correctness
    assert_eq!(ss1, ss2);
    println!("✓ ML-KEM-512 test PASSED");
    println!("");
    
    Ok(())
}

/// Test ML-KEM-768 with individual results
#[test]
fn test_mlkem768_individual() -> Result<(), PqcError> {
    println!("=== ML-KEM-768 Individual Test Results ===");
    
    let kem = Kem::mlkem768();
    println!("Algorithm: ML-KEM-768");
    println!("Public key size: {} bytes", kem.public_key_size());
    println!("Secret key size: {} bytes", kem.secret_key_size());
    println!("Ciphertext size: {} bytes", kem.ciphertext_size());
    println!("Shared secret size: {} bytes", kem.shared_secret_size());
    
    // Key generation test
    let start = std::time::Instant::now();
    let (pk, sk) = kem.keygen()?;
    let keygen_time = start.elapsed();
    println!("Key generation: {}ms", keygen_time.as_millis());
    
    // Encapsulation test
    let start = std::time::Instant::now();
    let (ct, ss1) = kem.encapsulate(&pk)?;
    let encaps_time = start.elapsed();
    println!("Encapsulation: {}ms", encaps_time.as_millis());
    
    // Decapsulation test
    let start = std::time::Instant::now();
    let ss2 = kem.decapsulate(&ct, &sk)?;
    let decaps_time = start.elapsed();
    println!("Decapsulation: {}ms", decaps_time.as_millis());
    
    // Verify correctness
    assert_eq!(ss1, ss2);
    println!("✓ ML-KEM-768 test PASSED");
    println!("");
    
    Ok(())
}

/// Test ML-KEM-1024 with individual results
#[test]
fn test_mlkem1024_individual() -> Result<(), PqcError> {
    println!("=== ML-KEM-1024 Individual Test Results ===");
    
    let kem = Kem::mlkem1024();
    println!("Algorithm: ML-KEM-1024");
    println!("Public key size: {} bytes", kem.public_key_size());
    println!("Secret key size: {} bytes", kem.secret_key_size());
    println!("Ciphertext size: {} bytes", kem.ciphertext_size());
    println!("Shared secret size: {} bytes", kem.shared_secret_size());
    
    // Key generation test
    let start = std::time::Instant::now();
    let (pk, sk) = kem.keygen()?;
    let keygen_time = start.elapsed();
    println!("Key generation: {}ms", keygen_time.as_millis());
    
    // Encapsulation test
    let start = std::time::Instant::now();
    let (ct, ss1) = kem.encapsulate(&pk)?;
    let encaps_time = start.elapsed();
    println!("Encapsulation: {}ms", encaps_time.as_millis());
    
    // Decapsulation test
    let start = std::time::Instant::now();
    let ss2 = kem.decapsulate(&ct, &sk)?;
    let decaps_time = start.elapsed();
    println!("Decapsulation: {}ms", decaps_time.as_millis());
    
    // Verify correctness
    assert_eq!(ss1, ss2);
    println!("✓ ML-KEM-1024 test PASSED");
    println!("");
    
    Ok(())
}

/// Test HQC-KEM-128 with individual results
#[test]
fn test_hqckem128_individual() -> Result<(), PqcError> {
    println!("=== HQC-KEM-128 Individual Test Results ===");
    
    let kem = Kem::hqckem128();
    println!("Algorithm: HQC-KEM-128");
    println!("Public key size: {} bytes", kem.public_key_size());
    println!("Secret key size: {} bytes", kem.secret_key_size());
    println!("Ciphertext size: {} bytes", kem.ciphertext_size());
    println!("Shared secret size: {} bytes", kem.shared_secret_size());
    
    // Key generation test
    let start = std::time::Instant::now();
    let (pk, sk) = kem.keygen()?;
    let keygen_time = start.elapsed();
    println!("Key generation: {}ms", keygen_time.as_millis());
    
    // Encapsulation test
    let start = std::time::Instant::now();
    let (ct, ss1) = kem.encapsulate(&pk)?;
    let encaps_time = start.elapsed();
    println!("Encapsulation: {}ms", encaps_time.as_millis());
    
    // Decapsulation test
    let start = std::time::Instant::now();
    let ss2 = kem.decapsulate(&ct, &sk)?;
    let decaps_time = start.elapsed();
    println!("Decapsulation: {}ms", decaps_time.as_millis());
    
    // Verify correctness
    assert_eq!(ss1, ss2);
    println!("✓ HQC-KEM-128 test PASSED");
    println!("");
    
    Ok(())
}

/// Test HQC-KEM-192 with individual results
#[test]
fn test_hqckem192_individual() -> Result<(), PqcError> {
    println!("=== HQC-KEM-192 Individual Test Results ===");
    
    let kem = Kem::hqckem192();
    println!("Algorithm: HQC-KEM-192");
    println!("Public key size: {} bytes", kem.public_key_size());
    println!("Secret key size: {} bytes", kem.secret_key_size());
    println!("Ciphertext size: {} bytes", kem.ciphertext_size());
    println!("Shared secret size: {} bytes", kem.shared_secret_size());
    
    // Key generation test
    let start = std::time::Instant::now();
    let (pk, sk) = kem.keygen()?;
    let keygen_time = start.elapsed();
    println!("Key generation: {}ms", keygen_time.as_millis());
    
    // Encapsulation test
    let start = std::time::Instant::now();
    let (ct, ss1) = kem.encapsulate(&pk)?;
    let encaps_time = start.elapsed();
    println!("Encapsulation: {}ms", encaps_time.as_millis());
    
    // Decapsulation test
    let start = std::time::Instant::now();
    let ss2 = kem.decapsulate(&ct, &sk)?;
    let decaps_time = start.elapsed();
    println!("Decapsulation: {}ms", decaps_time.as_millis());
    
    // Verify correctness
    assert_eq!(ss1, ss2);
    println!("✓ HQC-KEM-192 test PASSED");
    println!("");
    
    Ok(())
}

/// Test HQC-KEM-256 with individual results
#[test]
fn test_hqckem256_individual() -> Result<(), PqcError> {
    println!("=== HQC-KEM-256 Individual Test Results ===");
    
    let kem = Kem::hqckem256();
    println!("Algorithm: HQC-KEM-256");
    println!("Public key size: {} bytes", kem.public_key_size());
    println!("Secret key size: {} bytes", kem.secret_key_size());
    println!("Ciphertext size: {} bytes", kem.ciphertext_size());
    println!("Shared secret size: {} bytes", kem.shared_secret_size());
    
    // Key generation test
    let start = std::time::Instant::now();
    let (pk, sk) = kem.keygen()?;
    let keygen_time = start.elapsed();
    println!("Key generation: {}ms", keygen_time.as_millis());
    
    // Encapsulation test
    let start = std::time::Instant::now();
    let (ct, ss1) = kem.encapsulate(&pk)?;
    let encaps_time = start.elapsed();
    println!("Encapsulation: {}ms", encaps_time.as_millis());
    
    // Decapsulation test
    let start = std::time::Instant::now();
    let ss2 = kem.decapsulate(&ct, &sk)?;
    let decaps_time = start.elapsed();
    println!("Decapsulation: {}ms", decaps_time.as_millis());
    
    // Verify correctness
    assert_eq!(ss1, ss2);
    println!("✓ HQC-KEM-256 test PASSED");
    println!("");
    
    Ok(())
}

/// Test ML-DSA-44 with individual results
#[test]
fn test_mldsa44_individual() -> Result<(), PqcError> {
    println!("=== ML-DSA-44 Individual Test Results ===");
    
    let signer = Sign::mldsa44();
    let message = b"ML-DSA-44 test message";
    println!("Algorithm: ML-DSA-44");
    println!("Public key size: {} bytes", signer.public_key_size());
    println!("Secret key size: {} bytes", signer.secret_key_size());
    println!("Signature size: {} bytes", signer.signature_size());
    println!("Message size: {} bytes", message.len());
    
    // Key generation test
    let start = std::time::Instant::now();
    let (pk, sk) = signer.keygen()?;
    let keygen_time = start.elapsed();
    println!("Key generation: {}ms", keygen_time.as_millis());
    
    // Signing test
    let start = std::time::Instant::now();
    let signature = signer.sign(message, &sk)?;
    let sign_time = start.elapsed();
    println!("Signing: {}ms", sign_time.as_millis());
    
    // Verification test
    let start = std::time::Instant::now();
    let is_valid = signer.verify(message, &signature, &pk)?;
    let verify_time = start.elapsed();
    println!("Verification: {}ms", verify_time.as_millis());
    
    // Verify correctness
    assert!(is_valid);
    println!("✓ ML-DSA-44 test PASSED");
    println!("");
    
    Ok(())
}

/// Test ML-DSA-65 with individual results
#[test]
fn test_mldsa65_individual() -> Result<(), PqcError> {
    println!("=== ML-DSA-65 Individual Test Results ===");
    
    let signer = Sign::mldsa65();
    let message = b"ML-DSA-65 test message";
    println!("Algorithm: ML-DSA-65");
    println!("Public key size: {} bytes", signer.public_key_size());
    println!("Secret key size: {} bytes", signer.secret_key_size());
    println!("Signature size: {} bytes", signer.signature_size());
    println!("Message size: {} bytes", message.len());
    
    // Key generation test
    let start = std::time::Instant::now();
    let (pk, sk) = signer.keygen()?;
    let keygen_time = start.elapsed();
    println!("Key generation: {}ms", keygen_time.as_millis());
    
    // Signing test
    let start = std::time::Instant::now();
    let signature = signer.sign(message, &sk)?;
    let sign_time = start.elapsed();
    println!("Signing: {}ms", sign_time.as_millis());
    
    // Verification test
    let start = std::time::Instant::now();
    let is_valid = signer.verify(message, &signature, &pk)?;
    let verify_time = start.elapsed();
    println!("Verification: {}ms", verify_time.as_millis());
    
    // Verify correctness
    assert!(is_valid);
    println!("✓ ML-DSA-65 test PASSED");
    println!("");
    
    Ok(())
}

/// Test ML-DSA-87 with individual results
#[test]
fn test_mldsa87_individual() -> Result<(), PqcError> {
    println!("=== ML-DSA-87 Individual Test Results ===");
    
    let signer = Sign::mldsa87();
    let message = b"ML-DSA-87 test message";
    println!("Algorithm: ML-DSA-87");
    println!("Public key size: {} bytes", signer.public_key_size());
    println!("Secret key size: {} bytes", signer.secret_key_size());
    println!("Signature size: {} bytes", signer.signature_size());
    println!("Message size: {} bytes", message.len());
    
    // Key generation test
    let start = std::time::Instant::now();
    let (pk, sk) = signer.keygen()?;
    let keygen_time = start.elapsed();
    println!("Key generation: {}ms", keygen_time.as_millis());
    
    // Signing test
    let start = std::time::Instant::now();
    let signature = signer.sign(message, &sk)?;
    let sign_time = start.elapsed();
    println!("Signing: {}ms", sign_time.as_millis());
    
    // Verification test
    let start = std::time::Instant::now();
    let is_valid = signer.verify(message, &signature, &pk)?;
    let verify_time = start.elapsed();
    println!("Verification: {}ms", verify_time.as_millis());
    
    // Verify correctness
    assert!(is_valid);
    println!("✓ ML-DSA-87 test PASSED");
    println!("");
    
    Ok(())
}

/// Test FN-DSA-512 with individual results
#[test]
fn test_fndsa512_individual() -> Result<(), PqcError> {
    println!("=== FN-DSA-512 Individual Test Results ===");
    
    let signer = Sign::fndsa512();
    let message = b"FN-DSA-512 test message";
    println!("Algorithm: FN-DSA-512");
    println!("Public key size: {} bytes", signer.public_key_size());
    println!("Secret key size: {} bytes", signer.secret_key_size());
    println!("Signature size: {} bytes", signer.signature_size());
    println!("Message size: {} bytes", message.len());
    
    // Key generation test
    let start = std::time::Instant::now();
    let (pk, sk) = signer.keygen()?;
    let keygen_time = start.elapsed();
    println!("Key generation: {}ms", keygen_time.as_millis());
    
    // Signing test
    let start = std::time::Instant::now();
    let signature = signer.sign(message, &sk)?;
    let sign_time = start.elapsed();
    println!("Signing: {}ms", sign_time.as_millis());
    
    // Verification test
    let start = std::time::Instant::now();
    let is_valid = signer.verify(message, &signature, &pk)?;
    let verify_time = start.elapsed();
    println!("Verification: {}ms", verify_time.as_millis());
    
    // Verify correctness
    assert!(is_valid);
    println!("✓ FN-DSA-512 test PASSED");
    println!("");
    
    Ok(())
}

/// Test FN-DSA-1024 with individual results
#[test]
fn test_fndsa1024_individual() -> Result<(), PqcError> {
    println!("=== FN-DSA-1024 Individual Test Results ===");
    
    let signer = Sign::fndsa1024();
    let message = b"FN-DSA-1024 test message";
    println!("Algorithm: FN-DSA-1024");
    println!("Public key size: {} bytes", signer.public_key_size());
    println!("Secret key size: {} bytes", signer.secret_key_size());
    println!("Signature size: {} bytes", signer.signature_size());
    println!("Message size: {} bytes", message.len());
    
    // Key generation test
    let start = std::time::Instant::now();
    let (pk, sk) = signer.keygen()?;
    let keygen_time = start.elapsed();
    println!("Key generation: {}ms", keygen_time.as_millis());
    
    // Signing test
    let start = std::time::Instant::now();
    let signature = signer.sign(message, &sk)?;
    let sign_time = start.elapsed();
    println!("Signing: {}ms", sign_time.as_millis());
    
    // Verification test
    let start = std::time::Instant::now();
    let is_valid = signer.verify(message, &signature, &pk)?;
    let verify_time = start.elapsed();
    println!("Verification: {}ms", verify_time.as_millis());
    
    // Verify correctness
    assert!(is_valid);
    println!("✓ FN-DSA-1024 test PASSED");
    println!("");
    
    Ok(())
}
