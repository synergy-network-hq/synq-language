//! Comprehensive Performance Benchmarks for PQSynQ
//!
//! These benchmarks provide legitimate performance assessment of all
//! PQC algorithms with proper statistical analysis and multiple test scenarios.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use pqsynq::{Kem, Sign, KeyEncapsulation, DigitalSignature};
use std::time::Duration;

/// Benchmark ML-KEM algorithms with throughput measurements
fn bench_mlkem_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("ML-KEM Performance");
    group.measurement_time(Duration::from_secs(10));
    
    let mlkem_variants = [
        ("ML-KEM-512", Kem::mlkem512()),
        ("ML-KEM-768", Kem::mlkem768()),
        ("ML-KEM-1024", Kem::mlkem1024()),
    ];
    
    for (name, kem) in mlkem_variants {
        // Key generation benchmark
        group.bench_with_input(BenchmarkId::new(format!("{}_keygen", name), ""), &(), |b, _| {
            b.iter(|| kem.keygen())
        });
        
        // Encapsulation benchmark
        let (pk, _) = kem.keygen().unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_encapsulate", name), ""), &pk, |b, pk| {
            b.iter(|| kem.encapsulate(&pk))
        });
        
        // Decapsulation benchmark
        let (ct, sk) = kem.encapsulate(&pk).unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_decapsulate", name), ""), &(ct, sk), |b, (ct, sk)| {
            b.iter(|| kem.decapsulate(black_box(ct), &sk))
        });
        
        // Throughput measurements
        group.throughput(Throughput::Elements(1));
    }
    
    group.finish();
}

/// Benchmark ML-DSA algorithms with throughput measurements
fn bench_mldsa_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("ML-DSA Performance");
    group.measurement_time(Duration::from_secs(10));
    
    let mldsa_variants = [
        ("ML-DSA-44", Sign::mldsa44()),
        ("ML-DSA-65", Sign::mldsa65()),
        ("ML-DSA-87", Sign::mldsa87()),
    ];
    
    let message = b"Performance benchmark message for ML-DSA algorithms";
    
    for (name, signer) in mldsa_variants {
        // Key generation benchmark
        group.bench_with_input(BenchmarkId::new(format!("{}_keygen", name), ""), &(), |b, _| {
            b.iter(|| signer.keygen())
        });
        
        // Signing benchmark
        let (_, sk) = DigitalSignature::keygen(&signer).unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_sign", name), ""), &(message, sk), |b, (msg, sk)| {
            b.iter(|| signer.sign(&*msg, &*sk))
        });
        
        // Verification benchmark
        let (pk, sk) = DigitalSignature::keygen(&signer).unwrap();
        let sig = signer.sign(message, &sk).unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_verify", name), ""), &(message, sig, pk), |b, (msg, sig, pk)| {
            b.iter(|| signer.verify(&*msg, &*sig, &*pk))
        });
        
        // Detached signature benchmark
        let (pk, sk) = DigitalSignature::keygen(&signer).unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_detached_sign", name), ""), &(message, sk), |b, (msg, sk)| {
            b.iter(|| signer.sign(&*msg, &*sk))
        });
        
        // Contextual signing benchmark (if supported)
        // Note: Contextual signing not implemented in current version
        
        group.throughput(Throughput::Elements(1));
    }
    
    group.finish();
}

/// Benchmark FN-DSA algorithms
fn bench_fndsa_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("FN-DSA Performance");
    group.measurement_time(Duration::from_secs(10));
    
    let fndsa_variants = [
        ("FN-DSA-512", Sign::fndsa512()),
        ("FN-DSA-1024", Sign::fndsa1024()),
    ];
    
    let message = b"Performance benchmark message for FN-DSA algorithms";
    
    for (name, signer) in fndsa_variants {
        // Key generation benchmark
        group.bench_with_input(BenchmarkId::new(format!("{}_keygen", name), ""), &(), |b, _| {
            b.iter(|| signer.keygen())
        });
        
        // Signing benchmark
        let (_, sk) = DigitalSignature::keygen(&signer).unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_sign", name), ""), &(message, sk), |b, (msg, sk)| {
            b.iter(|| signer.sign(&*msg, &*sk))
        });
        
        // Verification benchmark
        let (pk, sk) = DigitalSignature::keygen(&signer).unwrap();
        let sig = signer.sign(message, &sk).unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_verify", name), ""), &(message, sig, pk), |b, (msg, sig, pk)| {
            b.iter(|| signer.verify(&*msg, &*sig, &*pk))
        });
        
        // Detached signature benchmark
        let (pk, sk) = DigitalSignature::keygen(&signer).unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_detached_sign", name), ""), &(message, sk), |b, (msg, sk)| {
            b.iter(|| signer.sign(&*msg, &*sk))
        });
        
        group.throughput(Throughput::Elements(1));
    }
    
    group.finish();
}

