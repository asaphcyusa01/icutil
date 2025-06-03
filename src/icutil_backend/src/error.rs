#[derive(Debug, thiserror::Error)]
pub enum SensorError {
    #[error("Validation failed: {0}")]
    Validation(String),
    
    #[error("Storage error: {context}")]
    Storage { 
        #[source]
        source: ic_cdk::api::error::Error,
        context: String
    },

    #[error("Rate limited: {0}")]
    RateLimit(String)
}