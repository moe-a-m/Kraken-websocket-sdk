use kraken_sdk::{Client, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder().ws_url("wss://ws.kraken.com/v2").build();

    let mut client = Client::from_conf(config);
    let mut rx = client.connect().await?;

    client
        .subscribe_ticker()
        .symbol("BTC/USD")
        .symbol("ETH/USD")
        .send()
        .await?;

    let mut count = 0;
    while let Some(event) = rx.recv().await {
        count += 1;

        match event {
            kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) => {
                for ticker in ticker_wrapper.data {
                    println!(
                        "Event #{}: {} | ${:.2} | Bid: ${:.2} | Ask: ${:.2}",
                        count, ticker.symbol, ticker.last, ticker.bid, ticker.ask
                    );
                }
            }
            _ => println!("Event #{}: {:?}", count, event),
        }
    }

    Ok(())
}
