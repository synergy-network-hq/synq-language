use aegis_crypto_core::{
    kyber::{ KyberKeyPair, KyberEncapsulation },
    dilithium::{ DilithiumKeyPair, DilithiumSignature },
    falcon::{ FalconKeyPair, FalconSignature },
    hash::sha3_256_hash,
    utils::bytes_to_hex,
};
use std::collections::HashMap;
use std::time::{ SystemTime, UNIX_EPOCH };

/// Represents a cryptocurrency wallet with PQC security
#[derive(Debug, Clone)]
struct CryptoWallet {
    _wallet_id: String,
    owner_name: String,
    public_address: String,
    balance: f64,
    currency: String,
    _created_at: u64,
    transaction_count: u32,
    security_level: WalletSecurityLevel,
}

/// Security levels for wallet operations
#[derive(Debug, Clone, PartialEq)]
enum WalletSecurityLevel {
    Standard, // Single PQC algorithm
    Enhanced, // Dual PQC algorithms
    Maximum, // Triple PQC algorithms
}

/// Wallet keypair with multiple PQC algorithms
#[derive(Debug)]
struct WalletKeypair {
    _wallet_id: String,
    kyber_keys: (Vec<u8>, Vec<u8>), // (public_key, secret_key)
    dilithium_keys: (Vec<u8>, Vec<u8>), // (public_key, secret_key)
    falcon_keys: (Vec<u8>, Vec<u8>), // (public_key, secret_key)
}

/// Cryptocurrency transaction
#[derive(Debug, Clone)]
struct Transaction {
    tx_id: String,
    from_wallet: String,
    to_wallet: String,
    amount: f64,
    currency: String,
    timestamp: u64,
    fee: f64,
    signature: Vec<u8>,
    signature_type: String,
    status: TransactionStatus,
}

/// Transaction status
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

/// Blockchain wallet management system
struct BlockchainWalletSystem {
    wallets: HashMap<String, CryptoWallet>,
    keypairs: HashMap<String, WalletKeypair>,
    transactions: HashMap<String, Transaction>,
    blockchain_state: BlockchainState,
}

/// Blockchain state for demonstration
#[derive(Debug)]
struct BlockchainState {
    block_height: u64,
    total_transactions: u64,
    total_value: f64,
    last_block_time: u64,
}

