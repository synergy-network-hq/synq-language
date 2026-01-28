//! Performance measurement utilities for cryptographic operations.

use std::time::{ Duration, Instant };

/// Performance measurement results
#[derive(Debug, Clone)]
pub struct PerformanceResult {
    pub operation: String,
    pub algorithm: String,
    pub variant: String,
    pub duration: Duration,
    pub iterations: usize,
    pub average_duration: Duration,
}

/// Performance targets from the specification
pub const PERFORMANCE_TARGETS: &[(&str, Duration)] = &[
    ("key_generation", Duration::from_millis(100)),
    ("encapsulation", Duration::from_millis(50)),
    ("decapsulation", Duration::from_millis(50)),
    ("signature_generation", Duration::from_millis(100)),
    ("signature_verification", Duration::from_millis(50)),
];

/// Measure performance of a cryptographic operation
pub fn measure_performance<F, T>(
    operation_name: &str,
    algorithm: &str,
    variant: &str,
    iterations: usize,
    operation: F
) -> PerformanceResult
    where F: Fn() -> T
{
    let start = Instant::now();

    for _ in 0..iterations {
        let _result = operation();
    }

    let total_duration = start.elapsed();
    let average_duration = total_duration / (iterations as u32);

    PerformanceResult {
        operation: operation_name.to_string(),
        algorithm: algorithm.to_string(),
        variant: variant.to_string(),
        duration: total_duration,
        iterations,
        average_duration,
    }
}

/// Check if performance meets target
pub fn meets_target(result: &PerformanceResult, target: Duration) -> bool {
    result.average_duration <= target
}

/// Format duration for display
pub fn format_duration(duration: Duration) -> String {
    let millis = duration.as_millis();
    if millis > 0 {
        format!("{}ms", millis)
    } else {
        let micros = duration.as_micros();
        format!("{}μs", micros)
    }
}

