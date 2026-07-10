use ::age::secrecy::SecretString;
use std::error::Error;
use std::io::{Read, Write};

use crate::engine::tool::CryptoTool;

const AGE_ARMOR_HEADER: &[u8] = b"-----BEGIN AGE ENCRYPTED FILE-----";

pub struct AgeTool;

pub struct AgeArmoredTool;

impl CryptoTool for AgeTool {
    fn encrypt(&self, data: &[u8], passphrase: SecretString) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut encrypted = vec![];
        let encryptor = ::age::Encryptor::with_user_passphrase(passphrase);

        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(data)?;
        writer.finish()?;

        Ok(encrypted)
    }

    fn decrypt(&self, encrypted: &[u8], passphrase: SecretString) -> Result<Vec<u8>, Box<dyn Error>> {
        let identity = ::age::scrypt::Identity::new(passphrase);

        // First try as binary age stream.
        match ::age::Decryptor::new(encrypted) {
            Ok(decryptor) => {
                let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn ::age::Identity))?;
                let mut decrypted = vec![];
                reader.read_to_end(&mut decrypted)?;
                Ok(decrypted)
            }
            Err(_) => {
                // Fall back to armored reader.
                let decryptor = ::age::Decryptor::new(::age::armor::ArmoredReader::new(encrypted))?;
                let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn ::age::Identity))?;
                let mut decrypted = vec![];
                reader.read_to_end(&mut decrypted)?;
                Ok(decrypted)
            }
        }
    }

    fn is_my_file(&self, encrypted: &[u8]) -> bool {
        if encrypted.starts_with(AGE_ARMOR_HEADER) {
            return true;
        }

        ::age::Decryptor::new(encrypted).is_ok()
    }
}

impl CryptoTool for AgeArmoredTool {
    fn encrypt(&self, data: &[u8], passphrase: SecretString) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut encrypted = vec![];
        let encryptor = ::age::Encryptor::with_user_passphrase(passphrase);

        let mut armored_writer =
            ::age::armor::ArmoredWriter::wrap_output(&mut encrypted, ::age::armor::Format::AsciiArmor)?;
        let mut writer = encryptor.wrap_output(&mut armored_writer)?;
        writer.write_all(data)?;
        writer.finish()?;
        let _ = armored_writer.finish()?;

        Ok(encrypted)
    }

    fn decrypt(&self, encrypted: &[u8], passphrase: SecretString) -> Result<Vec<u8>, Box<dyn Error>> {
        if !encrypted.starts_with(AGE_ARMOR_HEADER) {
            return Err("Input is not an armored age file".into());
        }

        let identity = ::age::scrypt::Identity::new(passphrase);
        let decryptor = ::age::Decryptor::new(::age::armor::ArmoredReader::new(encrypted))?;
        let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn ::age::Identity))?;
        let mut decrypted = vec![];
        reader.read_to_end(&mut decrypted)?;
        Ok(decrypted)
    }

    fn is_my_file(&self, encrypted: &[u8]) -> bool {
        encrypted.starts_with(AGE_ARMOR_HEADER)
    }
}