use ic_cdk_macros::{query, update};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct Device {
    id: String,
    device_type: String,
    config: String,
    firmware_version: String,
    last_heartbeat: u64,
    registered_at: u64,
}

static mut DEVICES: Option<HashMap<String, Device>> = None;

#[init]
fn init() {
    unsafe { DEVICES = Some(HashMap::new()) };
}

#[update]
fn register_device(device_id: String, jwt_token: String) -> Result<String, String> {
    let claims = auth_backend::validate_token(&jwt_token)?;
    
    if !claims.roles.contains(&UserRole::DeviceManager) {
        return Err("Insufficient permissions for device registration".into());
    }
    ic_cdk::println!("Registering device: {}", device_id);
    
    unsafe {
        if DEVICES.as_ref().unwrap().contains_key(&device_id) {
            return Err("Device already exists".to_string());
        }
        
        let device = Device {
            id: device_id.clone(),
            device_type,
            config,
            firmware_version: "1.0.0".to_string(),
            last_heartbeat: ic_cdk::api::time(),
            registered_at: ic_cdk::api::time(),
        };
        
        DEVICES.as_mut().unwrap().insert(device_id, device);
        Ok("Device registered successfully".to_string())
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);
                Ok(())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[query]
fn get_device_info(device_id: String) -> Result<Device, String> {
    unsafe {
        DEVICES
            .as_ref()
            .unwrap()
            .get(&device_id)
            .cloned()
            .ok_or_else(|| "Device not found".to_string())
    }
}

#[update]
fn update_firmware(device_id: String, version: String) -> Result<String, String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.firmware_version = version;
                ic_cdk::println!("Updated firmware for device: {} to {}", device_id, device.firmware_version);
                Ok("Firmware updated successfully".to_string())
            }
            None => Err("Device not found".to_string()),
        }
    }
}

#[update]
fn update_device_status(device_id: String, status: String) -> Result<(), String> {
    unsafe {
        match DEVICES.as_mut().unwrap().get_mut(&device_id) {
            Some(device) => {
                device.last_heartbeat = ic_cdk::api::time();
                ic_cdk::println!("Updated status for device: {}", device_id);