/// Run comprehensive performance tests
#[cfg(
    all(
        feature = "kyber",
        feature = "hqc",
        feature = "dilithium",
        feature = "falcon",
        feature = "sphincsplus"
    )
)]
pub fn run_performance_tests() -> Vec<PerformanceResult> {
    use crate::{
        kyber_keygen,
        kyber_encapsulate,
        kyber_decapsulate,
        hqc_keygen,
        hqc_encapsulate,
        hqc_decapsulate,
        dilithium_keygen,
        dilithium_sign,
        dilithium_verify,
        falcon_keygen,
        falcon_sign,
        falcon_verify,
        sphincsplus_keygen,
        sphincsplus_sign,
        sphincsplus_verify,
    };

    let mut results = Vec::new();
    let iterations = 10; // Number of iterations for averaging (reduced for testing)

    // Test Kyber operations
    results.push(
        measure_performance("key_generation", "Kyber", "ML-KEM-768", iterations, kyber_keygen)
    );

    let kyber_keypair = kyber_keygen();
    let kyber_pk = kyber_keypair.public_key();
    let kyber_sk = kyber_keypair.secret_key();

    results.push(
        measure_performance("encapsulation", "Kyber", "ML-KEM-768", iterations, ||
            kyber_encapsulate(&kyber_pk)
        )
    );

    let kyber_encapsulated = kyber_encapsulate(&kyber_pk).expect("Encapsulation should succeed");
    let kyber_ct = kyber_encapsulated.ciphertext();

    results.push(
        measure_performance("decapsulation", "Kyber", "ML-KEM-768", iterations, ||
            kyber_decapsulate(&kyber_sk, &kyber_ct)
        )
    );

    // Test HQC operations
    results.push(measure_performance("key_generation", "HQC", "HQC-256", iterations, hqc_keygen));

    let hqc_keypair = hqc_keygen();
    let hqc_pk = hqc_keypair.public_key();
    let hqc_sk = hqc_keypair.secret_key();

    results.push(
        measure_performance("encapsulation", "HQC", "HQC-256", iterations, ||
            hqc_encapsulate(&hqc_pk)
        )
    );

    let hqc_encapsulated = hqc_encapsulate(&hqc_pk).expect("Encapsulation should succeed");
    let hqc_ct = hqc_encapsulated.ciphertext();

    results.push(
        measure_performance("decapsulation", "HQC", "HQC-256", iterations, ||
            hqc_decapsulate(&hqc_sk, &hqc_ct)
        )
    );

    // Test Dilithium operations
    results.push(
        measure_performance(
            "key_generation",
            "Dilithium",
            "ML-DSA-87",
            iterations,
            dilithium_keygen
        )
    );

    let dilithium_keypair = dilithium_keygen();
    let dilithium_pk = dilithium_keypair.public_key();
    let dilithium_sk = dilithium_keypair.secret_key();
    let message = b"Performance test message for Dilithium";

    results.push(
        measure_performance("signature_generation", "Dilithium", "ML-DSA-87", iterations, ||
            dilithium_sign(&dilithium_sk, message)
        )
    );

    let dilithium_signed = dilithium_sign(&dilithium_sk, message);

    results.push(
        measure_performance("signature_verification", "Dilithium", "ML-DSA-87", iterations, ||
            dilithium_verify(&dilithium_pk, &dilithium_signed)
        )
    );

    // Test Falcon operations
    results.push(
        measure_performance("key_generation", "Falcon", "Falcon-512", iterations, falcon_keygen)
    );

    let falcon_keypair = falcon_keygen();
    let falcon_pk = falcon_keypair.public_key();
    let falcon_sk = falcon_keypair.secret_key();
    let message = b"Performance test message for Falcon";

    results.push(
        measure_performance("signature_generation", "Falcon", "Falcon-512", iterations, ||
            falcon_sign(&falcon_sk, message)
        )
    );

    let falcon_signature = falcon_sign(&falcon_sk, message);

    results.push(
        measure_performance("signature_verification", "Falcon", "Falcon-512", iterations, ||
            falcon_verify(&falcon_pk, message, &falcon_signature)
        )
    );

    // Test SPHINCS+ operations
    results.push(
        measure_performance(
            "key_generation",
            "SPHINCS+",
            "SHA2-128f",
            iterations,
            sphincsplus_keygen
        )
    );

    let sphincsplus_keypair = sphincsplus_keygen();
    let sphincsplus_pk = sphincsplus_keypair.public_key();
    let sphincsplus_sk = sphincsplus_keypair.secret_key();
    let message = b"Performance test message for SPHINCS+";

    results.push(
        measure_performance("signature_generation", "SPHINCS+", "SHA2-128f", iterations, ||
            sphincsplus_sign(&sphincsplus_sk, message)
        )
    );

    let sphincsplus_signed = sphincsplus_sign(&sphincsplus_sk, message);

    results.push(
        measure_performance("signature_verification", "SPHINCS+", "SHA2-128f", iterations, ||
            sphincsplus_verify(&sphincsplus_pk, &sphincsplus_signed)
        )
    );

    results
}

/// Print performance report
pub fn print_performance_report(results: &[PerformanceResult]) {
    println!("=== PERFORMANCE BENCHMARK REPORT ===");
    println!("Targets:");
    for (operation, target) in PERFORMANCE_TARGETS {
        println!("  {}: {}", operation, format_duration(*target));
    }
    println!();

    println!("Results:");
    for result in results {
        let target = PERFORMANCE_TARGETS.iter()
            .find(|(op, _)| *op == result.operation)
            .map(|(_, duration)| *duration)
            .unwrap_or(Duration::from_millis(100));

        let meets = meets_target(result, target);
        let status = if meets { "✅" } else { "❌" };

        println!(
            "{} {} {} {}: {} (target: {})",
            status,
            result.algorithm,
            result.variant,
            result.operation,
            format_duration(result.average_duration),
            format_duration(target)
        );
    }

    println!();
    println!("Summary:");
    let total_tests = results.len();
    let passed_tests = results
        .iter()
        .filter(|result| {
            let target = PERFORMANCE_TARGETS.iter()
                .find(|(op, _)| *op == result.operation)
                .map(|(_, duration)| *duration)
                .unwrap_or(Duration::from_millis(100));
            meets_target(result, target)
        })
        .count();

    println!("  Tests: {}/{} passed", passed_tests, total_tests);
    println!("  Success rate: {:.1}%", ((passed_tests as f64) / (total_tests as f64)) * 100.0);
}
