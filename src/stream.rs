use crate::metrics::Metrics;
use crate::protocol::event::KrakenEvent;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use tokio_stream::Stream;

pub struct KrakenStream {
    rx: mpsc::Receiver<KrakenEvent>,
    metrics: Metrics,
}

impl KrakenStream {
    pub(crate) fn new(rx: mpsc::Receiver<KrakenEvent>) -> Self {
        Self {
            rx,
            metrics: Metrics::default(),
        }
    }

    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }

    pub fn is_backpressured(&self) -> bool {
        // Simple backpressure detection
        self.metrics.msg_per_sec() > 1000.0
    }
}

impl Stream for KrakenStream {
    type Item = KrakenEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.rx.poll_recv(cx) {
            Poll::Ready(Some(event)) => {
                self.metrics.record_message();
                Poll::Ready(Some(event))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
