use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PingMessage {
    pub method: &'static str,
}

impl PingMessage {
    pub fn new() -> Self {
        Self { method: "ping" }
    }
}

#[derive(Deserialize, Debug)]
pub struct PongResponse {
    pub method: String,
}
