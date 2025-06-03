struct KeyManager {
    current: Arc<Mutex<HashMap<u64, DeviceAuth>>>,
    pending_rotation: Arc<Mutex<HashMap<u64, (Vec<u8>, u32)>>>
}
// rotate keys every 24 hours
async fn rotate_keys_heartbeat() {
    ic_cdk::println!("Initiating key rotation at timestamp: {}", ic_cdk::api::time());
    let mut success_count = 0;

    for (device_id, auth) in &*KEY_MANAGER.current.lock().await {
        match KEY_MANAGER.validate_with_external_canister(*device_id).await {
            Ok(_) => success_count += 1,
            Err(e) => ic_cdk::println!("Rotation failed for {}: {}", device_id, e),
        }
    }

    ic_cdk::println!("Key rotation completed. {} devices updated successfully.", success_count);
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