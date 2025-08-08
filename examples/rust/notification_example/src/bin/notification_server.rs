//! # Notification Server Binary
//! 
//! Starts the notification server that automatically registers with ScoutQuest.

use anyhow::Result;
use clap::Parser;
use notification_example::server::start_server;

#[derive(Parser)]
#[command(
    name = "notification-server",
    about = "Notification server with ScoutQuest service discovery"
)]
struct Args {
    /// Port to start the server on
    #[arg(short, long, default_value = "3001")]
    port: u16,

    /// ScoutQuest server URL
    #[arg(short = 'u', long, default_value = "http://localhost:8080")]
    scoutquest_url: String,

    /// Service name (optional)
    #[arg(short = 'n', long)]
    service_name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    let args = Args::parse();

    println!("🔧 Configuration:");
    println!("   Port: {}", args.port);
    println!("   ScoutQuest URL: {}", args.scoutquest_url);
    println!("   Service Name: {}", args.service_name.as_deref().unwrap_or("notification-service"));

    // Start the server
    start_server(args.port, &args.scoutquest_url, args.service_name).await?;

    Ok(())
}
