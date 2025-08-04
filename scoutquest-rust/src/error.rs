use thiserror::Error;


#[derive(Error, Debug)]
pub enum ScoutQuestError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Service isn't found: {service_name}")]
    ServiceNotFound { service_name: String },

    #[error("Instance isn't found: {instance_id}")]
    InstanceNotFound { instance_id: String },

    #[error("Registration failed: {status} - {message}")]
    RegistrationFailed { status: u16, message: String },

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("ScoutQuest Server unavailable")]
    ServerUnavailable,

    #[error("Operation timeout")]
    Timeout,

    #[error("No healthy instances available for service: {service_name}")]
    NoHealthyInstances { service_name: String },

    #[error("Internal error: {0}")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, ScoutQuestError>;