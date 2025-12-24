//! # Kraken SDK
//!
//! **Minimal, async, type-safe Rust SDK** for Kraken's real-time market data.
//!
//! ## Features
//!
//! -  **Async-first** with Tokio
//! -  **Type-safe** event parsing
//! -  **Zero-copy** parsing (optional)
//! -  **Real-time** ticker, trades, orderbook
//! -  **Composable** API design
//! - 🔄 **Auto-retry** with exponential backoff
//! -  **Metrics** (latency, throughput)
//! - 🌐 **WASM support**
//! - ⚖️ **Backpressure** handling
//! - 📦 **Batch operations**
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use kraken_sdk::{Client, Config, KrakenStream};
//! use tokio_stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::builder().build();
//!     let mut client = Client::from_conf(config);
//!     let mut stream: KrakenStream = client.stream().await?;
//!
//!     // Subscribe to ticker
//!     client.subscribe_ticker()
//!         .symbol("BTC/USD")
//!         .send()
//!         .await?;
//!
//!     while let Some(event) = stream.next().await {
//!         println!("Event: {:?}", event);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Examples
//!
//! See the `examples/` directory for comprehensive usage examples.

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod metrics;
pub mod operation;
pub mod parser;
pub mod rate_limit;
pub mod stream;
pub mod types;
#[cfg(target_arch = "wasm32")]
pub mod wasm_ws;

#[allow(dead_code)]
mod protocol;

pub use auth::Auth;
pub use client::Client;
pub use config::Config;
pub use error::{Error, Result};
pub use metrics::Metrics;
pub use protocol::event::KrakenEvent;
pub use rate_limit::RateLimiter;
pub use stream::KrakenStream;
pub use types::{AddOrderRequest, BatchOrderRequest, CancelOrderRequest};
