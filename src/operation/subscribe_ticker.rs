use crate::{types::SubscribeInput, Client, Result};

pub struct SubscribeTickerFluentBuilder {
    client: Client,
    symbols: Vec<String>,
}

impl SubscribeTickerFluentBuilder {
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            symbols: Vec::new(),
        }
    }

    pub fn symbols(mut self, symbols: Vec<String>) -> Self {
        self.symbols = symbols;
        self
    }

    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbols.push(symbol.into());
        self
    }

    pub async fn send(self) -> Result<()> {
        let input = SubscribeInput::ticker(self.symbols);
        self.client.send_subscription(input).await
    }
}
