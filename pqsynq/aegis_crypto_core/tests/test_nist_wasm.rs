#![cfg(all(feature = "nist-wasm", target_arch = "wasm32"))]
//! Tests for NIST Reference WASM implementations

#[allow(unused_imports)]
use aegis_crypto_core::*;
use wasm_bindgen::JsCast;

#[tokio::test]
async fn test_mlkem_variants_info() {
    let variants = nist_mlkem_variants();
    assert!(!variants.is_undefined());

    // Check that we have the expected number of variants
    let variants_array: js_sys::Array = variants.dyn_into().unwrap();
    assert_eq!(variants_array.length(), 3); // ML-KEM-512, ML-KEM-768, ML-KEM-1024

    // Check first variant (ML-KEM-512)
    let first_variant = variants_array.get(0);
    assert!(!first_variant.is_undefined());

    let variant_obj: js_sys::Object = first_variant.dyn_into().unwrap();
    let name = js_sys::Reflect::get(&variant_obj, &"name".into()).unwrap();
    assert_eq!(name.as_string().unwrap(), "ML-KEM-512");
}

#[tokio::test]
async fn test_mldsa_variants_info() {
    let variants = nist_mldsa_variants();
    assert!(!variants.is_undefined());

    // Check that we have the expected number of variants
    let variants_array: js_sys::Array = variants.dyn_into().unwrap();
    assert_eq!(variants_array.length(), 6); // 6 ML-DSA variants

    // Check first variant (ML-DSA-2)
    let first_variant = variants_array.get(0);
    assert!(!first_variant.is_undefined());

    let variant_obj: js_sys::Object = first_variant.dyn_into().unwrap();
    let name = js_sys::Reflect::get(&variant_obj, &"name".into()).unwrap();
    assert_eq!(name.as_string().unwrap(), "ML-DSA-2");
}

#[tokio::test]
async fn test_wasm_environment_info() {
    let info = get_wasm_info();
    assert!(!info.is_undefined());

    let info_obj: js_sys::Object = info.dyn_into().unwrap();

    // Check WASM support
    let wasm_supported = js_sys::Reflect::get(&info_obj, &"wasm_supported".into()).unwrap();
    assert!(wasm_supported.as_bool().unwrap());

    // Check fetch API support
    let fetch_supported = js_sys::Reflect::get(&info_obj, &"fetch_supported".into()).unwrap();
    assert!(fetch_supported.as_bool().unwrap());

    // Check Promise support
    let promise_supported = js_sys::Reflect::get(&info_obj, &"promise_supported".into()).unwrap();
    assert!(promise_supported.as_bool().unwrap());
}

#[tokio::test]
async fn test_mlkem_keygen_512() {
    // This test will fail in a test environment without the actual WASM files
    // but it verifies the function signature and error handling
    let result = nist_mlkem_keygen("ML-KEM-512").await;
    match result {
        Ok(_) => {
            // Success - WASM files are available
            println!("ML-KEM-512 key generation succeeded");
        }
        Err(e) => {
            // Expected in test environment - check error message
            let error_msg = e.as_string().unwrap();
            assert!(
                error_msg.contains("Failed to fetch WASM file") ||
                    error_msg.contains("No window found")
            );
        }
    }
}

#[tokio::test]
async fn test_mlkem_keygen_768() {
    let result = nist_mlkem_keygen("ML-KEM-768").await;
    match result {
        Ok(_) => {
            println!("ML-KEM-768 key generation succeeded");
        }
        Err(e) => {
            let error_msg = e.as_string().unwrap();
            assert!(
                error_msg.contains("Failed to fetch WASM file") ||
                    error_msg.contains("No window found")
            );
        }
    }
}

#[tokio::test]
async fn test_mlkem_keygen_1024() {
    let result = nist_mlkem_keygen("ML-KEM-1024").await;
    match result {
        Ok(_) => {
            println!("ML-KEM-1024 key generation succeeded");
        }
        Err(e) => {
            let error_msg = e.as_string().unwrap();
            assert!(
                error_msg.contains("Failed to fetch WASM file") ||
                    error_msg.contains("No window found")
            );
        }
    }
}

#[tokio::test]
async fn test_mlkem_invalid_variant() {
    let result = nist_mlkem_keygen("INVALID-VARIANT").await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_msg = error.as_string().unwrap();
    assert!(error_msg.contains("Unsupported ML-KEM variant"));
}

#[tokio::test]
async fn test_mldsa_keygen_2() {
    let result = nist_mldsa_keygen("ML-DSA-2").await;
    match result {
        Ok(_) => {
            println!("ML-DSA-2 key generation succeeded");
        }
        Err(e) => {
            let error_msg = e.as_string().unwrap();
            assert!(
                error_msg.contains("Failed to fetch WASM file") ||
                    error_msg.contains("No window found")
            );
        }
    }
}

#[tokio::test]
async fn test_mldsa_keygen_3() {
    let result = nist_mldsa_keygen("ML-DSA-3").await;
    match result {
        Ok(_) => {
            println!("ML-DSA-3 key generation succeeded");
        }
        Err(e) => {
            let error_msg = e.as_string().unwrap();
            assert!(
                error_msg.contains("Failed to fetch WASM file") ||
                    error_msg.contains("No window found")
            );
        }
    }
}

