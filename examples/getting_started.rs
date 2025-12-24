use kraken_sdk::{Client, Config, KrakenStream};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with default config
    let config = Config::builder().build();
    let mut client = Client::from_conf(config);

    // Connect and get event stream
    let mut stream: KrakenStream = client.stream().await?;

    // Subscribe to BTC/USD ticker
    client.subscribe_ticker().symbol("BTC/USD").send().await?;

    println!("Listening for BTC/USD ticker events...");

    // Process first 10 events
    let mut count = 0;
    while let Some(event) = stream.next().await {
        count += 1;
        match event {
            kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) => {
                for ticker in ticker_wrapper.data {
                    println!(
                        "[{}] {} ticker: ${:.2} (bid: ${:.2}, ask: ${:.2})",
                        count, ticker.symbol, ticker.last, ticker.bid, ticker.ask
                    );
                }
            }
            _ => println!("[{}] {:?}", count, event),
        }

        if count >= 10 {
            break;
        }
    }

    println!("Done!");
    Ok(())
}
