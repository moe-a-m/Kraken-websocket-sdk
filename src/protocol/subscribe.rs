use serde::Serialize;

#[derive(Serialize)]
pub struct SubscribeMessage<'a> {
    pub method: &'static str,
    pub params: SubscribeParams<'a>,
}

#[derive(Serialize)]
pub struct SubscribeParams<'a> {
    pub channel: &'a str,
    pub symbol: Vec<&'a str>,
}

impl<'a> SubscribeMessage<'a> {
    pub fn ticker(symbols: Vec<&'a str>) -> Self {
        Self {
            method: "subscribe",
            params: SubscribeParams {
                channel: "ticker",
                symbol: symbols,
            },
        }
    }
}
