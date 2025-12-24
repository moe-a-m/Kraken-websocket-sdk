use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[cfg(not(target_arch = "wasm32"))]
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] Box<tokio_tungstenite::tungstenite::Error>),

    #[error("JSON error: {0}")]
    Json(#[from] Box<serde_json::Error>),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[cfg(feature = "zero-copy")]
    #[error("SIMD JSON error: {0}")]
    SimdJson(#[from] simd_json::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Connection closed")]
    ConnectionClosed,
}
