use kraken_sdk::{Client, Config};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Default configuration
    let default_config = Config::builder().build();
    println!("Default config: {:?}", default_config);

    // Custom configuration
    let custom_config = Config::builder()
        .ws_url("wss://ws.kraken.com/v2")
        .max_retries(5)
        .initial_backoff(Duration::from_millis(500))
        .build();

    let mut client = Client::from_conf(custom_config);
    let _stream = client.stream().await?;

    println!("Client configured with custom settings");
    Ok(())
}
