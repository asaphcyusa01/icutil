use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_cdk_macros::{init, query, update};
use ic_cdk::api::print;
use lazy_static::lazy_static;
use metrics::{Gauge, register_gauge};
use serde::Serialize;
use std::collections::VecDeque;

// Structure to store water volume readings
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct VolumeReading {
    pub timestamp: u64,  // Timestamp as UNIX epoch time (in seconds)
    pub volume: f64,     // Total water volume in cubic meters
    pub device_id: Option<String>, // Optional device identifier
}

// Error types for better error handling
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum VolumeError {
    InvalidVolume(String),
    StorageError(String),
    DataNotFound,
    RateLimit(String),
    Unauthorized(String),
}

// Result type for API responses
type VolumeResult<T> = Result<T, VolumeError>;

// Type alias for volume readings using VecDeque for better performance
type VolumeReadings = VecDeque<VolumeReading>;

// Constants for configuration
const MAX_READINGS: usize = 1000;
const MIN_VOLUME: f64 = 0.0;
const MAX_VOLUME: f64 = 10000.0; // Maximum reasonable volume in cubic meters
const DEVICE_ID_MAX_LENGTH: usize = 32;

// Initialization function to set up stable storage
#[init]
fn init() {
    let initial_readings: VolumeReadings = VecDeque::new();
    if let Err(_) = storage::stable_save((initial_readings,)) {
        ic_cdk::trap("Failed to initialize stable storage");
    }
}

// Validation functions
fn validate_volume(volume: f64) -> Result<(), String> {
    if volume.is_nan() {
        return Err("Invalid volume: NaN value".into());
    }
    if volume < MIN_VOLUME {
        return Err("Volume cannot be negative".into());
    }
    if volume > MAX_VOLUME {
        return Err(format!("Volume exceeds maximum allowed value of {} cubic meters", MAX_VOLUME));
    }
    if volume.is_infinite() {
        return Err("Volume must be a finite number".into());
    }
    Ok(())
}

fn validate_device_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("Device ID cannot be empty".into());
    }
    if id.len() > DEVICE_ID_MAX_LENGTH {
        return Err(format!("Device ID exceeds {} characters", DEVICE_ID_MAX_LENGTH));
    }
    if !id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err("Device ID contains invalid characters".into());
    }
    Ok(())
}

// Update function to record a new volume reading with validation
#[update]
fn record_volume_data(volume: f64, device_id: Option<String>) -> VolumeResult<String> {
    // Validate input parameters
    validate_volume(volume).map_err(|e| VolumeError::InvalidVolume(e))?;
    
    if let Some(ref device_id) = device_id {
        validate_device_id(device_id).map_err(|e| VolumeError::InvalidVolume(e))?;
    }

    // Retrieve the current list of volume readings
    let mut volume_readings: VolumeReadings = storage::stable_restore()
        .map_err(|_| VolumeError::StorageError("Failed to retrieve stable storage".to_string()))?;

    // Create a new volume reading
    let new_reading = VolumeReading {
        timestamp: ic_cdk::api::time() / 1_000_000_000,  // Convert nanoseconds to seconds
        volume,
        device_id,
    };

    // Append the new reading
    volume_readings.push_back(new_reading);

    // Keep only the last MAX_READINGS to avoid unbounded growth
    while volume_readings.len() > MAX_READINGS {
        volume_readings.pop_front();
    }

    // Save the updated list back to stable storage
    storage::stable_save((volume_readings,))
        .map_err(|_| VolumeError::StorageError("Failed to save to stable storage".to_string()))?;

    ic_cdk::println!("Recording volume: {} cubic meters", volume);
    Ok("Volume data recorded successfully".to_string())
}

// Query function to retrieve recent volume readings with error handling
#[query]
fn get_recent_readings(count: usize) -> VolumeResult<Vec<VolumeReading>> {
    let volume_readings: VolumeReadings = storage::stable_restore()
        .map_err(|_| VolumeError::StorageError("Failed to retrieve stable storage".to_string()))?;

    if volume_readings.is_empty() {
        return Err(VolumeError::DataNotFound);
    }

    // Return the most recent `count` readings (limited to reasonable size)
    let safe_count = count.min(MAX_READINGS);
    let result: Vec<VolumeReading> = volume_readings
        .iter()
        .rev()
        .take(safe_count)
        .cloned()
        .collect();

    Ok(result)
}

// Query function to calculate the average volume with error handling
#[query]
fn get_average_volume() -> VolumeResult<f64> {
    let volume_readings: VolumeReadings = storage::stable_restore()
        .map_err(|_| VolumeError::StorageError("Failed to retrieve stable storage".to_string()))?;

    if volume_readings.is_empty() {
        return Err(VolumeError::DataNotFound);
    }

    let total: f64 = volume_readings.iter().map(|r| r.volume).sum();
    let average = total / volume_readings.len() as f64;
    
    Ok(average)
}

// Structure for volume statistics
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct VolumeStatistics {
    pub count: usize,
    pub average: f64,
    pub min: f64,
    pub max: f64,
    pub std_deviation: f64,
    pub total_volume: f64,
    pub latest_volume: f64,
}

