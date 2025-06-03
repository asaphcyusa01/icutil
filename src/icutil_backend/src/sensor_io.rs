use candid::{Principal, CandidType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct SensorData {
    pub device_id: String,
    pub readings: Vec<SensorReading>,
    pub timestamp: u64,
    pub signature: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum SensorReading {
    Electricity {
        voltage: f64,
        current: f64,
        power_factor: f64,
        kwh: f64,
    },
    Water {
        flow_rate: f64,
        total_liters: f64,
        temperature: Option<f64>,
    },
}

#[update]
async fn handle_esp32_payload(payload: SensorData) -> Result<HashMap<u64, bool>, String> {
    // Handles JSON payloads from ESP32 over HTTPS
}

async fn validate_device(device_id: &str, signature: &str) -> Result<(), String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let key = KEY_MANAGER.get_current_key(device_id).await?;
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())
        .map_err(|_| "Invalid key length")?;
    mac.update(device_id.as_bytes());
    mac.verify_slice(hex::decode(signature).unwrap().as_slice())
        .map_err(|_| "Invalid HMAC signature")?;
    Ok(())
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct DeviceAuth {
    hmac_key: Vec<u8>,
    enc_key: Vec<u8>, // Derived via HKDF
    key_version: u32
}

//Split into domain-specific modules
mod electricity;
mod water;
mod security;

async fn process_electricity_reading(data: ElectricityPayload) -> Result<(), String> {
    // Calls electricity_backend canister
    ic_cdk::call("add_electricity_reading", (data.kwh,))
}

async fn process_water_reading(
    data: WaterPayload,
    timestamp: u64
) -> Result<(), String> {
    ic_cdk::call::<(f64,), _>(
        Principal::from_text(&get_water_canister_id())
            .map_err(|e| format!("Invalid canister ID: {}", e))?,
        "add_water_reading",
        (data.total_liters,)
    )
    .await
    .map_err(|e| format!("Failed to store reading: {}", e))?;

    Ok(())
}

// Offline storage implementation
#[derive(candid::CandidType, Deserialize, Serialize)]
struct QueuedReading {
    device_id: String,
    reading: SensorReading,
    timestamp: u64,
    retries: u8,
}

#[pre_upgrade]
fn pre_upgrade() {
    storage::stable_save((storage::stable_restore::<Vec<QueuedReading>>().unwrap_or_default(),))
        .expect("Failed to save queue");
}

#[post_upgrade]
fn post_upgrade() {
    let queue = storage::stable_restore::<Vec<QueuedReading>>().unwrap_or_default();
    storage::stable_save((queue,)).expect("Failed to restore queue");
}

fn queue_reading(reading: QueuedReading) {
    let mut queue = storage::stable_restore::<Vec<QueuedReading>>().unwrap_or_default();
    queue.push(reading);
    storage::stable_save((queue,)).expect("Failed to save queue");
}

#[heartbeat]
async fn process_queued_readings() {
    let mut queue = storage::stable_restore::<Vec<QueuedReading>>().unwrap_or_default();
    let mut successes = vec![];

    for (index, reading) in queue.iter_mut().enumerate() {
        if reading.retries >= 3 {
            continue;
        }

        let result = match &reading.reading {
            SensorReading::Electricity(data) => {
                process_electricity_reading(data.clone(), reading.timestamp).await
            }
            SensorReading::Water(data) => {
                process_water_reading(data.clone(), reading.timestamp).await
            }
        };

        if result.is_ok() {
            successes.push(index);
        } else {
            reading.retries += 1;
        }
    }

    // Remove successfully processed readings
    for index in successes.into_iter().rev() {
        queue.remove(index);
    }

    storage::stable_save((queue,)).expect("Failed to update queue");
}

use ciborium::{from_reader, into_writer};

async fn handle_esp32_payload(payload: Vec<u8>) -> Result<(), String> {
    let reading: SensorReading = from_reader(payload.as_slice())
        .map_err(|e| format!("CBOR deserialization failed: {}", e))?;
    // Process reading
}

async fn secure_listener() {
    let listener = TcpListener::bind("0.0.0.0:4433").await.unwrap();
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(load_certs(), load_private_key());
    // Accept DTLS connections
}

#[derive(ciborium::Encode)]
struct SensorPacket {
    device_id: u64,
    timestamp: u64,
    readings: Vec<f32>,
}

// Key rotation system
#[derive(candid::CandidType, Deserialize, Serialize)]
struct KeyRotation {
    current: Vec<u8>,
    previous: Vec<u8>,
    valid_until: u64
}

async fn handle_encrypted_payload(
    iv: [u8; 12],
    ciphertext: Vec<u8>,
    tag: [u8; 16]
) -> Result<(), SensorError> {
    let device = validate_device_via_hmac()?;
    
    let mut payload = vec![iv.to_vec(), ciphertext, tag.to_vec()].concat();
    let (nonce, ciphertext) = payload.split_at_mut(12);
    
    let cipher = aes_gcm::Aes256Gcm::new_from_slice(&device.enc_key)
        .map_err(|_| SensorError::DecryptionFailed)?;
    
    let plaintext = cipher.decrypt(nonce.into(), ciphertext)
        .map_err(|_| SensorError::DecryptionFailed)?;
    
    handle_decrypted_payload(plaintext).await
}