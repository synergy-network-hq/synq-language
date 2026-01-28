use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::hint::black_box;
use aegis_crypto_core::{
    // KEMs
    kyber::{kyber512_keygen, kyber512_encapsulate, kyber512_decapsulate},
    hqc::{hqc128_keygen, hqc128_encapsulate, hqc128_decapsulate},
    // Add other variants like kyber768_keygen, etc.
    // Signatures
    dilithium::{dilithium_keygen, dilithium_sign, dilithium_verify},
    falcon::{falcon512_keygen, falcon512_sign, falcon512_verify},
    sphincsplus::{sphincsplus_sha2_128f_keygen, sphincsplus_sha2_128f_sign, sphincsplus_sha2_128f_verify},
    // Add other variants
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nist_enabled = cfg!(feature = "nist-ref");

    let iterations = 1000;
    let results_pqrust = Vec::new();
    let results_nist = if nist_enabled { Vec::new() } else { vec![] };

    // Helper function to measure time
    fn measure_time<F>(mut f: F, iterations: usize) -> (f64, f64)
    where
        F: FnMut() -> (),
    {
        let mut times = Vec::new();
        for _ in 0..iterations {
            let start = Instant::now();
            f();
            let duration = start.elapsed().as_nanos() as f64;
            times.push(duration);
        }
        let mean = times.iter().sum::<f64>() / times.len() as f64;
        let variance = times.iter().map(|&t| (t - mean).powi(2)).sum::<f64>() / times.len() as f64;
        let std_dev = variance.sqrt();
        (mean, std_dev)
    }

    // Example for Kyber 512 keygen
    let keygen_f = || {
        let keypair = kyber512_keygen();
        black_box(keypair);
    };
    let (mean, std_dev) = measure_time(keygen_f, iterations);
    results_pqrust.push(("pqrust".to_string(), "kyber".to_string(), "512".to_string(), "keygen".to_string(), mean, std_dev, iterations as f64));

    // Encapsulate (setup once)
    let keypair = kyber512_keygen();
    let pk = keypair.public_key();
    let encap_f = || {
        let encapsulated = kyber512_encapsulate(&pk).expect("Encap fail");
        black_box(encapsulated);
    };
    let (mean, std_dev) = measure_time(encap_f, iterations);
    results_pqrust.push(("pqrust".to_string(), "kyber".to_string(), "512".to_string(), "encapsulate".to_string(), mean, std_dev, iterations as f64));

    // Decapsulate setup
    let sk = keypair.secret_key();
    let encapsulated = kyber512_encapsulate(&pk).expect("Encap fail");
    let ct = encapsulated.ciphertext();
    let decap_f = || {
        let ss = kyber512_decapsulate(&sk, &ct).expect("Decap fail");
        black_box(ss);
    };
    let (mean, std_dev) = measure_time(decap_f, iterations);
    results_pqrust.push(("pqrust".to_string(), "kyber".to_string(), "512".to_string(), "decapsulate".to_string(), mean, std_dev, iterations as f64));

    // Expand results vec to include all variants
    // For ML-KEM:
    let (mean, std_dev) = measure_time(|| kyber768_keygen(), iterations);
    results_pqrust.push(("pqrust".to_string(), "kyber".to_string(), "768".to_string(), "keygen".to_string(), mean, std_dev, iterations as f64));
    // ... encapsulate/decapsulate for 768, 1024 similarly

    // For ML-DSA:
    let (mean, std_dev) = measure_time(|| dilithium44_keygen(), iterations);
    results_pqrust.push(("pqrust".to_string(), "dilithium".to_string(), "44".to_string(), "keygen".to_string(), mean, std_dev, iterations as f64));
    // sign/verify for 44, 65, 87 with fixed msg

    // For SLH-DSA: All 12 variants (sha2_128f, shake_256s, etc.)
    let (mean, std_dev) = measure_time(|| sphincs_sha2_128f_keygen(), iterations);
    results_pqrust.push(("pqrust".to_string(), "sphincs".to_string(), "sha2-128f".to_string(), "keygen".to_string(), mean, std_dev, iterations as f64));
    // ... sign/verify for each

    // For FN-DSA: Falcon 512/1024
    let (mean, std_dev) = measure_time(|| falcon1024_keygen(), iterations);
    results_pqrust.push(("pqrust".to_string(), "falcon".to_string(), "1024".to_string(), "keygen".to_string(), mean, std_dev, iterations as f64));
    // sign/verify

    // For HQC: 128/192/256
    let (mean, std_dev) = measure_time(|| hqc192_keygen(), iterations);
    results_pqrust.push(("pqrust".to_string(), "hqc".to_string(), "192".to_string(), "keygen".to_string(), mean, std_dev, iterations as f64));
    // encapsulate/decapsulate

    // Classic McEliece (experimental)
    #[cfg(feature = "classicmceliece")]
    {
        std::env::set_var("RUST_MIN_STACK", "800000000");
        let (mean, std_dev) = measure_time(|| mceliece348864_keygen(), iterations);
        results_pqrust.push(("pqrust".to_string(), "mceliece".to_string(), "348864".to_string(), "keygen".to_string(), mean, std_dev, iterations as f64));
        // ... for other levels
    }

    #[cfg(feature = "nist-ref")]
    {
        // NIST ML-KEM 512 keygen
        let (mean, std_dev) = measure_time(|| {
            let mut pk = [0u8; 800];
            let mut sk = [0u8; 1632];
            nist_ml_kem_512_keypair(&mut pk, &mut sk);
            black_box((pk, sk));
        }, iterations);
        results_nist.push(("nist".to_string(), "nist_ml_kem".to_string(), "512".to_string(), "keygen".to_string(), mean, std_dev, iterations as f64));
        // ... enc/dec, all algos/levels
    }

    // Write separate CSVs
    let path_pqrust = "../../performance_results/pqrust_benchmarks.csv";
    write_csv(&results_pqrust, path_pqrust, "pqrust");

    if nist_enabled {
        let path_nist = "../../performance_results/nist_benchmarks.csv";
        write_csv(&results_nist, path_nist, "nist");
    }

    println!("Generated CSVs for completed impls.");
}

fn write_csv(results: &Vec<(String, String, String, String, f64, f64, f64)>, path: &str, impl_name: &str) {
    // Clean folder first? rm non-generated, but since generated only, skip
    let mut file = File::create(path).expect("CSV create fail");
    writeln!(file, "impl,algorithm,variant,operation,mean_time_ns,std_dev_ns,iterations").unwrap();
    for (impl_, alg, var, op, mean, std, iters) in results {
        writeln!(file, "{},{},{},{},{:.2},{:.2},{}", impl_name, alg, var, op, mean, std, iters).unwrap();
    }
}
