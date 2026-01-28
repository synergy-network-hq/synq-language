//! Native tests for the MLKEM key encapsulation mechanism (KEM).
//!
//! These tests ensure that encapsulation and decapsulation produce matching
//! shared secrets and that incorrect inputs are handled correctly.

use aegis_crypto_core::{ mlkem_keygen, mlkem_encapsulate, mlkem_decapsulate };

#[test]
fn test_mlkem_encaps_and_decaps() {
    // Generate a recipient key pair
    let keypair = mlkem_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    // Encapsulate a shared secret
    let encapsulated = mlkem_encapsulate(&public_key).expect("encapsulation should succeed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret_enc = encapsulated.shared_secret();

    // Decapsulate the shared secret
    let shared_secret_dec = mlkem_decapsulate(&secret_key, &ciphertext).expect(
        "decapsulation should succeed"
    );

    assert_eq!(shared_secret_enc, shared_secret_dec, "Shared secrets should match");

    // Tamper with ciphertext
    let mut tampered_ct = ciphertext.clone();
    tampered_ct[0] ^= 0x01;

    // Decapsulation with tampered ciphertext should still succeed but produce a different secret
    let tampered_secret_res = mlkem_decapsulate(&secret_key, &tampered_ct);
    assert!(tampered_secret_res.is_ok(), "Decapsulation of tampered ciphertext should not fail");
    let tampered_secret = tampered_secret_res.unwrap();
    assert_ne!(
        shared_secret_enc,
        tampered_secret,
        "Decapsulation of tampered ciphertext should produce a different secret"
    );
}

#[test]
fn test_mlkem_keygen_encapsulate_decapsulate() {
    let keypair = mlkem_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = mlkem_encapsulate(&public_key).expect("Encapsulation failed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = mlkem_decapsulate(&secret_key, &ciphertext).expect("Decapsulation failed");

    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}

#[test]
fn test_mlkem_keypair_methods() {
    let keypair = mlkem_keygen();
    // Just test that the methods exist and return valid data
    assert!(!keypair.public_key().is_empty());
    assert!(!keypair.secret_key().is_empty());
}

#[test]
fn test_mlkem_encapsulated_methods() {
    let keypair = mlkem_keygen();
    let public_key = keypair.public_key();
    let encapsulated = mlkem_encapsulate(&public_key).expect("Encapsulation failed");

    // Just test that the methods exist and return valid data
    assert!(!encapsulated.ciphertext().is_empty());
    assert!(!encapsulated.shared_secret().is_empty());
}
