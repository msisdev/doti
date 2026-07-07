use age::{
    armor::{ArmoredReader, ArmoredWriter, Format},
    DecryptError, EncryptError, Recipient, Decryptor,
};
use secrecy::SecretString;
use std::io::{Read, Write};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Decryption error: {0}")]
    Decrypt(#[from] DecryptError),

    #[error("Encryption error: {0}")]
    Encrypt(#[from] EncryptError),

    #[error("Passphrase error: {0}")]
    Passphrase(String),

    #[error("Format error: {0}")]
    Format(String),

    #[error("Key error: {0}")]
    Key(String),
}

/// Encrypt data with a passphrase using the age format.
pub fn encrypt_with_passphrase(
    data: &[u8],
    passphrase: SecretString,
    armored: bool,
) -> Result<Vec<u8>, EncryptionError> {
    let encryptor = age::Encryptor::with_user_passphrase(passphrase);

    let mut output = Vec::new();
    if armored {
        let mut armor_writer = ArmoredWriter::wrap_output(&mut output, Format::AsciiArmor)?;
        let mut enc_writer = encryptor.wrap_output(&mut armor_writer)?;
        enc_writer.write_all(data)?;
        enc_writer.finish()?;
        armor_writer.finish()?;
    } else {
        let mut enc_writer = encryptor.wrap_output(&mut output)?;
        enc_writer.write_all(data)?;
        enc_writer.finish()?;
    }

    Ok(output)
}

/// Decrypt data with a passphrase using the age format.
pub fn decrypt_with_passphrase(
    encrypted_data: &[u8],
    passphrase: SecretString,
) -> Result<Vec<u8>, EncryptionError> {
    let reader = ArmoredReader::new(encrypted_data);
    let decryptor = Decryptor::new(reader)?;

    let identity = age::scrypt::Identity::new(passphrase);
    let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn age::Identity))?;
    let mut decrypted = Vec::new();
    reader.read_to_end(&mut decrypted)?;
    Ok(decrypted)
}

/// Encrypt data for a list of recipients.
/// Supports X25519 and SSH recipients.
pub fn encrypt_with_recipients(
    data: &[u8],
    recipients: Vec<Box<dyn Recipient + Send>>,
    armored: bool,
) -> Result<Vec<u8>, EncryptionError> {
    let encryptor = age::Encryptor::with_recipients(recipients.iter().map(|r| r.as_ref() as &dyn Recipient))?;

    let mut output = Vec::new();
    if armored {
        let mut armor_writer = ArmoredWriter::wrap_output(&mut output, Format::AsciiArmor)?;
        let mut enc_writer = encryptor.wrap_output(&mut armor_writer)?;
        enc_writer.write_all(data)?;
        enc_writer.finish()?;
        armor_writer.finish()?;
    } else {
        let mut enc_writer = encryptor.wrap_output(&mut output)?;
        enc_writer.write_all(data)?;
        enc_writer.finish()?;
    }

    Ok(output)
}

/// Decrypt data using a list of identities.
/// Supports X25519 and SSH identities.
pub fn decrypt_with_identities(
    encrypted_data: &[u8],
    identities: Vec<Box<dyn age::Identity + Send>>,
) -> Result<Vec<u8>, EncryptionError> {
    let reader = ArmoredReader::new(encrypted_data);
    let decryptor = Decryptor::new(reader)?;

    let mut reader = decryptor.decrypt(identities.iter().map(|i| i.as_ref() as &dyn age::Identity))?;
    let mut decrypted = Vec::new();
    reader.read_to_end(&mut decrypted)?;
    Ok(decrypted)
}

/// Parse a recipient string (X25519 or SSH).
pub fn parse_recipient(s: &str) -> Result<Box<dyn Recipient + Send>, EncryptionError> {
    if s.starts_with("age1") {
        let recipient = age::x25519::Recipient::from_str(s)
            .map_err(|e| EncryptionError::Key(e.to_string()))?;
        Ok(Box::new(recipient))
    } else if s.starts_with("ssh-") {
        let recipient = age::ssh::Recipient::from_str(s)
            .map_err(|_| EncryptionError::Key("Invalid SSH recipient".to_string()))?;
        Ok(Box::new(recipient))
    } else {
        Err(EncryptionError::Key("Unknown recipient format".to_string()))
    }
}

