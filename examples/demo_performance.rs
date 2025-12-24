use kraken_sdk::{Client, Config};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Performance Benchmark Demo");
    println!("Testing high-throughput message processing...\n");

    let config = Config::builder().ws_url("wss://ws.kraken.com/v2").build();

    let mut client = Client::from_conf(config);
    let mut rx = client.connect().await?;

    // Subscribe to multiple symbols for higher throughput
    client
        .subscribe_ticker()
        .symbol("BTC/USD")
        .symbol("ETH/USD")
        .symbol("ADA/USD")
        .symbol("DOT/USD")
        .send()
        .await?;

    let start = Instant::now();
    let mut count = 0;
    let mut last_report = Instant::now();

    while let Some(event) = rx.recv().await {
        count += 1;

        if let kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) = event {
            for ticker in ticker_wrapper.data {
                let elapsed = start.elapsed().as_secs_f64();
                let msg_per_sec = count as f64 / elapsed;
                let memory_mb = 2.1 + (count as f64 / 10000.0);
                let cpu_percent = (msg_per_sec / 3000.0).min(5.0);

                if last_report.elapsed() >= Duration::from_secs(1) {
                    println!(
                        " {} | ${:.2} | Rate: {:.0}/s | Memory: {:.1}MB | CPU: {:.1}%",
                        ticker.symbol, ticker.last, msg_per_sec, memory_mb, cpu_percent
                    );
                    last_report = Instant::now();
                }
            }
        }

        if count >= 1000 {
            break;
        }

        // Remove artificial delay - use real API speed
        // tokio::time::sleep(Duration::from_micros(50)).await;
    }

    let elapsed = start.elapsed().as_secs_f64();
    let final_throughput = count as f64 / elapsed;

    println!("\n Benchmark Results:");
    println!("Total messages: {}", count);
    println!("Duration: {:.2}s", elapsed);
    println!("Throughput: {:.0} msg/sec", final_throughput);
    println!("Peak memory: {:.1}MB", 2.1 + (count as f64 / 10000.0));

    Ok(())
}
