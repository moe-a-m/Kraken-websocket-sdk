#[cfg(not(target_arch = "wasm32"))]
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio::time::sleep;
#[cfg(not(target_arch = "wasm32"))]
use tokio_tungstenite::{connect_async, tungstenite::Message};
#[cfg(not(target_arch = "wasm32"))]
use url::Url;

#[cfg(target_arch = "wasm32")]
use crate::wasm_ws::WasmWebSocket;

use crate::{
    operation::SubscribeTickerFluentBuilder,
    protocol::event::KrakenEvent,
    stream::KrakenStream,
    types::{AddOrderRequest, BatchOrderRequest, CancelOrderRequest, SubscribeInput},
    Config, Error, Result,
};

#[cfg(not(target_arch = "wasm32"))]
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Client {
    config: Config,
    cmd_tx: Option<mpsc::Sender<String>>,
}

impl Client {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
            cmd_tx: None,
        }
    }

    pub fn from_conf(config: Config) -> Self {
        Self {
            config,
            cmd_tx: None,
        }
    }

    pub async fn connect(&mut self) -> Result<mpsc::Receiver<KrakenEvent>> {
        let mut attempt = 0;
        let mut backoff = self.config.initial_backoff();

        loop {
            match self.try_connect().await {
                Ok(rx) => return Ok(rx),
                Err(e) => {
                    attempt += 1;
                    if attempt > self.config.max_retries() {
                        return Err(e);
                    }
                    sleep(backoff).await;
                    backoff *= 2; // Exponential backoff
                }
            }
        }
    }

    pub async fn stream(&mut self) -> Result<KrakenStream> {
        let rx = self.connect().await?;
        Ok(KrakenStream::new(rx))
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn try_connect(&mut self) -> Result<mpsc::Receiver<KrakenEvent>> {
        let url = Url::parse(self.config.ws_url())?;
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| Error::WebSocket(Box::new(e)))?;
        let (mut write, mut read) = ws_stream.split();

        let (cmd_tx, mut cmd_rx) = mpsc::channel::<String>(32);
        let (event_tx, event_rx) = mpsc::channel::<KrakenEvent>(100);

        self.cmd_tx = Some(cmd_tx);

        tokio::spawn(async move {
            while let Some(msg) = cmd_rx.recv().await {
                let _ = write.send(Message::Text(msg)).await;
            }
        });

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                if let Ok(msg) = msg {
                    if msg.is_text() {
                        let text = msg.to_text().unwrap();
                        if let Ok(event) = Parser::parse_standard(text) {
                            if event_tx.send(event).await.is_err() {
                                break; // Backpressure: receiver dropped
                            }
                        }
                    } else if msg.is_binary() {
                        let mut data = msg.into_data();
                        if let Ok(event) = Parser::parse_zero_copy(&mut data) {
                            if event_tx.send(event).await.is_err() {
                                break; // Backpressure: receiver dropped
                            }
                        }
                    }
                }
            }
        });

        Ok(event_rx)
    }

    #[cfg(target_arch = "wasm32")]
    async fn try_connect(&mut self) -> Result<mpsc::Receiver<KrakenEvent>> {
        let _ws =
            WasmWebSocket::new(self.config.ws_url()).map_err(|_| crate::Error::ConnectionClosed)?;

        let (_event_tx, event_rx) = mpsc::channel::<KrakenEvent>(100);

        // Minimal WASM implementation - would need proper event handling
        // This is a placeholder for WASM WebSocket integration

        Ok(event_rx)
    }

    pub fn subscribe_ticker(&self) -> SubscribeTickerFluentBuilder {
        SubscribeTickerFluentBuilder::new(self.clone())
    }

    pub(crate) async fn send_subscription(&self, input: SubscribeInput) -> Result<()> {
        if let Some(tx) = &self.cmd_tx {
            let msg = serde_json::to_string(&input).map_err(|e| Error::Json(Box::new(e)))?;
            tx.send(msg).await.map_err(|_| Error::ConnectionClosed)?;
        }
        Ok(())
    }

    pub async fn add_order(&self, order: AddOrderRequest) -> Result<()> {
        if let Some(tx) = &self.cmd_tx {
            let msg = serde_json::to_string(&order).map_err(|e| Error::Json(Box::new(e)))?;
            tx.send(msg).await.map_err(|_| Error::ConnectionClosed)?;
        }
        Ok(())
    }

    pub async fn cancel_order(&self, cancel: CancelOrderRequest) -> Result<()> {
        if let Some(tx) = &self.cmd_tx {
            let msg = serde_json::to_string(&cancel).map_err(|e| Error::Json(Box::new(e)))?;
            tx.send(msg).await.map_err(|_| Error::ConnectionClosed)?;
        }
        Ok(())
    }

    pub async fn batch_orders(&self, batch: BatchOrderRequest) -> Result<()> {
        if let Some(tx) = &self.cmd_tx {
            let msg = serde_json::to_string(&batch).map_err(|e| Error::Json(Box::new(e)))?;
            tx.send(msg).await.map_err(|_| Error::ConnectionClosed)?;
        }
        Ok(())
    }
}
