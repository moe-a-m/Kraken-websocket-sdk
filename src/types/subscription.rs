use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SubscribeInput {
    pub method: String,
    pub params: SubscribeParams,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubscribeParams {
    pub channel: String,
    pub symbol: Vec<String>,
}

impl SubscribeInput {
    pub fn ticker(symbols: Vec<String>) -> Self {
        Self {
            method: "subscribe".to_string(),
            params: SubscribeParams {
                channel: "ticker".to_string(),
                symbol: symbols,
            },
        }
    }
}
