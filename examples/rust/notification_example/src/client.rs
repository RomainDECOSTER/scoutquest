//! # Client for the notification service
//!
//! This module provides a simple client to interact with the notification service
//! via the ScoutQuest service discovery system.

use crate::types::*;
use anyhow::Result;
use scoutquest_rust::ServiceDiscoveryClient;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

/// Client for the notification service
///
/// Uses ScoutQuest to discover and communicate with the service.
#[derive(Clone)]
pub struct NotificationClient {
    scoutquest: Arc<ServiceDiscoveryClient>,
    service_name: String,
}

impl NotificationClient {
    /// Creates a new notification client
    ///
    /// # Arguments
    /// * `scoutquest_url` - URL of the ScoutQuest server
    /// * `service_name` - Name of the notification service (default: "notification-service")
    pub fn new(scoutquest_url: &str, service_name: Option<String>) -> Result<Self> {
        let scoutquest = Arc::new(ServiceDiscoveryClient::new(scoutquest_url)?);
        let service_name = service_name.unwrap_or_else(|| "notification-service".to_string());

        Ok(Self {
            scoutquest,
            service_name,
        })
    }

    /// Creates a new notification
    pub async fn create_notification(&self, request: CreateNotificationRequest) -> Result<Notification> {
        let body = json!(request);
        self.scoutquest.post(&self.service_name, "/api/notifications", body).await
            .map_err(|e| anyhow::anyhow!("Error creating notification: {}", e))
    }

    /// Retrieves a notification by its ID
    pub async fn get_notification(&self, id: Uuid) -> Result<Option<Notification>> {
        let path = format!("/api/notifications/{}", id);

        match self.scoutquest.get::<Notification>(&self.service_name, &path).await {
            Ok(notification) => Ok(Some(notification)),
            Err(_) => {
                // If the error is a 404, we return None, otherwise we propagate the error
                // For simplicity, we consider any error as "not found"
                // In production, we could inspect the error more carefully
                Ok(None)
            }
        }
    }

    /// Lists notifications with optional search parameters
    pub async fn list_notifications(&self, query: Option<NotificationQuery>) -> Result<NotificationList> {
        let mut path = "/api/notifications".to_string();

        // Add query parameters if provided
        if let Some(q) = query {
            let mut params = Vec::new();

            if let Some(recipient) = q.recipient {
                params.push(format!("recipient={}", urlencoding::encode(&recipient)));
            }
            if let Some(channel) = q.channel {
                params.push(format!("channel={}", serde_json::to_string(&channel).unwrap_or_default()));
            }
            if let Some(status) = q.status {
                params.push(format!("status={}", serde_json::to_string(&status).unwrap_or_default()));
            }
            if let Some(priority) = q.priority {
                params.push(format!("priority={}", serde_json::to_string(&priority).unwrap_or_default()));
            }
            if let Some(limit) = q.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(offset) = q.offset {
                params.push(format!("offset={}", offset));
            }

            if !params.is_empty() {
                path.push('?');
                path.push_str(&params.join("&"));
            }
        }

        self.scoutquest.get(&self.service_name, &path).await
            .map_err(|e| anyhow::anyhow!("Error listing notifications: {}", e))
    }

    /// Sends a notification (changes its status to sent)
    pub async fn send_notification(&self, id: Uuid) -> Result<ActionResponse> {
        let path = format!("/api/notifications/{}/send", id);
        self.scoutquest.post(&self.service_name, &path, json!({})).await
            .map_err(|e| anyhow::anyhow!("Error sending notification: {}", e))
    }

    /// Cancels a notification
    pub async fn cancel_notification(&self, id: Uuid) -> Result<ActionResponse> {
        let path = format!("/api/notifications/{}/cancel", id);
        self.scoutquest.post(&self.service_name, &path, json!({})).await
            .map_err(|e| anyhow::anyhow!("Error cancelling notification: {}", e))
    }

    /// Checks the service health
    pub async fn health_check(&self) -> Result<ServiceHealth> {
        self.scoutquest.get(&self.service_name, "/health").await
            .map_err(|e| anyhow::anyhow!("Health check error: {}", e))
    }
}

/// Convenience function to create a client with default parameters
pub fn create_client(scoutquest_url: &str) -> Result<NotificationClient> {
    NotificationClient::new(scoutquest_url, None)
}

/// Convenience function to create a client with a custom service name
pub fn create_client_with_service(scoutquest_url: &str, service_name: &str) -> Result<NotificationClient> {
    NotificationClient::new(scoutquest_url, Some(service_name.to_string()))
}
