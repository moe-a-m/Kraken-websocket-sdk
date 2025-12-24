use kraken_sdk::{Client, Config, KrakenStream};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder().build();
    let mut client = Client::from_conf(config);

    let mut stream: KrakenStream = client.stream().await?;

    client.subscribe_ticker().symbol("BTC/USD").send().await?;

    let mut count = 0;
    while let Some(event) = stream.next().await {
        count += 1;

        // Check metrics every 100 messages
        if count % 100 == 0 {
            let metrics = stream.metrics();
            println!("Messages/sec: {:.2}", metrics.msg_per_sec());
            println!("Latency: {}ms", metrics.latency_ms());

            // Handle backpressure
            if stream.is_backpressured() {
                println!("Backpressure detected, slowing down...");
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        }

        match event {
            kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) => {
                for ticker in ticker_wrapper.data {
                    println!(
                        "Event #{}: {} | ${:.2} | Vol: {:.2}",
                        count, ticker.symbol, ticker.last, ticker.volume
                    );
                }
            }
            _ => println!("Event #{}: {:?}", count, event),
        }

        if count >= 1000 {
            break;
        }
    }

    Ok(())
}
