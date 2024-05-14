use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Image {
    root: PathBuf,
    local: PathBuf,
}

impl Image {
    pub fn new(root: PathBuf, local:PathBuf) -> Self {
        Self { root, local }
    }

    pub fn with_root(mut self, root: PathBuf) -> Self {
        self.root = root;
        self
    }

    pub fn from(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let local = PathBuf::from(file);
        let _root = local
            .parent()
            .ok_or("No parent directory found")?;
        let root = PathBuf::from(format!("{}_cap", _root.display()));
        Ok(Self::new(root, local))
    }

    pub async fn _base64(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut f = tokio::fs::File::open(&self.local).await?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).await?;
        Ok(BASE64.encode(buffer))
    }

    pub async fn save(&self, cap: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _ = tokio::fs::create_dir_all(&self.root).await?;
        let local = self.local.file_stem().unwrap_or_default();
        let path = self.root.join(format!("{}.txt", local.to_string_lossy()));
        let mut f = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .await?;
        f.write_all(cap.as_bytes()).await?;
        Ok(())
    }

    pub fn existed(&self) -> bool {
        let local = self.local.file_stem().unwrap_or_default();
        let caption_path = self.root.join(format!("{}.txt", local.to_string_lossy()));

        std::fs::metadata(&caption_path).is_ok()
    }
}