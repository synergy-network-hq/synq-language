//! Comprehensive Known Answer Tests (KAT) validation.
//! This file validates that all KAT files exist and have the correct number of test vectors.

use std::fs;

/// Count the number of test vectors in a KAT file.
fn count_kat_vectors(file_path: &str) -> usize {
    let content = fs::read_to_string(file_path).expect("Failed to read KAT file");
    let mut count_entries = 0;

    for line in content.lines() {
        let line = line.trim();
        // Look for "count = X" entries to count test vectors
        if line.starts_with("count = ") {
            count_entries += 1;
        }
    }

    count_entries
}

// ML-KEM (MLKEM) KAT Tests
#[test]
fn test_ml_kem_512_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-ml-kem/KAT/mlkem512/PQCkemKAT_1632.rsp");
    assert_eq!(vector_count, 100, "ML-KEM-512 should have 100 test vectors");
}

#[test]
fn test_ml_kem_768_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-ml-kem/KAT/mlkem768/PQCkemKAT_2400.rsp");
    assert_eq!(vector_count, 100, "ML-KEM-768 should have 100 test vectors");
}

#[test]
fn test_ml_kem_1024_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-ml-kem/KAT/mlkem1024/PQCkemKAT_3168.rsp");
    assert_eq!(vector_count, 100, "ML-KEM-1024 should have 100 test vectors");
}

// ML-DSA (MLDSA) KAT Tests
#[test]
fn test_ml_dsa_44_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-ml-dsa/KAT/mldsa2/PQCsignKAT_2544.rsp");
    assert_eq!(vector_count, 100, "ML-DSA-44 should have 100 test vectors");
}

#[test]
fn test_ml_dsa_65_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-ml-dsa/KAT/mldsa3/PQCsignKAT_4016.rsp");
    assert_eq!(vector_count, 100, "ML-DSA-65 should have 100 test vectors");
}

#[test]
fn test_ml_dsa_87_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-ml-dsa/KAT/mldsa5/PQCsignKAT_4880.rsp");
    assert_eq!(vector_count, 100, "ML-DSA-87 should have 100 test vectors");
}

// FN-DSA KAT Tests
#[test]
fn test_fndsa_512_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-fn-dsa/KAT/fndsa512-KAT.rsp");
    assert_eq!(vector_count, 100, "FN-DSA-512 should have 100 test vectors");
}

#[test]
fn test_fndsa_1024_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-fn-dsa/KAT/fndsa1024-KAT.rsp");
    assert_eq!(vector_count, 100, "FN-DSA-1024 should have 100 test vectors");
}

// SPHINCS+ KAT Tests
#[test]
fn test_sphincs_sha2_128f_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-sha256-128f-simple/PQCsignKAT_64.rsp");
    assert_eq!(vector_count, 100, "SPHINCS+-SHA2-128f should have 100 test vectors");
}

#[test]
fn test_sphincs_sha2_192f_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-sha256-192f-simple/PQCsignKAT_96.rsp");
    assert_eq!(vector_count, 100, "SPHINCS+-SHA2-192f should have 100 test vectors");
}

#[test]
fn test_sphincs_sha2_256f_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-sha256-256f-simple/PQCsignKAT_128.rsp");
    assert_eq!(vector_count, 100, "SPHINCS+-SHA2-256f should have 100 test vectors");
}

#[test]
fn test_sphincs_shake_128f_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-shake256-128f-simple/PQCsignKAT_64.rsp");
    assert_eq!(vector_count, 100, "SPHINCS+-SHAKE-128f should have 100 test vectors");
}

#[test]
fn test_sphincs_shake_192f_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-shake256-192f-simple/PQCsignKAT_96.rsp");
    assert_eq!(vector_count, 100, "SPHINCS+-SHAKE-192f should have 100 test vectors");
}

#[test]
fn test_sphincs_shake_256f_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-shake256-256f-simple/PQCsignKAT_128.rsp");
    assert_eq!(vector_count, 100, "SPHINCS+-SHAKE-256f should have 100 test vectors");
}

// HQC KAT Tests
#[test]
fn test_hqc_128_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-hqckem/KATs/Optimized_Implementation/hqc-128/hqc-128_kat.rsp");
    assert_eq!(vector_count, 100, "HQC-128 should have 100 test vectors");
}

#[test]
fn test_hqc_192_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-hqckem/KATs/Optimized_Implementation/hqc-192/hqc-192_kat.rsp");
    assert_eq!(vector_count, 100, "HQC-192 should have 100 test vectors");
}

#[test]
fn test_hqc_256_kat_file() {
    let vector_count = count_kat_vectors("../../pqc/pqkat/NIST-hqckem/KATs/Optimized_Implementation/hqc-256/hqc-256_kat.rsp");
    assert_eq!(vector_count, 100, "HQC-256 should have 100 test vectors");
}

// Note: Classic McEliece KAT tests removed as KAT files are not available in the restored pqkat directory

// Summary test to verify all KAT files exist and have correct counts
#[test]
fn test_all_kat_files_summary() {
    let kat_files = vec![
        "../../pqc/pqkat/NIST-ml-kem/KAT/mlkem512/PQCkemKAT_1632.rsp",
        "../../pqc/pqkat/NIST-ml-kem/KAT/mlkem768/PQCkemKAT_2400.rsp",
        "../../pqc/pqkat/NIST-ml-kem/KAT/mlkem1024/PQCkemKAT_3168.rsp",
        "../../pqc/pqkat/NIST-ml-dsa/KAT/mldsa2/PQCsignKAT_2544.rsp",
        "../../pqc/pqkat/NIST-ml-dsa/KAT/mldsa3/PQCsignKAT_4016.rsp",
        "../../pqc/pqkat/NIST-ml-dsa/KAT/mldsa5/PQCsignKAT_4880.rsp",
        "../../pqc/pqkat/NIST-fn-dsa/KAT/fndsa512-KAT.rsp",
        "../../pqc/pqkat/NIST-fn-dsa/KAT/fndsa1024-KAT.rsp",
        "../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-sha256-128f-simple/PQCsignKAT_64.rsp",
        "../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-sha256-192f-simple/PQCsignKAT_96.rsp",
        "../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-sha256-256f-simple/PQCsignKAT_128.rsp",
        "../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-shake256-128f-simple/PQCsignKAT_64.rsp",
        "../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-shake256-192f-simple/PQCsignKAT_96.rsp",
        "../../pqc/pqkat/NIST-slh-dsa/KAT/sphincs-shake256-256f-simple/PQCsignKAT_128.rsp",
        "../../pqc/pqkat/NIST-hqckem/KATs/Optimized_Implementation/hqc-128/hqc-128_kat.rsp",
        "../../pqc/pqkat/NIST-hqckem/KATs/Optimized_Implementation/hqc-192/hqc-192_kat.rsp",
        "../../pqc/pqkat/NIST-hqckem/KATs/Optimized_Implementation/hqc-256/hqc-256_kat.rsp"
    ];

    let mut total_vectors = 0;
    for file_path in &kat_files {
        let vector_count = count_kat_vectors(file_path);
        assert_eq!(vector_count, 100, "{} should have 100 test vectors", file_path);
        total_vectors += vector_count;
    }

    // Total: 17 algorithms × 100 vectors each = 1700 test vectors
    assert_eq!(total_vectors, 1700, "Total KAT vectors should be 1700");
    println!(
        "✅ All {} KAT files validated with {} total test vectors",
        kat_files.len(),
        total_vectors
    );
}
