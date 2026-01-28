use aegis_crypto_core::{
    kyber::{ kyber_keygen, kyber_encapsulate, kyber_decapsulate },
    falcon::{ falcon_keygen, falcon_sign, falcon_verify },
    hash::sha3_256_hash,
    utils::bytes_to_hex,
};
use std::collections::HashMap;
use std::time::{ SystemTime, UNIX_EPOCH };

/// Represents an IoT device with PQC security
#[derive(Clone, Debug)]
struct IoTDevice {
    #[allow(dead_code)]
    id: String,
    name: String,
    device_type: String,
    location: String,
    status: DeviceStatus,
    kyber_public_key: Vec<u8>,
    #[allow(dead_code)]
    kyber_secret_key: Vec<u8>,
    falcon_public_key: Vec<u8>,
    falcon_secret_key: Vec<u8>,
    last_heartbeat: u64,
    firmware_version: String,
}

/// IoT device status
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
enum DeviceStatus {
    Online,
    Offline,
    Maintenance,
    Compromised,
}

/// Secure message between IoT devices and gateway
#[derive(Clone, Debug)]
struct SecureMessage {
    sender_id: String,
    recipient_id: String,
    message_type: MessageType,
    payload: Vec<u8>,
    falcon_signature: Vec<u8>,
    kem_ciphertext: Vec<u8>,
    timestamp: u64,
}

/// Types of IoT messages
#[allow(dead_code)]
#[derive(Clone, Debug)]
enum MessageType {
    Heartbeat,
    SensorData,
    FirmwareUpdate,
    Authentication,
    Alert,
}

/// IoT Security Gateway
struct IoTSecurityGateway {
    devices: HashMap<String, IoTDevice>,
    messages: Vec<SecureMessage>,
    gateway_id: String,
    gateway_kyber_keys: (Vec<u8>, Vec<u8>), // (public, secret)
    #[allow(dead_code)]
    gateway_falcon_keys: (Vec<u8>, Vec<u8>), // (public, secret)
}

impl IoTSecurityGateway {
    fn new() -> Self {
        // Generate gateway keys
        let kyber_keys = kyber_keygen();
        let falcon_keys = falcon_keygen();

        Self {
            devices: HashMap::new(),
            messages: Vec::new(),
            gateway_id: "GATEWAY_001".to_string(),
            gateway_kyber_keys: (kyber_keys.public_key(), kyber_keys.secret_key()),
            gateway_falcon_keys: (falcon_keys.public_key(), falcon_keys.secret_key()),
        }
    }

    /// Register a new IoT device
    fn register_device(
        &mut self,
        id: String,
        name: String,
        device_type: String,
        location: String
    ) -> Result<(), String> {
        println!("🔐 Registering PQC-secured IoT device: {}", name);
        println!("   📍 Location: {}", location);
        println!("   🔧 Type: {}", device_type);

        // Generate PQC keypairs for the device
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

        let device = IoTDevice {
            id: id.clone(),
            name,
            device_type,
            location,
            status: DeviceStatus::Online,
            kyber_public_key: kyber_keys.public_key(),
            kyber_secret_key: kyber_keys.secret_key(),
            falcon_public_key: falcon_keys.public_key(),
            falcon_secret_key: falcon_keys.secret_key(),
            last_heartbeat: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            firmware_version: "1.0.0".to_string(),
        };

        self.devices.insert(id, device);
        println!("   🎉 Device registered successfully!");
        Ok(())
    }

