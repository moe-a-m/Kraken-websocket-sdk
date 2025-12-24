use kraken_sdk::{Client, Config};
use std::collections::HashMap;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Arbitrage Opportunity Scanner");
    println!("Scanning for cross-exchange price differences...\n");

    let config = Config::builder().ws_url("wss://ws.kraken.com/v2").build();

    let mut client = Client::from_conf(config);
    let mut rx = client.connect().await?;

    // Major trading pairs for arbitrage
    let symbols = vec!["BTC/USD", "ETH/USD", "ADA/USD", "DOT/USD", "LINK/USD"];

    for symbol in &symbols {
        client.subscribe_ticker().symbol(*symbol).send().await?;
    }

    let mut prices: HashMap<String, f64> = HashMap::new();
    let mut opportunities = 0;
    let start = Instant::now();
    let mut count = 0;

    println!(
        "Monitoring {} pairs for arbitrage opportunities...\n",
        symbols.len()
    );

    while let Some(event) = rx.recv().await {
        count += 1;

        if let kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) = event {
            for ticker in ticker_wrapper.data {
                prices.insert(ticker.symbol.clone(), ticker.last);

                // Compare bid/ask spread as proxy for arbitrage opportunity
                let spread_pct = ((ticker.ask - ticker.bid) / ticker.last) * 100.0;

                if spread_pct > 0.5 {
                    opportunities += 1;
                    println!(
                        " OPPORTUNITY #{}: {} | Spread: {:.2}% | Bid: ${:.2} | Ask: ${:.2}",
                        opportunities, ticker.symbol, spread_pct, ticker.bid, ticker.ask
                    );
                }

                if count % 10 == 0 {
                    println!(
                        " {} | Price: ${:.2} | Vol: {:.2} | Change: {:+.2}%",
                        ticker.symbol, ticker.last, ticker.volume, ticker.change_pct
                    );
                }
            }
        }

        if count >= 200 {
            break;
        }

        // Remove artificial delay
    }

    let total_time = start.elapsed().as_secs_f64();

    println!("\n Arbitrage Scan Results:");
    println!("Duration: {:.1}s", total_time);
    println!("Price updates: {}", count);
    println!("Spread opportunities: {}", opportunities);
    println!("Real-time analysis: ");

    Ok(())
}
