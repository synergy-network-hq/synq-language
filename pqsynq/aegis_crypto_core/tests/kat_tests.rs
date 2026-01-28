//! Known Answer Tests (KAT) for cryptographic algorithms.

use std::path::Path;
use aegis_crypto_core::traits::Kem;
use aegis_crypto_core::mlkem::traits::{ MLKEM768, MLKEMPublicKey, MLKEMSecretKey };

/// Parse KAT file and extract test vectors.
fn parse_kat_file<P: AsRef<Path>>(path: P) -> Result<Vec<KatVector>, Box<dyn std::error::Error>> {
    let path_ref = path.as_ref();
    let content = std::fs::read_to_string(path_ref)?;
    let mut vectors = Vec::new();
    let mut current_vector = KatVector::default();

    println!("Parsing KAT file: {}", path_ref.display());
    println!("File content length: {} bytes", content.len());

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("count = ") {
            if current_vector.count != 0 {
                vectors.push(current_vector.clone());
            }
            current_vector = KatVector::default();
            current_vector.count = line.split(" = ").nth(1).unwrap().parse()?;
            println!("Found count: {}", current_vector.count);
        } else if line.starts_with("pk = ") {
            current_vector.public_key = hex::decode(line.split(" = ").nth(1).unwrap())?;
            println!("Found public key: {} bytes", current_vector.public_key.len());
        } else if line.starts_with("sk = ") {
            current_vector.secret_key = hex::decode(line.split(" = ").nth(1).unwrap())?;
            println!("Found secret key: {} bytes", current_vector.secret_key.len());
        } else if line.starts_with("ct = ") {
            current_vector.ciphertext = hex::decode(line.split(" = ").nth(1).unwrap())?;
            println!("Found ciphertext: {} bytes", current_vector.ciphertext.len());
        } else if line.starts_with("ss = ") {
            current_vector.shared_secret = hex::decode(line.split(" = ").nth(1).unwrap())?;
            println!("Found shared secret: {} bytes", current_vector.shared_secret.len());
        }
    }

    // Add the last vector if it has data
    if current_vector.count != 0 || !current_vector.public_key.is_empty() {
        vectors.push(current_vector);
    }

    println!("Total vectors found: {}", vectors.len());
    Ok(vectors)
}

#[derive(Default, Clone)]
struct KatVector {
    count: usize,
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[test]
fn test_mlkem768_kat() {
    let kat_path = "../kat_files/ml-kem-768_clean_nist.rsp";
    if !Path::new(kat_path).exists() {
        eprintln!("KAT file not found: {}. Run generate_kat_files.sh first.", kat_path);
        return;
    }

    let vectors = parse_kat_file(kat_path).expect("Failed to parse KAT file");
    assert!(!vectors.is_empty(), "No KAT vectors found");

    for vector in vectors.iter().take(10) {
        // Test first 10 vectors
        // Test key generation (we can't test this against KAT as it's random)
        let _keypair = MLKEM768::keygen().expect("Key generation should succeed");

        // Test encapsulation with KAT public key
        let kat_pk = MLKEMPublicKey(vector.public_key.clone());
        let _encapsulated = MLKEM768::encapsulate(&kat_pk).expect("Encapsulation should succeed");

        // Test decapsulation with KAT secret key and ciphertext
        let kat_sk = MLKEMSecretKey(vector.secret_key.clone());
        let decapsulated = MLKEM768::decapsulate(&kat_sk, vector.ciphertext.as_slice()).expect(
            "Decapsulation should succeed"
        );

        // Verify the decapsulated shared secret matches the KAT shared secret
        assert_eq!(
            decapsulated.as_bytes(),
            vector.shared_secret,
            "Decapsulated shared secret should match KAT shared secret"
        );
    }
}

#[test]
fn test_mlkem768_kat_invalid_inputs() {
    let kat_path = "../kat_files/ml-kem-768_clean_nist.rsp";
    if !Path::new(kat_path).exists() {
        eprintln!("KAT file not found: {}. Run generate_kat_files.sh first.", kat_path);
        return;
    }

    let vectors = parse_kat_file(kat_path).expect("Failed to parse KAT file");
    assert!(!vectors.is_empty(), "No KAT vectors found");

    let vector = &vectors[0];

    // Test with invalid public key length
    let invalid_pk = MLKEMPublicKey(vec![0; 100]);
    let result = MLKEM768::encapsulate(&invalid_pk);
    assert!(result.is_err(), "Encapsulation with invalid public key should fail");

    // Test with invalid secret key length
    let invalid_sk = MLKEMSecretKey(vec![0; 100]);
    let result = MLKEM768::decapsulate(&invalid_sk, vector.ciphertext.as_slice());
    assert!(result.is_err(), "Decapsulation with invalid secret key should fail");
}
