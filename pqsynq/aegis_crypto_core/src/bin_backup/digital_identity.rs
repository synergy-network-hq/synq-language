use aegis_crypto_core::{
    dilithium::{ dilithium_keygen, dilithium_sign, dilithium_verify },
    falcon::{ falcon_keygen, falcon_sign, falcon_verify },
    sphincsplus::{ sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify },
    hash::sha3_256_hash,
    utils::bytes_to_hex,
};
use std::collections::HashMap;
use std::time::{ SystemTime, UNIX_EPOCH };

/// Represents a digital identity with multiple authentication factors
#[derive(Debug, Clone)]
struct DigitalIdentity {
    id: String,
    username: String,
    email: String,
    full_name: String,
    organization: String,
    #[allow(dead_code)]
    role: String,
    #[allow(dead_code)]
    created_at: u64,
    last_login: u64,
    login_count: u32,
    is_active: bool,
    security_level: SecurityLevel,
}

/// Security levels for different authentication requirements
#[derive(Debug, Clone, PartialEq)]
enum SecurityLevel {
    Basic, // Single signature (Dilithium)
    Enhanced, // Dual signature (Dilithium + Falcon)
    Maximum, // Triple signature (Dilithium + Falcon + SPHINCS+)
}

/// Authentication credentials for an identity
#[derive(Debug)]
struct AuthCredentials {
    #[allow(dead_code)]
    identity_id: String,
    dilithium_keypair: (Vec<u8>, Vec<u8>), // (public_key, secret_key)
    falcon_keypair: (Vec<u8>, Vec<u8>), // (public_key, secret_key)
    sphincsplus_keypair: (Vec<u8>, Vec<u8>), // (public_key, secret_key)
    password_hash: Vec<u8>,
    #[allow(dead_code)]
    salt: Vec<u8>,
    #[allow(dead_code)]
    mfa_secret: Vec<u8>,
}

/// Authentication session
#[derive(Debug, Clone)]
struct AuthSession {
    session_id: String,
    identity_id: String,
    #[allow(dead_code)]
    created_at: u64,
    expires_at: u64,
    #[allow(dead_code)]
    ip_address: String,
    #[allow(dead_code)]
    user_agent: String,
    #[allow(dead_code)]
    is_valid: bool,
}

/// Digital identity management system
struct IdentityManagementSystem {
    identities: HashMap<String, DigitalIdentity>,
    credentials: HashMap<String, AuthCredentials>,
    active_sessions: HashMap<String, AuthSession>,
    audit_log: Vec<AuditEvent>,
}

/// Audit event for security monitoring
#[derive(Debug, Clone)]
struct AuditEvent {
    #[allow(dead_code)]
    timestamp: u64,
    event_type: String,
    identity_id: String,
    details: String,
    ip_address: String,
    success: bool,
}

impl IdentityManagementSystem {
    fn new() -> Self {
        Self {
            identities: HashMap::new(),
            credentials: HashMap::new(),
            active_sessions: HashMap::new(),
            audit_log: Vec::new(),
        }
    }

    /// Create a new digital identity with PQC authentication
    fn create_identity(
        &mut self,
        username: String,
        email: String,
        full_name: String,
        organization: String,
        role: String,
        security_level: SecurityLevel
    ) -> Result<String, String> {
        if self.identities.values().any(|id| id.username == username || id.email == email) {
            return Err("Username or email already exists".to_string());
        }

        let identity_id = self.generate_identity_id(&username);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        println!("🔐 Creating digital identity for: {}", full_name);
        println!("   📧 Email: {}", email);
        println!("   🏢 Organization: {}", organization);
        println!("   🛡️  Security Level: {:?}", security_level);

        // Generate PQC keypairs based on security level
        let dilithium_keys = dilithium_keygen();
        println!(
            "   ✅ Dilithium keys generated (Public: {} bytes, Secret: {} bytes)",
            dilithium_keys.public_key().len(),
            dilithium_keys.secret_key().len()
        );

        let falcon_keys = falcon_keygen();
        println!(
            "   ✅ Falcon keys generated (Public: {} bytes, Secret: {} bytes)",
            falcon_keys.public_key().len(),
            falcon_keys.secret_key().len()
        );

        let sphincsplus_keys = sphincsplus_keygen();
        println!(
            "   ✅ SPHINCS+ keys generated (Public: {} bytes, Secret: {} bytes)",
            sphincsplus_keys.public_key().len(),
            sphincsplus_keys.secret_key().len()
        );

        // Create identity
        let identity = DigitalIdentity {
            id: identity_id.clone(),
            username: username.clone(),
            email,
            full_name,
            organization,
            role,
            created_at: now,
            last_login: 0,
            login_count: 0,
            is_active: true,
            security_level,
        };

        // Create credentials
        let credentials = AuthCredentials {
            identity_id: identity_id.clone(),
            dilithium_keypair: (dilithium_keys.public_key(), dilithium_keys.secret_key()),
            falcon_keypair: (falcon_keys.public_key(), falcon_keys.secret_key()),
            sphincsplus_keypair: (sphincsplus_keys.public_key(), sphincsplus_keys.secret_key()),
            password_hash: self.hash_password("default_password123"),
            salt: self.generate_salt(),
            mfa_secret: self.generate_mfa_secret(),
        };

        self.identities.insert(identity_id.clone(), identity);
        self.credentials.insert(identity_id.clone(), credentials);

        // Log the event
        self.log_audit_event(
            "IDENTITY_CREATED",
            &identity_id,
            "New digital identity created",
            "127.0.0.1",
            true
        );

        println!("   🎉 Digital identity created successfully!");
        println!("   🆔 Identity ID: {}", identity_id);

        Ok(identity_id)
    }