// Note: SLH-DSA algorithms not implemented in current version

/// Benchmark HQC-KEM algorithms
fn bench_hqckem_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("HQC-KEM Performance");
    group.measurement_time(Duration::from_secs(10));
    
    let hqckem_variants = [
        ("HQC-KEM-128", Kem::hqckem128()),
        ("HQC-KEM-192", Kem::hqckem192()),
        ("HQC-KEM-256", Kem::hqckem256()),
    ];
    
    for (name, kem) in hqckem_variants {
        // Key generation benchmark
        group.bench_with_input(BenchmarkId::new(format!("{}_keygen", name), ""), &(), |b, _| {
            b.iter(|| kem.keygen())
        });
        
        // Encapsulation benchmark
        let (pk, _) = kem.keygen().unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_encapsulate", name), ""), &pk, |b, pk| {
            b.iter(|| kem.encapsulate(&pk))
        });
        
        // Decapsulation benchmark
        let (ct, sk) = kem.encapsulate(&pk).unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_decapsulate", name), ""), &(ct, sk), |b, (ct, sk)| {
            b.iter(|| kem.decapsulate(black_box(ct), &sk))
        });
        
        group.throughput(Throughput::Elements(1));
    }
    
    group.finish();
}

/// Benchmark CMCE-KEM algorithms
fn bench_cmce_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("CMCE-KEM Performance");
    group.measurement_time(Duration::from_secs(15)); // CMCE can be slower
    
    let cmce_variants = [
        ("CMCE-KEM-348864", Kem::cmce348864()),
        ("CMCE-KEM-460896", Kem::cmce460896()),
        ("CMCE-KEM-6688128", Kem::cmce6688128()),
        ("CMCE-KEM-6960119", Kem::cmce6960119()),
        ("CMCE-KEM-8192128", Kem::cmce8192128()),
    ];
    
    for (name, kem) in cmce_variants {
        // Key generation benchmark
        group.bench_with_input(BenchmarkId::new(format!("{}_keygen", name), ""), &(), |b, _| {
            b.iter(|| kem.keygen())
        });
        
        // Encapsulation benchmark
        let (pk, _) = kem.keygen().unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_encapsulate", name), ""), &pk, |b, pk| {
            b.iter(|| kem.encapsulate(&pk))
        });
        
        // Decapsulation benchmark
        let (ct, sk) = kem.encapsulate(&pk).unwrap();
        group.bench_with_input(BenchmarkId::new(format!("{}_decapsulate", name), ""), &(ct, sk), |b, (ct, sk)| {
            b.iter(|| kem.decapsulate(black_box(ct), &sk))
        });
        
        group.throughput(Throughput::Elements(1));
    }
    
    group.finish();
}

/// Memory usage benchmarks
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Usage");
    
    // Test key sizes
    let kem = Kem::mlkem768();
    let (pk, sk) = kem.keygen().unwrap();
    let (ct, ss) = kem.encapsulate(&pk).unwrap();
    
    group.bench_function("mlkem768_key_sizes", |b| {
        b.iter(|| {
            black_box(pk.len() + sk.len() + ct.len() + ss.len())
        })
    });
    
    let signer = Sign::mldsa65();
    let (pk, sk) = signer.keygen().unwrap();
    let sig = signer.sign(b"test", &sk).unwrap();
    
    group.bench_function("mldsa65_key_sizes", |b| {
        b.iter(|| {
            black_box(pk.len() + sk.len() + sig.len())
        })
    });
    
    group.finish();
}

/// Stress test benchmarks
fn bench_stress_tests(c: &mut Criterion) {
    let mut group = c.benchmark_group("Stress Tests");
    group.measurement_time(Duration::from_secs(30));
    
    // Multiple operations in sequence
    group.bench_function("mlkem768_multiple_operations", |b| {
        b.iter(|| {
            let kem = Kem::mlkem768();
            for _ in 0..100 {
                let (pk, sk) = KeyEncapsulation::keygen(&kem).unwrap();
                let (ct, ss1) = kem.encapsulate(&pk).unwrap();
                let ss2 = kem.decapsulate(&ct, &sk).unwrap();
                assert_eq!(ss1, ss2);
            }
        })
    });
    
    group.bench_function("mldsa65_multiple_operations", |b| {
        b.iter(|| {
            let signer = Sign::mldsa65();
            for _ in 0..100 {
                let (pk, sk) = DigitalSignature::keygen(&signer).unwrap();
                let message = b"stress test message";
                let sig = signer.sign(message, &sk).unwrap();
                let valid = signer.verify(message, &sig, &pk).unwrap();
                assert!(valid);
            }
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_mlkem_algorithms,
    bench_mldsa_algorithms,
    bench_fndsa_algorithms,
    bench_hqckem_algorithms,
    bench_cmce_algorithms,
    bench_memory_usage,
    bench_stress_tests
);
criterion_main!(benches);
