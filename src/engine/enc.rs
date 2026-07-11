use age::{
    armor::{ArmoredWriter, Format},
    Decryptor, Encryptor,
};
use std::io::{Read, Write};

/// Represents the encryption state of a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileState {
    Plaintext,
    Age { armored: bool },
}

/// Encrypts data with the given passphrase.
/// If `armor` is true, the output will be ASCII armored.
pub fn encrypt(
    data: &[u8],
    passphrase: age::secrecy::SecretString,
    armor: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut output = Vec::new();

    if armor {
        let mut armored_writer = ArmoredWriter::wrap_output(&mut output, Format::AsciiArmor)?;
        {
            let encryptor = Encryptor::with_user_passphrase(passphrase);
            let mut writer = encryptor.wrap_output(&mut armored_writer)?;
            writer.write_all(data)?;
            writer.finish()?;
        }
        armored_writer.finish()?;
    } else {
        let encryptor = Encryptor::with_user_passphrase(passphrase);
        let mut writer = encryptor.wrap_output(&mut output)?;
        writer.write_all(data)?;
        writer.finish()?;
    }

    Ok(output)
}

/// Decrypts data using the given passphrase.
pub fn decrypt(
    encrypted_data: &[u8],
    passphrase: age::secrecy::SecretString,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let decryptor = Decryptor::new(encrypted_data)?;

    if !decryptor.is_scrypt() {
        return Err("Expected passphrase-encrypted data".into());
    }

    let identity = age::scrypt::Identity::new(passphrase);
    let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn age::Identity))?;
    let mut decrypted = Vec::new();
    reader.read_to_end(&mut decrypted)?;

    Ok(decrypted)
}

/// Verifies whether the input is age-encrypted data and whether the passphrase matches.
///
/// Returns `(is_age_encrypted, passphrase_matches)`.
///
/// - `(false, false)`: input is not valid age data.
/// - `(true, false)`: input is age data, but not scrypt(passphrase) data or passphrase is wrong.
/// - `(true, true)`: input is age scrypt data and passphrase is correct.
pub fn verify_age_passphrase(
    encrypted_data: &[u8],
    passphrase: age::secrecy::SecretString,
) -> (bool, bool) {
    let decryptor = match Decryptor::new(encrypted_data) {
        Ok(d) => d,
        Err(_) => return (false, false),
    };

    if !decryptor.is_scrypt() {
        return (true, false);
    }

    let identity = age::scrypt::Identity::new(passphrase);
    let mut reader = match decryptor.decrypt(std::iter::once(&identity as &dyn age::Identity)) {
        Ok(r) => r,
        Err(_) => return (true, false),
    };

    let mut decrypted = Vec::new();
    if reader.read_to_end(&mut decrypted).is_err() {
        return (true, false);
    }

    (true, true)
}
