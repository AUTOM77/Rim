pub mod client;
pub mod media;

pub use media::Media;
pub use client::Service;
use crate::client::base::API;
use crate::media::MediaProcessor;

use futures::StreamExt;

async fn caption(
    prompt: &str,
    media: &Media,
    clt: &Service,
    idx: usize
) -> Result<(usize, String), Box<dyn std::error::Error + Send + Sync>> {
    let images = media.process().await.unwrap();
    let _delay = (idx % 100) * 200;
    let mut retries = 0;
    let (caption, consumption) = loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(_delay as u64)).await;
        match clt.get_caption(prompt, images.clone()).await {
            Ok(res) => break res,
            Err(e) => {
                retries += 1;
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                println!("Retry {retries} - {:#?}", e);
            }
        };
    };
    let _ = media.save_result(caption).await?;
    Ok((idx, consumption))
}

async fn processing(
    prompt: &str,
    media: Vec<Media>,
    clients: Vec<Service>,
    limit: usize
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut tasks = futures::stream::FuturesUnordered::new();
    let mut num = 0;

    for chunk in media.chunks(limit) {
        for m in chunk {
            let clt = &clients[num % clients.len()];
            tasks.push(caption(prompt, m, clt, num));
            num += 1;
        }

        while let Some(handle) = tasks.next().await {
            let _ = match handle {
                Ok((i, c)) => eprintln!("Success: {:?}, Consumption: {}", i, c),
                Err(e) => eprintln!("Task failed: {:?}", e),
            };
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        tasks.clear();
    }
    Ok(())
}

pub fn runtime(
mut pth: std::path::PathBuf, 
    conf: String, 
    limit: Option<usize>, 
    qps: Option<usize>
) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    if pth.is_file() {
        let abs = std::fs::canonicalize(pth)?;
        pth = std::path::PathBuf::from(abs).parent().unwrap().to_path_buf();
    }

    let client_config = conf.parse::<client::Config>()?;
    let prompt = client_config.prompt();
    let azure: Vec<Service> = client_config.get("azure")
        .unwrap()
        .into_iter()
        .filter_map(|p| Service::from("azure", p.endpoint.clone(), p.key.clone(), p.model.clone()))
        .collect();

    let media: Vec<_> = std::fs::read_dir(pth)
        .unwrap()
        .filter_map(Result::ok)
        .filter_map(|f| Media::from(f.path()))
        .filter(|c| !c.is_processed() )
        .collect();

    let limit_num = limit.unwrap_or(100);
    let limit_media: Vec<Media> = media.into_iter().take(limit_num).collect();

    match qps {
        Some(n) => rt.block_on(processing(&prompt, limit_media, azure, n)),
        None => rt.block_on(processing(&prompt, limit_media, azure, 30))
    };
    Ok(())
}