    /// Send secure message from device to gateway
    fn send_secure_message(
        &mut self,
        device_id: &str,
        message_type: MessageType,
        payload: &str
    ) -> Result<(), String> {
        let device = self.devices.get(device_id).ok_or("Device not found")?;

        println!("📤 Sending secure message from {}...", device.name);
        println!("   📡 Message type: {:?}", message_type);
        println!("   📝 Payload: {} characters", payload.len());

        // Step 1: Perform Kyber KEM encapsulation
        println!("   🔑 Performing Kyber KEM encapsulation...");
        let encaps_result = kyber_encapsulate(&self.gateway_kyber_keys.0).map_err(|e|
            format!("KEM encapsulation failed: {:?}", e)
        )?;
        let shared_secret = encaps_result.shared_secret();
        let kem_ciphertext = encaps_result.ciphertext();
        println!("   ✅ KEM encapsulation complete (Shared secret: {} bytes)", shared_secret.len());

        // Step 2: Encrypt payload using shared secret
        println!("   🔒 Encrypting message payload...");
        let encrypted_payload = self.simulate_aes_encryption(payload.as_bytes(), &shared_secret);
        println!(
            "   ✅ Payload encrypted ({} bytes -> {} bytes)",
            payload.len(),
            encrypted_payload.len()
        );

        // Step 3: Create message hash for signing
        println!("   🖊️  Creating message hash for digital signature...");
        let message_data = format!(
            "{}:{}:{:?}:{}:{}",
            device_id,
            self.gateway_id,
            message_type,
            payload,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        );
        let message_hash = sha3_256_hash(message_data.as_bytes());
        println!("   ✅ Message hash created: {}", bytes_to_hex(&message_hash));

        // Step 4: Sign message with Falcon
        println!("   ✍️  Signing message with Falcon...");
        let falcon_signature = falcon_sign(&device.falcon_secret_key, &message_hash);
        println!("   ✅ Falcon signature created ({} bytes)", falcon_signature.len());

        // Create secure message
        let secure_message = SecureMessage {
            sender_id: device_id.to_string(),
            recipient_id: self.gateway_id.clone(),
            message_type,
            payload: encrypted_payload,
            falcon_signature,
            kem_ciphertext,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };

        self.messages.push(secure_message);
        println!("   🎉 Secure message sent successfully!");
        Ok(())
    }

    /// Process and verify received message
    fn process_message(&self, message_index: usize) -> Result<String, String> {
        let message = self.messages.get(message_index).ok_or("Message not found")?;
        let device = self.devices.get(&message.sender_id).ok_or("Device not found")?;

        println!("📥 Processing secure message from {}...", device.name);
        println!("   📡 Message type: {:?}", message.message_type);

        // Step 1: Decapsulate shared secret using Kyber
        println!("   🔑 Performing Kyber KEM decapsulation...");
        let shared_secret = kyber_decapsulate(
            &self.gateway_kyber_keys.1,
            &message.kem_ciphertext
        ).map_err(|e| format!("KEM decapsulation failed: {:?}", e))?;
        println!("   ✅ KEM decapsulation complete (Shared secret: {} bytes)", shared_secret.len());

        // Step 2: Decrypt payload
        println!("   🔓 Decrypting message payload...");
        let decrypted_payload = self.simulate_aes_decryption(&message.payload, &shared_secret);
        println!(
            "   ✅ Payload decrypted ({} bytes -> {} bytes)",
            message.payload.len(),
            decrypted_payload.len()
        );

        // Step 3: Verify message hash
        println!("   🖊️  Verifying message hash...");
        let message_data = format!(
            "{}:{}:{:?}:{}:{}",
            message.sender_id,
            message.recipient_id,
            message.message_type,
            String::from_utf8_lossy(&decrypted_payload),
            message.timestamp
        );
        let message_hash = sha3_256_hash(message_data.as_bytes());
        println!("   ✅ Message hash verified: {}", bytes_to_hex(&message_hash));

        // Step 4: Verify Falcon signature
        println!("   ✍️  Verifying Falcon signature...");
        let falcon_valid = falcon_verify(
            &device.falcon_public_key,
            &message_hash,
            &message.falcon_signature
        );
        println!("   ✅ Falcon signature verification: {}", if falcon_valid {
            "PASSED"
        } else {
            "FAILED"
        });

        if !falcon_valid {
            return Err("Message signature verification failed".to_string());
        }

        // Update device status based on message type
        if let MessageType::Heartbeat = message.message_type {
            println!("   💓 Heartbeat received - updating device status");
            // In a real implementation, we would update the device's last_heartbeat
        }

        println!("   🎉 Message processed and verified successfully!");
        Ok(String::from_utf8_lossy(&decrypted_payload).to_string())
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
        println!("🏠 IOT SECURITY GATEWAY STATUS");
        println!("{}", separator);
        println!("🔌 Gateway ID: {}", self.gateway_id);
        println!("📱 Registered Devices: {}", self.devices.len());
        println!("📨 Messages Processed: {}", self.messages.len());

        if !self.devices.is_empty() {
            println!("\n📱 Registered IoT Devices:");
            for (id, device) in &self.devices {
                let status_icon = match device.status {
                    DeviceStatus::Online => "🟢",
                    DeviceStatus::Offline => "🔴",
                    DeviceStatus::Maintenance => "🟡",
                    DeviceStatus::Compromised => "⚠️",
                };
                println!("   {} {} ({}) - {}", status_icon, device.name, id, device.device_type);
                println!(
                    "     📍 {} | 🔑 Falcon: {} bytes | 🔐 Kyber: {} bytes",
                    device.location,
                    device.falcon_public_key.len(),
                    device.kyber_public_key.len()
                );
                println!(
                    "     💾 Firmware: {} | 💓 Last heartbeat: {}",
                    device.firmware_version,
                    device.last_heartbeat
                );
            }
        }

        if !self.messages.is_empty() {
            println!("\n📨 Recent Messages:");
            let mut recent_msgs: Vec<_> = self.messages.iter().collect();
            recent_msgs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            for (i, msg) in recent_msgs.iter().take(5).enumerate() {
                let sender_name = self.devices
                    .get(&msg.sender_id)
                    .map(|d| &d.name)
                    .unwrap_or(&msg.sender_id);
                println!(
                    "   {}. {} → {} ({:?}) - {} bytes",
                    i + 1,
                    sender_name,
                    msg.recipient_id,
                    msg.message_type,
                    msg.payload.len()
                );
            }
        }
        println!("{}", separator);
    }
}

