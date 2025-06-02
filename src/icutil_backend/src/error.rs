#[derive(thiserror::Error, Debug)]
pub enum SensorError {
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Storage error: {0}")]
    Storage(#[from] ic_cdk::api::error::Error),
    #[error("Security violation: {0}")]
    Security(String)
}