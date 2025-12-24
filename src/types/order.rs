use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddOrderRequest {
    pub method: String,
    pub params: AddOrderParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddOrderParams {
    pub ordertype: String,
    pub pair: String,
    pub side: String,
    pub volume: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    pub method: String,
    pub params: CancelOrderParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderParams {
    pub txid: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOrderRequest {
    pub method: String,
    pub params: BatchOrderParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOrderParams {
    pub orders: Vec<AddOrderParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl AddOrderRequest {
    pub fn market_buy(pair: &str, volume: &str) -> Self {
        Self {
            method: "add_order".to_string(),
            params: AddOrderParams {
                ordertype: "market".to_string(),
                pair: pair.to_string(),
                side: "buy".to_string(),
                volume: volume.to_string(),
                price: None,
                token: None,
            },
        }
    }

    pub fn limit_buy(pair: &str, volume: &str, price: &str) -> Self {
        Self {
            method: "add_order".to_string(),
            params: AddOrderParams {
                ordertype: "limit".to_string(),
                pair: pair.to_string(),
                side: "buy".to_string(),
                volume: volume.to_string(),
                price: Some(price.to_string()),
                token: None,
            },
        }
    }

    pub fn market_sell(pair: &str, volume: &str) -> Self {
        Self {
            method: "add_order".to_string(),
            params: AddOrderParams {
                ordertype: "market".to_string(),
                pair: pair.to_string(),
                side: "sell".to_string(),
                volume: volume.to_string(),
                price: None,
                token: None,
            },
        }
    }

    pub fn limit_sell(pair: &str, volume: &str, price: &str) -> Self {
        Self {
            method: "add_order".to_string(),
            params: AddOrderParams {
                ordertype: "limit".to_string(),
                pair: pair.to_string(),
                side: "sell".to_string(),
                volume: volume.to_string(),
                price: Some(price.to_string()),
                token: None,
            },
        }
    }
}

impl CancelOrderRequest {
    pub fn new(txid: &str) -> Self {
        Self {
            method: "cancel_order".to_string(),
            params: CancelOrderParams {
                txid: vec![txid.to_string()],
                token: None,
            },
        }
    }
}

impl BatchOrderRequest {
    pub fn new(orders: Vec<AddOrderParams>) -> Self {
        Self {
            method: "batch_add".to_string(),
            params: BatchOrderParams {
                orders,
                token: None,
            },
        }
    }

    pub fn from_requests(requests: Vec<AddOrderRequest>) -> Self {
        let orders = requests.into_iter().map(|r| r.params).collect();
        Self::new(orders)
    }
}
