pub mod llm;
pub mod client;
pub mod modality;

use tokio;

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
                let b64 = modality::image::async_base64(f.into()).await;
                // println!("{:?} ", b64);
            });
    }

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}

pub fn batch_cap(d: &str) {
    let start_time = std::time::Instant::now();
    println!("Processing directory: {:?}", d);

    if let Ok(x) = _rt() {
        x.block_on(async {
            let mut _tasks = vec![];

            let buckets: Vec<_> = std::fs::read_dir(d).unwrap()
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| path.extension().unwrap_or_default() == "png")
                .collect();
            
            for f in buckets{
                let _e = tokio::spawn(async move {
                    let _ = modality::image::async_base64(f).await;
                    // println!("{:?} ", b64);
                });
                _tasks.push(_e);
            }

            for t in _tasks {
                t.await.unwrap();
            }

        });
    }

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}