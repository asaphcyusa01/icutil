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
}

#[init]
fn init() {
    let mut data = ElectricityData::default();
    data.total_kwh = 0.0;
    storage::stable_save((data,)).unwrap();
}

#[update]
fn add_electricity_reading(kwh: f64) -> bool {
    let mut data: ElectricityData = storage::stable_restore().unwrap().0;
    let timestamp = ic_cdk::api::time();
    let reading = ElectricityReading { timestamp, kwh };
    data.readings.insert(timestamp, reading);
    data.total_kwh += kwh;
    storage::stable_save((data,)).unwrap();
    true
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