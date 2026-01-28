use aegis_crypto_core::{
    falcon::{ falcon_keygen, falcon_sign, falcon_verify },
    hash::sha3_256_hash,
    utils::bytes_to_hex,
};
use std::collections::HashMap;
use std::time::{ SystemTime, UNIX_EPOCH };

/// Represents a document that can be signed
#[derive(Clone, Debug)]
struct Document {
    id: String,
    title: String,
    content: String,
    author: String,
    _created_at: u64,
    modified_at: u64,
    version: u32,
}

/// Represents a digital signature on a document
#[derive(Clone, Debug)]
struct DocumentSignature {
    document_id: String,
    signer_id: String,
    signature: Vec<u8>,
    _timestamp: u64,
    signature_type: String,
}

/// Represents a user who can sign documents
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct User {
    id: String,
    name: String,
    email: String,
    role: String,
    falcon_public_key: Vec<u8>,
    falcon_secret_key: Vec<u8>,
    _created_at: u64,
}

/// Document signing and verification system
struct DocumentSigningSystem {
    users: HashMap<String, User>,
    documents: HashMap<String, Document>,
    signatures: HashMap<String, DocumentSignature>,
}

impl DocumentSigningSystem {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            documents: HashMap::new(),
            signatures: HashMap::new(),
        }
    }

    /// Create a new user with PQC keypair
    fn create_user(
        &mut self,
        id: String,
        name: String,
        email: String,
        role: String
    ) -> Result<(), String> {
        println!("🔐 Creating PQC-secured user: {}", name);

        // Generate Falcon signature keypair
        let falcon_keys = falcon_keygen();
        println!(
            "   ✅ Falcon signature keys generated (Public: {} bytes, Secret: {} bytes)",
            falcon_keys.public_key().len(),
            falcon_keys.secret_key().len()
        );

        let user = User {
            id: id.clone(),
            name,
            email,
            role,
            falcon_public_key: falcon_keys.public_key(),
            falcon_secret_key: falcon_keys.secret_key(),
            _created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };

        self.users.insert(id, user);
        println!("   🎉 User created successfully!");
        Ok(())
    }

    /// Create a new document
    fn create_document(
        &mut self,
        id: String,
        title: String,
        content: String,
        author_id: &str
    ) -> Result<(), String> {
        let author = self.users.get(author_id).ok_or("Author not found")?;

        println!("📄 Creating document: {}", title);
        println!("   ✍️  Author: {}", author.name);
        println!("   📝 Content length: {} characters", content.len());

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let document = Document {
            id: id.clone(),
            title,
            content,
            author: author_id.to_string(),
            _created_at: now,
            modified_at: now,
            version: 1,
        };

        self.documents.insert(id, document);
        println!("   🎉 Document created successfully!");
        Ok(())
    }

    /// Sign a document with PQC digital signature
    fn sign_document(&mut self, document_id: &str, signer_id: &str) -> Result<(), String> {
        let document = self.documents.get(document_id).ok_or("Document not found")?;
        let signer = self.users.get(signer_id).ok_or("Signer not found")?;

        println!("✍️  Signing document '{}' with PQC signature...", document.title);
        println!("   🔐 Signer: {} ({})", signer.name, signer.role);

        // Create document hash for signing
        let document_data = format!(
            "{}:{}:{}:{}:{}:{}",
            document.id,
            document.title,
            document.content,
            document.author,
            document.version,
            document.modified_at
        );
        let document_hash = sha3_256_hash(document_data.as_bytes());
        println!("   🖊️  Document hash created: {}", bytes_to_hex(&document_hash));

        // Sign with Falcon
        let signature = falcon_sign(&signer.falcon_secret_key, &document_hash);
        println!("   ✅ Falcon signature created ({} bytes)", signature.len());

        // Create signature record
        let signature_record = DocumentSignature {
            document_id: document_id.to_string(),
            signer_id: signer_id.to_string(),
            signature,
            _timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            signature_type: "Falcon".to_string(),
        };

        let signature_id = format!("SIG_{}_{}", document_id, signer_id);
        self.signatures.insert(signature_id, signature_record);

        println!("   🎉 Document signed successfully!");
        Ok(())
    }

    /// Verify a document signature
    fn verify_signature(&self, document_id: &str, signer_id: &str) -> Result<bool, String> {
        let document = self.documents.get(document_id).ok_or("Document not found")?;
        let signer = self.users.get(signer_id).ok_or("Signer not found")?;
        let signature_id = format!("SIG_{}_{}", document_id, signer_id);
        let signature_record = self.signatures.get(&signature_id).ok_or("Signature not found")?;

        println!("🔍 Verifying signature on document '{}'...", document.title);
        println!("   🔐 Signer: {} ({})", signer.name, signer.role);

        // Recreate document hash
        let document_data = format!(
            "{}:{}:{}:{}:{}:{}",
            document.id,
            document.title,
            document.content,
            document.author,
            document.version,
            document.modified_at
        );
        let document_hash = sha3_256_hash(document_data.as_bytes());

        // Verify Falcon signature
        let is_valid = falcon_verify(
            &signer.falcon_public_key,
            &document_hash,
            &signature_record.signature
        );

        println!("   ✅ Signature verification: {}", if is_valid { "PASSED" } else { "FAILED" });
        Ok(is_valid)
    }

    /// Display system status
    fn display_status(&self) {
        let separator = "=".repeat(60);
        println!("\n{}", separator);
        println!("📄 DOCUMENT SIGNING SYSTEM STATUS");
        println!("{}", separator);
        println!("👥 Users: {}", self.users.len());
        println!("📄 Documents: {}", self.documents.len());
        println!("✍️  Signatures: {}", self.signatures.len());

        if !self.users.is_empty() {
            println!("\n👥 Registered Users:");
            for (id, user) in &self.users {
                println!("   • {} ({}) - {}", user.name, id, user.role);
                println!(
                    "     📧 {} | 🔑 Falcon public key: {} bytes",
                    user.email,
                    user.falcon_public_key.len()
                );
            }
        }

        if !self.documents.is_empty() {
            println!("\n📄 Documents:");
            for (id, doc) in &self.documents {
                let author_name = self.users
                    .get(&doc.author)
                    .map(|u| &u.name)
                    .unwrap_or(&doc.author);
                println!("   • {} ({}) - Version {}", doc.title, id, doc.version);
                println!("     ✍️  Author: {} | 📝 {} characters", author_name, doc.content.len());
            }
        }

        if !self.signatures.is_empty() {
            println!("\n✍️  Signatures:");
            for (sig_id, sig) in &self.signatures {
                let doc_title = self.documents
                    .get(&sig.document_id)
                    .map(|d| &d.title)
                    .unwrap_or(&sig.document_id);
                let signer_name = self.users
                    .get(&sig.signer_id)
                    .map(|u| &u.name)
                    .unwrap_or(&sig.signer_id);
                println!(
                    "   • {} - {} signed by {} ({})",
                    sig_id,
                    doc_title,
                    signer_name,
                    sig.signature_type
                );
            }
        }
        println!("{}", separator);
    }
}

