#[macro_use]
extern crate serde_derive;
extern crate serde;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::collections::{HashMap, BTreeMap};
use ic_cdk::api::Principal;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
struct WaterReading {
    timestamp: u64,
    liters: f64,
}

#[derive(CandidType, Serialize, Deserialize, Default)]
struct WaterData {
    readings: HashMap<u64, WaterReading>,
    total_liters: f64,
    daily_usage: BTreeMap<u64, f64>,
    anomaly_count: u32,
    rate_limits: HashMap<Principal, (u64, u64)>,
}

#[init]
fn init() {
    let mut data = WaterData::default();
    data.total_liters = 0.0;
    storage::stable_save((data,)).unwrap();
}

#[update]
fn add_water_reading(liters: f64) -> Result<bool, String> {
    if liters < 0.0 {
        return Err("Negative water usage impossible".into());
    }
    // Flow validation and metrics tracking
}

#[query]
fn get_total_water_usage() -> f64 {
    let data: WaterData = storage::stable_restore().unwrap().0;
    data.total_liters
}

#[query]
fn get_water_readings(skip: u64, limit: u64) -> Vec<WaterReading> {
    let data: WaterData = storage::stable_restore().unwrap().0;
    data.readings.values()
       .skip(skip as usize)
       .take(limit as usize)
       .cloned()
       .collect()
}

#[update]
fn reset_water_data() -> bool {
    let mut data = WaterData::default();
    data.total_liters = 0.0;
    storage::stable_save((data,)).unwrap();
    true
}

#[query]
fn get_water_readings_filtered(skip: u64, limit: u64, from: u64, to: u64) -> Vec<WaterReading> {
    let data: WaterData = storage::stable_restore().unwrap().0;
    data.readings.values()
        .filter(|r| r.timestamp >= from && r.timestamp <= to)
        .skip(skip as usize)
        .take(limit as usize)
        .cloned()
        .collect()
}

#[query]
fn get_daily_usage() -> Vec<(u64, f64)> {
    let data: WaterData = storage::stable_restore().unwrap().0;
    let mut daily: BTreeMap<u64, f64> = BTreeMap::new();
    
    for reading in data.readings.values() {
        let day = reading.timestamp / 86400_000_000_000; // Group by day
        *daily.entry(day).or_insert(0.0) += reading.liters;
    }
    
    daily.into_iter().collect()
}

const RATE_LIMIT_WINDOW: u64 = 60_000_000_000; // 1 minute in nanoseconds
const MAX_REQUESTS_PER_MINUTE: u64 = 100;

#[update]
fn check_rate_limit(caller: Principal) -> Result<(), String> {
    let mut data = storage::stable_restore()
    .map_err(|e| format!("Failed to restore data: {:?}", e))?.0;
    let now = ic_cdk::api::time();
    
    let (first_request, count) = data.rate_limits
        .get(caller)
        .unwrap_or(&(now, 0));

    let window_start = if now - first_request > RATE_LIMIT_WINDOW {
        // New time window
        now
    } else {
        *first_request
    };

    let new_count = if window_start == *first_request {
        count + 1
    } else {
        1
    };

    if new_count > MAX_REQUESTS_PER_MINUTE {
        return Err("Rate limit exceeded. Try again later.".to_string());
    }

    data.rate_limits.insert(*caller, (window_start, new_count));
    storage::stable_save((data,))
    .map_err(|e| format!("Failed to persist rate limit: {:?}", e))?;

IC_COUNTERS.with(|c| {
    let mut counters = c.borrow_mut();
    counters.rate_limit_checks += 1;
});
    
    Ok(())
}

#[update]
async fn record_reading(flow_rate: f64) -> Result<(), String> {
    // Existing recording logic
    backup_backend::create_backup("water_data".into(), data).await?;
}