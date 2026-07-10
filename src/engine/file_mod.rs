use std::path::{Path, PathBuf};

use crate::engine::state::{FileState, has_secret_extension, to_plain_path, to_secret_path};

/// A path object that serves as an IO target.
pub struct DotiPath {
    pub path: PathBuf,
    pub state: FileState,
    pub exists: bool,
}

// impl new() constructors
impl DotiPath {
    // Remove `.doti_secret` extension if it exists
    fn strip_secret_ext(path: &Path) -> PathBuf {
        to_plain_path(path)
    }

    /// Automatically detects the exact path and state regardless of whether 
    /// the provided path includes the `.doti_secret` extension or not.
    pub fn auto_detect(base_path: impl Into<PathBuf>) -> Self {
        let path = base_path.into();
        let plain_path = Self::strip_secret_ext(&path);
        let secret_path = to_secret_path(&plain_path);

        if secret_path.exists() {
            Self {
                path: secret_path,
                // Armored 여부는 파일 내용을 읽는 단계에서 판별합니다.
                state: FileState::Age { armored: false },
                exists: true,
            }
        } else if plain_path.exists() {
            Self {
                path: plain_path,
                state: FileState::Plaintext,
                exists: true,
            }
        } else {
            // If neither exists, determine the state based on the provided path's extension
            if has_secret_extension(&path) {
                Self {
                    path,
                    state: FileState::Age { armored: false },
                    exists: false,
                }
            } else {
                Self {
                    path,
                    state: FileState::Plaintext,
                    exists: false,
                }
            }
        }
    }

    /// Forces the target file to be considered as Plaintext. 
    /// The secret extensions are automatically removed.
    pub fn new_plaintext(path: impl AsRef<Path>) -> Self {
        let path = Self::strip_secret_ext(path.as_ref());
        let exists = path.exists();
        Self {
            path,
            state: FileState::Plaintext,
            exists,
        }
    }

    /// Forces the target file to be considered as Encrypted.
    /// The `.doti_secret` extension is automatically added to the file name.
    pub fn new_encrypted(path: impl AsRef<Path>, armored: bool) -> Self {
        let plain_path = Self::strip_secret_ext(path.as_ref());
        let path = to_secret_path(&plain_path);
        
        let exists = path.exists();
        Self {
            path,
            state: FileState::Age { armored },
            exists,
        }
    }
}
