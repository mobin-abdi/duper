use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub hash: String,
}

impl FileInfo {
    pub fn from_path(path: PathBuf) -> Self {
        let metadata = std::fs::metadata(&path).unwrap();

        Self {
            path,
            size: metadata.len(),
            hash: String::new(),
        }
    }

    pub fn calculate_hash(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];

        loop {
            let bytes_read = file.read(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        let hash = hasher.finalize();
        Ok(hash.iter().map(|byte| format!("{byte:02x}")).collect())
    }
}
