use serde::Deserialize;

pub use super::trades::{OrderbookWrapper, TradeWrapper};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum KrakenEvent {
    Ticker(TickerWrapper),
    Trade(TradeWrapper),
    Orderbook(OrderbookWrapper),
    Heartbeat(HeartbeatEvent),
    Pong(PongEvent),
    Status(StatusEvent),
    Order(super::order::OrderResponse),
}

#[derive(Deserialize, Debug)]
pub struct TickerWrapper {
    pub channel: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: Vec<TickerEvent>,
}

#[derive(Deserialize, Debug)]
pub struct TickerEvent {
    pub symbol: String,
    pub bid: f64,
    pub bid_qty: f64,
    pub ask: f64,
    pub ask_qty: f64,
    pub last: f64,
    pub volume: f64,
    pub vwap: f64,
    pub low: f64,
    pub high: f64,
    pub change: f64,
    pub change_pct: f64,
}

#[derive(Deserialize, Debug)]
pub struct HeartbeatEvent {
    pub channel: String,
}

#[derive(Deserialize, Debug)]
pub struct PongEvent {
    pub method: String,
    pub req_id: Option<u64>,
    pub time_in: String,
    pub time_out: String,
}

#[derive(Deserialize, Debug)]
pub struct StatusEvent {
    pub channel: String,
    pub data: Vec<StatusData>,
}

#[derive(Deserialize, Debug)]
pub struct StatusData {
    pub api_version: String,
    pub connection_id: u64,
    pub system: String,
    pub version: String,
}
