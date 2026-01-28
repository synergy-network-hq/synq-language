//! Known Answer Tests (KAT) for HQC key encapsulation.

use aegis_crypto_core::{ hqc_keygen, hqc_encapsulate, hqc_decapsulate };

#[test]
fn test_hqc_kat_round1() {
    let keypair = hqc_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc_encapsulate(&public_key).expect("Encapsulation failed");
    let decapsulated = hqc_decapsulate(&secret_key, &encapsulated.ciphertext()).expect(
        "Decapsulation failed"
    );

    assert_eq!(
        encapsulated.shared_secret(),
        decapsulated,
        "KAT round 1: Shared secrets should match"
    );
}

#[test]
fn test_hqc_kat_round2() {
    let keypair = hqc_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc_encapsulate(&public_key).expect("Encapsulation failed");
    let decapsulated = hqc_decapsulate(&secret_key, &encapsulated.ciphertext()).expect(
        "Decapsulation failed"
    );

    assert_eq!(
        encapsulated.shared_secret(),
        decapsulated,
        "KAT round 2: Shared secrets should match"
    );
}

#[test]
fn test_hqc_kat_round3() {
    let keypair = hqc_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc_encapsulate(&public_key).expect("Encapsulation failed");
    let decapsulated = hqc_decapsulate(&secret_key, &encapsulated.ciphertext()).expect(
        "Decapsulation failed"
    );

    assert_eq!(
        encapsulated.shared_secret(),
        decapsulated,
        "KAT round 3: Shared secrets should match"
    );
}

#[test]
fn test_hqc_kat_round4() {
    let keypair = hqc_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc_encapsulate(&public_key).expect("Encapsulation failed");
    let decapsulated = hqc_decapsulate(&secret_key, &encapsulated.ciphertext()).expect(
        "Decapsulation failed"
    );

    assert_eq!(
        encapsulated.shared_secret(),
        decapsulated,
        "KAT round 4: Shared secrets should match"
    );
}

#[test]
fn test_hqc_kat_round5() {
    let keypair = hqc_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = hqc_encapsulate(&public_key).expect("Encapsulation failed");
    let decapsulated = hqc_decapsulate(&secret_key, &encapsulated.ciphertext()).expect(
        "Decapsulation failed"
    );

    assert_eq!(
        encapsulated.shared_secret(),
        decapsulated,
        "KAT round 5: Shared secrets should match"
    );
}
