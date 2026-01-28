//! WASM tests for the MLDSA digital signature scheme.

#[cfg(target_arch = "wasm32")]
mod wasm_tests {
    use wasm_bindgen_test::*;
    use aegis_crypto_core::{mldsa_keygen, mldsa_sign, mldsa_verify};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_mldsa_wasm_keygen() {
        let keypair = mldsa_keygen().expect("WASM keygen should succeed");
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();

        assert!(!public_key.is_empty(), "Public key should not be empty");
        assert!(!secret_key.is_empty(), "Secret key should not be empty");
    }

    #[wasm_bindgen_test]
    fn test_mldsa_wasm_sign_verify() {
        let keypair = mldsa_keygen().expect("WASM keygen should succeed");
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let message = b"WASM test message for MLDSA";

        let signature = mldsa_sign(&secret_key, message).expect("WASM signing should succeed");
        assert!(!signature.is_empty(), "Signature should not be empty");

        let is_valid = mldsa_verify(&public_key, message, &signature);
        assert!(is_valid, "WASM signature verification should succeed");

        // Test with wrong message
        let wrong_message = b"Wrong message";
        let is_invalid = mldsa_verify(&public_key, wrong_message, &signature);
        assert!(!is_invalid, "WASM verification should fail with wrong message");
    }

    #[wasm_bindgen_test]
    fn test_mldsa_wasm_invalid_inputs() {
        let short_sk = vec![0u8; 10]; // Invalid length
        let message = b"test message";
        let result = mldsa_sign(&short_sk, message);
        assert!(result.is_err(), "WASM signing should fail with invalid secret key");

        let short_pk = vec![0u8; 10]; // Invalid length
        let signature = vec![0u8; 100];
        let is_valid = mldsa_verify(&short_pk, message, &signature);
        assert!(!is_valid, "WASM verification should fail with invalid public key");
    }

    #[wasm_bindgen_test]
    fn test_mldsa_wasm_keypair_methods() {
        let keypair = mldsa_keygen().expect("WASM keygen should succeed");

        assert_eq!(keypair.public_key().len(), keypair.public_key_length());
        assert_eq!(keypair.secret_key().len(), keypair.secret_key_length());
    }
}
