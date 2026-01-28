use aegis_crypto_core::{
    kyber::{ kyber_keygen, kyber_encapsulate, kyber_decapsulate },
    falcon::{ falcon_keygen, falcon_sign, falcon_verify },
    hash::sha3_256_hash,
    utils::bytes_to_hex,
};
use std::collections::HashMap;
use std::time::{ SystemTime, UNIX_EPOCH };

/// Represents a financial account with PQC security
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct FinancialAccount {
    account_id: String,
    account_type: AccountType,
    balance: f64,
    currency: String,
    owner_name: String,
    status: AccountStatus,
    kyber_public_key: Vec<u8>,
    kyber_secret_key: Vec<u8>,
    falcon_public_key: Vec<u8>,
    falcon_secret_key: Vec<u8>,
    created_at: u64,
    last_transaction: u64,
}

/// Types of financial accounts
#[derive(Clone, Debug)]
enum AccountType {
    Checking,
    Savings,
    Investment,
    Business,
}

/// Account status
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
enum AccountStatus {
    Active,
    Suspended,
    Closed,
    UnderReview,
}

/// Secure financial transaction
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct SecureTransaction {
    transaction_id: String,
    from_account: String,
    to_account: String,
    amount: f64,
    currency: String,
    transaction_type: TransactionType,
    description: String,
    falcon_signature: Vec<u8>,
    kem_ciphertext: Vec<u8>,
    timestamp: u64,
    risk_score: f64,
}

/// Types of financial transactions
#[derive(Clone, Debug)]
#[allow(dead_code)]
enum TransactionType {
    Transfer,
    Payment,
    Withdrawal,
    Deposit,
    Investment,
}

/// Financial Security System
struct FinancialSecuritySystem {
    accounts: HashMap<String, FinancialAccount>,
    transactions: Vec<SecureTransaction>,
    system_id: String,
    system_kyber_keys: (Vec<u8>, Vec<u8>), // (public, secret)
    #[allow(dead_code)]
    system_falcon_keys: (Vec<u8>, Vec<u8>), // (public, secret)
    fraud_detection_rules: Vec<FraudRule>,
}

/// Fraud detection rule
#[derive(Clone, Debug)]
struct FraudRule {
    rule_id: String,
    description: String,
    threshold: f64,
    risk_multiplier: f64,
}

impl FinancialSecuritySystem {
    fn new() -> Self {
        // Generate system keys
        let kyber_keys = kyber_keygen();
        let falcon_keys = falcon_keygen();

        // Initialize fraud detection rules
        let fraud_rules = vec![
            FraudRule {
                rule_id: "RULE_001".to_string(),
                description: "Large transaction amount".to_string(),
                threshold: 10000.0,
                risk_multiplier: 2.0,
            },
            FraudRule {
                rule_id: "RULE_002".to_string(),
                description: "High frequency transactions".to_string(),
                threshold: 5.0, // transactions per hour
                risk_multiplier: 1.5,
            },
            FraudRule {
                rule_id: "RULE_003".to_string(),
                description: "Unusual transaction time".to_string(),
                threshold: 0.0, // outside business hours
                risk_multiplier: 1.3,
            }
        ];

        Self {
            accounts: HashMap::new(),
            transactions: Vec::new(),
            system_id: "FINANCIAL_SYSTEM_001".to_string(),
            system_kyber_keys: (kyber_keys.public_key(), kyber_keys.secret_key()),
            system_falcon_keys: (falcon_keys.public_key(), falcon_keys.secret_key()),
            fraud_detection_rules: fraud_rules,
        }
    }

