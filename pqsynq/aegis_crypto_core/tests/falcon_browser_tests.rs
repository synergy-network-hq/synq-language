//! Browser (wasm32-unknown-unknown) tests for FN-DSA signatures using wasm-bindgen-test.
#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen_test::*;
use aegis_crypto_core::{fndsa_keygen, fndsa_sign, fndsa_verify};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_fndsa_sign_verify_browser() {
    let kp = fndsa_keygen().expect("fndsa keygen");
    let msg = b"Aegis FN-DSA browser test";
    let sig = fndsa_sign(kp.secret_key(), msg).expect("fndsa sign");
    assert!(fndsa_verify(kp.public_key(), msg, &sig));
}

#[wasm_bindgen_test]
fn test_fndsa_tampered_sig_browser() {
    let kp = fndsa_keygen().expect("fndsa keygen");
    let msg = b"Aegis FN-DSA tamper";
    let mut sig = fndsa_sign(kp.secret_key(), msg).expect("sign");
    if !sig.is_empty() { sig[0] ^= 0x01; }
    assert!(!fndsa_verify(kp.public_key(), msg, &sig));
}
