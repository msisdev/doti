use ::age::secrecy::SecretString;
use std::error::Error;

pub mod age;
pub mod plaintext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    Plaintext,
    Age,
    AgeArmored,
}

pub trait CryptoTool {
    fn encrypt(&self, data: &[u8], passphrase: SecretString) -> Result<Vec<u8>, Box<dyn Error>>;

    fn decrypt(&self, encrypted: &[u8], passphrase: SecretString) -> Result<Vec<u8>, Box<dyn Error>>;

    fn is_my_fmt(&self, encrypted: &[u8]) -> bool;
}
