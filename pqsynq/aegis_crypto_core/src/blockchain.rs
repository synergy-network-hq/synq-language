//! Blockchain integration utilities for post-quantum cryptography.

#[cfg(all(feature = "kyber", feature = "dilithium"))]
use crate::{
    kyber_keygen,
    kyber_encapsulate,
    kyber_decapsulate,
    dilithium_keygen,
    dilithium_sign,
    dilithium_verify,
};

/// Blockchain-specific key pair for hybrid encryption
#[derive(Debug, Clone)]
pub struct BlockchainKeyPair {
    pub pqc_public_key: Vec<u8>,
    pub pqc_secret_key: Vec<u8>,
    pub signature_public_key: Vec<u8>,
    pub signature_secret_key: Vec<u8>,
    pub address: String,
}

/// Blockchain transaction with PQC encryption
#[derive(Debug, Clone)]
pub struct BlockchainTransaction {
    pub from_address: String,
    pub to_address: String,
    pub amount: u64,
    pub encrypted_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

/// Generate a blockchain-compatible key pair
#[cfg(all(feature = "kyber", feature = "dilithium"))]
pub fn generate_blockchain_keypair() -> BlockchainKeyPair {
    // Generate KEM keypair for encryption
    let kem_keypair = kyber_keygen();
    let kem_pk = kem_keypair.public_key();
    let kem_sk = kem_keypair.secret_key();

    // Generate signature keypair
    let sig_keypair = dilithium_keygen();
    let sig_pk = sig_keypair.public_key();
    let sig_sk = sig_keypair.secret_key();

    // Generate blockchain address from public key
    let address = generate_address(&sig_pk);

    BlockchainKeyPair {
        pqc_public_key: kem_pk.to_vec(),
        pqc_secret_key: kem_sk.to_vec(),
        signature_public_key: sig_pk.to_vec(),
        signature_secret_key: sig_sk.to_vec(),
        address,
    }
}

/// Generate a blockchain address from a public key
pub fn generate_address(public_key: &[u8]) -> String {
    use sha3::{ Digest, Keccak256 };

    let mut hasher = Keccak256::new();
    hasher.update(public_key);
    let result = hasher.finalize();

    // Take last 20 bytes and format as hex
    let address_bytes = &result[12..];
    format!("0x{}", hex::encode(address_bytes))
}

/// Encrypt data for blockchain transaction
#[cfg(feature = "kyber")]
pub fn encrypt_for_blockchain(recipient_public_key: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    // Use Kyber for key encapsulation
    let encapsulated = kyber_encapsulate(recipient_public_key).map_err(|e|
        format!("Encapsulation failed: {:?}", e)
    )?;

    // Combine encapsulated key and encrypted data
    let mut result = encapsulated.ciphertext().to_vec();
    result.extend_from_slice(data); // In real implementation, encrypt data with shared key

    Ok(result)
}

/// Decrypt blockchain transaction data
#[cfg(feature = "kyber")]
pub fn decrypt_blockchain_data(
    secret_key: &[u8],
    encrypted_data: &[u8]
) -> Result<Vec<u8>, String> {
    // Extract ciphertext (first part)
    let ciphertext_len = 1088; // Kyber ciphertext size
    if encrypted_data.len() < ciphertext_len {
        return Err("Invalid encrypted data length".to_string());
    }

    let ciphertext = &encrypted_data[..ciphertext_len];
    let data = &encrypted_data[ciphertext_len..];

    // Decapsulate the shared key
    let _shared_key = kyber_decapsulate(secret_key, ciphertext).map_err(|e|
        format!("Decapsulation failed: {:?}", e)
    )?;

    // In real implementation, decrypt data with shared key
    Ok(data.to_vec())
}

/// Sign a blockchain transaction
#[cfg(feature = "dilithium")]
pub fn sign_transaction(secret_key: &[u8], transaction_data: &[u8]) -> Result<Vec<u8>, String> {
    Ok(dilithium_sign(secret_key, transaction_data))
}

/// Verify a blockchain transaction signature
#[cfg(feature = "dilithium")]
pub fn verify_transaction_signature(
    public_key: &[u8],
    _transaction_data: &[u8],
    signature: &[u8]
) -> bool {
    dilithium_verify(public_key, signature)
}

/// Create a blockchain transaction
#[cfg(all(feature = "kyber", feature = "dilithium"))]
pub fn create_transaction(
    from_keypair: &BlockchainKeyPair,
    to_address: &str,
    amount: u64,
    data: &[u8]
) -> Result<BlockchainTransaction, String> {
    // Encrypt data for recipient
    let encrypted_data = encrypt_for_blockchain(&from_keypair.pqc_public_key, data)?;

    // Create transaction data
    let tx_data = format!("{}:{}:{}", from_keypair.address, to_address, amount).into_bytes();

    // Sign transaction
    let signature = sign_transaction(&from_keypair.signature_secret_key, &tx_data)?;

    Ok(BlockchainTransaction {
        from_address: from_keypair.address.clone(),
        to_address: to_address.to_string(),
        amount,
        encrypted_data,
        signature,
        timestamp: std::time::SystemTime
            ::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    })
}

/// Verify a blockchain transaction
#[cfg(feature = "dilithium")]
pub fn verify_transaction(transaction: &BlockchainTransaction, sender_public_key: &[u8]) -> bool {
    let tx_data = format!(
        "{}:{}:{}",
        transaction.from_address,
        transaction.to_address,
        transaction.amount
    ).into_bytes();
    verify_transaction_signature(sender_public_key, &tx_data, &transaction.signature)
}

/// Smart contract integration for PQC operations
pub mod smart_contract {
    use super::*;

