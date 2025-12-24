use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickerData {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickerEvent {
    pub channel: String,
    pub data: Vec<TickerData>,
}