impl BlockchainWalletSystem {
    fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            keypairs: HashMap::new(),
            transactions: HashMap::new(),
            blockchain_state: BlockchainState {
                block_height: 1,
                total_transactions: 0,
                total_value: 0.0,
                last_block_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            },
        }
    }

    /// Create a new cryptocurrency wallet with PQC security
    fn create_wallet(
        &mut self,
        owner_name: String,
        currency: String,
        security_level: WalletSecurityLevel
    ) -> Result<String, String> {
        let wallet_id = self.generate_wallet_id(&owner_name);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        println!("🔐 Creating PQC-secured wallet for: {}", owner_name);
        println!("   💰 Currency: {}", currency);
        println!("   🛡️  Security Level: {:?}", security_level);

        // Generate PQC keypairs based on security level
        let kyber_keys = KyberKeyPair::generate();
        println!(
            "   ✅ Kyber KEM keys generated (Public: {} bytes, Secret: {} bytes)",
            kyber_keys.public_key().len(),
            kyber_keys.secret_key().len()
        );

        let dilithium_keys = DilithiumKeyPair::generate();
        println!(
            "   ✅ Dilithium signature keys generated (Public: {} bytes, Secret: {} bytes)",
            dilithium_keys.public_key().len(),
            dilithium_keys.secret_key().len()
        );

        let falcon_keys = FalconKeyPair::generate();
        println!(
            "   ✅ Falcon signature keys generated (Public: {} bytes, Secret: {} bytes)",
            falcon_keys.public_key().len(),
            falcon_keys.secret_key().len()
        );

        // Create wallet
        let wallet = CryptoWallet {
            _wallet_id: wallet_id.clone(),
            owner_name: owner_name.clone(),
            public_address: self.generate_public_address(&kyber_keys.public_key()),
            balance: 0.0,
            currency,
            _created_at: now,
            transaction_count: 0,
            security_level,
        };

        // Create keypair
        let keypair = WalletKeypair {
            _wallet_id: wallet_id.clone(),
            kyber_keys: (kyber_keys.public_key(), kyber_keys.secret_key()),
            dilithium_keys: (dilithium_keys.public_key(), dilithium_keys.secret_key()),
            falcon_keys: (falcon_keys.public_key(), falcon_keys.secret_key()),
        };

        self.wallets.insert(wallet_id.clone(), wallet);
        self.keypairs.insert(wallet_id.clone(), keypair);

        println!("   🎉 Wallet created successfully!");
        println!("   🆔 Wallet ID: {}", wallet_id);
        println!("   📍 Public Address: {}", self.wallets.get(&wallet_id).unwrap().public_address);

        Ok(wallet_id)
    }

    /// Send cryptocurrency from one wallet to another
    fn send_transaction(
        &mut self,
        from_wallet_id: &str,
        to_wallet_id: &str,
        amount: f64,
        fee: f64
    ) -> Result<String, String> {
        let from_wallet = self.wallets.get(from_wallet_id).ok_or("Sender wallet not found")?;

        let to_wallet = self.wallets.get(to_wallet_id).ok_or("Recipient wallet not found")?;

        if from_wallet.balance < amount + fee {
            return Err("Insufficient balance".to_string());
        }

        if from_wallet.currency != to_wallet.currency {
            return Err("Currency mismatch".to_string());
        }

        println!(
            "\n💰 Sending {} {} from {} to {}...",
            amount,
            from_wallet.currency,
            from_wallet.owner_name,
            to_wallet.owner_name
        );

        // Create transaction
        let tx_id = self.generate_transaction_id();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Create transaction data for signing
        let tx_data = format!(
            "{}:{}:{}:{}:{}:{}",
            tx_id,
            from_wallet_id,
            to_wallet_id,
            amount,
            fee,
            now
        );
        let tx_hash = sha3_256_hash(tx_data.as_bytes());

        // Sign transaction based on security level
        let (signature, signature_type) = match from_wallet.security_level {
            WalletSecurityLevel::Standard => {
                let keypair = self.keypairs.get(from_wallet_id).unwrap();
                let dilithium_keypair = DilithiumKeyPair::from_keys(
                    keypair.dilithium_keys.0.clone(),
                    keypair.dilithium_keys.1.clone()
                );
                let sig = dilithium_keypair.sign(&tx_hash);
                (sig.signature().to_vec(), "Dilithium".to_string())
            }
            WalletSecurityLevel::Enhanced => {
                let keypair = self.keypairs.get(from_wallet_id).unwrap();
                let dilithium_keypair = DilithiumKeyPair::from_keys(
                    keypair.dilithium_keys.0.clone(),
                    keypair.dilithium_keys.1.clone()
                );
                let falcon_keypair = FalconKeyPair::from_keys(
                    keypair.falcon_keys.0.clone(),
                    keypair.falcon_keys.1.clone()
                );
                let sig1 = dilithium_keypair.sign(&tx_hash);
                let sig2 = falcon_keypair.sign(&tx_hash);
                // Combine signatures (simplified)
                let combined = [sig1.signature(), sig2.signature()].concat();
                (combined, "Dual (Dilithium + Falcon)".to_string())
            }
            WalletSecurityLevel::Maximum => {
                let keypair = self.keypairs.get(from_wallet_id).unwrap();
                let dilithium_keypair = DilithiumKeyPair::from_keys(
                    keypair.dilithium_keys.0.clone(),
                    keypair.dilithium_keys.1.clone()
                );
                let falcon_keypair = FalconKeyPair::from_keys(
                    keypair.falcon_keys.0.clone(),
                    keypair.falcon_keys.1.clone()
                );
                let kyber_keypair = KyberKeyPair::from_keys(
                    keypair.kyber_keys.0.clone(),
                    keypair.kyber_keys.1.clone()
                );
                let sig1 = dilithium_keypair.sign(&tx_hash);
                let sig2 = falcon_keypair.sign(&tx_hash);
                // Use Kyber for additional security
                let encaps = kyber_keypair.encapsulate().map_err(|e|
                    format!("Kyber encapsulation failed: {:?}", e)
                )?;
                let combined = [sig1.signature(), sig2.signature(), encaps.ciphertext()].concat();
                (combined, "Triple (Dilithium + Falcon + Kyber)".to_string())
            }
        };

        println!("   ✍️  Transaction signed with {} ({} bytes)", signature_type, signature.len());

        // Create transaction
        let transaction = Transaction {
            tx_id: tx_id.clone(),
            from_wallet: from_wallet_id.to_string(),
            to_wallet: to_wallet_id.to_string(),
            amount,
            currency: from_wallet.currency.clone(),
            timestamp: now,
            fee,
            signature,
            signature_type,
            status: TransactionStatus::Pending,
        };

        // Store currency before mutable operations
        let currency = from_wallet.currency.clone();

        // Add transaction to blockchain first
        self.transactions.insert(tx_id.clone(), transaction);
        self.blockchain_state.total_transactions += 1;
        self.blockchain_state.total_value += amount;

        // Update wallet balances
        if let Some(wallet) = self.wallets.get_mut(from_wallet_id) {
            wallet.balance -= amount + fee;
            wallet.transaction_count += 1;
        }

        if let Some(wallet) = self.wallets.get_mut(to_wallet_id) {
            wallet.balance += amount;
        }

        // Simulate blockchain confirmation
        self.simulate_block_confirmation(&tx_id);

        println!("   🎉 Transaction sent successfully!");
        println!("   🆔 Transaction ID: {}", tx_id);
        println!("   💰 Amount: {} {}", amount, currency);
        println!("   💸 Fee: {} {}", fee, currency);

        Ok(tx_id)
    }

    /// Verify a transaction signature
    fn verify_transaction(&self, tx_id: &str) -> bool {
        let transaction = match self.transactions.get(tx_id) {
            Some(tx) => tx,
            None => {
                return false;
            }
        };

        if !self.wallets.contains_key(&transaction.from_wallet) {
            return false;
        }

        let keypair = match self.keypairs.get(&transaction.from_wallet) {
            Some(keys) => keys,
            None => {
                return false;
            }
        };

        println!("\n🔍 Verifying transaction: {}", tx_id);

        // Recreate transaction hash
        let tx_data = format!(
            "{}:{}:{}:{}:{}:{}",
            transaction.tx_id,
            transaction.from_wallet,
            transaction.to_wallet,
            transaction.amount,
            transaction.fee,
            transaction.timestamp
        );
        let tx_hash = sha3_256_hash(tx_data.as_bytes());

        // Verify signature based on type
        let valid = match transaction.signature_type.as_str() {
            "Dilithium" => {
                let dilithium_keypair = DilithiumKeyPair::from_keys(
                    keypair.dilithium_keys.0.clone(),
                    keypair.dilithium_keys.1.clone()
                );
                let sig = DilithiumSignature::from_bytes(&transaction.signature);
                dilithium_keypair.verify(&tx_hash, &sig)
            }
            "Dual (Dilithium + Falcon)" => {
                // Split combined signature (simplified)
                let sig_len = transaction.signature.len() / 2;
                let dilithium_sig_bytes = &transaction.signature[..sig_len];
                let falcon_sig_bytes = &transaction.signature[sig_len..];

                let dilithium_keypair = DilithiumKeyPair::from_keys(
                    keypair.dilithium_keys.0.clone(),
                    keypair.dilithium_keys.1.clone()
                );
                let falcon_keypair = FalconKeyPair::from_keys(
                    keypair.falcon_keys.0.clone(),
                    keypair.falcon_keys.1.clone()
                );

                let dilithium_sig = DilithiumSignature::from_bytes(dilithium_sig_bytes);
                let falcon_sig = FalconSignature::from_bytes(falcon_sig_bytes);

                let dilithium_valid = dilithium_keypair.verify(&tx_hash, &dilithium_sig);
                let falcon_valid = falcon_keypair.verify(&tx_hash, &falcon_sig);

                dilithium_valid && falcon_valid
            }
            "Triple (Dilithium + Falcon + Kyber)" => {
                // Split combined signature (simplified)
                let sig_len = transaction.signature.len() / 3;
                let dilithium_sig_bytes = &transaction.signature[..sig_len];
                let falcon_sig_bytes = &transaction.signature[sig_len..2 * sig_len];

                let dilithium_keypair = DilithiumKeyPair::from_keys(
                    keypair.dilithium_keys.0.clone(),
                    keypair.dilithium_keys.1.clone()
                );
                let falcon_keypair = FalconKeyPair::from_keys(
                    keypair.falcon_keys.0.clone(),
                    keypair.falcon_keys.1.clone()
                );

                let dilithium_sig = DilithiumSignature::from_bytes(dilithium_sig_bytes);
                let falcon_sig = FalconSignature::from_bytes(falcon_sig_bytes);

                let dilithium_valid = dilithium_keypair.verify(&tx_hash, &dilithium_sig);
                let falcon_valid = falcon_keypair.verify(&tx_hash, &falcon_sig);

                dilithium_valid && falcon_valid
            }
            _ => false,
        };

        println!(
            "   {} Transaction signature verification: {}",
            if valid {
                "✅"
            } else {
                "❌"
            },
            if valid {
                "PASSED"
            } else {
                "FAILED"
            }
        );

        valid
    }

    /// Add funds to a wallet (mining reward simulation)
    fn add_funds(&mut self, wallet_id: &str, amount: f64) -> Result<(), String> {
        if let Some(wallet) = self.wallets.get_mut(wallet_id) {
            wallet.balance += amount;
            println!(
                "   💰 Added {} {} to {}'s wallet. New balance: {} {}",
                amount,
                wallet.currency,
                wallet.owner_name,
                wallet.balance,
                wallet.currency
            );
            Ok(())
        } else {
            Err("Wallet not found".to_string())
        }
    }

    /// Simulate blockchain confirmation
    fn simulate_block_confirmation(&mut self, tx_id: &str) {
        if let Some(transaction) = self.transactions.get_mut(tx_id) {
            transaction.status = TransactionStatus::Confirmed;
            self.blockchain_state.block_height += 1;
            self.blockchain_state.last_block_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }

    /// Generate a unique wallet ID
    fn generate_wallet_id(&self, owner_name: &str) -> String {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let hash_input = format!("{}:{}", owner_name, timestamp);
        let hash = sha3_256_hash(hash_input.as_bytes());
        format!("WALLET_{}", bytes_to_hex(&hash[..8]))
    }

    /// Generate a public address from public key
    fn generate_public_address(&self, public_key: &[u8]) -> String {
        let hash = sha3_256_hash(public_key);
        format!("0x{}", bytes_to_hex(&hash[..20]))
    }

    /// Generate a unique transaction ID
    fn generate_transaction_id(&self) -> String {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let random_data = format!("tx_{}", timestamp);
        let hash = sha3_256_hash(random_data.as_bytes());
        format!("TX_{}", bytes_to_hex(&hash[..16]))
    }

    /// Display system status
    fn display_status(&self) {
        let separator = "=".repeat(70);
        println!("\n{}", separator);
        println!("🔗 BLOCKCHAIN WALLET SYSTEM STATUS");
        println!("{}", separator);
        println!("👛 Wallets: {}", self.wallets.len());
        println!("💸 Transactions: {}", self.transactions.len());
        println!("⛓️  Blockchain Height: {}", self.blockchain_state.block_height);
        println!("💰 Total Value: {} (all currencies)", self.blockchain_state.total_value);

        if !self.wallets.is_empty() {
            println!("\n👛 Registered Wallets:");
            for (id, wallet) in &self.wallets {
                let status = if wallet.balance > 0.0 { "💰 FUNDED" } else { "💸 EMPTY" };
                println!(
                    "   • {} ({}) - {} - {} {}",
                    wallet.owner_name,
                    id,
                    status,
                    wallet.balance,
                    wallet.currency
                );
                println!(
                    "     📍 {} | 🔑 {:?} | 📊 {} transactions",
                    wallet.public_address,
                    wallet.security_level,
                    wallet.transaction_count
                );
            }
        }

        if !self.transactions.is_empty() {
            println!("\n💸 Recent Transactions:");
            let mut recent_txs: Vec<_> = self.transactions.iter().collect();
            recent_txs.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
            for (tx_id, tx) in recent_txs.iter().take(5) {
                let status_icon = match tx.status {
                    TransactionStatus::Confirmed => "✅",
                    TransactionStatus::Pending => "⏳",
                    TransactionStatus::Failed => "❌",
                };
                println!(
                    "   {} {} - {} {} → {} ({} {}) - {}",
                    status_icon,
                    tx_id,
                    tx.from_wallet,
                    tx.amount,
                    tx.currency,
                    tx.to_wallet,
                    tx.fee,
                    tx.currency
                );
            }
        }
        println!("{}", separator);
    }
}