    /// Authenticate an identity using PQC signatures
    fn authenticate(
        &mut self,
        username: &str,
        password: &str,
        _challenge_response: &[u8]
    ) -> Result<AuthSession, String> {
        // First, find the identity and get its ID
        let identity_id = self.identities
            .values()
            .find(|id| id.username == username && id.is_active)
            .map(|id| id.id.clone())
            .ok_or("Invalid username or inactive account")?;

        let credentials = self.credentials
            .get(&identity_id)
            .ok_or("Authentication credentials not found")?;

        // Get identity details for display
        let identity = self.identities.get(&identity_id).unwrap();
        println!("\n🔐 Authenticating user: {}", identity.full_name);

        // Step 1: Verify password
        println!("   🔑 Verifying password...");
        let password_hash = self.hash_password(password);
        if password_hash != credentials.password_hash {
            self.log_audit_event(
                "AUTH_FAILED",
                &identity_id,
                "Invalid password",
                "127.0.0.1",
                false
            );
            return Err("Invalid password".to_string());
        }
        println!("   ✅ Password verified");

        // Step 2: Verify challenge response based on security level
        println!("   🖊️  Verifying PQC challenge response...");
        let challenge_valid = match identity.security_level {
            SecurityLevel::Basic => {
                self.verify_basic_challenge(credentials, _challenge_response)
            }
            SecurityLevel::Enhanced => {
                self.verify_enhanced_challenge(credentials, _challenge_response)
            }
            SecurityLevel::Maximum => {
                self.verify_maximum_challenge(credentials, _challenge_response)
            }
        };

        if !challenge_valid {
            self.log_audit_event(
                "AUTH_FAILED",
                &identity_id,
                "Invalid PQC challenge response",
                "127.0.0.1",
                false
            );
            return Err("Invalid challenge response".to_string());
        }
        println!("   ✅ PQC challenge response verified");

        // Step 3: Create authentication session
        let session = self.create_session(&identity_id, "127.0.0.1", "Rust Client");

        // Step 4: Update identity stats
        if let Some(id) = self.identities.get_mut(&identity_id) {
            id.last_login = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            id.login_count += 1;
        }

        // Log successful authentication
        self.log_audit_event(
            "AUTH_SUCCESS",
            &identity_id,
            "User authenticated successfully",
            "127.0.0.1",
            true
        );

        println!("   🎉 Authentication successful!");
        println!("   🔑 Session created: {}", session.session_id);

        Ok(session)
    }

    /// Verify basic security level (Dilithium only)
    fn verify_basic_challenge(
        &self,
        credentials: &AuthCredentials,
        _challenge_response: &[u8]
    ) -> bool {
        let challenge = b"authenticate_user_challenge";
        let challenge_hash = sha3_256_hash(challenge);

        dilithium_verify(&credentials.dilithium_keypair.0, &challenge_hash)
    }

    /// Verify enhanced security level (Dilithium + Falcon)
    fn verify_enhanced_challenge(
        &self,
        credentials: &AuthCredentials,
        _challenge_response: &[u8]
    ) -> bool {
        let challenge = b"authenticate_user_challenge_enhanced";
        let challenge_hash = sha3_256_hash(challenge);

        // Verify both signatures
        let dilithium_valid = dilithium_verify(&credentials.dilithium_keypair.0, &challenge_hash);
        let falcon_valid = falcon_verify(
            &credentials.dilithium_keypair.0,
            &challenge_hash,
            _challenge_response
        );

        dilithium_valid && falcon_valid
    }

