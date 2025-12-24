use crate::protocol::event::KrakenEvent;
use crate::Result;

pub struct Parser;

impl Parser {
    #[cfg(feature = "zero-copy")]
    pub fn parse_zero_copy(data: &mut [u8]) -> Result<KrakenEvent> {
        let value = simd_json::to_borrowed_value(data)?;
        let json_value: serde_json::Value =
            serde_json::to_value(value).map_err(|e| crate::Error::Json(Box::new(e)))?;
        serde_json::from_value(json_value).map_err(|e| crate::Error::Json(Box::new(e)))
    }

    #[cfg(not(feature = "zero-copy"))]
    pub fn parse_zero_copy(data: &mut [u8]) -> Result<KrakenEvent> {
        Self::parse_standard(std::str::from_utf8(data)?)
    }

    pub fn parse_standard(text: &str) -> Result<KrakenEvent> {
        serde_json::from_str(text).map_err(|e| crate::Error::Json(Box::new(e)))
    }
}