fn main() {
    println!("🚀 AEGIS BLOCKCHAIN WALLET SECURITY DEMO");
    println!("🔗 Post-Quantum Cryptography for Cryptocurrency");
    let separator = "=".repeat(60);
    println!("{}", separator);

    let mut system = BlockchainWalletSystem::new();

    // Create wallets with different security levels
    println!("\n👛 Creating PQC-secured wallets...");
    let alice_wallet = system
        .create_wallet("Alice Johnson".to_string(), "BTC".to_string(), WalletSecurityLevel::Maximum)
        .unwrap();

    let bob_wallet = system
        .create_wallet("Bob Smith".to_string(), "BTC".to_string(), WalletSecurityLevel::Enhanced)
        .unwrap();

    let charlie_wallet = system
        .create_wallet(
            "Charlie Brown".to_string(),
            "ETH".to_string(),
            WalletSecurityLevel::Standard
        )
        .unwrap();

    // Add initial funds
    println!("\n💰 Adding initial funds...");
    system.add_funds(&alice_wallet, 10.0).unwrap();
    system.add_funds(&bob_wallet, 5.0).unwrap();
    system.add_funds(&charlie_wallet, 20.0).unwrap();

    // Display initial status
    system.display_status();

    // Send transactions
    println!("\n💸 Sending transactions...");
    let tx1 = system.send_transaction(&alice_wallet, &bob_wallet, 2.0, 0.001).unwrap();
    let tx2 = system.send_transaction(&bob_wallet, &alice_wallet, 1.0, 0.001).unwrap();
    // Note: Charlie's wallet is in ETH, so we can't send BTC to it
    // Instead, let's create another BTC wallet for demonstration
    let david_wallet = system
        .create_wallet("David Wilson".to_string(), "BTC".to_string(), WalletSecurityLevel::Standard)
        .unwrap();
    system.add_funds(&david_wallet, 3.0).unwrap();
    let tx3 = system.send_transaction(&alice_wallet, &david_wallet, 1.5, 0.001).unwrap();

    // Verify transactions
    println!("\n🔍 Verifying transactions...");
    system.verify_transaction(&tx1);
    system.verify_transaction(&tx2);
    system.verify_transaction(&tx3);

    // Display final status
    system.display_status();

    println!("\n🎉 Demo completed successfully!");
    println!("🔗 Blockchain wallet system using post-quantum cryptography!");
    println!("✅ Multi-algorithm PQC security");
    println!("✅ Transaction signing and verification");
    println!("✅ Blockchain state management");
}