fn main() {
    println!("🚀 AEGIS DOCUMENT SIGNING SYSTEM DEMO");
    println!("📄 Post-Quantum Cryptography for Digital Signatures");
    let separator = "=".repeat(50);
    println!("{}", separator);

    let mut system = DocumentSigningSystem::new();

    // Create users with different roles
    println!("\n👥 Creating PQC-secured users...");
    system
        .create_user(
            "alice".to_string(),
            "Alice Johnson".to_string(),
            "alice.johnson@company.com".to_string(),
            "Senior Developer".to_string()
        )
        .unwrap();

    system
        .create_user(
            "bob".to_string(),
            "Bob Smith".to_string(),
            "bob.smith@company.com".to_string(),
            "Project Manager".to_string()
        )
        .unwrap();

    system
        .create_user(
            "charlie".to_string(),
            "Charlie Brown".to_string(),
            "charlie.brown@company.com".to_string(),
            "Security Officer".to_string()
        )
        .unwrap();

    // Create documents
    println!("\n📄 Creating documents...");
    system
        .create_document(
            "DOC_001".to_string(),
            "Security Policy v1.0".to_string(),
            "This document outlines the company's security policies and procedures for handling sensitive information. All employees must follow these guidelines to ensure data protection and compliance with industry standards.".to_string(),
            "alice"
        )
        .unwrap();

    system
        .create_document(
            "DOC_002".to_string(),
            "Project Requirements".to_string(),
            "This document contains the detailed requirements for the new post-quantum cryptography implementation project. It includes technical specifications, timeline, and resource allocation.".to_string(),
            "bob"
        )
        .unwrap();

    system
        .create_document(
            "DOC_003".to_string(),
            "Compliance Report".to_string(),
            "Annual compliance report for regulatory requirements. This document must be reviewed and signed by authorized personnel before submission to regulatory bodies.".to_string(),
            "charlie"
        )
        .unwrap();

    // Display initial status
    system.display_status();

    // Sign documents
    println!("\n✍️  Signing documents with PQC signatures...");
    system.sign_document("DOC_001", "alice").unwrap();
    system.sign_document("DOC_001", "charlie").unwrap(); // Security review
    system.sign_document("DOC_002", "bob").unwrap();
    system.sign_document("DOC_002", "alice").unwrap(); // Technical review
    system.sign_document("DOC_003", "charlie").unwrap();
    system.sign_document("DOC_003", "bob").unwrap(); // Management approval

    // Verify signatures
    println!("\n🔍 Verifying document signatures...");
    system.verify_signature("DOC_001", "alice").unwrap();
    system.verify_signature("DOC_001", "charlie").unwrap();
    system.verify_signature("DOC_002", "bob").unwrap();
    system.verify_signature("DOC_002", "alice").unwrap();
    system.verify_signature("DOC_003", "charlie").unwrap();
    system.verify_signature("DOC_003", "bob").unwrap();

    // Display final status
    system.display_status();

    println!("\n🎉 Demo completed successfully!");
    println!("📄 Document signing system using post-quantum cryptography!");
    println!("✅ Falcon digital signatures");
    println!("✅ Document integrity verification");
    println!("✅ Multi-user signing workflow");
    println!("✅ Audit trail and compliance");
}
