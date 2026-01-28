//! Native tests for the SPHINCS+ digital signature scheme.

use aegis_crypto_core::{ slhdsa_keygen, slhdsa_sign, slhdsa_verify };

#[test]
fn test_slhdsa_sign_and_verify() {
    // Generate a key pair
    let keypair = slhdsa_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    // Sign a message
    let message = b"Hello, SPHINCS+!";
    let signed_message = slhdsa_sign(&secret_key, message);

    // Verify the signature
    let is_valid = slhdsa_verify(&public_key, &signed_message);
    assert!(is_valid, "Signature should be valid");

    // Tamper with the signed message
    let mut tampered_signed_message = signed_message.clone();
    tampered_signed_message[0] ^= 0x01;
    let is_valid_tampered = slhdsa_verify(&public_key, &tampered_signed_message);
    assert!(!is_valid_tampered, "Signature should be invalid for tampered message");
}

#[test]
fn test_slhdsa_keygen_sign_verify() {
    let keypair = slhdsa_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for SPHINCS+";
    let signed_message = slhdsa_sign(&secret_key, message);

    let is_valid = slhdsa_verify(&public_key, &signed_message);
    assert!(is_valid, "Signature should be valid");
}

#[test]
fn test_slhdsa_keypair_methods() {
    let keypair = slhdsa_keygen();
    // Just test that the methods exist and return valid data
    assert!(!keypair.public_key().is_empty());
    assert!(!keypair.secret_key().is_empty());
}
