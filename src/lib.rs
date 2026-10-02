pub mod gate;
pub mod package;
pub mod planning;
pub mod routing;
pub mod strict_json;

use sha2::{Digest, Sha256};
use std::{fs::File, io::Read, path::Path};

pub type Result<T> = std::result::Result<T, String>;

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn digest(path: &Path) -> Result<String> {
    let metadata = path.metadata().map_err(|_| "File unavailable")?;
    if !metadata.is_file() || metadata.len() > 256 * 1024 * 1024 {
        return Err("Unsupported file type or size".into());
    }
    let mut file = File::open(path).map_err(|_| "File unavailable")?;
    let mut hash = Sha256::new();
    let mut buffer = [0_u8; 65536];
    let mut total = 0;
    loop {
        let count = file.read(&mut buffer).map_err(|_| "File read failed")?;
        if count == 0 {
            break;
        }
        total += count;
        if total > 256 * 1024 * 1024 {
            return Err("File exceeds supported size".into());
        }
        hash.update(&buffer[..count]);
    }
    Ok(hex(&hash.finalize()))
}

pub fn read_bounded(path: &Path, limit: usize) -> Result<Vec<u8>> {
    if !path.is_file() {
        return Err("Input must be a regular file".into());
    }
    let file = File::open(path).map_err(|_| "Input unavailable")?;
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Input read failed")?;
    if bytes.len() > limit {
        return Err("Input exceeds supported size".into());
    }
    Ok(bytes)
}