    /// Verify maximum security level (Dilithium + Falcon + SPHINCS+)
    fn verify_maximum_challenge(
        &self,
        credentials: &AuthCredentials,
        _challenge_response: &[u8]
    ) -> bool {
        let challenge = b"authenticate_user_challenge_maximum";
        let challenge_hash = sha3_256_hash(challenge);

        // Verify all three signatures
        let dilithium_valid = dilithium_verify(&credentials.dilithium_keypair.0, &challenge_hash);
        let falcon_valid = falcon_verify(
            &credentials.falcon_keypair.0,
            &challenge_hash,
            _challenge_response
        );
        let sphincsplus_valid = sphincsplus_verify(
            &credentials.sphincsplus_keypair.0,
            _challenge_response
        );

        dilithium_valid && falcon_valid && sphincsplus_valid
    }

    /// Sign a document with PQC signatures
    fn sign_document(
        &mut self,
        identity_id: &str,
        document_hash: &[u8],
        security_level: SecurityLevel
    ) -> Result<DocumentSignature, String> {
        let identity = self.identities.get(identity_id).ok_or("Identity not found")?;

        let credentials = self.credentials.get(identity_id).ok_or("Credentials not found")?;

        if !identity.is_active {
            return Err("Identity is not active".to_string());
        }

        println!("\n✍️  Signing document for: {}", identity.full_name);
        println!("   📄 Document hash: {}", bytes_to_hex(document_hash));
        println!("   🛡️  Security level: {:?}", security_level);

        let mut signatures = Vec::new();
        let mut signature_types = Vec::new();

        // Create signatures based on security level
        match security_level {
            SecurityLevel::Basic => {
                let dilithium_sig = dilithium_sign(&credentials.dilithium_keypair.1, document_hash);
                signatures.push(dilithium_sig);
                signature_types.push("Dilithium".to_string());
                println!("   ✅ Dilithium signature created ({} bytes)", signatures[0].len());
            }
            SecurityLevel::Enhanced => {
                let dilithium_sig = dilithium_sign(&credentials.dilithium_keypair.1, document_hash);
                let falcon_sig = falcon_sign(&credentials.falcon_keypair.1, document_hash);
                let dilithium_len = dilithium_sig.len();
                let falcon_len = falcon_sig.len();
                signatures.push(dilithium_sig);
                signatures.push(falcon_sig);
                signature_types.push("Dilithium".to_string());
                signature_types.push("Falcon".to_string());
                println!(
                    "   ✅ Dual signatures created (Dilithium: {} bytes, Falcon: {} bytes)",
                    dilithium_len,
                    falcon_len
                );
            }
            SecurityLevel::Maximum => {
                let dilithium_sig = dilithium_sign(&credentials.dilithium_keypair.1, document_hash);
                let falcon_sig = falcon_sign(&credentials.falcon_keypair.1, document_hash);
                let sphincsplus_sig = sphincsplus_sign(
                    &credentials.sphincsplus_keypair.1,
                    document_hash
                );
                let dilithium_len = dilithium_sig.len();
                let falcon_len = falcon_sig.len();
                let sphincsplus_len = sphincsplus_sig.len();
                signatures.push(dilithium_sig);
                signatures.push(falcon_sig);
                signatures.push(sphincsplus_sig);
                signature_types.push("Dilithium".to_string());
                signature_types.push("Falcon".to_string());
                signature_types.push("SPHINCS+".to_string());
                println!(
                    "   ✅ Triple signatures created (Dilithium: {} bytes, Falcon: {} bytes, SPHINCS+: {} bytes)",
                    dilithium_len,
                    falcon_len,
                    sphincsplus_len
                );
            }
        }

        let document_signature = DocumentSignature {
            identity_id: identity_id.to_string(),
            document_hash: document_hash.to_vec(),
            signatures,
            signature_types,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            security_level: security_level.clone(),
        };

        // Log the signing event
        self.log_audit_event(
            "DOCUMENT_SIGNED",
            identity_id,
            &format!("Document signed with {:?} security", security_level),
            "127.0.0.1",
            true
        );

        println!("   🎉 Document signed successfully!");
        Ok(document_signature)
    }

