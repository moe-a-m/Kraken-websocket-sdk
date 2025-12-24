use kraken_sdk::{Client, Config, KrakenEvent, KrakenStream};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder().build();
    let mut client = Client::from_conf(config);

    // Get stream instead of receiver
    let mut stream: KrakenStream = client.stream().await?;

    // Subscribe to ticker
    client.subscribe_ticker().symbol("BTC/USD").send().await?;

    // Use Stream API
    while let Some(event) = stream.next().await {
        match event {
            KrakenEvent::Ticker(ticker_wrapper) => {
                for ticker in ticker_wrapper.data {
                    println!(
                        "Ticker: {} | ${:.2} | Bid: ${:.2} | Ask: ${:.2}",
                        ticker.symbol, ticker.last, ticker.bid, ticker.ask
                    );
                }
            }
            _ => println!("Other event: {:?}", event),
        }
    }

    Ok(())
}
