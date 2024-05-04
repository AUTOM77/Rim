pub mod llm;
pub mod client;
pub mod modality;

use tokio;
use tokio_stream::StreamExt;

pub fn _rt() -> Result<tokio::runtime::Runtime, Box<dyn std::error::Error>>  {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    Ok(rt)
}

pub fn single_cap(f: &str) {
    let start_time = std::time::Instant::now();
    println!("Processing file: {:?}", f);

    if let Ok(x) = _rt() {
        x.block_on(async {
            let _b64 = modality::image::async_base64(f.into()).await;
        });
    }

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}

pub fn batch_cap(d: &str) {
    let start_time = std::time::Instant::now();
    println!("Processing directory: {:?}", d);

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut tasks: Vec<_> = std::fs::read_dir(d)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().unwrap_or_default() == "png")
            .map(|f| tokio::spawn(async move { modality::image::async_base64_log(f).await; }))
            .collect();
        for task in tasks {
            task.await.unwrap();
        }
    });

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}