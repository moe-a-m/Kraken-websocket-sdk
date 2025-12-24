use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AddOrderMessage<'a> {
    pub method: &'static str,
    pub params: AddOrderParams<'a>,
}

#[derive(Serialize)]
pub struct AddOrderParams<'a> {
    pub token: &'a str,
    pub order_type: &'a str,
    pub side: &'a str,
    pub symbol: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_qty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_userref: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Triggers<'a>>,
}

#[derive(Serialize)]
pub struct Triggers<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type: Option<&'a str>,
}

impl<'a> AddOrderMessage<'a> {
    pub fn market_buy(token: &'a str, symbol: &'a str, qty: f64) -> Self {
        Self {
            method: "add_order",
            params: AddOrderParams {
                token,
                order_type: "market",
                side: "buy",
                symbol,
                limit_price: None,
                order_qty: Some(qty),
                order_userref: None,
                post_only: None,
                reduce_only: None,
                triggers: None,
            },
        }
    }

    pub fn limit_buy(token: &'a str, symbol: &'a str, qty: f64, price: f64) -> Self {
        Self {
            method: "add_order",
            params: AddOrderParams {
                token,
                order_type: "limit",
                side: "buy",
                symbol,
                limit_price: Some(price),
                order_qty: Some(qty),
                order_userref: None,
                post_only: None,
                reduce_only: None,
                triggers: None,
            },
        }
    }

    pub fn limit_sell(token: &'a str, symbol: &'a str, qty: f64, price: f64) -> Self {
        Self {
            method: "add_order",
            params: AddOrderParams {
                token,
                order_type: "limit",
                side: "sell",
                symbol,
                limit_price: Some(price),
                order_qty: Some(qty),
                order_userref: None,
                post_only: None,
                reduce_only: None,
                triggers: None,
            },
        }
    }

    pub fn stop_loss(token: &'a str, symbol: &'a str, qty: f64, trigger_price: f64) -> Self {
        Self {
            method: "add_order",
            params: AddOrderParams {
                token,
                order_type: "stop-loss",
                side: "sell",
                symbol,
                limit_price: None,
                order_qty: Some(qty),
                order_userref: None,
                post_only: None,
                reduce_only: None,
                triggers: Some(Triggers {
                    reference: Some("last"),
                    price: Some(trigger_price),
                    price_type: Some("static"),
                }),
            },
        }
    }

    pub fn take_profit(token: &'a str, symbol: &'a str, qty: f64, trigger_price: f64) -> Self {
        Self {
            method: "add_order",
            params: AddOrderParams {
                token,
                order_type: "take-profit",
                side: "sell",
                symbol,
                limit_price: None,
                order_qty: Some(qty),
                order_userref: None,
                post_only: None,
                reduce_only: None,
                triggers: Some(Triggers {
                    reference: Some("last"),
                    price: Some(trigger_price),
                    price_type: Some("static"),
                }),
            },
        }
    }

    pub fn with_post_only(mut self) -> Self {
        self.params.post_only = Some(true);
        self
    }

    pub fn with_reduce_only(mut self) -> Self {
        self.params.reduce_only = Some(true);
        self
    }
}

#[derive(Deserialize, Debug)]
pub struct OrderResponse {
    pub method: String,
    pub success: bool,
    pub result: Option<OrderResult>,
    pub error: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OrderResult {
    pub order_id: String,
    pub order_userref: Option<u32>,
}
