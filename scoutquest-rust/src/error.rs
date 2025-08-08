use thiserror::Error;

/// Errors that can occur when using the ScoutQuest Rust SDK.
/// 
/// This enum covers all possible error conditions including network failures,
/// service discovery issues, and protocol-level errors.
#[derive(Error, Debug)]
pub enum ScoutQuestError {
    /// Network-related errors (connection failures, timeouts, etc.)
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// The requested service was not found in the discovery registry
    #[error("Service isn't found: {service_name}")]
    ServiceNotFound { service_name: String },

    /// A specific service instance was not found
    #[error("Instance isn't found: {instance_id}")]
    InstanceNotFound { instance_id: String },

    /// Service registration failed with the discovery server
    #[error("Registration failed: {status} - {message}")]
    RegistrationFailed { status: u16, message: String },

    /// JSON serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Invalid URL format provided
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// The ScoutQuest discovery server is not available
    #[error("ScoutQuest Server unavailable")]
    ServerUnavailable,

    /// Operation timed out
    #[error("Operation timeout")]
    Timeout,

    /// No healthy instances are available for the requested service
    #[error("No healthy instances available for service: {service_name}")]
    NoHealthyInstances { service_name: String },

    /// Internal error or unexpected condition
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Convenience type alias for Results in the ScoutQuest SDK.
pub type Result<T> = std::result::Result<T, ScoutQuestError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ScoutQuestError::ServiceNotFound {
            service_name: "test-service".to_string(),
        };
        assert_eq!(error.to_string(), "Service isn't found: test-service");

        let error = ScoutQuestError::InstanceNotFound {
            instance_id: "instance-123".to_string(),
        };
        assert_eq!(error.to_string(), "Instance isn't found: instance-123");

        let error = ScoutQuestError::RegistrationFailed {
            status: 500,
            message: "Internal server error".to_string(),
        };
        assert_eq!(error.to_string(), "Registration failed: 500 - Internal server error");

        let error = ScoutQuestError::NoHealthyInstances {
            service_name: "api-service".to_string(),
        };
        assert_eq!(error.to_string(), "No healthy instances available for service: api-service");

        let error = ScoutQuestError::InternalError("Something went wrong".to_string());
        assert_eq!(error.to_string(), "Internal error: Something went wrong");

        let error = ScoutQuestError::ServerUnavailable;
        assert_eq!(error.to_string(), "ScoutQuest Server unavailable");

        let error = ScoutQuestError::Timeout;
        assert_eq!(error.to_string(), "Operation timeout");
    }

    #[test]
    fn test_error_debug() {
        let error = ScoutQuestError::ServiceNotFound {
            service_name: "test".to_string(),
        };
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("ServiceNotFound"));
        assert!(debug_str.contains("test"));
    }
}