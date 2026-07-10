use age::secrecy::SecretString;

use crate::engine::state::{FileState, detect_file_state_from_path};
use crate::engine::tool::ToolKind;
use crate::engine::tool::age::{AgeArmoredTool, AgeTool};
use crate::engine::tool::plaintext::PlaintextTool;

fn select_tool(tool: ToolKind) -> Box<dyn crate::engine::tool::CryptoTool> {
    match tool {
        ToolKind::Plaintext => Box::new(PlaintextTool),
        ToolKind::Age => Box::new(AgeTool),
        ToolKind::AgeArmored => Box::new(AgeArmoredTool),
    }
}

pub fn create_secret(passphrase: impl Into<String>) -> SecretString {
    SecretString::new(passphrase.into().into())
}

pub fn encrypt_bytes(
    data: &[u8],
    passphrase: SecretString,
    tool: ToolKind,
) -> std::result::Result<Vec<u8>, Box<dyn std::error::Error>> {
    let tool_impl = select_tool(tool);
    tool_impl.encrypt(data, passphrase)
}

pub fn decrypt_bytes(
    encrypted: &[u8],
    passphrase: SecretString,
    tool: ToolKind,
) -> std::result::Result<Vec<u8>, Box<dyn std::error::Error>> {
    let tool_impl = select_tool(tool);
    tool_impl.decrypt(encrypted, passphrase)
}

pub fn is_my_file(encrypted: &[u8], tool: ToolKind) -> bool {
    let tool_impl = select_tool(tool);
    tool_impl.is_my_file(encrypted)
}

pub fn detect_file_state(path: impl AsRef<std::path::Path>) -> std::io::Result<FileState> {
    detect_file_state_from_path(path)
}
