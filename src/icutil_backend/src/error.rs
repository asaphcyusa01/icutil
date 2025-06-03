#[derive(Debug, thiserror::Error)]
pub enum SensorError {
    #[error("Validation failed: {0}")]
    Validation(String),
    
    #[error("Storage failure: {context}")]
    Storage {
        source: ic_cdk::api::error::Error,
        context: String,
        backtrace: Backtrace
    },

    #[error("Rate limited: {0}")]
    RateLimit(String)
}