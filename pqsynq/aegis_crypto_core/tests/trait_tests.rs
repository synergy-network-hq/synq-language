//! Tests for trait-based API implementations.

use aegis_crypto_core::traits::{ Kem, Algorithm };
use aegis_crypto_core::mlkem::traits::{ MLKEM768, MLKEMPublicKey, MLKEMSecretKey };

#[test]
fn test_mlkem768_trait_implementation() {
    // Test key generation
    let (public_key, secret_key) = MLKEM768::keygen().expect("Key generation should succeed");

    // Test encapsulation
    let encapsulated = MLKEM768::encapsulate(&public_key).expect("Encapsulation should succeed");

    // Test decapsulation
    let decapsulated = MLKEM768::decapsulate(&secret_key, encapsulated.0.as_ref()).expect(
        "Decapsulation should succeed"
    );

    // Verify shared secrets match
    assert_eq!(encapsulated.1.as_ref(), decapsulated.as_ref(), "Shared secrets should match");
}

#[test]
fn test_mlkem768_algorithm_trait() {
    assert_eq!(MLKEM768::name(), "MLKEM768");
    assert_eq!(MLKEM768::security_level(), 192);
}

#[test]
fn test_mlkem768_error_handling() {
    // Test with invalid public key
    let invalid_pk = MLKEMPublicKey(vec![0u8; 100]); // Wrong length
    let result = MLKEM768::encapsulate(&invalid_pk);
    assert!(result.is_err(), "Encapsulation should fail with invalid public key");

    // Test with invalid secret key
    let invalid_sk = MLKEMSecretKey(vec![0u8; 100]); // Wrong length
    let invalid_ct = vec![0u8; 100]; // Wrong length
    let result = MLKEM768::decapsulate(&invalid_sk, &invalid_ct);
    assert!(result.is_err(), "Decapsulation should fail with invalid inputs");
}
