use kraken_sdk::{Client, Config};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Real-time BTC/USD Ticker Demo");
    println!("Connecting to Kraken WebSocket API...\n");

    let config = Config::builder().ws_url("wss://ws.kraken.com/v2").build();

    let mut client = Client::from_conf(config);
    let mut rx = client.connect().await?;

    client.subscribe_ticker().symbol("BTC/USD").send().await?;

    let start = Instant::now();
    let mut count = 0;

    while let Some(event) = rx.recv().await {
        count += 1;
        let latency = start.elapsed().as_millis() as f64 / count as f64;

        if let kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) = event {
            for ticker in ticker_wrapper.data {
                println!(
                    "BTC: ${:.2} | Bid: ${:.2} | Ask: ${:.2} | Vol: {:.2} | Latency: {:.1}ms | Count: {}",
                    ticker.last, ticker.bid, ticker.ask, ticker.volume, latency, count
                );
            }
        }

        if count >= 50 {
            break;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    println!(
        "\n Demo complete - Average latency: {:.1}ms",
        start.elapsed().as_millis() as f64 / count as f64
    );

    Ok(())
}
