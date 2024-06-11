pub mod client;
pub mod media;

pub use media::Media;
pub use client::{Service, Prompt};
use crate::client::base::API;
use crate::media::MediaProcessor;

use futures::StreamExt;

async fn caption(
    prompt: Prompt,
    media: &Media,
    clt: &Service,
    idx: usize
) -> Result<(usize, String), Box<dyn std::error::Error + Send + Sync>> {
    let images = media.process().await.unwrap();
    let _delay = (idx % 100) * 200;
    let mut retries = 0;
    let (caption, consumption) = loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(_delay as u64)).await;
        match clt.get_caption(&prompt.value, images.clone()).await {
            Ok(res) => break res,
            Err(e) => {
                retries += 1;
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                println!("Retry {retries} - {:#?}", e);
            }
        };
    };
    let _ = media.save_result(caption, clt.current_model(), prompt.name).await?;
    Ok((idx, consumption))
}

async fn processing(
    prompts: Vec<Prompt>,
    media: Vec<Media>,
    clients: Vec<Service>,
    limit: usize
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if clients.is_empty() {
        eprintln!("No services configured");
        return Ok(());
    }

    let mut tasks = futures::stream::FuturesUnordered::new();
    for prompt in prompts{
        let mut num = 0;
        for chunk in media.chunks(limit) {
            for m in chunk {
                let clt = &clients[num % clients.len()];
                if !m.is_processed(clt.current_model(), &prompt.name){
                    tasks.push(caption(prompt.clone(), m, clt, num));
                    num += 1;
                }
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
    }

    Ok(())
}

fn load_services(conf: &client::Config, key: &str) -> Vec<Service> {
    conf.get(key)
        .map(|s| {
            s.into_iter()
                .filter_map(|p| Service::from(key, p.endpoint.clone(), p.key.clone(), p.model.clone()))
                .collect()
        })
        .unwrap_or_default()
}


pub fn interface(pth: std::path::PathBuf, conf: String, limit: Option<usize>, qps: Option<usize>) -> Result<(), Box<dyn std::error::Error>>{
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let pth = match pth.is_file() {
        true => {
            let abs = std::fs::canonicalize(&pth)?;
            std::path::PathBuf::from(abs).parent().unwrap().to_path_buf()
        },
        false => pth,
    };

    let media: Vec<_> = std::fs::read_dir(pth)
        .unwrap()
        .filter_map(Result::ok)
        .filter_map(|f| Media::from(f.path()))
        // .filter(|c| !c.is_processed() )
        .collect();

    let conf = conf.parse::<client::Config>()?;
    let prompts = conf.prompts();
    let azure = load_services(&conf, "azure");
    let gemini = load_services(&conf, "gemini");

    let limit_num = limit.unwrap_or(100);
    let qps_num = qps.unwrap_or(30);

    let limited_media: Vec<Media> = media.into_iter().take(limit_num).collect();

    rt.block_on(processing(prompts.clone(), limited_media.clone(), azure, qps_num));
    rt.block_on(processing(prompts.clone(), limited_media.clone(), gemini, qps_num));
    Ok(())
}