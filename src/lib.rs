use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_rustls::TlsConnector;

mod config;
mod errors;

use config::initialize_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize your configuration
    let config = initialize_config();

    // Create a Reqwest client with Rustls as the TLS backend
    let client = Client::builder().build()?;

    // Make an HTTP request
    let response = client.get("https://example.com").send().await?;

    // Handle the response here
    // ...

    Ok(())
}
