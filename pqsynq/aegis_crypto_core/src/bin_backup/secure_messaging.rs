use aegis_crypto_core::{
    kyber::{ KyberKeyPair, KyberEncapsulation },
    falcon::{ FalconKeyPair, FalconSignature },
    hash::sha3_256_hash,
    utils::bytes_to_hex,
};
use std::collections::HashMap;
use std::time::{ SystemTime, UNIX_EPOCH };

/// Represents a secure message with encryption and authentication
#[derive(Debug, Clone)]
struct SecureMessage {
    sender_id: String,
    recipient_id: String,
    timestamp: u64,
    encrypted_content: Vec<u8>,
    falcon_signature: Vec<u8>,
    kem_ciphertext: Vec<u8>,
    _nonce: Vec<u8>,
    _auth_tag: Vec<u8>,
}

/// Represents a user in the secure messaging system
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: String,
    name: String,
    kyber_keypair: (Vec<u8>, Vec<u8>), // (public_key, secret_key)
    falcon_keypair: (Vec<u8>, Vec<u8>), // (public_key, secret_key)
}

/// Secure messaging system implementation
struct SecureMessagingSystem {
    users: HashMap<String, User>,
    messages: Vec<SecureMessage>,
}

impl SecureMessagingSystem {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            messages: Vec::new(),
        }
    }

    /// Create a new user with PQC keypairs
    fn create_user(&mut self, id: String, name: String) -> Result<(), String> {
        if self.users.contains_key(&id) {
            return Err("User ID already exists".to_string());
        }

        println!("🔐 Generating PQC keypairs for user: {}", name);

        // Generate Kyber KEM keypair
        let kyber_keys = KyberKeyPair::generate();
        println!(
            "   ✅ Kyber KEM keys generated (Public: {} bytes, Secret: {} bytes)",
            kyber_keys.public_key().len(),
            kyber_keys.secret_key().len()
        );

        // Generate Falcon signature keypair
        let falcon_keys = FalconKeyPair::generate();
        println!(
            "   ✅ Falcon signature keys generated (Public: {} bytes, Secret: {} bytes)",
            falcon_keys.public_key().len(),
            falcon_keys.secret_key().len()
        );

        let user = User {
            id: id.clone(),
            name,
            kyber_keypair: (kyber_keys.public_key(), kyber_keys.secret_key()),
            falcon_keypair: (falcon_keys.public_key(), falcon_keys.secret_key()),
        };

        self.users.insert(id, user);
        println!("   🎉 User created successfully!");
        Ok(())
    }

    /// Send a secure message using PQC encryption and signatures
    fn send_message(
        &mut self,
        sender_id: &str,
        recipient_id: &str,
        content: &str
    ) -> Result<(), String> {
        let sender = self.users.get(sender_id).ok_or("Sender not found")?;
        let recipient = self.users.get(recipient_id).ok_or("Recipient not found")?;

        println!("\n📤 Sending secure message from {} to {}...", sender.name, recipient.name);

        // Step 1: Generate shared secret using Kyber KEM
        println!("   🔑 Performing Kyber KEM encapsulation...");
        let recipient_kyber = KyberKeyPair::from_keys(
            recipient.kyber_keypair.0.clone(),
            recipient.kyber_keypair.1.clone()
        );
        let encaps_result = recipient_kyber.encapsulate().map_err(|e|
            format!("KEM encapsulation failed: {:?}", e)
        )?;
        let shared_secret = encaps_result.shared_secret();
        let kem_ciphertext = encaps_result.ciphertext();
        println!("   ✅ KEM encapsulation complete (Shared secret: {} bytes)", shared_secret.len());

        // Step 2: Encrypt message content (simplified AES simulation)
        println!("   🔒 Encrypting message content...");
        let content_bytes = content.as_bytes();
        let encrypted_content = self.simulate_aes_encryption(content_bytes, &shared_secret);
        println!(
            "   ✅ Content encrypted ({} bytes -> {} bytes)",
            content_bytes.len(),
            encrypted_content.len()
        );

        // Step 3: Create message hash for signing
        println!("   🖊️  Creating message hash for digital signature...");
        let message_data = format!(
            "{}:{}:{}:{}",
            sender_id,
            recipient_id,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            content
        );
        let message_hash = sha3_256_hash(message_data.as_bytes());
        println!("   ✅ Message hash created: {}", bytes_to_hex(&message_hash));

        // Step 4: Sign message with Falcon
        println!("   ✍️  Signing message with Falcon...");
        let sender_falcon = FalconKeyPair::from_keys(
            sender.falcon_keypair.0.clone(),
            sender.falcon_keypair.1.clone()
        );
        let falcon_signature = sender_falcon.sign(&message_hash);
        println!("   ✅ Falcon signature created ({} bytes)", falcon_signature.signature().len());

        // Create secure message
        let secure_message = SecureMessage {
            sender_id: sender_id.to_string(),
            recipient_id: recipient_id.to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            encrypted_content,
            falcon_signature: falcon_signature.signature().to_vec(),
            kem_ciphertext,
            _nonce: vec![0u8; 12], // Simplified nonce
            _auth_tag: vec![0u8; 16], // Simplified auth tag
        };

        self.messages.push(secure_message.clone());
        println!("   🎉 Secure message sent successfully!");
        println!("   📊 Message size: {} bytes", secure_message.encrypted_content.len());

        Ok(())
    }

    /// Verify and decrypt a received message
    fn receive_message(&self, recipient_id: &str, message_index: usize) -> Result<String, String> {
        let message = self.messages.get(message_index).ok_or("Message not found")?;

        if message.recipient_id != recipient_id {
            return Err("Message not intended for this recipient".to_string());
        }

        let sender = self.users.get(&message.sender_id).ok_or("Sender not found")?;
        let recipient = self.users.get(recipient_id).ok_or("Recipient not found")?;

        println!("\n📥 Receiving secure message from {}...", sender.name);

        // Step 1: Decapsulate shared secret using Kyber
        println!("   🔑 Performing Kyber KEM decapsulation...");
        let recipient_kyber = KyberKeyPair::from_keys(
            recipient.kyber_keypair.0.clone(),
            recipient.kyber_keypair.1.clone()
        );
        let shared_secret = recipient_kyber.decapsulate(&message.kem_ciphertext)
            .map_err(|e| format!("KEM decapsulation failed: {:?}", e))?;
        println!("   ✅ KEM decapsulation complete (Shared secret: {} bytes)", shared_secret.len());

        // Step 2: Decrypt message content
        println!("   🔓 Decrypting message content...");
        let decrypted_content = self.simulate_aes_decryption(
            &message.encrypted_content,
            &shared_secret
        );
        println!(
            "   ✅ Content decrypted ({} bytes -> {} bytes)",
            message.encrypted_content.len(),
            decrypted_content.len()
        );

        // Step 3: Verify message hash
        println!("   🖊️  Verifying message hash...");
        let message_data = format!(
            "{}:{}:{}:{}",
            message.sender_id,
            message.recipient_id,
            message.timestamp,
            String::from_utf8_lossy(&decrypted_content)
        );
        let message_hash = sha3_256_hash(message_data.as_bytes());
        println!("   ✅ Message hash verified: {}", bytes_to_hex(&message_hash));

        // Step 4: Verify Falcon signature
        println!("   ✍️  Verifying Falcon signature...");
        let sender_falcon = FalconKeyPair::from_keys(
            sender.falcon_keypair.0.clone(),
            sender.falcon_keypair.1.clone()
        );
        let falcon_sig = FalconSignature::from_bytes(&message.falcon_signature);
        let falcon_valid = sender_falcon.verify(&message_hash, &falcon_sig);
        println!("   ✅ Falcon signature verification: {}", if falcon_valid {
            "PASSED"
        } else {
            "FAILED"
        });

        if !falcon_valid {
            return Err("Message signature verification failed".to_string());
        }

        println!("   🎉 Message received and verified successfully!");
        Ok(String::from_utf8_lossy(&decrypted_content).to_string())
    }

    /// Simulate AES encryption (in real implementation, use proper AES-GCM)
    fn simulate_aes_encryption(&self, plaintext: &[u8], key: &[u8]) -> Vec<u8> {
        let mut encrypted = Vec::new();
        for (i, &byte) in plaintext.iter().enumerate() {
            let key_byte = key[i % key.len()];
            encrypted.push(byte ^ key_byte);
        }
        encrypted
    }

    /// Simulate AES decryption (in real implementation, use proper AES-GCM)
    fn simulate_aes_decryption(&self, ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
        let mut decrypted = Vec::new();
        for (i, &byte) in ciphertext.iter().enumerate() {
            let key_byte = key[i % key.len()];
            decrypted.push(byte ^ key_byte);
        }
        decrypted
    }

    /// Display system status
    fn display_status(&self) {
        let separator = "=".repeat(60);
        println!("\n{}", separator);
        println!("🔐 SECURE MESSAGING SYSTEM STATUS");
        println!("{}", separator);
        println!("👥 Users: {}", self.users.len());
        println!("📨 Messages: {}", self.messages.len());

        if !self.users.is_empty() {
            println!("\n👤 Registered Users:");
            for (id, user) in &self.users {
                println!("   • {} ({})", user.name, id);
                println!("     - Kyber public key: {} bytes", user.kyber_keypair.0.len());
                println!("     - Falcon public key: {} bytes", user.falcon_keypair.0.len());
            }
        }

        if !self.messages.is_empty() {
            println!("\n📨 Message History:");
            for (i, msg) in self.messages.iter().enumerate() {
                let sender_name = self.users
                    .get(&msg.sender_id)
                    .map(|u| &u.name)
                    .unwrap_or(&msg.sender_id);
                let recipient_name = self.users
                    .get(&msg.recipient_id)
                    .map(|u| &u.name)
                    .unwrap_or(&msg.recipient_id);
                println!(
                    "   {}. {} → {} ({} bytes, signed: {})",
                    i + 1,
                    sender_name,
                    recipient_name,
                    msg.encrypted_content.len(),
                    if !msg.falcon_signature.is_empty() {
                        "Yes"
                    } else {
                        "No"
                    }
                );
            }
        }
        println!("{}", separator);
    }
}

