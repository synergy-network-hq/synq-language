use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use pqrust_traits::kem::SharedSecret;
use pqrust_traits::sign::Signature;
// Import all expanded functions from mod.rs for each algo

#[test]
fn full_kat_validation_ml_kem_512() {
    let kat_path = "../../pqc/pqkat/NIST-ml-kem/KAT/mlkem512/PQCkemKAT_1632.req";
    let mut count = 0;
    let file = File::open(kat_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut entropy = [0u8; 32]; // Seed from KAT
    let mut expected_ss = [0u8; 32];
    let mut expected_ct = vec![];

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.starts_with("entropy = ") {
            // Parse entropy hex to bytes
            let hex = line.trim_start_matches("entropy = ").trim();
            hex::decode_to_slice(hex, &mut entropy).unwrap();
        } else if line.starts_with("count = ") {
            // Parse count, seed RNG with entropy for reproducible keygen/encap
            let seed_rng = /* Custom PRNG seeded with entropy */;
            // Generate keypair with seeded RNG
            let (pk, sk) = mlkem512_keygen_with_rng(&seed_rng);
            // Generate encapsulate with same seed
            let (ss, ct) = mlkem512_encapsulate_with_rng(&pk, &seed_rng);
            // Parse expected from .rsp file (parallel read)
            // Compare ss.as_bytes() == expected_ss, ct.as_bytes() == expected_ct
            assert_eq!(ss.as_bytes(), &expected_ss);
            assert_eq!(ct.as_bytes(), &expected_ct);
            count += 1;
        }
    }
    assert_eq!(count, 100);
}

// Repeat for all levels/algos: ML-KEM {512/768/1024}, ML-DSA {44/65/87} (sign/verify with msg), SLH-DSA all variants, FN-DSA {512/1024}, HQC {128/192/256}
// For signatures, parse msg, signature, pk/sk from KAT
// Use SHAKE-256 for PRNG seeding as per NIST/PQClean
// Classic McEliece: Similar if KATs available (experimental, skip if no vectors)

// Helper: Implement seeded RNG using sha3::Shake256
fn seeded_rng(seed: &[u8]) -> /* RNG impl for keygen/ops */ {
    // Use shake256.absorb(seed); shake256.squeeze for random bytes
}
