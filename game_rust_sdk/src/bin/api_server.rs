use std::env;
use std::process;
use dotenv::dotenv;
use game_rust_sdk::api;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up the tracing subscriber");

    // Load environment variables
    dotenv().ok();

    // Get port from environment variable or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or_else(|e| {
            eprintln!("Invalid PORT specified: {}", e);
            process::exit(1);
        });

    // Start the API server
    info!("Starting API server on port {}", port);
    api::start_server(port).await?;

    Ok(())
} 