struct KeyManager {
    current: Arc<Mutex<HashMap<u64, DeviceAuth>>>,
    pending_rotation: Arc<Mutex<HashMap<u64, (Vec<u8>, u32)>>>
}
// rotate keys every 24 hours
async fn rotate_keys_heartbeat() {
}