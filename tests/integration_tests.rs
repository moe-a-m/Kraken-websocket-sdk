use kraken_sdk::{Auth, Client, Config, RateLimiter};
use std::time::Duration;

#[tokio::test]
async fn test_client_creation() {
    let config = Config::builder()
        .ws_url("wss://test.example.com")
        .max_retries(1)
        .build();

    let _client = Client::from_conf(config);
    // Client creation successful
}

#[test]
fn test_auth_integration() {
    let auth = Auth::new("test_key".to_string(), "dGVzdF9zZWNyZXQ=".to_string());
    let signature = auth.sign("test_message").unwrap();

    // Test that signature is consistent
    let signature2 = auth.sign("test_message").unwrap();
    assert_eq!(signature, signature2);
}

#[tokio::test]
async fn test_rate_limiter() {
    let mut limiter = RateLimiter::new(2, Duration::from_millis(100));

    // Should allow first two requests immediately
    let start = std::time::Instant::now();
    limiter.acquire().await;
    limiter.acquire().await;
    let elapsed = start.elapsed();

    // Should be very fast for first two
    assert!(elapsed < Duration::from_millis(50));
}

#[test]
fn test_order_types() {
    use kraken_sdk::{AddOrderRequest, BatchOrderRequest};

    let order = AddOrderRequest::market_buy("BTC/USD", "0.001");
    assert_eq!(order.params.ordertype, "market");
    assert_eq!(order.params.side, "buy");

    let orders = vec![order];
    let batch = BatchOrderRequest::from_requests(orders);
    assert_eq!(batch.method, "batch_add");
    assert_eq!(batch.params.orders.len(), 1);
}
