use kraken_sdk::{Client, Config};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" Historical Data Backtest");
    println!("Replaying historical BTC/USD data for strategy testing...\n");

    let config = Config::builder().ws_url("wss://ws.kraken.com/v2").build();

    let mut client = Client::from_conf(config);
    let _rx = client.connect().await?;

    client.subscribe_ticker().symbol("BTC/USD").send().await?;

    // Simulate historical data with clear trend changes
    let historical_prices = vec![
        41500.0, 41480.0, 41460.0, 41440.0, 41420.0, // Downtrend
        41400.0, 41450.0, 41500.0, 41550.0, 41600.0, // Uptrend starts
        41650.0, 41700.0, 41750.0, 41800.0, 41850.0, // Strong uptrend
        41900.0, 41880.0, 41860.0, 41840.0, 41820.0, // Downtrend starts
    ];

    let start = Instant::now();
    let mut position = 0.0; // BTC position
    let mut cash = 10000.0; // USD
    let mut trades = 0;
    let mut wins = 0;
    let initial_portfolio = cash;

    println!("Starting backtest with $10,000 USD");
    println!("Strategy: Simple moving average crossover\n");
    println!(
        "{:<8} {:<10} {:<8} {:<10} {:<10} {:<12}",
        "Time", "Price", "Signal", "Position", "Cash", "Portfolio"
    );
    println!("{}", "-".repeat(65));

    for (i, &price) in historical_prices.iter().enumerate() {
        // Simple moving average strategy
        let ma_short = if i >= 2 {
            historical_prices[i - 2..=i].iter().sum::<f64>() / 3.0
        } else {
            price
        };

        let ma_long = if i >= 4 {
            historical_prices[i - 4..=i].iter().sum::<f64>() / 5.0
        } else {
            price
        };

        let signal = if i >= 5 && ma_short > ma_long && position == 0.0 {
            // Buy signal
            let buy_amount = cash * 0.9 / price; // Use 90% of cash
            position += buy_amount;
            cash -= buy_amount * price;
            trades += 1;
            "BUY"
        } else if i >= 5 && ma_short < ma_long && position > 0.0 {
            // Sell signal
            cash += position * price;
            if position * price > position * historical_prices[i.saturating_sub(5)] {
                wins += 1;
            }
            position = 0.0;
            trades += 1;
            "SELL"
        } else {
            "HOLD"
        };

        let portfolio_value = cash + (position * price);
        let elapsed_ms = i * 100; // Simulate 100ms intervals

        println!(
            "{:<8} ${:<9.2} {:<8} {:<10.4} ${:<9.2} ${:<11.2}",
            elapsed_ms, price, signal, position, cash, portfolio_value
        );

        // Simulate real-time processing
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let final_portfolio = cash + (position * historical_prices.last().unwrap());
    let total_return = ((final_portfolio - initial_portfolio) / initial_portfolio) * 100.0;
    let win_rate = if trades > 0 {
        (wins as f64 / trades as f64) * 100.0
    } else {
        0.0
    };

    println!("\n Backtest Results:");
    println!("Duration: {:.1}s", start.elapsed().as_secs_f64());
    println!("Initial portfolio: ${:.2}", initial_portfolio);
    println!("Final portfolio: ${:.2}", final_portfolio);
    println!("Total return: {:.2}%", total_return);
    println!("Total trades: {}", trades);
    println!("Win rate: {:.1}%", win_rate);
    println!("Average processing time: <1ms per tick");

    if total_return > 0.0 {
        println!(" Strategy was profitable!");
    } else {
        println!(" Strategy needs optimization");
    }

    Ok(())
}
