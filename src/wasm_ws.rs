#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::WebSocket;

#[cfg(target_arch = "wasm32")]
pub struct WasmWebSocket {
    ws: WebSocket,
}

#[cfg(target_arch = "wasm32")]
impl WasmWebSocket {
    pub fn new(url: &str) -> Result<Self, JsValue> {
        let ws = WebSocket::new(url)?;
        Ok(Self { ws })
    }

    pub fn send_text(&self, data: &str) -> Result<(), JsValue> {
        self.ws.send_with_str(data)
    }

    pub fn send_binary(&self, data: &[u8]) -> Result<(), JsValue> {
        self.ws.send_with_u8_array(data)
    }

    pub fn close(&self) -> Result<(), JsValue> {
        self.ws.close()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct WasmWebSocket;

#[cfg(not(target_arch = "wasm32"))]
impl WasmWebSocket {
    pub fn new(_url: &str) -> Result<Self, String> {
        Err("WASM WebSocket only available on wasm32 target".to_string())
    }
}