    /// Create a new financial account with PQC security
    fn create_account(
        &mut self,
        account_id: String,
        account_type: AccountType,
        owner_name: String,
        currency: String,
        initial_balance: f64
    ) -> Result<(), String> {
        println!("🔐 Creating PQC-secured financial account: {}", account_id);
        println!("   👤 Owner: {}", owner_name);
        println!(
            "   💰 Type: {:?} | Currency: {} | Balance: {}",
            account_type,
            currency,
            initial_balance
        );

        // Generate PQC keypairs for the account
        let kyber_keys = kyber_keygen();
        let falcon_keys = falcon_keygen();

        println!(
            "   ✅ Kyber KEM keys generated (Public: {} bytes, Secret: {} bytes)",
            kyber_keys.public_key().len(),
            kyber_keys.secret_key().len()
        );
        println!(
            "   ✅ Falcon signature keys generated (Public: {} bytes, Secret: {} bytes)",
            falcon_keys.public_key().len(),
            falcon_keys.secret_key().len()
        );

        let account = FinancialAccount {
            account_id: account_id.clone(),
            account_type,
            balance: initial_balance,
            currency,
            owner_name,
            status: AccountStatus::Active,
            kyber_public_key: kyber_keys.public_key(),
            kyber_secret_key: kyber_keys.secret_key(),
            falcon_public_key: falcon_keys.public_key(),
            falcon_secret_key: falcon_keys.secret_key(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            last_transaction: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };

        self.accounts.insert(account_id, account);
        println!("   🎉 Account created successfully!");
        Ok(())
    }

    /// Execute a secure financial transaction
    fn execute_transaction(
        &mut self,
        from_account_id: &str,
        to_account_id: &str,
        amount: f64,
        description: &str
    ) -> Result<String, String> {
        let from_account = self.accounts.get(from_account_id).ok_or("From account not found")?;
        let to_account = self.accounts.get(to_account_id).ok_or("To account not found")?;

        if from_account.balance < amount {
            return Err("Insufficient funds".to_string());
        }

        if from_account.currency != to_account.currency {
            return Err("Currency mismatch".to_string());
        }

        println!("💸 Executing secure transaction...");
        println!("   📤 From: {} ({})", from_account.owner_name, from_account_id);
        println!("   📥 To: {} ({})", to_account.owner_name, to_account_id);
        println!("   💰 Amount: {} {}", amount, from_account.currency);
        println!("   📝 Description: {}", description);

        // Step 1: Perform Kyber KEM encapsulation
        println!("   🔑 Performing Kyber KEM encapsulation...");
        let encaps_result = kyber_encapsulate(&self.system_kyber_keys.0).map_err(|e|
            format!("KEM encapsulation failed: {:?}", e)
        )?;
        let shared_secret = encaps_result.shared_secret();
        let kem_ciphertext = encaps_result.ciphertext();
        println!("   ✅ KEM encapsulation complete (Shared secret: {} bytes)", shared_secret.len());

        // Step 2: Create transaction data for signing
        println!("   🖊️  Creating transaction hash for digital signature...");
        let transaction_data = format!(
            "{}:{}:{}:{}:{}:{}:{}",
            from_account_id,
            to_account_id,
            amount,
            from_account.currency,
            description,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            shared_secret.len()
        );
        let transaction_hash = sha3_256_hash(transaction_data.as_bytes());
        println!("   ✅ Transaction hash created: {}", bytes_to_hex(&transaction_hash));

        // Step 3: Sign transaction with Falcon
        println!("   ✍️  Signing transaction with Falcon...");
        let falcon_signature = falcon_sign(&from_account.falcon_secret_key, &transaction_hash);
        println!("   ✅ Falcon signature created ({} bytes)", falcon_signature.len());

        // Step 4: Calculate risk score
        let risk_score = self.calculate_risk_score(from_account_id, amount);
        println!("   ⚠️  Risk score calculated: {:.2}", risk_score);

        // Create secure transaction
        let transaction_id = format!("TX_{}", bytes_to_hex(&transaction_hash[..8]));
        let secure_transaction = SecureTransaction {
            transaction_id: transaction_id.clone(),
            from_account: from_account_id.to_string(),
            to_account: to_account_id.to_string(),
            amount,
            currency: from_account.currency.clone(),
            transaction_type: TransactionType::Transfer,
            description: description.to_string(),
            falcon_signature,
            kem_ciphertext,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            risk_score,
        };

        self.transactions.push(secure_transaction);

        // Update account balances
        if let Some(account) = self.accounts.get_mut(from_account_id) {
            account.balance -= amount;
            account.last_transaction = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
        if let Some(account) = self.accounts.get_mut(to_account_id) {
            account.balance += amount;
            account.last_transaction = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }

        println!("   🎉 Transaction executed successfully!");
        println!("   🆔 Transaction ID: {}", transaction_id);
        Ok(transaction_id)
    }

    /// Verify a transaction signature
    fn verify_transaction(&self, transaction_index: usize) -> Result<bool, String> {
        let transaction = self.transactions.get(transaction_index).ok_or("Transaction not found")?;
        let from_account = self.accounts
            .get(&transaction.from_account)
            .ok_or("From account not found")?;

        println!("🔍 Verifying transaction signature...");
        println!("   🆔 Transaction ID: {}", transaction.transaction_id);
        println!("   📤 From: {} ({})", from_account.owner_name, transaction.from_account);
        println!("   💰 Amount: {} {}", transaction.amount, transaction.currency);

        // Step 1: Decapsulate shared secret using Kyber
        println!("   🔑 Performing Kyber KEM decapsulation...");
        let shared_secret = kyber_decapsulate(
            &self.system_kyber_keys.1,
            &transaction.kem_ciphertext
        ).map_err(|e| format!("KEM decapsulation failed: {:?}", e))?;
        println!("   ✅ KEM decapsulation complete (Shared secret: {} bytes)", shared_secret.len());

        // Step 2: Recreate transaction hash
        println!("   🖊️  Recreating transaction hash...");
        let transaction_data = format!(
            "{}:{}:{}:{}:{}:{}:{}",
            transaction.from_account,
            transaction.to_account,
            transaction.amount,
            transaction.currency,
            transaction.description,
            transaction.timestamp,
            shared_secret.len()
        );
        let transaction_hash = sha3_256_hash(transaction_data.as_bytes());
        println!("   ✅ Transaction hash recreated: {}", bytes_to_hex(&transaction_hash));

        // Step 3: Verify Falcon signature
        println!("   ✍️  Verifying Falcon signature...");
        let falcon_valid = falcon_verify(
            &from_account.falcon_public_key,
            &transaction_hash,
            &transaction.falcon_signature
        );
        println!("   ✅ Falcon signature verification: {}", if falcon_valid {
            "PASSED"
        } else {
            "FAILED"
        });

        if !falcon_valid {
            return Err("Transaction signature verification failed".to_string());
        }

        println!("   🎉 Transaction verified successfully!");
        Ok(true)
    }

    /// Calculate risk score for a transaction
    fn calculate_risk_score(&self, account_id: &str, amount: f64) -> f64 {
        let mut risk_score = 1.0;

        // Apply fraud detection rules
        for rule in &self.fraud_detection_rules {
            match rule.rule_id.as_str() {
                "RULE_001" => {
                    if amount > rule.threshold {
                        risk_score *= rule.risk_multiplier;
                    }
                }
                "RULE_002" => {
                    // Count recent transactions for this account
                    let recent_count = self.transactions
                        .iter()
                        .filter(|tx| tx.from_account == account_id)
                        .filter(|tx| {
                            let now = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs();
                            now - tx.timestamp < 3600 // Last hour
                        })
                        .count() as f64;

                    if recent_count > rule.threshold {
                        risk_score *= rule.risk_multiplier;
                    }
                }
                "RULE_003" => {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                    let hour = (now / 3600) % 24;
                    if !(6..=22).contains(&hour) {
                        // Outside 6 AM - 10 PM
                        risk_score *= rule.risk_multiplier;
                    }
                }
                _ => {}
            }
        }

        risk_score
    }

    /// Display system status
    fn display_status(&self) {
        let separator = "=".repeat(60);
        println!("\n{}", separator);
        println!("💰 FINANCIAL SECURITY SYSTEM STATUS");
        println!("{}", separator);
        println!("🏦 System ID: {}", self.system_id);
        println!("👤 Active Accounts: {}", self.accounts.len());
        println!("💸 Total Transactions: {}", self.transactions.len());

        if !self.accounts.is_empty() {
            println!("\n👤 Financial Accounts:");
            for (id, account) in &self.accounts {
                let status_icon = match account.status {
                    AccountStatus::Active => "🟢",
                    AccountStatus::Suspended => "🟡",
                    AccountStatus::Closed => "🔴",
                    AccountStatus::UnderReview => "⚠️",
                };
                println!(
                    "   {} {} ({}) - {:?}",
                    status_icon,
                    account.owner_name,
                    id,
                    account.account_type
                );
                println!(
                    "     💰 {} {} | 🔑 Falcon: {} bytes | 🔐 Kyber: {} bytes",
                    account.balance,
                    account.currency,
                    account.falcon_public_key.len(),
                    account.kyber_public_key.len()
                );
            }
        }

        if !self.transactions.is_empty() {
            println!("\n💸 Recent Transactions:");
            let mut recent_txs: Vec<_> = self.transactions.iter().collect();
            recent_txs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            for (i, tx) in recent_txs.iter().take(5).enumerate() {
                let from_name = self.accounts
                    .get(&tx.from_account)
                    .map(|a| &a.owner_name)
                    .unwrap_or(&tx.from_account);
                let to_name = self.accounts
                    .get(&tx.to_account)
                    .map(|a| &a.owner_name)
                    .unwrap_or(&tx.to_account);
                println!(
                    "   {}. {} → {} ({} {}) - Risk: {:.2}",
                    i + 1,
                    from_name,
                    to_name,
                    tx.amount,
                    tx.currency,
                    tx.risk_score
                );
            }
        }

        println!("\n🛡️  Fraud Detection Rules:");
        for rule in &self.fraud_detection_rules {
            println!(
                "   • {}: {} (Threshold: {}, Risk Multiplier: {})",
                rule.rule_id,
                rule.description,
                rule.threshold,
                rule.risk_multiplier
            );
        }
        println!("{}", separator);
    }
}

fn main() {
    println!("🚀 AEGIS FINANCIAL SECURITY SYSTEM DEMO");
    println!("💰 Post-Quantum Cryptography for Financial Transactions");
    let separator = "=".repeat(50);
    println!("{}", separator);

    let mut system = FinancialSecuritySystem::new();

    // Create financial accounts
    println!("\n🏦 Creating PQC-secured financial accounts...");
    system
        .create_account(
            "ACC_001".to_string(),
            AccountType::Checking,
            "Alice Johnson".to_string(),
            "USD".to_string(),
            50000.0
        )
        .unwrap();

    system
        .create_account(
            "ACC_002".to_string(),
            AccountType::Savings,
            "Bob Smith".to_string(),
            "USD".to_string(),
            100000.0
        )
        .unwrap();

    system
        .create_account(
            "ACC_003".to_string(),
            AccountType::Business,
            "Charlie Brown Corp".to_string(),
            "USD".to_string(),
            250000.0
        )
        .unwrap();

    system
        .create_account(
            "ACC_004".to_string(),
            AccountType::Investment,
            "David Wilson".to_string(),
            "USD".to_string(),
            75000.0
        )
        .unwrap();

    // Display initial status
    system.display_status();

    // Execute secure transactions
    println!("\n💸 Executing secure financial transactions...");
    let _tx1 = system
        .execute_transaction("ACC_001", "ACC_002", 5000.0, "Monthly savings transfer")
        .unwrap();

    let _tx2 = system
        .execute_transaction("ACC_003", "ACC_001", 15000.0, "Business payment for services")
        .unwrap();

    let _tx3 = system
        .execute_transaction("ACC_002", "ACC_004", 10000.0, "Investment contribution")
        .unwrap();

    let _tx4 = system
        .execute_transaction("ACC_001", "ACC_003", 2500.0, "Equipment purchase")
        .unwrap();

    // Verify transactions
    println!("\n🔍 Verifying transaction signatures...");
    system.verify_transaction(0).unwrap(); // First transaction
    system.verify_transaction(1).unwrap(); // Second transaction
    system.verify_transaction(2).unwrap(); // Third transaction
    system.verify_transaction(3).unwrap(); // Fourth transaction

    // Display final status
    system.display_status();

    println!("\n🎉 Demo completed successfully!");
    println!("💰 Financial security system using post-quantum cryptography!");
    println!("✅ Kyber KEM for secure key exchange");
    println!("✅ Falcon digital signatures for transaction authentication");
    println!("✅ Fraud detection and risk scoring");
    println!("✅ Secure financial transaction processing");
}
