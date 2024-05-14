use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn async_base64(path: std::path::PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut f = fs::File::open(&path).await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;

    Ok(BASE64.encode(buffer))
}

pub async fn async_base64_log(path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = fs::File::open(&path).await?;

    let _f = path.file_stem().and_then(|stem| stem.to_str()).unwrap();
    let _pth = std::path::PathBuf::from("/dev/shm/tmp").join(format!("{}.txt", _f));
    let mut file = fs::File::create(_pth).await?;

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;

    file.write_all(BASE64.encode(buffer).as_bytes()).await?;
    Ok(())
}