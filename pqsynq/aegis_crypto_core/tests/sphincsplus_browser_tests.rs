//! Browser (wasm32-unknown-unknown) tests for SPHINCS+ signatures using wasm-bindgen-test.
#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen_test::*;
use aegis_crypto_core::{slhdsa_keygen, slhdsa_sign, slhdsa_verify};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_slhdsa_sign_verify_browser() {
    let kp = slhdsa_keygen().expect("sphincs+ keygen");
    let msg = b"Aegis SPHINCS+ browser test";
    let sig = slhdsa_sign(kp.secret_key(), msg).expect("sphincs+ sign");
    assert!(slhdsa_verify(kp.public_key(), msg, &sig));
}

#[wasm_bindgen_test]
fn test_slhdsa_tampered_sig_browser() {
    let kp = slhdsa_keygen().expect("sphincs+ keygen");
    let msg = b"Aegis SPHINCS+ tamper";
    let mut sig = slhdsa_sign(kp.secret_key(), msg).expect("sign");
    if !sig.is_empty() { sig[0] ^= 0x01; }
    assert!(!slhdsa_verify(kp.public_key(), msg, &sig));
}
