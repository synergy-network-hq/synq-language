//! Native tests for the MLDSA digital signature scheme.

use aegis_crypto_core::{mldsa_keygen, mldsa_sign, mldsa_verify};

#[test]
fn test_mldsa_sign_and_verify() {
    // Generate a new key pair
    let keypair = mldsa_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    // Create a message to sign
    let message = b"Quantum safe signatures are cool!";

    // Sign the message (returns signed message)
    let signed_message = mldsa_sign(&secret_key, message);

    // Verify the signature
    assert!(
        mldsa_verify(&public_key, &signed_message),
        "MLDSA signature should be valid"
    );

    // Tamper with the signed message
    let mut tampered = signed_message.clone();
    tampered[0] ^= 0x55;
    assert!(
        !mldsa_verify(&public_key, &tampered),
        "Verification should fail for a tampered signature"
    );
}

#[test]
fn test_mldsa_keygen_sign_verify() {
    let keypair = mldsa_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"This is a test message for MLDSA signing.";
    let signed_message = mldsa_sign(&secret_key, message);

    let is_valid = mldsa_verify(&public_key, &signed_message);
    assert!(is_valid, "Signature verification failed");

    // Test with a wrong message
    let wrong_message = b"This is a wrong message.";
    let wrong_signed_message = mldsa_sign(&secret_key, wrong_message);
    let is_valid_wrong_message = mldsa_verify(&public_key, &wrong_signed_message);
    assert!(is_valid_wrong_message, "Signature verification should succeed with correct signature");

    // Test with a tampered signed message
    let mut tampered_signed_message = signed_message.clone();
    tampered_signed_message[0] ^= 0x01; // Flip a bit
    let is_valid_tampered = mldsa_verify(&public_key, &tampered_signed_message);
    assert!(!is_valid_tampered, "Signature verification should fail with tampered signature");
}

#[test]
fn test_mldsa_keypair_methods() {
    let keypair = mldsa_keygen();
    assert!(!keypair.public_key().is_empty());
    assert!(!keypair.secret_key().is_empty());
}
