use kraken_sdk::{AddOrderRequest, BatchOrderRequest, CancelOrderRequest, Client, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .ws_url("wss://ws-auth.kraken.com/v2")
        .build();

    let mut client = Client::from_conf(config);
    let mut _rx = client.connect().await?;

    // Single orders
    let market_order = AddOrderRequest::market_buy("BTC/USD", "0.001");
    client.add_order(market_order).await?;
    println!(" Market buy order sent");

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Batch orders
    let orders = vec![
        AddOrderRequest::limit_sell("BTC/USD", "0.001", "50000"),
        AddOrderRequest::limit_sell("ETH/USD", "0.1", "3000"),
    ];
    let batch = BatchOrderRequest::from_requests(orders);
    client.batch_orders(batch).await?;
    println!(" Batch orders sent");

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Cancel order
    let cancel_order = CancelOrderRequest::new("ORDER_ID_123");
    client.cancel_order(cancel_order).await?;
    println!(" Cancel order sent");

    Ok(())
}
