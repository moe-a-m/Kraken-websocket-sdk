use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TradeWrapper {
    pub channel: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: Vec<TradeEvent>,
}

#[derive(Deserialize, Debug)]
pub struct TradeEvent {
    pub symbol: String,
    pub side: String,
    pub qty: String,
    pub price: String,
    pub trade_id: u64,
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct OrderbookWrapper {
    pub channel: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: Vec<OrderbookEvent>,
}

#[derive(Deserialize, Debug)]
pub struct OrderbookEvent {
    pub symbol: String,
    pub bids: Vec<[String; 3]>,
    pub asks: Vec<[String; 3]>,
    pub checksum: Option<u32>,
}
