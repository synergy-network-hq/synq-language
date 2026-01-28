//! Native tests for the FN-DSA digital signature scheme.

use aegis_crypto_core::{ fndsa_keygen, fndsa_sign, fndsa_verify };

#[test]
fn test_fndsa_sign_and_verify() {
    // Generate a new key pair
    let keypair = fndsa_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();
    // Create a message to sign
    let message = b"Quantum safe signatures are cool!";

    // Sign the message (returns detached signature)
    let signature = fndsa_sign(&secret_key, message);

    // Verify the signature
    assert!(fndsa_verify(&public_key, message, &signature), "FN-DSA signature should be valid");

    // Tamper with the signature
    let mut tampered = signature.clone();
    tampered[0] ^= 0x55;
    assert!(
        !fndsa_verify(&public_key, message, &tampered),
        "Verification should fail for a tampered signature"
    );
}

#[test]
fn test_fndsa_keygen_sign_verify() {
    let keypair = fndsa_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"This is a test message for FN-DSA signing.";
    let signature = fndsa_sign(&secret_key, message);

    let is_valid = fndsa_verify(&public_key, message, &signature);
    assert!(is_valid, "Signature verification failed");

    // Test with a wrong message
    let wrong_message = b"This is a wrong message.";
    let is_valid_wrong_message = fndsa_verify(&public_key, wrong_message, &signature);
    assert!(!is_valid_wrong_message, "Signature verification should fail with wrong message");

    // Test with a tampered signature
    let mut tampered_signature = signature.clone();
    tampered_signature[0] ^= 0x01; // Flip a bit
    let is_valid_tampered = fndsa_verify(&public_key, message, &tampered_signature);
    assert!(!is_valid_tampered, "Signature verification should fail with tampered signature");
}

#[test]
fn test_fndsa_keypair_methods() {
    let keypair = fndsa_keygen();
    assert!(!keypair.public_key().is_empty());
    assert!(!keypair.secret_key().is_empty());
}
