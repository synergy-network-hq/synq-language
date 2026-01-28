use criterion::{ criterion_group, criterion_main, Criterion };
use std::hint::black_box;
use aegis_crypto_core::{
    // KEM algorithms
    kyber::{ kyber512_keygen, kyber512_encapsulate, kyber512_decapsulate },
    hqc::{ hqc128_keygen, hqc128_encapsulate, hqc128_decapsulate },
    // Signature algorithms
    dilithium::{ dilithium_keygen, dilithium_sign, dilithium_verify },
    falcon::{ falcon512_keygen, falcon512_sign, falcon512_verify },
    sphincsplus::{ sphincsplus_sha2_128f_keygen, sphincsplus_sha2_128f_sign, sphincsplus_sha2_128f_verify },
};

// Classic McEliece benchmarks removed as not available in current implementation

fn bench_kyber_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Kyber KEM Operations");

    // Key generation benchmark
    group.bench_function("kyber512_keygen", |b| {
        b.iter(|| {
            let keypair = black_box(kyber512_keygen());
            black_box(keypair)
        })
    });

    // Encapsulation benchmark
    group.bench_function("kyber512_encapsulate", |b| {
        let keypair = kyber512_keygen();
        let public_key = keypair.public_key();
        b.iter(|| {
            let encapsulated = black_box(kyber512_encapsulate(&public_key).expect("Encapsulation should succeed"));
            black_box(encapsulated)
        })
    });

    // Decapsulation benchmark
    group.bench_function("kyber512_decapsulate", |b| {
        let keypair = kyber512_keygen();
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let encapsulated = kyber512_encapsulate(&public_key).expect("Encapsulation should succeed");
        let ciphertext = encapsulated.ciphertext();

        b.iter(|| {
            let decapsulated = black_box(kyber512_decapsulate(&secret_key, &ciphertext).expect("Decapsulation should succeed"));
            black_box(decapsulated)
        })
    });

    group.finish();
}

fn bench_hqc_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("HQC KEM Operations");

    // Key generation benchmark
    group.bench_function("hqc128_keygen", |b| {
        b.iter(|| {
            let keypair = black_box(hqc128_keygen());
            black_box(keypair)
        })
    });

    // Encapsulation benchmark
    group.bench_function("hqc128_encapsulate", |b| {
        let keypair = hqc128_keygen();
        let public_key = keypair.public_key();
        b.iter(|| {
            let encapsulated = black_box(hqc128_encapsulate(&public_key).expect("Encapsulation should succeed"));
            black_box(encapsulated)
        })
    });

    // Decapsulation benchmark
    group.bench_function("hqc128_decapsulate", |b| {
        let keypair = hqc128_keygen();
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let encapsulated = hqc128_encapsulate(&public_key).expect("Encapsulation should succeed");
        let ciphertext = encapsulated.ciphertext();

        b.iter(|| {
            let decapsulated = black_box(hqc128_decapsulate(&secret_key, &ciphertext).expect("Decapsulation should succeed"));
            black_box(decapsulated)
        })
    });

    group.finish();
}

// Classic McEliece benchmark function removed as not available in current implementation

fn bench_dilithium_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dilithium Signature Operations");

    // Key generation benchmark
    group.bench_function("dilithium_keygen", |b| {
        b.iter(|| {
            let keypair = black_box(dilithium_keygen());
            black_box(keypair)
        })
    });

    // Signature generation benchmark
    group.bench_function("dilithium_sign", |b| {
        let keypair = dilithium_keygen();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for Dilithium signature generation";

        b.iter(|| {
            let signature = black_box(dilithium_sign(&secret_key, message));
            black_box(signature)
        })
    });

    // Signature verification benchmark
    group.bench_function("dilithium_verify", |b| {
        let keypair = dilithium_keygen();
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for Dilithium signature verification";
        let signed_message = dilithium_sign(&secret_key, message);

        b.iter(|| {
            let is_valid = black_box(dilithium_verify(&public_key, &signed_message));
            black_box(is_valid)
        })
    });

    group.finish();
}

fn bench_falcon_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Falcon Signature Operations");

    // Key generation benchmark
    group.bench_function("falcon512_keygen", |b| {
        b.iter(|| {
            let keypair = black_box(falcon512_keygen());
            black_box(keypair)
        })
    });

    // Signature generation benchmark
    group.bench_function("falcon512_sign", |b| {
        let keypair = falcon512_keygen();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for Falcon signature generation";

        b.iter(|| {
            let signature = black_box(falcon512_sign(&secret_key, message));
            black_box(signature)
        })
    });

    // Signature verification benchmark
    group.bench_function("falcon512_verify", |b| {
        let keypair = falcon512_keygen();
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for Falcon signature verification";
        let signature = falcon512_sign(&secret_key, message);

        b.iter(|| {
            let is_valid = black_box(falcon512_verify(&public_key, message, &signature));
            black_box(is_valid)
        })
    });

    group.finish();
}

fn bench_sphincsplus_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("SPHINCS+ Signature Operations");

    // Key generation benchmark
    group.bench_function("sphincsplus_sha2_128f_keygen", |b| {
        b.iter(|| {
            let keypair = black_box(sphincsplus_sha2_128f_keygen());
            black_box(keypair)
        })
    });

    // Signature generation benchmark
    group.bench_function("sphincsplus_sha2_128f_sign", |b| {
        let keypair = sphincsplus_sha2_128f_keygen();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for SPHINCS+ signature generation";

        b.iter(|| {
            let signature = black_box(sphincsplus_sha2_128f_sign(&secret_key, message));
            black_box(signature)
        })
    });

    // Signature verification benchmark
    group.bench_function("sphincsplus_sha2_128f_verify", |b| {
        let keypair = sphincsplus_sha2_128f_keygen();
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();
        let message = b"Benchmark test message for SPHINCS+ signature verification";
        let signed_message = sphincsplus_sha2_128f_sign(&secret_key, message);

        b.iter(|| {
            let is_valid = black_box(sphincsplus_sha2_128f_verify(&public_key, &signed_message));
            black_box(is_valid)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_kyber_operations,
    bench_hqc_operations,
    bench_dilithium_operations,
    bench_falcon_operations,
    bench_sphincsplus_operations
);
criterion_main!(benches);