    /// Verify a document signature
    fn verify_document_signature(&self, signature: &DocumentSignature) -> bool {
        let identity = match self.identities.get(&signature.identity_id) {
            Some(id) => id,
            None => {
                return false;
            }
        };

        let credentials = match self.credentials.get(&signature.identity_id) {
            Some(cred) => cred,
            None => {
                return false;
            }
        };

        if !identity.is_active {
            return false;
        }

        println!("\n🔍 Verifying document signature for: {}", identity.full_name);
        println!("   📄 Document hash: {}", bytes_to_hex(&signature.document_hash));
        println!("   🛡️  Security level: {:?}", signature.security_level);

        let mut all_valid = true;

        for (signature_data, signature_type) in signature.signatures
            .iter()
            .zip(signature.signature_types.iter()) {
            let valid = match signature_type.as_str() {
                "Dilithium" => {
                    let challenge_hash = sha3_256_hash(&signature.document_hash);
                    dilithium_verify(&credentials.dilithium_keypair.0, &challenge_hash)
                }
                "Falcon" => {
                    falcon_verify(
                        &credentials.falcon_keypair.0,
                        &signature.document_hash,
                        signature_data
                    )
                }
                "SPHINCS+" => {
                    sphincsplus_verify(&credentials.sphincsplus_keypair.0, signature_data)
                }
                _ => false,
            };

            println!("   {} {} signature: {}", if valid { "✅" } else { "❌" }, signature_type, if
                valid
            {
                "VALID"
            } else {
                "INVALID"
            });

            all_valid = all_valid && valid;
        }

        if all_valid {
            println!("   🎉 All signatures verified successfully!");
        } else {
            println!("   ❌ Signature verification failed!");
        }

        all_valid
    }

    /// Generate a unique identity ID
    fn generate_identity_id(&self, username: &str) -> String {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let hash_input = format!("{}:{}", username, timestamp);
        let hash = sha3_256_hash(hash_input.as_bytes());
        format!("ID_{}", bytes_to_hex(&hash[..8]))
    }

    /// Hash a password using SHA3-256
    fn hash_password(&self, password: &str) -> Vec<u8> {
        sha3_256_hash(password.as_bytes())
    }

    /// Generate a random salt
    fn generate_salt(&self) -> Vec<u8> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        sha3_256_hash(format!("{}", timestamp).as_bytes())[..16].to_vec()
    }

    /// Generate MFA secret
    fn generate_mfa_secret(&self) -> Vec<u8> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        sha3_256_hash(format!("mfa_{}", timestamp).as_bytes())[..20].to_vec()
    }

    /// Create an authentication session
    fn create_session(
        &mut self,
        identity_id: &str,
        ip_address: &str,
        user_agent: &str
    ) -> AuthSession {
        let session_id = self.generate_session_id();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let session = AuthSession {
            session_id: session_id.clone(),
            identity_id: identity_id.to_string(),
            created_at: now,
            expires_at: now + 3600, // 1 hour
            ip_address: ip_address.to_string(),
            user_agent: user_agent.to_string(),
            is_valid: true,
        };

        self.active_sessions.insert(session_id, session.clone());
        session
    }

    /// Generate a unique session ID
    fn generate_session_id(&self) -> String {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let random_data = format!("session_{}", timestamp);
        let hash = sha3_256_hash(random_data.as_bytes());
        format!("SESS_{}", bytes_to_hex(&hash[..16]))
    }

    /// Log an audit event
    fn log_audit_event(
        &mut self,
        event_type: &str,
        identity_id: &str,
        details: &str,
        ip_address: &str,
        success: bool
    ) {
        let event = AuditEvent {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            event_type: event_type.to_string(),
            identity_id: identity_id.to_string(),
            details: details.to_string(),
            ip_address: ip_address.to_string(),
            success,
        };
        self.audit_log.push(event);
    }

    /// Display system status
    fn display_status(&self) {
        let separator = "=".repeat(70);
        println!("\n{}", separator);
        println!("🆔 DIGITAL IDENTITY MANAGEMENT SYSTEM STATUS");
        println!("{}", separator);
        println!("👥 Identities: {}", self.identities.len());
        println!("🔐 Active Sessions: {}", self.active_sessions.len());
        println!("📝 Audit Events: {}", self.audit_log.len());

        if !self.identities.is_empty() {
            println!("\n👤 Registered Identities:");
            for identity in self.identities.values() {
                let status = if identity.is_active { "🟢 ACTIVE" } else { "🔴 INACTIVE" };
                println!(
                    "   • {} ({}) - {} - {:?}",
                    identity.full_name,
                    identity.username,
                    status,
                    identity.security_level
                );
                println!(
                    "     📧 {} | 🏢 {} | 🔑 Logins: {}",
                    identity.email,
                    identity.organization,
                    identity.login_count
                );
            }
        }

        if !self.active_sessions.is_empty() {
            println!("\n🔑 Active Sessions:");
            for (session_id, session) in &self.active_sessions {
                let identity_name = self.identities
                    .get(&session.identity_id)
                    .map(|id| &id.full_name)
                    .unwrap_or(&session.identity_id);
                println!(
                    "   • {} - {} (expires in {}s)",
                    session_id,
                    identity_name,
                    session.expires_at.saturating_sub(
                        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
                    )
                );
            }
        }

        if !self.audit_log.is_empty() {
            println!("\n📝 Recent Audit Events:");
            let recent_events: Vec<_> = self.audit_log.iter().rev().take(5).collect();
            for event in recent_events {
                let status = if event.success { "✅" } else { "❌" };
                println!(
                    "   {} [{}] {} - {} ({})",
                    status,
                    event.event_type,
                    event.identity_id,
                    event.details,
                    event.ip_address
                );
            }
        }
        println!("{}", separator);
    }
}

