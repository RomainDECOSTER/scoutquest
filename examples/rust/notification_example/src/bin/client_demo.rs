//! # Client Demo Binary
//! 
//! Demonstration of the notification client usage.

use anyhow::Result;
use clap::Parser;
use notification_example::{
    client::NotificationClient,
    types::{CreateNotificationRequest, Channel, Priority, NotificationQuery},
};
use std::collections::HashMap;

#[derive(Parser)]
#[command(
    name = "client-demo",
    about = "Notification client demonstration"
)]
struct Args {
    /// ScoutQuest server URL
    #[arg(short = 'u', long, default_value = "http://localhost:8080")]
    scoutquest_url: String,

    /// Notification service name
    #[arg(short = 'n', long, default_value = "notification-service")]
    service_name: String,

    /// Action to perform
    #[arg(short, long, default_value = "demo")]
    action: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("🚀 Notification client demonstration");
    println!("   ScoutQuest URL: {}", args.scoutquest_url);
    println!("   Service: {}", args.service_name);

    // Create client
    let client = NotificationClient::new(&args.scoutquest_url, Some(args.service_name))?;

    match args.action.as_str() {
        "demo" => run_full_demo(&client).await?,
        "health" => check_health(&client).await?,
        "create" => create_test_notification(&client).await?,
        "list" => list_notifications(&client).await?,
        _ => {
            println!("❌ Unknown action: {}. Available actions: demo, health, create, list", args.action);
        }
    }

    Ok(())
}

/// Complete client demonstration
async fn run_full_demo(client: &NotificationClient) -> Result<()> {
    println!("\n🔍 1. Checking service health...");
    match client.health_check().await {
        Ok(health) => {
            println!("✅ Service is healthy:");
            println!("   Status: {}", health.status);
            println!("   Pending notifications: {}", health.pending_notifications);
            println!("   Processed today: {}", health.processed_today);
        }
        Err(e) => {
            println!("❌ Health check error: {}", e);
            return Err(e);
        }
    }

    println!("\n📧 2. Creating an email notification...");
    let mut metadata = HashMap::new();
    metadata.insert("user_id".to_string(), "12345".to_string());
    metadata.insert("campaign".to_string(), "welcome".to_string());

    let email_request = CreateNotificationRequest {
        recipient: "alice@example.com".to_string(),
        channel: Channel::Email,
        subject: Some("Welcome!".to_string()),
        content: "Welcome to our notification service!".to_string(),
        priority: Some(Priority::Normal),
        scheduled_at: None,
        metadata: Some(metadata),
    };

    let email_notification = client.create_notification(email_request).await?;
    println!("✅ Email created: {}", email_notification.id);

    println!("\n📱 3. Creating a critical push notification...");
    let push_request = CreateNotificationRequest {
        recipient: "user:67890".to_string(),
        channel: Channel::Push,
        subject: None,
        content: "Security alert: suspicious login detected".to_string(),
        priority: Some(Priority::Critical),
        scheduled_at: None,
        metadata: None,
    };

    let push_notification = client.create_notification(push_request).await?;
    println!("✅ Push created: {}", push_notification.id);

    println!("\n📤 4. Sending email notification...");
    let send_result = client.send_notification(email_notification.id).await?;
    if send_result.success {
        println!("✅ {}", send_result.message);
    } else {
        println!("⚠️ {}", send_result.message);
    }

    println!("\n📋 5. Listing notifications...");
    let query = NotificationQuery {
        limit: Some(10),
        ..Default::default()
    };
    let list = client.list_notifications(Some(query)).await?;
    println!("✅ {} notifications found (total: {})", list.notifications.len(), list.total);

    for notification in &list.notifications {
        println!("   📝 {}: {:?} -> {} [{:?}]", 
            notification.id, 
            notification.channel,
            notification.recipient,
            notification.status
        );
    }

    println!("\n🔍 6. Retrieving a specific notification...");
    if let Some(retrieved) = client.get_notification(push_notification.id).await? {
        println!("✅ Notification retrieved:");
        println!("   ID: {}", retrieved.id);
        println!("   Channel: {:?}", retrieved.channel);
        println!("   Status: {:?}", retrieved.status);
        println!("   Content: {}", retrieved.content);
    }

    println!("\n🚫 7. Cancelling push notification...");
    let cancel_result = client.cancel_notification(push_notification.id).await?;
    if cancel_result.success {
        println!("✅ {}", cancel_result.message);
    } else {
        println!("⚠️ {}", cancel_result.message);
    }

    println!("\n🎉 Demonstration completed successfully!");
    Ok(())
}

/// Simple health check
async fn check_health(client: &NotificationClient) -> Result<()> {
    println!("\n🔍 Health check...");
    let health = client.health_check().await?;
    println!("Status: {}", health.status);
    println!("Timestamp: {}", health.timestamp);
    println!("Pending notifications: {}", health.pending_notifications);
    println!("Processed today: {}", health.processed_today);
    Ok(())
}

/// Create a test notification
async fn create_test_notification(client: &NotificationClient) -> Result<()> {
    println!("\n📧 Creating a test notification...");
    let request = CreateNotificationRequest {
        recipient: "test@example.com".to_string(),
        channel: Channel::Email,
        subject: Some("Test".to_string()),
        content: "This is a test of the notification service.".to_string(),
        priority: Some(Priority::Low),
        scheduled_at: None,
        metadata: None,
    };

    let notification = client.create_notification(request).await?;
    println!("✅ Notification created: {}", notification.id);
    println!("   Recipient: {}", notification.recipient);
    println!("   Channel: {:?}", notification.channel);
    println!("   Priority: {:?}", notification.priority);
    Ok(())
}

/// List notifications
async fn list_notifications(client: &NotificationClient) -> Result<()> {
    println!("\n📋 Notification list...");
    let list = client.list_notifications(None).await?;
    
    if list.notifications.is_empty() {
        println!("No notifications found.");
    } else {
        println!("{} notification(s) found:", list.notifications.len());
        for notification in list.notifications {
            println!("  📝 {} | {} | {:?} | {:?} | {}", 
                notification.id,
                notification.recipient,
                notification.channel,
                notification.status,
                notification.created_at.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }
    Ok(())
}
