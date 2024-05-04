use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

use tokio::fs;
use tokio::io::AsyncReadExt;

pub async fn async_base64(path: std::path::PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut f = fs::File::open(path).await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;

    Ok(BASE64.encode(buffer))
}