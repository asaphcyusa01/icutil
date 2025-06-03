#[macro_use]
extern crate serde_derive;
extern crate serde;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
struct ElectricityReading {
    timestamp: u64,
    kwh: f64,
}

#[derive(CandidType, Serialize, Deserialize, Default)]
struct ElectricityData {
    readings: HashMap<u64, ElectricityReading>,
    total_kwh: f64,
    rate_limits: HashMap<Principal, (u64, u64)>, // (timestamp, count)
}

// Rate limit configuration
const RATE_LIMIT_WINDOW: u64 = 60_000_000_000; // 1 minute in nanoseconds
const MAX_REQUESTS_PER_MINUTE: u64 = 30;

#[init]
fn init() {
    let mut data = ElectricityData::default();
    data.total_kwh = 0.0;
    storage::stable_save((data,)).unwrap();
}

#[update]
fn add_electricity_reading(kwh: f64) -> Result<bool, SensorError> {
    check_rate_limit(&caller)
        .map_err(|e| SensorError::RateLimit(e))?;

    storage::stable_restore()
        .map_err(|e| SensorError::Storage {
            source: e,
            context: "Failed to load electricity data".into()
        })?;
    let caller = ic_cdk::api::caller();
    
    // Check rate limit before processing
    check_rate_limit(&caller)?;

    // Existing validation
    if kwh < 0.0 || kwh > 10000.0 {
        return Err("Invalid electricity reading".to_string());
    }

    let mut data: ElectricityData = storage::stable_restore().unwrap().0;
    let timestamp = ic_cdk::api::time();
    let reading = ElectricityReading { timestamp, kwh };
    data.readings.insert(timestamp, reading);
    data.total_kwh += kwh;
    storage::stable_save((data,)).unwrap();
    Ok(true)
}

#[query]
fn get_total_kwh() -> f64 {
    let data: ElectricityData = storage::stable_restore().unwrap().0;
    data.total_kwh
}

#[query]
fn get_electricity_readings() -> Vec<ElectricityReading> {
    let data: ElectricityData = storage::stable_restore().unwrap().0;
    data.readings.values().cloned().collect()
}

#[update(guard = "is_authorized")]
fn reset_electricity_data() -> bool {
    let mut data = ElectricityData::default();
    data.total_kwh = 0.0;
    storage::stable_save((data,)).unwrap();
    true
}

fn is_authorized() -> Result<(), String> {
    let caller = ic_cdk::api::caller();
    let admins: Vec<Principal> = storage::stable_restore().unwrap().1;
    if admins.contains(&caller) {
        Ok(())
    } else {
        Err("Unauthorized".to_string())
    }
}

fn load_data() -> Result<ElectricityData, String> {
    storage::stable_restore()
        .map(|(data, _)| data)
        .map_err(|e| format!("Storage error: {:?}", e))
}

fn validate_reading(timestamp: u64) -> Result<(), String> {
    let current_time = ic_cdk::api::time();
    if timestamp > current_time + 60_000_000_000 { // 1 minute future tolerance
        Err("Invalid timestamp".to_string())
    } else {
        Ok(())
    }
}

// Add rate limiting check
thread_local! {
    static RATE_LIMIT: RefCell<HashMap<Principal, (u64, u32)>> = RefCell::new(HashMap::new());
}

fn check_rate_limit(caller: &Principal) -> Result<(), String> {
    let now = ic_cdk::api::time();
    RATE_LIMIT.with(|rl| {
        let mut map = rl.borrow_mut();
        let (last_called, attempts) = map.entry(*caller).or_insert((0, 0));

        let backoff = 2u64.pow(*attempts) * 1_000_000_000; // Exponential backoff in nanoseconds
        
        if now - *last_called < backoff {
            Err(format!("Too many requests. Try again in {} seconds", 
                (backoff - (now - *last_called)) / 1_000_000_000))
        } else {
            *last_called = now;
            *attempts = (*attempts + 1).min(5); // Cap at 32 seconds backoff
            Ok(())
        }
    })
}

#[query]
fn get_electricity_readings() -> Vec<ElectricityReading> {
    let data: ElectricityData = storage::stable_restore().unwrap().0;
    data.readings.values().cloned().collect()
}

#[update(guard = "is_authorized")]
fn reset_electricity_data() -> bool {
    let mut data = ElectricityData::default();
    data.total_kwh = 0.0;
    storage::stable_save((data,)).unwrap();
    true
}

fn is_authorized() -> Result<(), String> {
    let caller = ic_cdk::api::caller();
    let admins: Vec<Principal> = storage::stable_restore().unwrap().1;
    if admins.contains(&caller) {
        Ok(())
    } else {
        Err("Unauthorized".to_string())
    }
}

fn load_data() -> Result<ElectricityData, String> {
    storage::stable_restore()
        .map(|(data, _)| data)
        .map_err(|e| format!("Storage error: {:?}", e))
}

fn validate_reading(timestamp: u64) -> Result<(), String> {
    let current_time = ic_cdk::api::time();
    if timestamp > current_time + 60_000_000_000 { // 1 minute future tolerance
        Err("Invalid timestamp".to_string())
    } else {
        Ok(())
    }
}