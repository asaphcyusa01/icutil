struct KeyManager {
    current: Arc<Mutex<HashMap<u64, DeviceAuth>>>,
    pending_rotation: Arc<Mutex<HashMap<u64, (Vec<u8>, u32)>>>
}
// rotate keys every 24 hours
async fn rotate_keys_heartbeat() {
    // After rotation, validate with external canisters
    for (device_id, auth) in &*KEY_MANAGER.current.lock().await {
        if let Err(e) = KEY_MANAGER.validate_with_external_canister(*device_id).await {
            ic_cdk::println!("Validation failed for device {}: {}", device_id, e);
        }
    }
}
impl KeyManager {
    async fn validate_with_external_canister(&self, device_id: u64) -> Result<bool, String> {
        // Cross-canister call to authorization service
        let result: Result<(bool,), _> = ic_cdk::api::call::call(
            auth.auth_canister_id,
            "validate_key",
            (&auth.public_key,)
        ).await;

        result.map(|(valid,)| valid).map_err(|e| format!("Call failed: {e:?}"))
    }
}