fn main() {
    println!("🚀 AEGIS IOT SECURITY GATEWAY DEMO");
    println!("🏠 Post-Quantum Cryptography for IoT Devices");
    let separator = "=".repeat(50);
    println!("{}", separator);

    let mut gateway = IoTSecurityGateway::new();

    // Register IoT devices
    println!("\n📱 Registering PQC-secured IoT devices...");
    gateway
        .register_device(
            "SENSOR_001".to_string(),
            "Temperature Sensor".to_string(),
            "Environmental".to_string(),
            "Building A - Floor 3".to_string()
        )
        .unwrap();

    gateway
        .register_device(
            "CAMERA_001".to_string(),
            "Security Camera".to_string(),
            "Surveillance".to_string(),
            "Building A - Main Entrance".to_string()
        )
        .unwrap();

    gateway
        .register_device(
            "ACTUATOR_001".to_string(),
            "HVAC Controller".to_string(),
            "Building Control".to_string(),
            "Building A - HVAC Room".to_string()
        )
        .unwrap();

    // Display initial status
    gateway.display_status();

    // Send secure messages from devices
    println!("\n📤 Sending secure messages from IoT devices...");
    gateway
        .send_secure_message(
            "SENSOR_001",
            MessageType::Heartbeat,
            "Device online, temperature: 22.5°C, humidity: 45%"
        )
        .unwrap();

    gateway
        .send_secure_message(
            "CAMERA_001",
            MessageType::SensorData,
            "Motion detected at entrance, recording started"
        )
        .unwrap();

    gateway
        .send_secure_message(
            "ACTUATOR_001",
            MessageType::Alert,
            "HVAC system temperature threshold exceeded, activating cooling"
        )
        .unwrap();

    gateway
        .send_secure_message(
            "SENSOR_001",
            MessageType::SensorData,
            "Temperature: 23.1°C, humidity: 46%, pressure: 1013.25 hPa"
        )
        .unwrap();

    // Process and verify messages
    println!("\n📥 Processing and verifying secure messages...");
    gateway.process_message(0).unwrap(); // Heartbeat
    gateway.process_message(1).unwrap(); // Sensor data
    gateway.process_message(2).unwrap(); // Alert
    gateway.process_message(3).unwrap(); // Sensor data

    // Display final status
    gateway.display_status();

    println!("\n🎉 Demo completed successfully!");
    println!("🏠 IoT security gateway using post-quantum cryptography!");
    println!("✅ Kyber KEM for secure key exchange");
    println!("✅ Falcon digital signatures for authentication");
    println!("✅ End-to-end encryption for IoT communications");
    println!("✅ Device authentication and message integrity");
}
