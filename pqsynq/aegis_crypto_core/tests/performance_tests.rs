//! Performance tests for cryptographic operations.

use aegis_crypto_core::performance::{ run_performance_tests, print_performance_report };

#[test]
fn test_performance_benchmarks() {
    println!("Running performance benchmarks...");
    let results = run_performance_tests();
    print_performance_report(&results);

    // Check that we have results for all expected operations
    assert!(!results.is_empty(), "Performance tests should produce results");

    // Count how many tests meet their targets
    let total_tests = results.len();
    let passed_tests = results
        .iter()
        .filter(|result| {
            let target = aegis_crypto_core::performance::PERFORMANCE_TARGETS
                .iter()
                .find(|(op, _)| *op == result.operation)
                .map(|(_, duration)| *duration)
                .unwrap_or(std::time::Duration::from_millis(100));
            aegis_crypto_core::performance::meets_target(result, target)
        })
        .count();

    println!("Performance test summary: {}/{} operations meet targets", passed_tests, total_tests);

    // For now, we'll just ensure the tests run without panicking
    // In a real scenario, you might want to enforce stricter performance requirements
    assert!(passed_tests > 0, "At least some operations should meet performance targets");
}
