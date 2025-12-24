use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    ws_url: String,
    max_retries: u32,
    initial_backoff: Duration,
}

impl Config {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn ws_url(&self) -> &str {
        &self.ws_url
    }

    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    pub fn initial_backoff(&self) -> Duration {
        self.initial_backoff
    }
}

#[derive(Debug, Clone)]
pub struct Builder {
    ws_url: Option<String>,
    max_retries: Option<u32>,
    initial_backoff: Option<Duration>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            ws_url: Some("wss://ws.kraken.com/v2".to_string()),
            max_retries: Some(3),
            initial_backoff: Some(Duration::from_millis(1000)),
        }
    }
}

impl Builder {
    pub fn ws_url(mut self, url: impl Into<String>) -> Self {
        self.ws_url = Some(url.into());
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    pub fn initial_backoff(mut self, backoff: Duration) -> Self {
        self.initial_backoff = Some(backoff);
        self
    }

    pub fn build(self) -> Config {
        Config {
            ws_url: self
                .ws_url
                .unwrap_or_else(|| "wss://ws.kraken.com/v2".to_string()),
            max_retries: self.max_retries.unwrap_or(3),
            initial_backoff: self.initial_backoff.unwrap_or(Duration::from_millis(1000)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = Config::builder()
            .ws_url("wss://test.com")
            .max_retries(5)
            .initial_backoff(Duration::from_millis(500))
            .build();

        assert_eq!(config.ws_url(), "wss://test.com");
        assert_eq!(config.max_retries(), 5);
        assert_eq!(config.initial_backoff(), Duration::from_millis(500));
    }

    #[test]
    fn test_config_defaults() {
        let config = Config::builder().build();

        assert_eq!(config.ws_url(), "wss://ws.kraken.com/v2");
        assert_eq!(config.max_retries(), 3);
        assert_eq!(config.initial_backoff(), Duration::from_millis(1000));
    }
}
