#[cfg(any())]
mod test_rustpqc_integration {
    use aegis_crypto_core::rustpqc_mlkem::*;
    use aegis_crypto_core::rustpqc_mldsa::*;

    #[test]
    fn test_rustpqc_mlkem768_integration() {
        // Test key generation
        let keypair = rustpqc_mlkem768_keygen_rust();
        let (pk, sk) = keypair;

        // Test encapsulation
        let encapsulated = rustpqc_mlkem768_encapsulate_rust(&pk).unwrap();
        let (ciphertext, shared_secret1) = encapsulated;

        // Test decapsulation
        let shared_secret2 = rustpqc_mlkem768_decapsulate_rust(&sk, &ciphertext).unwrap();

        // Verify that both shared secrets match
        assert_eq!(shared_secret1, shared_secret2);

        println!("✅ RustPQC ML-KEM-768 integration test passed!");
    }

    #[test]
    fn test_rustpqc_mldsa65_integration() {
        // Test key generation
        let keypair = rustpqc_mldsa65_keygen_rust();
        let (pk, sk) = keypair;

        // Test signing
        let message = b"Hello, RustPQC ML-DSA-65!";
        let signature = rustpqc_mldsa65_sign_rust(&sk, message).unwrap();

        // Test verification
        let is_valid = rustpqc_mldsa65_verify_rust(&pk, &signature, message).unwrap();
        assert!(is_valid);

        // Test verification with wrong message
        let wrong_message = b"Wrong message!";
        let is_valid_wrong = rustpqc_mldsa65_verify_rust(
            &pk,
            &signature,
            wrong_message
        ).unwrap();
        assert!(!is_valid_wrong);

        println!("✅ RustPQC ML-DSA-65 integration test passed!");
    }

    #[test]
    fn test_rustpqc_parameter_sizes() {
        // Test ML-KEM-768 parameter sizes
        let keypair = rustpqc_mlkem768_keygen_rust();
        let (pk, sk) = keypair;

        // These sizes should match the parameters in rustpqc/ml-kem/src/params.rs
        assert_eq!(pk.len(), 1184); // PUBLICKEYBYTES for ML-KEM-768
        assert_eq!(sk.len(), 2400); // SECRETKEYBYTES for ML-KEM-768

        let encapsulated = rustpqc_mlkem768_encapsulate_rust(&pk).unwrap();
        let (ciphertext, shared_secret) = encapsulated;

        assert_eq!(ciphertext.len(), 1088); // CIPHERTEXTBYTES for ML-KEM-768
        assert_eq!(shared_secret.len(), 32); // SYMBYTES

        // Test ML-DSA-65 parameter sizes
        let mldsa_keypair = rustpqc_mldsa65_keygen_rust();
        let (mldsa_pk, mldsa_sk) = mldsa_keypair;

        // These sizes should match the parameters in rustpqc/ml-dsa/src/params.rs
        assert_eq!(mldsa_pk.len(), 1952); // CRYPTO_PUBLICKEYBYTES for ML-DSA-65
        assert_eq!(mldsa_sk.len(), 4016); // CRYPTO_SECRETKEYBYTES for ML-DSA-65

        let message = b"Test message";
        let signature = rustpqc_mldsa65_sign_rust(&mldsa_sk, message).unwrap();

        assert_eq!(signature.len(), 3293); // CRYPTO_BYTES for ML-DSA-65

        println!("✅ RustPQC parameter sizes test passed!");
    }
}

#[cfg(not(any()))]
mod test_rustpqc_integration {
    #[test]
    fn test_rustpqc_features_disabled() {
        println!(
            "ℹ️  RustPQC features are disabled. Run with --features rustpqc-mlkem,rustpqc-mldsa to test integration."
        );
    }
}
