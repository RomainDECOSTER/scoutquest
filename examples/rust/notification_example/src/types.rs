//! # Shared types for the notification service
//!
//! This module contains all data types shared between client and server.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Priority levels for notifications
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Available notification channels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Channel {
    Email,
    Sms,
    Push,
    Webhook,
    InApp,
}

/// Notification status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
    Cancelled,
}

impl Default for NotificationStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Main notification structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub recipient: String,
    pub channel: Channel,
    pub subject: Option<String>,
    pub content: String,
    pub priority: Priority,
    pub status: NotificationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Request to create a new notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    pub recipient: String,
    pub channel: Channel,
    pub subject: Option<String>,
    pub content: String,
    pub priority: Option<Priority>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

/// Search parameters for listing notifications
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationQuery {
    pub recipient: Option<String>,
    pub channel: Option<Channel>,
    pub status: Option<NotificationStatus>,
    pub priority: Option<Priority>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Response containing a list of notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationList {
    pub notifications: Vec<Notification>,
    pub total: u64,
    pub has_more: bool,
}

/// Service health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub pending_notifications: u64,
    pub processed_today: u64,
}

/// Action response (send, cancel, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResponse {
    pub success: bool,
    pub message: String,
}
