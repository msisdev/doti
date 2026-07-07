use std::path::PathBuf;

/**
 * Target:
 *    - Remote
 *    - Local
 *    - LocalEncrypted
 */

pub struct Target {
  path: PathBuf,
  exists: bool,
  is_encrypted: bool,
}

impl Target {

}