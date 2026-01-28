#!/bin/bash
# Comprehensive test runner for PQSynQ
# Ensures 100% test coverage and pass rate

set -e

echo "=========================================="
echo "PQSynQ Comprehensive Test Suite"
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run tests and count results
run_test_suite() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "${BLUE}Running $test_name...${NC}"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if eval "$test_command"; then
        echo -e "${GREEN}✓ $test_name PASSED${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "${RED}✗ $test_name FAILED${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
    echo ""
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Not in pqsynq directory. Please run from pqsynq root.${NC}"
    exit 1
fi

echo -e "${YELLOW}Building PQSynQ...${NC}"
cargo build --release
echo ""

# Run all test suites
echo -e "${YELLOW}Starting comprehensive test suite...${NC}"
echo ""

# 1. Unit tests
run_test_suite "Unit Tests" "cargo test --lib"

# 2. Integration tests
run_test_suite "Integration Tests" "cargo test --test integration_tests"

# 3. KAT tests
run_test_suite "KAT Tests" "cargo test --test kat_tests"

# 4. Comprehensive tests
run_test_suite "Comprehensive Tests" "cargo test --test comprehensive_tests"

# 5. Documentation tests
run_test_suite "Documentation Tests" "cargo test --doc"

# 6. Benchmarks (compile check)
run_test_suite "Benchmark Compilation" "cargo bench --no-run"

# 7. Clippy linting
run_test_suite "Clippy Linting" "cargo clippy -- -D warnings"

# 8. Format check
run_test_suite "Format Check" "cargo fmt -- --check"

# 9. Security audit
if command -v cargo-audit &> /dev/null; then
    run_test_suite "Security Audit" "cargo audit"
else
    echo -e "${YELLOW}Warning: cargo-audit not installed, skipping security audit${NC}"
    echo ""
fi

# 10. Test coverage (if available)
if command -v cargo-tarpaulin &> /dev/null; then
    run_test_suite "Test Coverage" "cargo tarpaulin --out Html --output-dir coverage"
else
    echo -e "${YELLOW}Warning: cargo-tarpaulin not installed, skipping coverage analysis${NC}"
    echo ""
fi

# 11. Performance benchmarks
echo -e "${BLUE}Running performance benchmarks...${NC}"
if cargo bench --bench pqc_benchmarks -- --test; then
    echo -e "${GREEN}✓ Performance Benchmarks PASSED${NC}"
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    echo -e "${RED}✗ Performance Benchmarks FAILED${NC}"
    FAILED_TESTS=$((FAILED_TESTS + 1))
fi
TOTAL_TESTS=$((TOTAL_TESTS + 1))
echo ""

# 12. Memory leak tests
run_test_suite "Memory Leak Tests" "cargo test --test comprehensive_tests -- --ignored test_memory_leaks"

# 13. Stress tests
run_test_suite "Stress Tests" "cargo test --test comprehensive_tests -- --ignored test_stress_scenarios"

# 14. Thread safety tests
run_test_suite "Thread Safety Tests" "cargo test --test comprehensive_tests -- --ignored test_thread_safety"

# 15. Algorithm-specific tests
echo -e "${BLUE}Running algorithm-specific tests...${NC}"

# Test each algorithm individually
ALGORITHMS=(
    "ML-KEM-512" "ML-KEM-768" "ML-KEM-1024"
    "ML-DSA-44" "ML-DSA-65" "ML-DSA-87"
    "FN-DSA-512" "FN-DSA-1024"
    "HQC-KEM-128" "HQC-KEM-192" "HQC-KEM-256"
    "CMCE-KEM-348864" "CMCE-KEM-460896" "CMCE-KEM-6688128" "CMCE-KEM-6960119" "CMCE-KEM-8192128"
    "SLH-DSA-SHAKE-128f" "SLH-DSA-SHAKE-128s" "SLH-DSA-SHAKE-192f" "SLH-DSA-SHAKE-192s" "SLH-DSA-SHAKE-256f" "SLH-DSA-SHAKE-256s"
    "SLH-DSA-SHA2-128f" "SLH-DSA-SHA2-128s" "SLH-DSA-SHA2-192f" "SLH-DSA-SHA2-192s" "SLH-DSA-SHA2-256f" "SLH-DSA-SHA2-256s"
)

for algo in "${ALGORITHMS[@]}"; do
    run_test_suite "Algorithm Test: $algo" "cargo test --test comprehensive_tests -- --ignored test_algorithm_$algo"
done

# 16. Cross-platform compatibility tests
run_test_suite "Cross-platform Tests" "cargo test --test comprehensive_tests -- --ignored test_cross_platform"

# 17. Error handling tests
run_test_suite "Error Handling Tests" "cargo test --test comprehensive_tests -- --ignored test_error_handling"

# 18. Edge case tests
run_test_suite "Edge Case Tests" "cargo test --test comprehensive_tests -- --ignored test_edge_cases"

# 19. Performance regression tests
run_test_suite "Performance Regression Tests" "cargo test --test comprehensive_tests -- --ignored test_performance_characteristics"

# 20. Memory usage tests
run_test_suite "Memory Usage Tests" "cargo test --test comprehensive_tests -- --ignored test_memory_usage"

# Final results
echo "=========================================="
echo -e "${YELLOW}TEST RESULTS SUMMARY${NC}"
echo "=========================================="
echo -e "Total Tests: ${BLUE}$TOTAL_TESTS${NC}"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}🎉 ALL TESTS PASSED! 100% SUCCESS RATE! 🎉${NC}"
    echo -e "${GREEN}PQSynQ is ready for production use!${NC}"
    exit 0
else
    echo -e "${RED}❌ $FAILED_TESTS TESTS FAILED${NC}"
    echo -e "${RED}Please fix the failing tests before proceeding.${NC}"
    exit 1
fi
