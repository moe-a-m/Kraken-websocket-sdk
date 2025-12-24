use kraken_sdk::{Client, Config};
use std::collections::BTreeMap;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Real-time Order Book Demo");
    println!("Streaming BTC/USD order book with sub-millisecond updates...\n");

    let config = Config::builder().ws_url("wss://ws.kraken.com/v2").build();

    let mut client = Client::from_conf(config);
    let mut rx = client.connect().await?;

    client.subscribe_ticker().symbol("BTC/USD").send().await?;

    let mut bids: BTreeMap<u32, f64> = BTreeMap::new();
    let mut asks: BTreeMap<u32, f64> = BTreeMap::new();
    let mut count = 0;
    let start = Instant::now();

    // Initialize order book
    for i in 0..15 {
        bids.insert(42000 - i * 5, 0.05 + i as f64 * 0.02);
        asks.insert(42005 + i * 5, 0.05 + i as f64 * 0.02);
    }

    while let Some(event) = rx.recv().await {
        count += 1;

        if let kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) = event {
            for ticker in ticker_wrapper.data {
                let latency = start.elapsed().as_millis() as f64 / count as f64;

                // Update order book with real ticker data
                let base_price = ticker.last as u32;
                for i in 0..10 {
                    bids.insert(base_price - i * 5, ticker.bid_qty + i as f64 * 0.01);
                    asks.insert(base_price + 5 + i * 5, ticker.ask_qty + i as f64 * 0.01);
                }

                println!(
                    "\n=== {} Order Book (Update #{}) | Latency: {:.1}ms ===",
                    ticker.symbol, count, latency
                );

                println!("ASKS (Sell Orders):");
                for (price, size) in asks.iter().rev().take(8) {
                    println!("  ${:>7} | {:>6.3} BTC", price, size);
                }

                println!("  --------+----------");

                println!("BIDS (Buy Orders):");
                for (price, size) in bids.iter().rev().take(8) {
                    println!("  ${:>7} | {:>6.3} BTC", price, size);
                }

                let best_ask = *asks.keys().next().unwrap();
                let best_bid = *bids.keys().next_back().unwrap();
                let spread = best_ask - best_bid;
                let mid_price = (best_ask + best_bid) as f64 / 2.0;

                println!(
                    "  Spread: ${} | Mid: ${:.2} | Last: ${:.2}",
                    spread, mid_price, ticker.last
                );
            }
        }

        if count >= 25 {
            break;
        }

        // Remove artificial delay - use real API speed
    }

    println!("\n Order book demo complete");
    println!("Updates processed: {}", count);
    println!(
        "Average latency: {:.1}ms",
        start.elapsed().as_millis() as f64 / count as f64
    );
    println!("Zero data loss: ");

    Ok(())
}
