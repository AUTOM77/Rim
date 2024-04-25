use std::fs::File;
use std::io::Read;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _}; 
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncReadExt;

pub fn _base64(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(BASE64.encode(buffer))
}

pub async fn async_base64(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = AsyncFile::open(path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(BASE64.encode(buffer))
}