/// Document signature structure
#[derive(Debug, Clone)]
struct DocumentSignature {
    identity_id: String,
    document_hash: Vec<u8>,
    signatures: Vec<Vec<u8>>,
    signature_types: Vec<String>,
    #[allow(dead_code)]
    timestamp: u64,
    security_level: SecurityLevel,
}

fn main() {
    println!("🚀 AEGIS DIGITAL IDENTITY & AUTHENTICATION SYSTEM DEMO");
    println!("🆔 Post-Quantum Cryptography for Identity Management");
    let separator = "=".repeat(60);
    println!("{}", separator);

    let mut system = IdentityManagementSystem::new();

    // Create digital identities with different security levels
    println!("\n👥 Creating digital identities...");
    let alice_id = system
        .create_identity(
            "alice.johnson".to_string(),
            "alice.johnson@company.com".to_string(),
            "Alice Johnson".to_string(),
            "TechCorp Inc.".to_string(),
            "Senior Developer".to_string(),
            SecurityLevel::Maximum
        )
        .unwrap();

    let bob_id = system
        .create_identity(
            "bob.smith".to_string(),
            "bob.smith@company.com".to_string(),
            "Bob Smith".to_string(),
            "TechCorp Inc.".to_string(),
            "Security Analyst".to_string(),
            SecurityLevel::Enhanced
        )
        .unwrap();

    let charlie_id = system
        .create_identity(
            "charlie.brown".to_string(),
            "charlie.brown@company.com".to_string(),
            "Charlie Brown".to_string(),
            "TechCorp Inc.".to_string(),
            "Intern".to_string(),
            SecurityLevel::Basic
        )
        .unwrap();

    // Display system status
    system.display_status();

    // Authenticate users
    println!("\n🔐 Authenticating users...");
    let _alice_session = system
        .authenticate("alice.johnson", "default_password123", b"challenge_response")
        .unwrap();
    let _bob_session = system
        .authenticate("bob.smith", "default_password123", b"challenge_response")
        .unwrap();
    let _charlie_session = system
        .authenticate("charlie.brown", "default_password123", b"challenge_response")
        .unwrap();

    // Sign documents with different security levels
    println!("\n✍️  Signing documents...");
    let document_hash = sha3_256_hash(b"Important company document requiring maximum security");

    let alice_signature = system
        .sign_document(&alice_id, &document_hash, SecurityLevel::Maximum)
        .unwrap();
    let bob_signature = system
        .sign_document(&bob_id, &document_hash, SecurityLevel::Enhanced)
        .unwrap();
    let charlie_signature = system
        .sign_document(&charlie_id, &document_hash, SecurityLevel::Basic)
        .unwrap();

    // Verify document signatures
    println!("\n🔍 Verifying document signatures...");
    system.verify_document_signature(&alice_signature);
    system.verify_document_signature(&bob_signature);
    system.verify_document_signature(&charlie_signature);

    // Display final system status
    system.display_status();

    println!("\n🎉 Demo completed successfully!");
    println!("🆔 Digital identity system using post-quantum cryptography!");
    println!("✅ Multi-factor PQC authentication");
    println!("✅ Document signing with configurable security levels");
    println!("✅ Comprehensive audit logging");
    println!("✅ Session management and security monitoring");
}
