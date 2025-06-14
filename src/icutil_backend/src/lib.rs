use candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_cdk_macros::{init, query, update};
use ic_cdk::api::print;
use lazy_static::lazy_static;
use metrics::{Gauge, register_gauge};
use serde::Serialize;
use std::collections::VecDeque;

// Structure to store flow sensor readings
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct FlowReading {
    pub timestamp: u64,  // Timestamp as UNIX epoch time (in seconds)
    pub flow_rate: f64,  // Flow rate in liters per minute
    pub device_id: Option<String>, // Optional device identifier
}

// Error types for better error handling
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum FlowError {
    InvalidFlowRate(String),
    StorageError(String),
    DataNotFound,
}

// Result type for API responses
type FlowResult<T> = Result<T, FlowError>;

// Type alias for flow readings using VecDeque for better performance
type FlowReadings = VecDeque<FlowReading>;

// Constants for configuration
const MAX_READINGS: usize = 1000;
const MIN_FLOW_RATE: f64 = 0.0;
const MAX_FLOW_RATE: f64 = 1000.0; // Maximum reasonable flow rate

// Initialization function to set up stable storage
#[init]
fn init() {
    let initial_readings: FlowReadings = VecDeque::new();
    if let Err(_) = storage::stable_save((initial_readings,)) {
        ic_cdk::trap("Failed to initialize stable storage");
    }
}

// Update function to record a new flow reading with validation
#[update]
const MAX_FLOW_RATE: f64 = 1000.0;
const DEVICE_ID_MAX_LENGTH: usize = 32;

