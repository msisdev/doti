use ::age::secrecy::SecretString;
use std::error::Error;

pub mod age;
pub mod plaintext;

macro_rules! register_crypto_tools {
    ($($tool_type:ty),* $(,)?) => {
        pub fn match_tool(data: &[u8]) -> Result<(ToolKind, Box<dyn CryptoTool>), Box<dyn std::error::Error>> {
            $(
                if <$tool_type>::default().is_my_fmt(data) {
                    let tool = <$tool_type>::default();
                    return Ok((tool.kind(), Box::new(tool)));
                }
            )*

            Err("No matching crypto tool found.".into())
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    Plaintext,
    Age,
    AgeArmored,
}

pub trait CryptoTool {
    fn kind(&self) -> ToolKind;

    fn encrypt(&self, data: &[u8], passphrase: SecretString) -> Result<Vec<u8>, Box<dyn Error>>;

    fn decrypt(&self, encrypted: &[u8], passphrase: SecretString) -> Result<Vec<u8>, Box<dyn Error>>;

    fn is_my_fmt(&self, encrypted: &[u8]) -> bool;
}

register_crypto_tools!(
    age::AgeArmoredTool,
    age::AgeTool,
    plaintext::PlaintextTool,
);
