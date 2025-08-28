//! # ScoutQuest Rust SDK
//!
//! This SDK allows easy interaction with ScoutQuest Service Discovery.
//! It provides registration and discovery functionalities
//! for your Rust microservices.
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use scoutquest_rust::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create the client
//!     let client = ServiceDiscoveryClient::new("http://localhost:8080")?;
//!
//!     // Register the service
//!     let options = ServiceRegistrationOptions::new()
//!         .with_tags(vec!["api".to_string(), "microservice".to_string()]);
//!
//!     client.register_service("user-service", "localhost", 3000, Some(options)).await?;
//!
//!     // Discover other services
//!     let instance = client.discover_service("user-service", None).await?;
//!
//!     // Call another service
//!     let response: serde_json::Value = client.get("user-service", "/api/users").await?;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;

pub use client::ServiceDiscoveryClient;
pub use error::ScoutQuestError;
pub use models::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
