//! Browser (wasm32-unknown-unknown) tests for MLKEM KEM using wasm-bindgen-test.
#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen_test::*;
use aegis_crypto_core::{mlkem_keygen, mlkem_encapsulate, mlkem_decapsulate};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_mlkem_encaps_and_decaps_browser() {
    let kp = mlkem_keygen().expect("mlkem keygen");
    let ct_ss = mlkem_encapsulate(kp.public_key()).expect("mlkem encaps");
    let ss_dec = mlkem_decapsulate(kp.secret_key(), ct_ss.ciphertext()).expect("mlkem decaps");
    assert_eq!(ct_ss.shared_secret(), ss_dec);
}

#[wasm_bindgen_test]
fn test_mlkem_decaps_tampered_browser() {
    let kp = mlkem_keygen().expect("mlkem keygen");
    let ct_ss = mlkem_encapsulate(kp.public_key()).expect("mlkem encaps");
    let mut tampered = ct_ss.ciphertext().to_vec();
    if !tampered.is_empty() { tampered[0] ^= 0x01; }
    let ss_tampered = mlkem_decapsulate(kp.secret_key(), &tampered).expect("mlkem decaps tampered");
    assert_ne!(ct_ss.shared_secret(), ss_tampered.as_slice());
}
