use ::age::secrecy::SecretString;
use std::error::Error;

use crate::engine::tool::CryptoTool;

pub struct PlaintextTool;

impl CryptoTool for PlaintextTool {
    fn encrypt(&self, data: &[u8], _passphrase: SecretString) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(data.to_vec())
    }

    fn decrypt(&self, encrypted: &[u8], _passphrase: SecretString) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(encrypted.to_vec())
    }

    fn is_my_fmt(&self, _encrypted: &[u8]) -> bool {
        true
    }
}