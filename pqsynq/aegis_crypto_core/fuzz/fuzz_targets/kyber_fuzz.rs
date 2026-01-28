#![no_main]

use libfuzzer_sys::fuzz_target;
use aegis_crypto_core::kyber::{ kyber_keygen, kyber_encapsulate, kyber_decapsulate };

fuzz_target!(|data: &[u8]| {
    // Skip if data is too small
    if data.len() < 10 {
        return;
    }

    // Generate keypair
    let keypair = kyber_keygen();
    let public_key = keypair.public_key();
    let secret_key = keypair.secret_key();

    // Test encapsulation
    if let Ok(encapsulated) = kyber_encapsulate(&public_key) {
        let ciphertext = encapsulated.ciphertext();

        // Test decapsulation
        let _decapsulated = kyber_decapsulate(&secret_key, &ciphertext);

        // Test with fuzzed data (this should fail gracefully)
        if data.len() >= ciphertext.len() {
            let fuzzed_ciphertext = &data[..ciphertext.len()];
            let _result = kyber_decapsulate(&secret_key, fuzzed_ciphertext);
        }
    }
});
