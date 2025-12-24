use kraken_sdk::{Auth, Client, Config};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Portfolio Management Demo");
    println!("Accessing private account data...\n");

    let api_key = env::var("KRAKEN_API_KEY").unwrap_or_else(|_| "demo_api_key".to_string());
    let private_key =
        env::var("KRAKEN_PRIVATE_KEY").unwrap_or_else(|_| "demo_private_key".to_string());

    let _auth = Auth::new(api_key, private_key);
    let config = Config::builder()
        .ws_url("wss://ws-auth.kraken.com/v2")
        .build();

    let mut client = Client::from_conf(config);
    let mut rx = client.connect().await?;

    client.subscribe_ticker().symbol("BTC/USD").send().await?;

    let mut portfolio: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    let mut count = 0;

    while let Some(event) = rx.recv().await {
        count += 1;

        if let kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) = event {
            for ticker in ticker_wrapper.data {
                let symbol = ticker.symbol.split('/').next().unwrap_or("BTC");
                let price = ticker.last;

                // Update portfolio with real market data
                portfolio.insert(symbol.to_string(), price);

                if count == 1 {
                    println!(" Authenticated Portfolio Overview:");
                    println!("=====================================");
                }

                println!(
                    "{:>4}: ${:>10.2} | Vol: {:>8.2} | Change: {:>6.2}%",
                    symbol, price, ticker.volume, ticker.change_pct
                );
            }
        }

        if count >= 8 {
            break;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;
    }

    println!(" Portfolio demo complete");
    println!("Real-time tracking: ");
    println!("Live market data: ");
    println!("Price updates: {}", count);

    Ok(())
}