fn validate_flow_rate(rate: f64) -> Result<(), String> {
    if rate.is_nan() {
        return Err("Invalid flow rate: NaN value".into());
    }
    if rate < 0.0 {
        return Err("Flow rate cannot be negative".into());
    }
    if rate > MAX_FLOW_RATE {
        return Err(format!("Flow rate exceeds maximum allowed value of {}", MAX_FLOW_RATE));
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

#[update]
fn record_flow_data(flow_rate: f64, device_id: Option<String>) -> FlowResult<String> {
    // Validate input parameters
    validate_flow_rate(flow_rate)?;
    if let Some(device_id) = &device_id {
        validate_device_id(device_id)?;
    }

    let shard_key = get_shard_key(&device_id);
    let mut flow_readings: FlowReadings = storage::stable_restore(shard_key)
        .map_err(|_| FlowError::StorageError("Failed to retrieve shard".to_string()))?;

    // Validate flow rate
    if flow_rate < MIN_FLOW_RATE || flow_rate > MAX_FLOW_RATE {
        return Err(FlowError::InvalidFlowRate(
            format!("Flow rate must be between {} and {} L/min", MIN_FLOW_RATE, MAX_FLOW_RATE)
        ));
    }

    if flow_rate.is_nan() || flow_rate.is_infinite() {
        return Err(FlowError::InvalidFlowRate("Flow rate must be a valid number".to_string()));
    }

    // Retrieve the current list of flow readings
    let mut flow_readings: FlowReadings = storage::stable_restore()
        .map_err(|_| FlowError::StorageError("Failed to retrieve stable storage".to_string()))?;

    // Create a new flow reading
    let new_reading = FlowReading {
        timestamp: ic_cdk::api::time() / 1_000_000_000,  // Convert nanoseconds to seconds
        flow_rate,
        device_id,
    };

    check_rate_limit(&caller)
        .map_err(|e| FlowError::RateLimit(e))?;

    // Append the new reading
    flow_readings.push_back(new_reading);

    // Keep only the last MAX_READINGS to avoid unbounded growth
    while flow_readings.len() > MAX_READINGS {
        flow_readings.pop_front();
    }

    // Save the updated list back to stable storage
    storage::stable_save((flow_readings,))
        .map_err(|_| FlowError::StorageError("Failed to save to stable storage".to_string()))?;

    Ok("Data recorded successfully".to_string())
}

// Query function to retrieve recent flow readings with error handling
#[query]
fn get_recent_readings(count: usize) -> FlowResult<Vec<FlowReading>> {
    let flow_readings: FlowReadings = storage::stable_restore()
        .map_err(|_| FlowError::StorageError("Failed to retrieve stable storage".to_string()))?;

    if flow_readings.is_empty() {
        return Err(FlowError::DataNotFound);
    }

    // Return the most recent `count` readings (limited to reasonable size)
    let safe_count = count.min(MAX_READINGS);
    let result: Vec<FlowReading> = flow_readings
        .iter()
        .rev()
        .take(safe_count)
        .cloned()
        .collect();

    Ok(result)
}

// Query function to calculate the average flow rate with error handling
#[query]
fn get_average_flow_rate() -> FlowResult<f64> {
    let flow_readings: FlowReadings = storage::stable_restore()
        .map_err(|_| FlowError::StorageError("Failed to retrieve stable storage".to_string()))?;

    if flow_readings.is_empty() {
        return Err(FlowError::DataNotFound);
    }

    let total: f64 = flow_readings.iter().map(|r| r.flow_rate).sum();
    let average = total / flow_readings.len() as f64;
    
    Ok(average)
}

// Query function to get flow statistics
#[query]
fn get_flow_statistics() -> FlowResult<FlowStatistics> {
    let flow_readings: FlowReadings = storage::stable_restore()
        .map_err(|_| FlowError::StorageError("Failed to retrieve stable storage".to_string()))?;

    if flow_readings.is_empty() {
        return Err(FlowError::DataNotFound);
    }

    let flow_rates: Vec<f64> = flow_readings.iter().map(|r| r.flow_rate).collect();
    let total: f64 = flow_rates.iter().sum();
    let count = flow_rates.len() as f64;
    let average = total / count;
    
    let min = flow_rates.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = flow_rates.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    // Calculate standard deviation
    let variance: f64 = flow_rates.iter()
        .map(|&x| (x - average).powi(2))
        .sum::<f64>() / count;
    let std_dev = variance.sqrt();

    Ok(FlowStatistics {
        count: flow_readings.len(),
        average,
        min,
        max,
        std_deviation: std_dev,
        total_volume: total,
    })
}

// Structure for flow statistics
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct FlowStatistics {
    pub count: usize,
    pub average: f64,
    pub min: f64,
    pub max: f64,
    pub std_deviation: f64,
    pub total_volume: f64,
}

// Query function to export all data as JSON with error handling
#[query]
fn export_all_readings() -> FlowResult<String> {
    let flow_readings: FlowReadings = storage::stable_restore()
        .map_err(|_| FlowError::StorageError("Failed to retrieve stable storage".to_string()))?;

    if flow_readings.is_empty() {
        return Err(FlowError::DataNotFound);
    }

    // Convert readings to JSON format
    let json_data = serde_json::to_string(&flow_readings)
        .map_err(|_| FlowError::StorageError("Failed to convert to JSON".to_string()))?;
    
    Ok(json_data)
}

// Query function to get readings count
#[query]
fn get_readings_count() -> FlowResult<usize> {
    let flow_readings: FlowReadings = storage::stable_restore()
        .map_err(|_| FlowError::StorageError("Failed to retrieve stable storage".to_string()))?;
    
    Ok(flow_readings.len())
}

#[query]
fn documentation() -> String {
    format!("# ICUtil API\n\n{}", include_str!("../icutil_backend.did"))
}

#[query]
fn openapi_spec() -> String {
    candid::export_service!();
    __export_service()
}

// Update function to clear all readings (admin function)
#[update]
fn clear_all_readings() -> FlowResult<String> {
    auth::authenticate(&["admin"])?;
    let empty_readings: FlowReadings = VecDeque::new();
    storage::stable_save((empty_readings,))
        .map_err(|_| FlowError::StorageError("Failed to clear storage".to_string()))?;
    Ok("All readings cleared successfully".to_string())
}

#[update]
const MAX_FLOW_RATE: f64 = 1000.0;
const DEVICE_ID_MAX_LENGTH: usize = 32;

fn validate_flow_rate(rate: f64) -> Result<(), String> {
    if rate.is_nan() {
        return Err("Invalid flow rate: NaN value".into());
    }
    if rate < 0.0 {
        return Err("Flow rate cannot be negative".into());
    }
    if rate > MAX_FLOW_RATE {
        return Err(format!("Flow rate exceeds maximum allowed value of {}", MAX_FLOW_RATE));
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

#[update]
fn record_flow_data(flow_rate: f64, device_id: Option<String>) -> FlowResult<String> {
    auth::authenticate(&["sensor", "admin"])?;
    // Validate input parameters
    validate_flow_rate(flow_rate)?;
    if let Some(device_id) = &device_id {
        validate_device_id(device_id)?;
    }

    // Validate flow rate
    if flow_rate < MIN_FLOW_RATE || flow_rate > MAX_FLOW_RATE {
        return Err(FlowError::InvalidFlowRate(
            format!("Flow rate must be between {} and {} L/min", MIN_FLOW_RATE, MAX_FLOW_RATE)
        ));
    }

    if flow_rate.is_nan() || flow_rate.is_infinite() {
        return Err(FlowError::InvalidFlowRate("Flow rate must be a valid number".to_string()));
    }

    // Retrieve the current list of flow readings
    let mut flow_readings: FlowReadings = storage::stable_restore()
        .map_err(|_| FlowError::StorageError("Failed to retrieve stable storage".to_string()))?;

    // Create a new flow reading
    let new_reading = FlowReading {
        timestamp: ic_cdk::api::time() / 1_000_000_000,  // Convert nanoseconds to seconds
        flow_rate,
        device_id,
    };

    check_rate_limit(&caller)
        .map_err(|e| FlowError::RateLimit(e))?;

    // Append the new reading
    flow_readings.push_back(new_reading);

    // Keep only the last MAX_READINGS to avoid unbounded growth
    while flow_readings.len() > MAX_READINGS {
        flow_readings.pop_front();
    }

    // Save the updated list back to stable storage
    storage::stable_save((flow_readings,))
        .map_err(|_| FlowError::StorageError("Failed to save to stable storage".to_string()))?;

    Ok("Data recorded successfully".to_string())
}

lazy_static! {
    static ref CRITICAL_FLOW_ALERT: Gauge = register_gauge!(
        "flow_rate_critical",
        "Critical flow rate threshold"
    ).unwrap();
}

#[ic_cdk::heartbeat]
fn check_alerts() {
    let stats = match get_flow_statistics() {
        Ok(s) => s,
        Err(_) => return,
    };
    
    if stats.average > 900.0 {
        CRITICAL_FLOW_ALERT.set(1.0);
        log("ALERT", "Critical flow rate exceeded", None);
    }
}

fn stable_retry<F, T>(mut f: F) -> Result<T, SensorError> 
where
    F: FnMut() -> Result<T, ic_cdk::api::error::Error>
{
    let mut retries = 0;
    loop {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) if retries < 3 => {
                ic_cdk::println!("Retry {} for storage operation", retries);
                retries += 1;
            }
            Err(e) => return Err(SensorError::Storage {
                source: e,
                context: format!("Failed after {} retries", retries)
            })
        }
    }
}
ic_cdk::println!("Recording flow: {} L/min", flow_rate);
COUNTER.with(|c| c.borrow_mut().inc());


// RBAC Middleware
async fn require_role(jwt_token: &str, required_role: &str) -> Result<Claims, Error> {
    let claims = auth_backend::validate_token(jwt_token)
        .await
        .map_err(|_| Error::Unauthorized {
            message: "Invalid token".into(),
        })?;

    if claims.roles.contains(&required_role.to_string()) {
        Ok(claims)
    } else {
        Err(Error::Unauthorized {
            message: format!("Requires {} role", required_role),
        })
    }
}

// Modified device status update with RBAC
pub async fn update_device_status(jwt_token: String, device_id: String, status: DeviceStatus) -> Result<(), Error> {
    let _claims = require_role(&jwt_token, "device_manager").await?;
    
    // Original implementation
    devices::update_status(&device_id, status)
}

// Updated flow control endpoint
pub async fn adjust_flow_rate(jwt_token: String, device_id: String, rate: u64) -> Result<(), Error> {
    let claims = require_role(&jwt_token, "flow_operator").await?;
    
    if rate > MAX_FLOW_RATE {
        return Err(Error::InvalidInput {
            message: format!("Exceeds max rate of {}", MAX_FLOW_RATE),
        });
    }
    
    flow_control::set_rate(&device_id, rate)
}

