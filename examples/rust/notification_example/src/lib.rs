//! # Notification Service Library
//! 
//! This library provides a client to interact with the notification service
//! via ScoutQuest, as well as all necessary types.
//! 
//! ## Usage
//! 
//! ```rust
//! use notification_example::{
//!     client::NotificationClient,
//!     types::{CreateNotificationRequest, Channel, Priority},
//! };
//! 
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = NotificationClient::new("http://localhost:8080", None)?;
//!     
//!     let request = CreateNotificationRequest {
//!         recipient: "user@example.com".to_string(),
//!         channel: Channel::Email,
//!         subject: Some("Test".to_string()),
//!         content: "Hello world!".to_string(),
//!         priority: Some(Priority::High),
//!         scheduled_at: None,
//!         metadata: None,
//!     };
//!     
//!     let notification = client.create_notification(request).await?;
//!     println!("Notification created: {}", notification.id);
//!     
//!     Ok(())
//! }
//! ```

pub mod types;
pub mod client;
pub mod server;

// Re-exports for easier usage
pub use client::{NotificationClient, create_client, create_client_with_service};
pub use types::*;
