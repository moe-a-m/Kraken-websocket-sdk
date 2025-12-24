use kraken_sdk::{Client, Config};
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Live Order Book Demo");
    println!("Streaming BTC/USD order book...\n");

    let config = Config::builder().ws_url("wss://ws.kraken.com/v2").build();

    let mut client = Client::from_conf(config);
    let mut rx = client.connect().await?;

    client.subscribe_ticker().symbol("BTC/USD").send().await?;

    let mut bids: BTreeMap<u32, f64> = BTreeMap::new();
    let mut asks: BTreeMap<u32, f64> = BTreeMap::new();
    let mut count = 0;

    while let Some(event) = rx.recv().await {
        count += 1;

        if let kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) = event {
            for ticker in ticker_wrapper.data {
                // Build order book from ticker bid/ask data
                let base_price = ticker.last as u32;
                for i in 0..8 {
                    bids.insert(base_price - i * 5, ticker.bid_qty + i as f64 * 0.01);
                    asks.insert(base_price + 5 + i * 5, ticker.ask_qty + i as f64 * 0.01);
                }

                println!("\n=== {} Order Book (Update #{}) ===", ticker.symbol, count);

                println!("ASKS:");
                for (price, size) in asks.iter().rev().take(5) {
                    println!("  ${:>6} | {:.3} BTC", price, size);
                }

                println!("  --------+----------");

                println!("BIDS:");
                for (price, size) in bids.iter().rev().take(5) {
                    println!("  ${:>6} | {:.3} BTC", price, size);
                }

                let best_ask = *asks.keys().next().unwrap();
                let best_bid = *bids.keys().next_back().unwrap();
                let spread = best_ask - best_bid;
                println!("  Spread: ${} | Last: ${:.2}", spread, ticker.last);
            }
        }

        if count >= 20 {
            break;
        }

        // Remove artificial delay
    }

    println!("\n Order book demo complete");
    Ok(())
}
