// Similar to kat_validation.rs but for NIST
#[cfg(feature = "nist-ref")]
#[test]
fn nist_full_kat_ml_kem_512() {
    // Parse KAT, seed PRNG (SHAKE-256 with entropy)
    // Call nist_ml_kem_512_keypair_with_seed, encap/decap
    // Compare to expected in .rsp
    // Also compare to pqrust_ml_kem_512 outputs (same seed)
    // Assert equality
    let count = /* parse and validate 100 vectors */;
    assert_eq!(count, 100);
}

// Repeat for all algos/levels; signatures parse msg/sig/pk
// Cross-validation: assert nist_output == pqrust_output for same seed
