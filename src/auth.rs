use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct Auth {
    api_key: String,
    api_secret: String,
}

impl Auth {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
        }
    }

    pub fn sign(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let secret = general_purpose::STANDARD.decode(&self.api_secret)?;
        let mut mac = HmacSha256::new_from_slice(&secret)?;
        mac.update(message.as_bytes());
        let signature = mac.finalize().into_bytes();
        Ok(general_purpose::STANDARD.encode(signature))
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_new() {
        let auth = Auth::new("key123".to_string(), "c2VjcmV0".to_string());
        assert_eq!(auth.api_key(), "key123");
    }

    #[test]
    fn test_sign_message() {
        let auth = Auth::new("key".to_string(), "c2VjcmV0".to_string()); // "secret" in base64
        let signature = auth.sign("test message").unwrap();
        assert!(!signature.is_empty());
        assert!(signature.len() > 10); // Basic signature length check
    }
}
