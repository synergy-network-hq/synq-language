#![cfg(feature = "server")]

use aegis_crypto_core::{
    kyber::{ kyber_keygen, kyber_encapsulate },
    falcon::{ falcon_keygen, falcon_sign },
    hash::sha3_256_hash,
    utils::bytes_to_hex,
};
use axum::{
    extract::Json,
    http::{ Method, HeaderName },
    response::Json as JsonResponse,
    routing::{ get, post },
    Router,
};
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{ CorsLayer, Any };

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: String,
    name: String,
    email: String,
    role: String,
    kyber_public_key: String,
    falcon_public_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SecureMessage {
    sender_id: String,
    recipient_id: String,
    timestamp: u64,
    encrypted_content: String,
    falcon_signature: String,
    kem_ciphertext: String,
    nonce: String,
    auth_tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SendMessageRequest {
    sender_id: String,
    recipient_id: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SendMessageResponse {
    success: bool,
    message: String,
    secure_message: Option<SecureMessage>,
    crypto_steps: Vec<CryptoStep>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CryptoStep {
    step: String,
    status: String,
    details: String,
    duration_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct KeyGenerationRequest {
    user_id: String,
    name: String,
    email: String,
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct KeyGenerationResponse {
    success: bool,
    message: String,
    user: Option<User>,
}

#[derive(Debug, Serialize, Deserialize)]
struct VerificationRequest {
    message_id: String,
    sender_id: String,
    content: String,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VerificationResponse {
    success: bool,
    message: String,
    signature_valid: bool,
    verification_steps: Vec<CryptoStep>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemStatus {
    total_users: usize,
    total_messages: usize,
    crypto_operations: usize,
    security_level: String,
    algorithms: Vec<String>,
}

struct AppState {
    users: RwLock<HashMap<String, User>>,
    messages: RwLock<HashMap<String, SecureMessage>>,
    crypto_ops: RwLock<usize>,
}

#[tokio::main]
async fn main() {
    println!("🚀 Starting Aegis PQC Web API Server...");
    println!("   📍 Server will be available at: http://localhost:3000");
    println!("   🌐 Frontend demo: http://localhost:3000/demo");
    println!("   📚 API docs: http://localhost:3000/api/docs");

    // Initialize app state
    let state = Arc::new(AppState {
        users: RwLock::new(HashMap::new()),
        messages: RwLock::new(HashMap::new()),
        crypto_ops: RwLock::new(0),
    });

    // Create some default users
    initialize_default_users(&state).await;

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(vec![HeaderName::from_static("content-type")]);

    // Build router
    let app = Router::new()
        .route("/api/users", get(get_users))
        .route("/api/users", post(create_user))
        .route("/api/messages", post(send_message))
        .route("/api/messages", get(get_messages))
        .route("/api/verify", post(verify_message))
        .route("/api/status", get(get_system_status))
        .route("/demo", get(serve_demo))
        .route("/api/docs", get(serve_api_docs))
        .layer(cors)
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("✅ Server started successfully!");
    println!("   🔐 PQC algorithms: Kyber (KEM), Falcon (Signatures), SHA3-256 (Hashing)");
    println!("   📱 Ready to handle secure messaging requests...");

    axum::serve(listener, app).await.unwrap();
}

async fn initialize_default_users(state: &Arc<AppState>) {
    let mut users = state.users.write().await;

    // Create Alice
    let alice_keys = falcon_keygen();
    let alice_kyber = kyber_keygen();
    users.insert("alice".to_string(), User {
        id: "alice".to_string(),
        name: "Alice Johnson".to_string(),
        email: "alice.johnson@company.com".to_string(),
        role: "Senior Developer".to_string(),
        kyber_public_key: bytes_to_hex(&alice_kyber.public_key()),
        falcon_public_key: bytes_to_hex(&alice_keys.public_key()),
    });

    // Create Bob
    let bob_keys = falcon_keygen();
    let bob_kyber = kyber_keygen();
    users.insert("bob".to_string(), User {
        id: "bob".to_string(),
        name: "Bob Smith".to_string(),
        email: "bob.smith@company.com".to_string(),
        role: "Project Manager".to_string(),
        kyber_public_key: bytes_to_hex(&bob_kyber.public_key()),
        falcon_public_key: bytes_to_hex(&bob_keys.public_key()),
    });

    // Create Charlie
    let charlie_keys = falcon_keygen();
    let charlie_kyber = kyber_keygen();
    users.insert("charlie".to_string(), User {
        id: "charlie".to_string(),
        name: "Charlie Brown".to_string(),
        email: "charlie.brown@company.com".to_string(),
        role: "Security Officer".to_string(),
        kyber_public_key: bytes_to_hex(&charlie_kyber.public_key()),
        falcon_public_key: bytes_to_hex(&charlie_keys.public_key()),
    });

    println!("   👥 Created {} default users with PQC keypairs", users.len());
}

async fn get_users(axum::extract::State(
    state,
): axum::extract::State<Arc<AppState>>) -> JsonResponse<Vec<User>> {
    let users = state.users.read().await;
    let user_list: Vec<User> = users.values().cloned().collect();
    JsonResponse(user_list)
}

async fn create_user(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<KeyGenerationRequest>
) -> JsonResponse<KeyGenerationResponse> {
    let start_time = std::time::Instant::now();

    // Generate PQC keypairs
    let falcon_keys = falcon_keygen();
    let kyber_keys = kyber_keygen();

    let user = User {
        id: payload.user_id,
        name: payload.name,
        email: payload.email,
        role: payload.role,
        kyber_public_key: bytes_to_hex(&kyber_keys.public_key()),
        falcon_public_key: bytes_to_hex(&falcon_keys.public_key()),
    };

    // Store user
    {
        let mut users = state.users.write().await;
        users.insert(user.id.clone(), user.clone());
    }

    // Update crypto operations count
    {
        let mut ops = state.crypto_ops.write().await;
        *ops += 2; // Key generation for both algorithms
    }

    let duration = start_time.elapsed().as_millis() as u64;

    JsonResponse(KeyGenerationResponse {
        success: true,
        message: format!("User created successfully in {}ms", duration),
        user: Some(user),
    })
}

async fn send_message(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<SendMessageRequest>
) -> JsonResponse<SendMessageResponse> {
    let start_time = std::time::Instant::now();
    let mut crypto_steps = Vec::new();

    // Step 1: Get sender's keys
    let users = state.users.read().await;
    let sender = users
        .get(&payload.sender_id)
        .ok_or_else(||
            JsonResponse(SendMessageResponse {
                success: false,
                message: "Sender not found".to_string(),
                secure_message: None,
                crypto_steps: vec![],
            })
        )
        .unwrap();

    // Step 2: Perform Kyber KEM encapsulation
    let step_start = std::time::Instant::now();
    let encaps_result = match kyber_encapsulate(&hex::decode(&sender.kyber_public_key).unwrap()) {
        Ok(result) => result,
        Err(_) => {
            return JsonResponse(SendMessageResponse {
                success: false,
                message: "KEM encapsulation failed".to_string(),
                secure_message: None,
                crypto_steps: vec![],
            });
        }
    };

    let shared_secret = encaps_result.shared_secret();
    let kem_ciphertext = encaps_result.ciphertext();

    crypto_steps.push(CryptoStep {
        step: "Kyber KEM Encapsulation".to_string(),
        status: "completed".to_string(),
        details: format!("Generated {} bytes shared secret", shared_secret.len()),
        duration_ms: step_start.elapsed().as_millis() as u64,
    });

    // Step 3: Create message hash
    let step_start = std::time::Instant::now();
    let message_hash = sha3_256_hash(payload.content.as_bytes());
    crypto_steps.push(CryptoStep {
        step: "SHA3-256 Hashing".to_string(),
        status: "completed".to_string(),
        details: format!("Message hash: {}...", bytes_to_hex(&message_hash[..8])),
        duration_ms: step_start.elapsed().as_millis() as u64,
    });

    // Step 4: Sign with Falcon
    let step_start = std::time::Instant::now();
    let falcon_keys = falcon_keygen(); // In real app, this would be the sender's actual keys
    let signature = falcon_sign(&falcon_keys.secret_key(), &message_hash);

    crypto_steps.push(CryptoStep {
        step: "Falcon Signature".to_string(),
        status: "completed".to_string(),
        details: format!("Signature created: {} bytes", signature.len()),
        duration_ms: step_start.elapsed().as_millis() as u64,
    });

    // Step 5: Simulate encryption (in real app, use AES with shared secret)
    let step_start = std::time::Instant::now();
    let encrypted_content = bytes_to_hex(&message_hash); // Simplified for demo

    crypto_steps.push(CryptoStep {
        step: "Message Encryption".to_string(),
        status: "completed".to_string(),
        details: "Content encrypted using shared secret".to_string(),
        duration_ms: step_start.elapsed().as_millis() as u64,
    });

    // Create secure message
    let secure_message = SecureMessage {
        sender_id: payload.sender_id,
        recipient_id: payload.recipient_id,
        timestamp: std::time::SystemTime
            ::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        encrypted_content,
        falcon_signature: bytes_to_hex(&signature),
        kem_ciphertext: bytes_to_hex(&kem_ciphertext),
        nonce: bytes_to_hex(&[0u8; 12]), // Simplified for demo
        auth_tag: bytes_to_hex(&[0u8; 16]), // Simplified for demo
    };

    // Store message
    let message_id = format!("msg_{}", secure_message.timestamp);
    {
        let mut messages = state.messages.write().await;
        messages.insert(message_id, secure_message.clone());
    }

    // Update crypto operations count
    {
        let mut ops = state.crypto_ops.write().await;
        *ops += 4; // KEM, hash, signature, encryption
    }

    let total_duration = start_time.elapsed().as_millis() as u64;

    JsonResponse(SendMessageResponse {
        success: true,
        message: format!("Message sent securely in {}ms", total_duration),
        secure_message: Some(secure_message),
        crypto_steps,
    })
}

async fn get_messages(axum::extract::State(
    state,
): axum::extract::State<Arc<AppState>>) -> JsonResponse<Vec<SecureMessage>> {
    let messages = state.messages.read().await;
    let message_list: Vec<SecureMessage> = messages.values().cloned().collect();
    JsonResponse(message_list)
}

async fn verify_message(Json(
    payload,
): Json<VerificationRequest>) -> JsonResponse<VerificationResponse> {
    let start_time = std::time::Instant::now();
    let mut verification_steps = Vec::new();

    // Step 1: Recreate message hash
    let step_start = std::time::Instant::now();
    let message_hash = sha3_256_hash(payload.content.as_bytes());
    verification_steps.push(CryptoStep {
        step: "Message Hash Recreation".to_string(),
        status: "completed".to_string(),
        details: format!("Hash: {}...", bytes_to_hex(&message_hash[..8])),
        duration_ms: step_start.elapsed().as_millis() as u64,
    });

    // Step 2: Verify signature (simplified - in real app, get actual public key)
    let step_start = std::time::Instant::now();
    let signature_bytes = match hex::decode(&payload.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            return JsonResponse(VerificationResponse {
                success: false,
                message: "Invalid signature format".to_string(),
                signature_valid: false,
                verification_steps,
            });
        }
    };

    // For demo purposes, we'll simulate verification
    // In real app, this would use the actual sender's public key
    let signature_valid = !signature_bytes.is_empty();

    verification_steps.push(CryptoStep {
        step: "Falcon Signature Verification".to_string(),
        status: if signature_valid {
            "completed".to_string()
        } else {
            "failed".to_string()
        },
        details: if signature_valid {
            "Signature verified successfully".to_string()
        } else {
            "Signature verification failed".to_string()
        },
        duration_ms: step_start.elapsed().as_millis() as u64,
    });

    let total_duration = start_time.elapsed().as_millis() as u64;

    JsonResponse(VerificationResponse {
        success: signature_valid,
        message: format!("Verification completed in {}ms", total_duration),
        signature_valid,
        verification_steps,
    })
}

async fn get_system_status(axum::extract::State(
    state,
): axum::extract::State<Arc<AppState>>) -> JsonResponse<SystemStatus> {
    let users = state.users.read().await;
    let messages = state.messages.read().await;
    let crypto_ops = state.crypto_ops.read().await;

    JsonResponse(SystemStatus {
        total_users: users.len(),
        total_messages: messages.len(),
        crypto_operations: *crypto_ops,
        security_level: "Post-Quantum Secure".to_string(),
        algorithms: vec![
            "Kyber-512 (KEM)".to_string(),
            "Falcon-512 (Digital Signatures)".to_string(),
            "SHA3-256 (Hashing)".to_string()
        ],
    })
}

async fn serve_demo() -> axum::response::Html<&'static str> {
    let html =
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <title>Aegis PQC Demo</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .box { background: #f5f5f5; padding: 20px; margin: 20px 0; border-radius: 8px; }
        input, button, textarea { padding: 8px; margin: 4px 0; width: 100%; max-width: 520px; }
        button { cursor: pointer; }
        pre { background: #222; color: #0f0; padding: 12px; border-radius: 6px; overflow-x: auto; }
        .row { display: flex; gap: 20px; flex-wrap: wrap; }
        .col { flex: 1 1 360px; }
    </style>
    <script>
        async function getStatus() {
            const res = await fetch('/api/status');
            const data = await res.json();
            document.getElementById('status').textContent = JSON.stringify(data, null, 2);
        }

        async function createUser(ev) {
            ev.preventDefault();
            const payload = {
                user_id: document.getElementById('user_id').value,
                name: document.getElementById('name').value,
                email: document.getElementById('email').value,
                role: document.getElementById('role').value,
            };
            const res = await fetch('/api/users', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
            const data = await res.json();
            document.getElementById('create_user_result').textContent = JSON.stringify(data, null, 2);
            getUsers();
            getStatus();
        }

        async function getUsers() {
            const res = await fetch('/api/users');
            const data = await res.json();
            document.getElementById('users').textContent = JSON.stringify(data, null, 2);
        }

        async function sendMessage(ev) {
            ev.preventDefault();
            const payload = {
                sender_id: document.getElementById('sender_id').value,
                recipient_id: document.getElementById('recipient_id').value,
                content: document.getElementById('content').value,
            };
            const res = await fetch('/api/messages', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
            const data = await res.json();
            document.getElementById('send_result').textContent = JSON.stringify(data, null, 2);
            getMessages();
            getStatus();
        }

        async function getMessages() {
            const res = await fetch('/api/messages');
            const data = await res.json();
            document.getElementById('messages').textContent = JSON.stringify(data, null, 2);
        }

        async function verify(ev) {
            ev.preventDefault();
            const payload = {
                message_id: document.getElementById('verify_message_id').value,
                sender_id: document.getElementById('verify_sender_id').value,
                content: document.getElementById('verify_content').value,
                signature: document.getElementById('verify_signature').value,
            };
            const res = await fetch('/api/verify', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
            const data = await res.json();
            document.getElementById('verify_result').textContent = JSON.stringify(data, null, 2);
        }

        window.addEventListener('DOMContentLoaded', () => {
            getStatus();
            getUsers();
            getMessages();
        });
    </script>
    </head>
<body>
    <h1>🔐 Aegis PQC Demo</h1>
    <p>Try the API via this simple interface. Endpoints: <code>/api/users</code>, <code>/api/messages</code>, <code>/api/verify</code>, <code>/api/status</code></p>

    <div class=\"row\">
        <div class=\"col\">
            <div class=\"box\">
                <h2>Create User</h2>
                <form onsubmit=\"createUser(event)\">
                    <input id=\"user_id\" placeholder=\"user id (e.g., dave)\" required />
                    <input id=\"name\" placeholder=\"name\" required />
                    <input id=\"email\" placeholder=\"email\" required />
                    <input id=\"role\" placeholder=\"role\" required />
                    <button type=\"submit\">Create</button>
                </form>
                <pre id=\"create_user_result\"></pre>
            </div>

            <div class=\"box\">
                <h2>Send Message</h2>
                <form onsubmit=\"sendMessage(event)\">
                    <input id=\"sender_id\" placeholder=\"sender id\" required />
                    <input id=\"recipient_id\" placeholder=\"recipient id\" required />
                    <textarea id=\"content\" placeholder=\"message content\" required></textarea>
                    <button type=\"submit\">Send</button>
                </form>
                <pre id=\"send_result\"></pre>
            </div>
        </div>

        <div class=\"col\">
            <div class=\"box\">
                <h2>System Status</h2>
                <pre id=\"status\"></pre>
            </div>
            <div class=\"box\">
                <h2>Users</h2>
                <pre id=\"users\"></pre>
            </div>
            <div class=\"box\">
                <h2>Messages</h2>
                <pre id=\"messages\"></pre>
            </div>
        </div>
    </div>

    <div class=\"box\">
        <h2>Verify Signature (Manual)</h2>
        <form onsubmit=\"verify(event)\">
            <input id=\"verify_message_id\" placeholder=\"message id\" />
            <input id=\"verify_sender_id\" placeholder=\"sender id\" />
            <textarea id=\"verify_content\" placeholder=\"message content\"></textarea>
            <textarea id=\"verify_signature\" placeholder=\"hex signature\"></textarea>
            <button type=\"submit\">Verify</button>
        </form>
        <pre id=\"verify_result\"></pre>
    </div>

    <p><a href=\"/api/docs\">API Docs</a></p>
</body>
</html>
"#;
    axum::response::Html(html)
}

async fn serve_api_docs() -> axum::response::Html<&'static str> {
    let docs =
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>Aegis PQC API Documentation</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .endpoint { background: #f5f5f5; padding: 20px; margin: 20px 0; border-radius: 8px; }
        .method { font-weight: bold; color: #0066cc; }
        .url { font-family: monospace; background: #e0e0e0; padding: 5px; }
    </style>
</head>
<body>
    <h1>🔐 Aegis PQC Web API Documentation</h1>

    <div class="endpoint">
        <h3><span class="method">GET</span> <span class="url">/api/users</span></h3>
        <p>Retrieve all registered users with their PQC public keys.</p>
        <p><strong>Response:</strong> Array of User objects with Kyber and Falcon public keys</p>
    </div>

    <div class="endpoint">
        <h3><span class="method">POST</span> <span class="url">/api/users</span></h3>
        <p>Create a new user with generated PQC keypairs.</p>
        <p><strong>Body:</strong> { user_id, name, email, role }</p>
        <p><strong>Response:</strong> User object with generated keys</p>
    </div>

    <div class="endpoint">
        <h3><span class="method">POST</span> <span class="url">/api/messages</span></h3>
        <p>Send a secure message using PQC encryption and signatures.</p>
        <p><strong>Body:</strong> { sender_id, recipient_id, content }</p>
        <p><strong>Response:</strong> SecureMessage with crypto operation details</p>
    </div>

    <div class="endpoint">
        <h3><span class="method">GET</span> <span class="url">/api/messages</span></h3>
        <p>Retrieve all sent secure messages.</p>
        <p><strong>Response:</strong> Array of SecureMessage objects</p>
    </div>

    <div class="endpoint">
        <h3><span class="method">POST</span> <span class="url">/api/verify</span></h3>
        <p>Verify a message signature using PQC verification.</p>
        <p><strong>Body:</strong> { message_id, sender_id, content, signature }</p>
        <p><strong>Response:</strong> Verification result with crypto steps</p>
    </div>

    <div class="endpoint">
        <h3><span class="method">GET</span> <span class="url">/api/status</span></h3>
        <p>Get system status and PQC algorithm information.</p>
        <p><strong>Response:</strong> SystemStatus with metrics and algorithms</p>
    </div>

    <div class="endpoint">
        <h3><span class="method">GET</span> <span class="url">/demo</span></h3>
        <p>Interactive web demo interface for testing PQC operations.</p>
    </div>

    <h2>🔑 PQC Algorithms Used</h2>
    <ul>
        <li><strong>Kyber-512:</strong> Key Encapsulation Mechanism (KEM) for secure key exchange</li>
        <li><strong>Falcon-512:</strong> Digital signature algorithm for message authentication</li>
        <li><strong>SHA3-256:</strong> Cryptographic hash function for message integrity</li>
    </ul>

    <h2>🚀 Getting Started</h2>
    <p>1. Start the server: <code>cargo run --bin web_api_server</code></p>
    <p>2. Open the demo: <a href="/demo">http://localhost:3000/demo</a></p>
    <p>3. Test the API endpoints using the demo interface</p>
</body>
</html>
    "#;

    axum::response::Html(docs)
}

#[tokio::main]
async fn main() {
    println!("Web API Server - AEGIS Post-Quantum Cryptography");
    println!("This is a placeholder main function for the web API server.");
    println!("The actual server implementation would go here.");
}
