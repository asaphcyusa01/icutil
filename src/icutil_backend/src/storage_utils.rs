pub const BACKUP_INTERVAL: u64 = 86400_000_000_000; // 24h in nanoseconds
pub const MAX_BACKUPS: usize = 30;
pub const RETENTION_DAYS: u64 = 30;

pub fn create_backup() -> Result<String, FlowError> {
    let last_backup = storage::get::<u64>("last_backup").unwrap_or(0);

    if ic_cdk::api::time() - last_backup < BACKUP_INTERVAL {
        return Err(FlowError::StorageError("Too frequent backups".into()));
    }

    let backup_data = storage::stable_restore()
        .map_err(|e| FlowError::StorageError(format!("Backup failed: {e:?}")))?;

    storage::stable_save((backup_data, "backup".to_string()))
        .map_err(|e| FlowError::StorageError(format!("Backup storage failed: {e:?}")))?;

    Ok("Backup created successfully".into())
}

pub fn enforce_retention() -> Result<(), FlowError> {
    let mut backups: Vec<(u64, String)> = storage::get("backups").unwrap_or_default();
    let now = ic_cdk::api::time();

    backups.retain(|(ts, _)| now - ts < RETENTION_DAYS * 86400_000_000_000);

    if backups.len() > MAX_BACKUPS {
        backups.drain(0..backups.len()-MAX_BACKUPS);
    }

    storage::stable_save(backups)
        .map_err(|e| FlowError::StorageError(format!("Retention enforcement failed: {e:?}")))
}

const NUM_SHARDS: usize = 8;

fn get_shard_key(device_id: &Option<String>) -> u64 {
    device_id.as_ref()
        .map(|id| seahash::hash(id.as_bytes()) % NUM_SHARDS as u64)
        .unwrap_or(0)
}

for shard in 0..NUM_SHARDS {
    let data = storage::stable_restore(shard)?;
    backups.insert(shard, data);
}