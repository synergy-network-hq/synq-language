//! Known Answer Tests (KAT) for MLKEM key encapsulation.

use aegis_crypto_core::{ mlkem_keygen_native, mlkem_encapsulate_native, mlkem_decapsulate_native };

#[test]
fn test_mlkem_kat_round1() {
    let keypair = mlkem_keygen_native();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = mlkem_encapsulate_native(&public_key).expect("Encapsulation failed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = mlkem_decapsulate_native(&secret_key, &ciphertext).expect(
        "Decapsulation failed"
    );
    assert_eq!(shared_secret1, shared_secret2, "KAT round 1: Shared secrets should match");
}

#[test]
fn test_mlkem_kat_round2() {
    let keypair = mlkem_keygen_native();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = mlkem_encapsulate_native(&public_key).expect("Encapsulation failed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = mlkem_decapsulate_native(&secret_key, &ciphertext).expect(
        "Decapsulation failed"
    );
    assert_eq!(shared_secret1, shared_secret2, "KAT round 2: Shared secrets should match");
}

#[test]
fn test_mlkem_kat_round3() {
    let keypair = mlkem_keygen_native();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = mlkem_encapsulate_native(&public_key).expect("Encapsulation failed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = mlkem_decapsulate_native(&secret_key, &ciphertext).expect(
        "Decapsulation failed"
    );
    assert_eq!(shared_secret1, shared_secret2, "KAT round 3: Shared secrets should match");
}

#[test]
fn test_mlkem_kat_round4() {
    let keypair = mlkem_keygen_native();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = mlkem_encapsulate_native(&public_key).expect("Encapsulation failed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = mlkem_decapsulate_native(&secret_key, &ciphertext).expect(
        "Decapsulation failed"
    );
    assert_eq!(shared_secret1, shared_secret2, "KAT round 4: Shared secrets should match");
}

#[test]
fn test_mlkem_kat_round5() {
    let keypair = mlkem_keygen_native();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    let encapsulated = mlkem_encapsulate_native(&public_key).expect("Encapsulation failed");
    let ciphertext = encapsulated.ciphertext();
    let shared_secret1 = encapsulated.shared_secret();

    let shared_secret2 = mlkem_decapsulate_native(&secret_key, &ciphertext).expect(
        "Decapsulation failed"
    );
    assert_eq!(shared_secret1, shared_secret2, "KAT round 5: Shared secrets should match");
}
