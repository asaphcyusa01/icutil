use candid::CandidType;
use ic_cdk::storage;
use super::FlowError;

use crate::storage_utils::{BACKUP_INTERVAL, MAX_BACKUPS, RETENTION_DAYS};

pub struct StorageManager;

impl StorageManager {
    pub fn create_backup() -> Result<String, FlowError> {
        crate::storage_utils::create_backup()
    }

    pub fn enforce_retention() -> Result<(), FlowError> {
        crate::storage_utils::enforce_retention()
    }
}