use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Image {
    root: PathBuf,
    local: String,
}

impl Image {
    pub fn new(root: PathBuf, local:String) -> Self {
        Self { root, local }
    }

    pub fn with_root(mut self, root: PathBuf) -> Self {
        self.root = root;
        self
    }

    pub fn from(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = PathBuf::from(file);
        let local = file_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or("Invalid file stem")?
            .to_string();
        let _root = file_path
            .parent()
            .ok_or("No parent directory found")?;
        let root = PathBuf::from(format!("{}_caption", _root.display()));
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

        let path = self.root.join(format!("{}.txt", self.local));
        let mut f = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .await?;
        f.write_all(cap.as_bytes()).await?;
        Ok(())
    }
}