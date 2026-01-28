//! Browser (wasm32-unknown-unknown) tests for HQC KEM using wasm-bindgen-test.
#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen_test::*;
use aegis_crypto_core::{
    hqc_keygen, hqc_encapsulate, hqc_decapsulate,
    hqc128_keygen, hqc128_encapsulate, hqc128_decapsulate,
    hqc192_keygen, hqc192_encapsulate, hqc192_decapsulate,
    hqc256_keygen, hqc256_encapsulate, hqc256_decapsulate,
};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_hqc_generic_encaps_and_decaps_browser() {
    let kp = hqc_keygen().expect("hqc keygen");
    let ct_ss = hqc_encapsulate(kp.public_key()).expect("hqc encaps");
    let ss_dec = hqc_decapsulate(kp.secret_key(), ct_ss.ciphertext()).expect("hqc decaps");
    assert_eq!(ct_ss.shared_secret(), ss_dec);
}

#[wasm_bindgen_test]
fn test_hqc128_encaps_and_decaps_browser() {
    let kp = hqc128_keygen().expect("hqc128 keygen");
    let ct_ss = hqc128_encapsulate(kp.public_key()).expect("hqc128 encaps");
    let ss_dec = hqc128_decapsulate(kp.secret_key(), ct_ss.ciphertext()).expect("hqc128 decaps");
    assert_eq!(ct_ss.shared_secret(), ss_dec);
}

#[wasm_bindgen_test]
fn test_hqc192_encaps_and_decaps_browser() {
    let kp = hqc192_keygen().expect("hqc192 keygen");
    let ct_ss = hqc192_encapsulate(kp.public_key()).expect("hqc192 encaps");
    let ss_dec = hqc192_decapsulate(kp.secret_key(), ct_ss.ciphertext()).expect("hqc192 decaps");
    assert_eq!(ct_ss.shared_secret(), ss_dec);
}

#[wasm_bindgen_test]
fn test_hqc256_encaps_and_decaps_browser() {
    let kp = hqc256_keygen().expect("hqc256 keygen");
    let ct_ss = hqc256_encapsulate(kp.public_key()).expect("hqc256 encaps");
    let ss_dec = hqc256_decapsulate(kp.secret_key(), ct_ss.ciphertext()).expect("hqc256 decaps");
    assert_eq!(ct_ss.shared_secret(), ss_dec);
}
