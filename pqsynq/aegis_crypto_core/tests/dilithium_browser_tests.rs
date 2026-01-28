//! Browser (wasm32-unknown-unknown) tests for MLDSA signatures using wasm-bindgen-test.
#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen_test::*;
use aegis_crypto_core::{mldsa_keygen, mldsa_sign, mldsa_verify};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_mldsa_sign_verify_browser() {
    let kp = mldsa_keygen().expect("mldsa keygen");
    let msg = b"Aegis MLDSA browser test";
    let sig = mldsa_sign(kp.secret_key(), msg).expect("mldsa sign");
    assert!(mldsa_verify(kp.public_key(), msg, &sig));
}

#[wasm_bindgen_test]
fn test_mldsa_tampered_sig_browser() {
    let kp = mldsa_keygen().expect("mldsa keygen");
    let msg = b"Aegis MLDSA tamper";
    let mut sig = mldsa_sign(kp.secret_key(), msg).expect("sign");
    if !sig.is_empty() { sig[0] ^= 0x01; }
    assert!(!mldsa_verify(kp.public_key(), msg, &sig));
}
