use kraken_sdk::{Client, Config};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Live Trades Stream Demo");
    println!("Streaming BTC/USD trades...\n");

    let config = Config::builder().ws_url("wss://ws.kraken.com/v2").build();

    let mut client = Client::from_conf(config);
    let mut rx = client.connect().await?;

    client.subscribe_ticker().symbol("BTC/USD").send().await?;

    let start = Instant::now();
    let mut count = 0;
    let mut volume = 0.0;

    while let Some(event) = rx.recv().await {
        count += 1;

        if let kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) = event {
            for ticker in ticker_wrapper.data {
                let timestamp = start.elapsed().as_millis();
                volume += ticker.volume;

                println!(
                    "{:>6}ms | {} | ${:>8.2} | Vol: {:.3} | Change: {:+.2}%",
                    timestamp, ticker.symbol, ticker.last, ticker.volume, ticker.change_pct
                );
            }
        }

        if count >= 30 {
            break;
        }

        // Remove artificial delay
    }

    println!("\n Trades stream demo complete");
    println!("Total updates: {}", count);
    println!("Total volume tracked: {:.3}", volume);

    Ok(())
}
