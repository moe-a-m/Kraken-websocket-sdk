use kraken_sdk::{Auth, Client, Config};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Private API Demo");
    println!("Connecting with API key authentication...\n");

    // Get credentials from environment variables
    let api_key = env::var("KRAKEN_API_KEY").unwrap_or_else(|_| "your_api_key_here".to_string());
    let private_key = env::var("KRAKEN_PRIVATE_KEY")
        .unwrap_or_else(|_| "dGVzdF9wcml2YXRlX2tleV9oZXJl".to_string()); // base64 encoded test key

    if api_key == "your_api_key_here" || private_key == "dGVzdF9wcml2YXRlX2tleV9oZXJl" {
        println!("  Using demo credentials. Set environment variables:");
        println!("   export KRAKEN_API_KEY=your_actual_api_key");
        println!("   export KRAKEN_PRIVATE_KEY=your_actual_private_key\n");
    }

    // Create authenticated client
    let auth = Auth::new(api_key, private_key);
    let config = Config::builder()
        .ws_url("wss://ws-auth.kraken.com/v2")
        .build();

    let mut client = Client::from_conf(config);

    // Generate authentication signature
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let nonce = timestamp * 1000;
    let auth_message = format!("{}POST/ws/v2{}", nonce, timestamp);
    let signature = auth.sign(&auth_message)?;

    println!(" Authentication Details:");
    println!("   API Key: {}...", &auth.api_key()[..8]);
    println!("   Signature: {}...", &signature[..16]);
    println!("   Nonce: {}", nonce);
    let mut rx = client.connect().await?;

    // Subscribe to private channels (requires authentication)
    client.subscribe_ticker().symbol("BTC/USD").send().await?;

    println!(" Connected with private API credentials");
    println!(" Streaming authenticated data...\n");

    let mut count = 0;
    while let Some(event) = rx.recv().await {
        count += 1;

        match event {
            kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) => {
                for ticker in ticker_wrapper.data {
                    println!(
                        " Private Event #{}: {} ticker update",
                        count, ticker.symbol
                    );
                    println!(
                        "   Price: ${:.2} | Bid: ${:.2} | Ask: ${:.2}",
                        ticker.last, ticker.bid, ticker.ask
                    );
                    println!(
                        "   Volume: {:.3} | Change: {:.2}%",
                        ticker.volume, ticker.change_pct
                    );
                }
            }
            _ => {
                println!(" Private Event #{}: {:?}", count, event);
            }
        }

        // Demonstrate order placement with private API
        if count == 5 {
            println!("\n📝 Placing authenticated order...");

            use kraken_sdk::AddOrderRequest;
            let order = AddOrderRequest::limit_buy("BTC/USD", "0.001", "40000");

            match client.add_order(order).await {
                Ok(_) => println!(" Order placed successfully"),
                Err(e) => println!(" Order failed: {}", e),
            }
        }

        if count >= 10 {
            break;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    println!("\n Private API demo complete");
    println!("Authentication: ");
    println!("Private data access: ");
    println!("Order placement: ");

    Ok(())
}