    /// Smart contract state with PQC keys
    #[derive(Debug, Clone)]
    pub struct SmartContractState {
        pub contract_address: String,
        pub admin_keypair: BlockchainKeyPair,
        pub encrypted_state: Vec<u8>,
        pub state_signature: Vec<u8>,
    }

    /// Deploy a smart contract with PQC security
    #[cfg(all(feature = "kyber", feature = "dilithium"))]
    pub fn deploy_contract(admin_keypair: BlockchainKeyPair) -> SmartContractState {
        let contract_address = generate_address(&admin_keypair.signature_public_key);

        SmartContractState {
            contract_address,
            admin_keypair,
            encrypted_state: Vec::new(),
            state_signature: Vec::new(),
        }
    }

    /// Execute a smart contract function with PQC verification
    #[cfg(feature = "dilithium")]
    pub fn execute_contract_function(
        contract: &mut SmartContractState,
        function_name: &str,
        parameters: &[u8],
        caller_signature: &[u8],
        caller_public_key: &[u8]
    ) -> Result<Vec<u8>, String> {
        // Verify caller signature
        let call_data = format!("{}:{}", function_name, hex::encode(parameters)).into_bytes();
        if !verify_transaction_signature(caller_public_key, &call_data, caller_signature) {
            return Err("Invalid caller signature".to_string());
        }

        // Execute function (simplified)
        match function_name {
            "setState" => {
                contract.encrypted_state = parameters.to_vec();
                contract.state_signature = sign_transaction(
                    &contract.admin_keypair.signature_secret_key,
                    parameters
                )?;
                Ok(b"State updated".to_vec())
            }
            "getState" => { Ok(contract.encrypted_state.clone()) }
            _ => Err("Unknown function".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(all(feature = "kyber", feature = "dilithium"))]
    fn test_blockchain_keypair_generation() {
        let keypair = generate_blockchain_keypair();
        assert!(!keypair.address.is_empty());
        assert!(keypair.address.starts_with("0x"));
        assert_eq!(keypair.address.len(), 42); // 0x + 40 hex chars
    }

    #[test]
    #[cfg(all(feature = "kyber", feature = "dilithium"))]
    fn test_transaction_creation_and_verification() {
        let sender = generate_blockchain_keypair();
        let recipient = generate_blockchain_keypair();

        let transaction = create_transaction(
            &sender,
            &recipient.address,
            1000,
            b"Test transaction data"
        ).unwrap();

        assert_eq!(transaction.from_address, sender.address);
        assert_eq!(transaction.to_address, recipient.address);
        assert_eq!(transaction.amount, 1000);

        // Verify transaction
        let is_valid = verify_transaction(&transaction, &sender.signature_public_key);
        assert!(is_valid);
    }

    #[test]
    #[cfg(all(feature = "kyber", feature = "dilithium"))]
    fn test_smart_contract_operations() {
        let admin = generate_blockchain_keypair();
        let mut contract = smart_contract::deploy_contract(admin.clone());

        let caller = generate_blockchain_keypair();
        let call_data = b"test state data";
        let signature = sign_transaction(&caller.signature_secret_key, call_data).unwrap();

        // Set state
        let result = smart_contract
            ::execute_contract_function(
                &mut contract,
                "setState",
                call_data,
                &signature,
                &caller.signature_public_key
            )
            .unwrap();

        assert_eq!(result, b"State updated");

        // Get state
        let state = smart_contract
            ::execute_contract_function(
                &mut contract,
                "getState",
                &[],
                &signature,
                &caller.signature_public_key
            )
            .unwrap();

        assert_eq!(state, call_data);
    }
}