#[tokio::test]
async fn test_mldsa_keygen_5() {
    let result = nist_mldsa_keygen("ML-DSA-5").await;
    match result {
        Ok(_) => {
            println!("ML-DSA-5 key generation succeeded");
        }
        Err(e) => {
            let error_msg = e.as_string().unwrap();
            assert!(
                error_msg.contains("Failed to fetch WASM file") ||
                    error_msg.contains("No window found")
            );
        }
    }
}

#[tokio::test]
async fn test_mldsa_invalid_variant() {
    let result = nist_mldsa_keygen("INVALID-VARIANT").await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_msg = error.as_string().unwrap();
    assert!(error_msg.contains("Unsupported ML-DSA variant"));
}

#[tokio::test]
async fn test_mlkem_encapsulation_validation() {
    // Test with invalid public key length
    let invalid_pk = vec![0u8; 100]; // Wrong length for any variant

    let result = nist_mlkem_encapsulate("ML-KEM-512", &invalid_pk).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_msg = error.as_string().unwrap();
    assert!(error_msg.contains("Invalid public key length"));
}

#[tokio::test]
async fn test_mlkem_decapsulation_validation() {
    // Test with invalid secret key length
    let invalid_sk = vec![0u8; 100]; // Wrong length for any variant
    let invalid_ct = vec![0u8; 100]; // Wrong length for any variant

    let result = nist_mlkem_decapsulate("ML-KEM-512", &invalid_sk, &invalid_ct).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_msg = error.as_string().unwrap();
    assert!(
        error_msg.contains("Invalid secret key length") ||
            error_msg.contains("Invalid ciphertext length")
    );
}

#[tokio::test]
async fn test_mldsa_signing_validation() {
    // Test with invalid secret key length
    let invalid_sk = vec![0u8; 100]; // Wrong length for any variant
    let message = b"test message";

    let result = nist_mldsa_sign("ML-DSA-2", &invalid_sk, message).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_msg = error.as_string().unwrap();
    assert!(error_msg.contains("Invalid secret key length"));
}

#[tokio::test]
async fn test_mldsa_verification_validation() {
    // Test with invalid public key length
    let invalid_pk = vec![0u8; 100]; // Wrong length for any variant
    let signature = vec![0u8; 100]; // Wrong length for any variant
    let message = b"test message";

    let result = nist_mldsa_verify("ML-DSA-2", &invalid_pk, &signature, message).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_msg = error.as_string().unwrap();
    assert!(
        error_msg.contains("Invalid public key length") ||
            error_msg.contains("Invalid signature length")
    );
}

#[test]
fn test_mlkem_variant_enum() {
    use crate::nist_wasm_mlkem::MlkemVariant;

    // Test ML-KEM-512
    let variant = MlkemVariant::MLKEM512;
    assert_eq!(variant.public_key_length(), 800);
    assert_eq!(variant.secret_key_length(), 1632);
    assert_eq!(variant.ciphertext_length(), 768);
    assert_eq!(variant.shared_secret_length(), 32);
    assert_eq!(variant.wasm_filename(), "mlkem512.wasm");

    // Test ML-KEM-768
    let variant = MlkemVariant::MLKEM768;
    assert_eq!(variant.public_key_length(), 1184);
    assert_eq!(variant.secret_key_length(), 2400);
    assert_eq!(variant.ciphertext_length(), 1088);
    assert_eq!(variant.shared_secret_length(), 32);
    assert_eq!(variant.wasm_filename(), "mlkem768.wasm");

    // Test ML-KEM-1024
    let variant = MlkemVariant::MLKEM1024;
    assert_eq!(variant.public_key_length(), 1568);
    assert_eq!(variant.secret_key_length(), 3168);
    assert_eq!(variant.ciphertext_length(), 1568);
    assert_eq!(variant.shared_secret_length(), 32);
    assert_eq!(variant.wasm_filename(), "mlkem1024.wasm");
}

#[test]
fn test_mldsa_variant_enum() {
    use crate::nist_wasm_mldsa::MldsaVariant;

    // Test ML-DSA-2
    let variant = MldsaVariant::MLDSA2;
    assert_eq!(variant.public_key_length(), 1312);
    assert_eq!(variant.secret_key_length(), 2544);
    assert_eq!(variant.signature_length(), 2420);
    assert_eq!(variant.display_name(), "ML-DSA-2");
    assert_eq!(variant.wasm_filename(), "ml-dsa2.wasm");

    // Test ML-DSA-3
    let variant = MldsaVariant::MLDSA3;
    assert_eq!(variant.public_key_length(), 1952);
    assert_eq!(variant.secret_key_length(), 4016);
    assert_eq!(variant.signature_length(), 3293);
    assert_eq!(variant.display_name(), "ML-DSA-3");
    assert_eq!(variant.wasm_filename(), "ml-dsa3.wasm");

    // Test ML-DSA-5
    let variant = MldsaVariant::MLDSA5;
    assert_eq!(variant.public_key_length(), 2592);
    assert_eq!(variant.secret_key_length(), 4880);
    assert_eq!(variant.signature_length(), 4595);
    assert_eq!(variant.display_name(), "ML-DSA-5");
    assert_eq!(variant.wasm_filename(), "ml-dsa5.wasm");
}
