use kraken_sdk::{Client, Config, KrakenStream};
use std::time::Duration;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder().build();
    let mut client = Client::from_conf(config);
    let mut stream: KrakenStream = client.stream().await?;

    client
        .subscribe_ticker()
        .symbol("BTC/USD")
        .symbol("ETH/USD")
        .send()
        .await?;

    let mut count = 0;
    while let Some(event) = stream.next().await {
        count += 1;

        if let kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) = event {
            for ticker in ticker_wrapper.data {
                if count % 10 == 0 {
                    println!(
                        "{} | ${:.2} | Vol: {:.2}",
                        ticker.symbol, ticker.last, ticker.volume
                    );
                }
            }
        }

        // Print metrics every 50 messages
        if count % 50 == 0 {
            let metrics = stream.metrics();
            println!("\n=== Metrics ===");
            println!("Total messages: {}", count);
            println!("Messages/sec: {:.2}", metrics.msg_per_sec());
            println!("Latency: {}ms", metrics.latency_ms());
            println!("Backpressured: {}", stream.is_backpressured());
            println!("===============\n");
        }

        // Handle backpressure
        if stream.is_backpressured() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        if count >= 200 {
            break;
        }
    }

    Ok(())
}
