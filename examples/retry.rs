use kraken_sdk::{Client, Config, Error};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure with aggressive retry settings
    let config = Config::builder()
        .ws_url("wss://invalid-url-for-demo.com") // This will fail
        .max_retries(3)
        .initial_backoff(Duration::from_millis(100))
        .build();

    let mut client = Client::from_conf(config);

    println!("Attempting connection with retries...");

    match client.stream().await {
        Ok(_stream) => {
            println!("Connected successfully!");
        }
        Err(e) => match e {
            Error::ConnectionClosed => {
                println!("Connection failed after all retries");
            }
            Error::UrlParse(url_err) => {
                println!("Invalid URL: {}", url_err);
            }
            _ => {
                println!("Other error: {}", e);
            }
        },
    }

    // Now try with valid URL
    let valid_config = Config::builder()
        .ws_url("wss://ws.kraken.com/v2")
        .max_retries(3)
        .initial_backoff(Duration::from_millis(1000))
        .build();

    let mut valid_client = Client::from_conf(valid_config);

    match valid_client.stream().await {
        Ok(_stream) => {
            println!("Successfully connected to Kraken!");
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    Ok(())
}
