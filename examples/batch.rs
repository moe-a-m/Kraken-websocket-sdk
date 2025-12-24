use kraken_sdk::{AddOrderRequest, BatchOrderRequest, Client, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder().build();
    let mut client = Client::from_conf(config);
    let mut _rx = client.connect().await?;

    // Batch multiple orders
    let orders = vec![
        AddOrderRequest::market_buy("BTC/USD", "0.001"),
        AddOrderRequest::limit_sell("BTC/USD", "0.002", "60000"),
        AddOrderRequest::limit_sell("ETH/USD", "0.1", "4000"),
    ];

    let batch = BatchOrderRequest::from_requests(orders);
    client.batch_orders(batch).await?;

    println!("Batch of 3 orders sent successfully");
    Ok(())
}
