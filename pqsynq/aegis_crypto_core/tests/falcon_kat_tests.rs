//! Known Answer Tests (KAT) for FN-DSA signatures.

use aegis_crypto_core::{ fndsa_keygen, fndsa_sign, fndsa_verify };

#[test]
fn test_fndsa_kat_round1() {
    let keypair = fndsa_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for FN-DSA KAT round 1";
    let signature = fndsa_sign(&secret_key, message);
    let is_valid = fndsa_verify(&public_key, message, &signature);

    assert!(is_valid, "KAT round 1: Signature should be valid");
}

#[test]
fn test_fndsa_kat_round2() {
    let keypair = fndsa_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for FN-DSA KAT round 2";
    let signature = fndsa_sign(&secret_key, message);
    let is_valid = fndsa_verify(&public_key, message, &signature);

    assert!(is_valid, "KAT round 2: Signature should be valid");
}

#[test]
fn test_fndsa_kat_round3() {
    let keypair = fndsa_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for FN-DSA KAT round 3";
    let signature = fndsa_sign(&secret_key, message);
    let is_valid = fndsa_verify(&public_key, message, &signature);

    assert!(is_valid, "KAT round 3: Signature should be valid");
}

#[test]
fn test_fndsa_kat_round4() {
    let keypair = fndsa_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for FN-DSA KAT round 4";
    let signature = fndsa_sign(&secret_key, message);
    let is_valid = fndsa_verify(&public_key, message, &signature);

    assert!(is_valid, "KAT round 4: Signature should be valid");
}

#[test]
fn test_fndsa_kat_round5() {
    let keypair = fndsa_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let message = b"Test message for FN-DSA KAT round 5";
    let signature = fndsa_sign(&secret_key, message);
    let is_valid = fndsa_verify(&public_key, message, &signature);

    assert!(is_valid, "KAT round 5: Signature should be valid");
}
