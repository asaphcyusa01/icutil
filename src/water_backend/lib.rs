#[macro_use]
extern crate serde_derive;
extern crate serde;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::collections::{HashMap, BTreeMap};

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
}

#[init]
fn init() {
    let mut data = WaterData::default();
    data.total_liters = 0.0;
    storage::stable_save((data,)).unwrap();
}

#[update]
fn add_water_reading(liters: f64) -> Result<bool, String> {
    if liters < 0.0 || liters > 1000.0 { // Realistic flow limits
        return Err("Invalid reading value".to_string());
    }
    let mut data: WaterData = storage::stable_restore().unwrap().0;
    let timestamp = ic_cdk::api::time();
    let reading = WaterReading { timestamp, liters };
    data.readings.insert(timestamp, reading);
    data.total_liters += liters;
    storage::stable_save((data,)).unwrap();
    Ok(true)
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