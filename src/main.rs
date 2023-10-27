use crate::errors::Result;
use reqwest::Client;

mod config;
mod errors;

use config::initialize_config;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize your configuration
    let _config = initialize_config();

    // Create a Reqwest client with Rustls as the TLS backend
    let client = Client::builder().build()?;

    // Make an HTTP request
    let _response = client.get("https://example.com").send().await?;

    dbg!(_response);

    Ok(())
}
