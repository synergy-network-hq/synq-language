//! Native tests for the HQC key encapsulation mechanism (KEM).

use aegis_crypto_core::{ hqc_keygen, hqc_encapsulate, hqc_decapsulate };

#[test]
fn test_hqc_keygen_encapsulate_decapsulate() {
    let keypair = hqc_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc_encapsulate(&public_key).expect("Encapsulation failed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = hqc_decapsulate(&secret_key, &ciphertext).expect("Decapsulation failed");

    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}

#[test]
fn test_hqc_keypair_methods() {
    let keypair = hqc_keygen();
    // Just test that the methods exist and return valid data
    assert!(!keypair.public_key().is_empty());
    assert!(!keypair.secret_key().is_empty());
}

#[test]
fn test_hqc_encapsulated_methods() {
    let keypair = hqc_keygen();
    let public_key = keypair.public_key();
    let encapsulated = hqc_encapsulate(&public_key).expect("Encapsulation failed");

    // Just test that the methods exist and return valid data
    assert!(!encapsulated.ciphertext().is_empty());
    assert!(!encapsulated.shared_secret().is_empty());
}