// Query function to get volume statistics
#[query]
fn get_volume_statistics() -> VolumeResult<VolumeStatistics> {
    let volume_readings: VolumeReadings = storage::stable_restore()
        .map_err(|_| VolumeError::StorageError("Failed to retrieve stable storage".to_string()))?;

    if volume_readings.is_empty() {
        return Err(VolumeError::DataNotFound);
    }

    let volumes: Vec<f64> = volume_readings.iter().map(|r| r.volume).collect();
    let total: f64 = volumes.iter().sum();
    let count = volumes.len() as f64;
    let average = total / count;
    
    let min = volumes.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = volumes.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    // Calculate standard deviation
    let variance: f64 = volumes.iter()
        .map(|&x| (x - average).powi(2))
        .sum::<f64>() / count;
    let std_dev = variance.sqrt();

    // Get the latest volume reading
    let latest_volume = volume_readings.back().map(|r| r.volume).unwrap_or(0.0);

    Ok(VolumeStatistics {
        count: volume_readings.len(),
        average,
        min,
        max,
        std_deviation: std_dev,
        total_volume: total,
        latest_volume,
    })
}

// Query function to get volume consumed over a time period
#[query]
fn get_volume_consumed(start_timestamp: u64, end_timestamp: u64) -> VolumeResult<f64> {
    let volume_readings: VolumeReadings = storage::stable_restore()
        .map_err(|_| VolumeError::StorageError("Failed to retrieve stable storage".to_string()))?;

    if volume_readings.is_empty() {
        return Err(VolumeError::DataNotFound);
    }

    // Find readings within the time range
    let readings_in_range: Vec<&VolumeReading> = volume_readings
        .iter()
        .filter(|r| r.timestamp >= start_timestamp && r.timestamp <= end_timestamp)
        .collect();

    if readings_in_range.is_empty() {
        return Ok(0.0);
    }

    // Calculate volume consumed (difference between max and min in the period)
    let volumes: Vec<f64> = readings_in_range.iter().map(|r| r.volume).collect();
    let min_volume = volumes.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_volume = volumes.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    Ok(max_volume - min_volume)
}

// Query function to export all data as JSON with error handling
#[query]
fn export_all_readings() -> VolumeResult<String> {
    let volume_readings: VolumeReadings = storage::stable_restore()
        .map_err(|_| VolumeError::StorageError("Failed to retrieve stable storage".to_string()))?;

    if volume_readings.is_empty() {
        return Err(VolumeError::DataNotFound);
    }

    // Convert readings to JSON format
    let json_data = serde_json::to_string(&volume_readings)
        .map_err(|_| VolumeError::StorageError("Failed to convert to JSON".to_string()))?;
    
    Ok(json_data)
}

// Query function to get readings count
#[query]
fn get_readings_count() -> VolumeResult<usize> {
    let volume_readings: VolumeReadings = storage::stable_restore()
        .map_err(|_| VolumeError::StorageError("Failed to retrieve stable storage".to_string()))?;
    
    Ok(volume_readings.len())
}

// Query function to get current total volume
#[query]
fn get_current_total_volume() -> VolumeResult<f64> {
    let volume_readings: VolumeReadings = storage::stable_restore()
        .map_err(|_| VolumeError::StorageError("Failed to retrieve stable storage".to_string()))?;

    if volume_readings.is_empty() {
        return Err(VolumeError::DataNotFound);
    }

    // Return the latest volume reading
    let latest = volume_readings.back().unwrap();
    Ok(latest.volume)
}

#[query]
fn documentation() -> String {
    r#"# Water Volume Measurement API

This canister tracks total water volume in cubic meters.

## Main Functions:
- `record_volume_data(volume: f64, device_id: Option<String>)` - Record new volume reading
- `get_recent_readings(count: usize)` - Get recent volume readings
- `get_current_total_volume()` - Get the latest total volume
- `get_volume_statistics()` - Get comprehensive volume statistics
- `get_volume_consumed(start: u64, end: u64)` - Get volume consumed in time period

## Units:
- Volume: cubic meters (m³)
- Timestamps: UNIX epoch seconds
"#.to_string()
}

// Update function to clear all readings (admin function)
#[update]
fn clear_all_readings() -> VolumeResult<String> {
    let empty_readings: VolumeReadings = VecDeque::new();
    storage::stable_save((empty_readings,))
        .map_err(|_| VolumeError::StorageError("Failed to clear storage".to_string()))?;
    Ok("All volume readings cleared successfully".to_string())
}

// Alert system for high volume usage
lazy_static! {
    static ref CRITICAL_VOLUME_ALERT: Gauge = register_gauge!(
        "volume_critical",
        "Critical volume threshold"
    ).unwrap();
}

#[ic_cdk::heartbeat]
fn check_alerts() {
    let stats = match get_volume_statistics() {
        Ok(s) => s,
        Err(_) => return,
    };
    
    // Alert if current volume exceeds 90% of maximum allowed
    if stats.latest_volume > MAX_VOLUME * 0.9 {
        CRITICAL_VOLUME_ALERT.set(1.0);
        ic_cdk::println!("ALERT: Critical volume threshold exceeded - {} m³", stats.latest_volume);
    }
}
