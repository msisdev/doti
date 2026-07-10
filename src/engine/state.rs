use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub const SECRET_EXT: &str = ".doti_secret";
const AGE_ARMOR_HEADER: &[u8] = b"-----BEGIN AGE ENCRYPTED FILE-----";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileState {
    Plaintext,
    Age { armored: bool },
}

pub fn has_secret_extension(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(SECRET_EXT))
}

pub fn to_secret_path(path: &Path) -> PathBuf {
    if has_secret_extension(path) {
        return path.to_path_buf();
    }

    let mut new_path = path.to_path_buf();
    let mut file_name = new_path.file_name().unwrap_or_default().to_os_string();
    file_name.push(SECRET_EXT);
    new_path.set_file_name(file_name);
    new_path
}

pub fn to_plain_path(path: &Path) -> PathBuf {
    if !has_secret_extension(path) {
        return path.to_path_buf();
    }

    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
    let stripped = &file_name[..file_name.len() - SECRET_EXT.len()];
    path.with_file_name(stripped)
}

pub fn detect_file_state_from_path(path: impl AsRef<Path>) -> io::Result<FileState> {
    let path = path.as_ref();

    let existing_path = if path.exists() {
        path.to_path_buf()
    } else {
        let counterpart = if has_secret_extension(path) {
            to_plain_path(path)
        } else {
            to_secret_path(path)
        };

        if counterpart.exists() {
            counterpart
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("No file found for path or counterpart: {}", path.display()),
            ));
        }
    };

    if has_secret_extension(&existing_path) {
        let mut f = File::open(existing_path)?;
        let mut prefix = [0_u8; 40];
        let n = f.read(&mut prefix)?;
        let armored = prefix[..n].starts_with(AGE_ARMOR_HEADER);
        Ok(FileState::Age { armored })
    } else {
        Ok(FileState::Plaintext)
    }
}