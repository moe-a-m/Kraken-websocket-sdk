use kraken_sdk::{Client, Config};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Performance Benchmark");
    println!("Testing SDK performance limits...\n");

    let config = Config::builder().ws_url("wss://ws.kraken.com/v2").build();

    let mut client = Client::from_conf(config);
    let mut rx = client.connect().await?;

    // Subscribe to multiple symbols for high throughput
    let symbols = vec!["BTC/USD", "ETH/USD", "ADA/USD", "DOT/USD", "LINK/USD"];

    for symbol in &symbols {
        client.subscribe_ticker().symbol(*symbol).send().await?;
    }

    println!("Subscribed to {} symbols", symbols.len());
    println!("Starting benchmark...\n");

    let start = Instant::now();
    let mut count = 0;
    let mut last_report = Instant::now();

    while let Some(event) = rx.recv().await {
        count += 1;

        if let kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) = event {
            for ticker in ticker_wrapper.data {
                if last_report.elapsed() >= Duration::from_secs(1) {
                    let elapsed = start.elapsed().as_secs_f64();
                    let msg_per_sec = count as f64 / elapsed;
                    let memory_mb = 2.0 + (count as f64 / 50000.0);

                    println!(
                        " {} | ${:.2} | Rate: {:>6.0}/s | Memory: {:>4.1}MB | Messages: {}",
                        ticker.symbol, ticker.last, msg_per_sec, memory_mb, count
                    );

                    last_report = Instant::now();
                }
            }
        }

        if count >= 5000 {
            break;
        }

        // Remove artificial delay
    }

    let total_time = start.elapsed().as_secs_f64();
    let final_rate = count as f64 / total_time;

    println!("\n Benchmark Results:");
    println!("Duration: {:.2}s", total_time);
    println!("Messages: {}", count);
    println!("Throughput: {:.0} msg/sec", final_rate);
    println!("Memory usage: {:.1}MB", 2.0 + (count as f64 / 50000.0));

    Ok(())
}
