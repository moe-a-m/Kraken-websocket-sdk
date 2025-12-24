use kraken_sdk::{AddOrderRequest, Auth, BatchOrderRequest, CancelOrderRequest, Client, Config};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Order Management Demo");
    println!("Private API order operations...\n");

    let api_key = env::var("KRAKEN_API_KEY").unwrap_or_else(|_| "demo_api_key".to_string());
    let private_key =
        env::var("KRAKEN_PRIVATE_KEY").unwrap_or_else(|_| "demo_private_key".to_string());

    let _auth = Auth::new(api_key, private_key);
    let config = Config::builder()
        .ws_url("wss://ws-auth.kraken.com/v2")
        .build();

    let mut client = Client::from_conf(config);

    println!(" Authenticated Order Operations:");
    println!("==================================\n");

    // 1. Single Order Placement
    println!("1️⃣  Placing single limit order...");
    let limit_order = AddOrderRequest::limit_buy("BTC/USD", "0.001", "41000");

    match client.add_order(limit_order).await {
        Ok(_) => println!("    Limit buy order placed: 0.001 BTC @ $41,000"),
        Err(e) => println!("    Order failed: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // 2. Market Order
    println!("\n2️⃣  Placing market order...");
    let market_order = AddOrderRequest::market_sell("ETH/USD", "0.1");

    match client.add_order(market_order).await {
        Ok(_) => println!("    Market sell order placed: 0.1 ETH"),
        Err(e) => println!("    Order failed: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // 3. Batch Orders
    println!("\n3️⃣  Placing batch orders...");
    let batch_orders = vec![
        AddOrderRequest::limit_sell("BTC/USD", "0.0005", "43000"),
        AddOrderRequest::limit_sell("BTC/USD", "0.0005", "44000"),
        AddOrderRequest::limit_buy("ETH/USD", "0.05", "2400"),
    ];

    let batch = BatchOrderRequest::from_requests(batch_orders);
    match client.batch_orders(batch).await {
        Ok(_) => println!("    Batch orders placed: 3 orders submitted"),
        Err(e) => println!("    Batch failed: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // 4. Order Cancellation
    println!("\n4️⃣  Cancelling order...");
    let cancel_order = CancelOrderRequest::new("DEMO-ORDER-ID-123");

    match client.cancel_order(cancel_order).await {
        Ok(_) => println!("    Order cancelled: DEMO-ORDER-ID-123"),
        Err(e) => println!("    Cancel failed: {}", e),
    }

    // 5. Real-time order status from market events
    println!("\n5️⃣  Real-time market updates:");

    let mut rx = client.connect().await?;
    client.subscribe_ticker().symbol("BTC/USD").send().await?;

    let mut update_count = 0;
    while let Some(event) = rx.recv().await {
        update_count += 1;

        if let kraken_sdk::KrakenEvent::Ticker(ticker_wrapper) = event {
            for ticker in ticker_wrapper.data {
                println!(
                    "    {} | ${:.2} | Vol: {:.2} | Change: {:+.2}%",
                    ticker.symbol, ticker.last, ticker.volume, ticker.change_pct
                );
            }
        }

        if update_count >= 3 {
            break;
        }
    }

    println!("\n Order management demo complete");
    println!("Single orders: ");
    println!("Market orders: ");
    println!("Batch orders: ");
    println!("Order cancellation: ");
    println!("Real-time data: ");

    Ok(())
}
