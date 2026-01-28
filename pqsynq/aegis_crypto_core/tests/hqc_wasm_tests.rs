//! WASM tests for the HQC key encapsulation mechanism (KEM).

#![cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
use aegis_crypto_core::{ hqc128_keygen, hqc128_encapsulate, hqc128_decapsulate };
use aegis_crypto_core::{ hqc192_keygen, hqc192_encapsulate, hqc192_decapsulate };
use aegis_crypto_core::{ hqc256_keygen, hqc256_encapsulate, hqc256_decapsulate };

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_hqc128_wasm_keygen_encapsulate_decapsulate() {
    let keypair = hqc128_keygen();

    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc128_encapsulate(&public_key).expect("HQC-128 encapsulation failed");

    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = hqc128_decapsulate(&secret_key, &ciphertext).expect(
        "HQC-128 decapsulation failed"
    );

    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}

#[wasm_bindgen_test]
fn test_hqc192_wasm_keygen_encapsulate_decapsulate() {
    let keypair = hqc192_keygen();

    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc192_encapsulate(&public_key).expect("HQC-192 encapsulation failed");

    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = hqc192_decapsulate(&secret_key, &ciphertext).expect(
        "HQC-192 decapsulation failed"
    );

    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}

#[wasm_bindgen_test]
fn test_hqc256_wasm_keygen_encapsulate_decapsulate() {
    let keypair = hqc256_keygen();

    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc256_encapsulate(&public_key).expect("HQC-256 encapsulation failed");

    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = hqc256_decapsulate(&secret_key, &ciphertext).expect(
        "HQC-256 decapsulation failed"
    );

    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}

#[wasm_bindgen_test]
fn test_hqc_wasm_keypair_methods() {
    let keypair128 = hqc128_keygen();
    assert!(keypair128.public_key().len() > 0);
    assert!(keypair128.secret_key().len() > 0);

    let keypair192 = hqc192_keygen();
    assert!(keypair192.public_key().len() > 0);
    assert!(keypair192.secret_key().len() > 0);

    let keypair256 = hqc256_keygen();
    assert!(keypair256.public_key().len() > 0);
    assert!(keypair256.secret_key().len() > 0);
}

#[wasm_bindgen_test]
fn test_hqc_wasm_encapsulated_methods() {
    let keypair = hqc128_keygen();
    let public_key = keypair.public_key();

    let encapsulated = hqc128_encapsulate(&public_key).expect("HQC-128 encapsulation failed");
    assert!(encapsulated.ciphertext().len() > 0);
    assert!(encapsulated.shared_secret().len() > 0);
}

#[wasm_bindgen_test]
fn test_hqc_wasm_different_security_levels() {
    // Test that different security levels produce different key sizes
    let keypair128 = hqc128_keygen();
    let keypair192 = hqc192_keygen();
    let keypair256 = hqc256_keygen();

    // Keys should have different sizes for different security levels
    assert_ne!(keypair128.public_key().len(), keypair192.public_key().len());
    assert_ne!(keypair128.public_key().len(), keypair256.public_key().len());
    assert_ne!(keypair192.public_key().len(), keypair256.public_key().len());

    assert_ne!(keypair128.secret_key().len(), keypair192.secret_key().len());
    assert_ne!(keypair128.secret_key().len(), keypair256.secret_key().len());
    assert_ne!(keypair192.secret_key().len(), keypair256.secret_key().len());
}

#[wasm_bindgen_test]
fn test_hqc_wasm_encapsulation_produces_different_outputs() {
    let keypair = hqc128_keygen();
    let public_key = keypair.public_key();

    // Multiple encapsulations should produce different ciphertexts and shared secrets
    let enc1 = hqc128_encapsulate(&public_key).expect("First encapsulation failed");
    let enc2 = hqc128_encapsulate(&public_key).expect("Second encapsulation failed");

    // Ciphertexts should be different (probabilistic encryption)
    assert_ne!(enc1.ciphertext(), enc2.ciphertext());
    // Shared secrets should also be different
    assert_ne!(enc1.shared_secret(), enc2.shared_secret());
}

#[wasm_bindgen_test]
fn test_hqc_wasm_cross_security_level_incompatibility() {
    // Test that keys from one security level cannot be used with functions from another
    let keypair128 = hqc128_keygen();
    let keypair192 = hqc192_keygen();

    let pk128 = keypair128.public_key();
    let _sk192 = keypair192.secret_key();

    // Using HQC-128 public key with HQC-192 should fail
    let result = hqc192_encapsulate(&pk128);
    assert!(result.is_err(), "Cross-security-level encapsulation should fail");

    // Create a valid HQC-192 encapsulation
    let pk192 = keypair192.public_key();
    let enc192 = hqc192_encapsulate(&pk192).expect("HQC-192 encapsulation should succeed");
    let ct192 = enc192.ciphertext();

    // Using HQC-192 ciphertext with HQC-128 secret key should fail
    let sk128 = keypair128.secret_key();
    let result = hqc128_decapsulate(&sk128, &ct192);
    assert!(result.is_err(), "Cross-security-level decapsulation should fail");
}

#[wasm_bindgen_test]
fn test_hqc_wasm_invalid_key_sizes() {
    // Test with invalid key sizes
    let invalid_pk = vec![0u8; 10]; // Too small
    let result = hqc128_encapsulate(&invalid_pk);
    assert!(result.is_err(), "Encapsulation with invalid public key should fail");

    let keypair = hqc128_keygen();
    let valid_pk = keypair.public_key();
    let encapsulated = hqc128_encapsulate(&valid_pk).expect("Encapsulation should succeed");
    let valid_ct = encapsulated.ciphertext();

    let invalid_sk = vec![0u8; 10]; // Too small
    let result = hqc128_decapsulate(&invalid_sk, &valid_ct);
    assert!(result.is_err(), "Decapsulation with invalid secret key should fail");

    let valid_sk = keypair.secret_key();
    let invalid_ct = vec![0u8; 10]; // Too small
    let result = hqc128_decapsulate(&valid_sk, &invalid_ct);
    assert!(result.is_err(), "Decapsulation with invalid ciphertext should fail");
}