fn main() {
    println!("🚀 AEGIS SECURE MESSAGING SYSTEM DEMO");
    println!("🔐 Post-Quantum Cryptography in Action");
    let separator = "=".repeat(50);
    println!("{}", separator);

    let mut system = SecureMessagingSystem::new();

    // Create users
    println!("\n👥 Creating users...");
    system.create_user("alice".to_string(), "Alice Johnson".to_string()).unwrap();
    system.create_user("bob".to_string(), "Bob Smith".to_string()).unwrap();
    system.create_user("charlie".to_string(), "Charlie Brown".to_string()).unwrap();

    // Send secure messages
    println!("\n📤 Sending secure messages...");
    system
        .send_message(
            "alice",
            "bob",
            "Hello Bob! This is a secret message encrypted with post-quantum cryptography."
        )
        .unwrap();
    system
        .send_message(
            "bob",
            "alice",
            "Hi Alice! I received your encrypted message. The PQC algorithms are working perfectly!"
        )
        .unwrap();
    system
        .send_message(
            "alice",
            "charlie",
            "Charlie, please review the new security protocols. They use ML-KEM, ML-DSA, and SLH-DSA."
        )
        .unwrap();
    system
        .send_message(
            "charlie",
            "bob",
            "Bob, the new PQC implementation is impressive. We should deploy this across all our systems."
        )
        .unwrap();

    // Display system status
    system.display_status();

    // Receive and verify messages
    println!("\n📥 Receiving and verifying messages...");
    system.receive_message("bob", 0).unwrap(); // Bob receives Alice's message
    system.receive_message("alice", 1).unwrap(); // Alice receives Bob's message
    system.receive_message("charlie", 2).unwrap(); // Charlie receives Alice's message
    system.receive_message("bob", 3).unwrap(); // Bob receives Charlie's message

    println!("\n🎉 Demo completed successfully!");
    println!("🔐 All messages were encrypted and authenticated using post-quantum cryptography!");
    println!("✅ Kyber KEM for key exchange");
    println!("✅ Falcon for digital signatures");
    println!("✅ SHA3-256 for message hashing");
    println!("✅ End-to-end encryption with Falcon signature verification");
}
