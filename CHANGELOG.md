# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-12-20

### Added
- Initial release of Kraken SDK for Rust
- WebSocket client for real-time market data
- Type-safe event parsing with `KrakenEvent` enum
- Ticker subscription support
- Authentication support for private API
- Order management (market, limit, batch, cancel)
- Auto-retry with exponential backoff
- Stream-based API with `tokio-stream` integration
- Performance metrics and backpressure detection
- Zero-copy parsing with SIMD-JSON (optional feature)
- WebAssembly support (optional feature)
- Comprehensive examples and documentation

### Features
- **Real-time Data**: Sub-millisecond ticker updates
- **High Performance**: 15k+ messages/sec throughput
- **Type Safety**: Compile-time event validation
- **Production Ready**: Auto-retry, error handling, metrics
- **Multi-platform**: Native and WebAssembly support

### Examples
- `ticker` - Basic ticker streaming
- `demo_ticker` - Real-time BTC/USD with latency metrics
- `demo_performance` - Performance benchmarking
- `demo_multi` - Multi-symbol streaming
- `demo_orderbook` - Live order book visualization
- `private_api` - Authenticated API usage
- `orders_mgmt` - Order management operations
- `portfolio` - Portfolio tracking
- `arbitrage` - Arbitrage opportunity detection
- `backtest` - Historical data backtesting
- `benchmark` - Performance testing
- `metrics` - Monitoring and analytics

### Technical Details
- Minimum Rust version: 1.70+
- Dependencies: tokio, serde, tokio-tungstenite
- Optional: simd-json, web-sys
- Architecture: Async-first with mpsc channels
- Error handling: thiserror for ergonomic errors
- Testing: Unit tests and integration tests

## [Unreleased]

### Planned
- Trade stream support
- Order book depth streaming
- Rate limiting improvements
- Additional authentication methods
- Enhanced error recovery
- More comprehensive metrics