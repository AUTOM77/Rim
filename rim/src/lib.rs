pub mod llm;
pub mod client;
pub mod modality;

use futures::StreamExt;

async fn caption(
    img: &modality::Image,
    clt: &client::RimClient,
    idx: usize
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let _b64 = img._base64().await?;
    let mut retries = 0;
    let _cap = loop {
        match clt.generate_caption(_b64.clone()).await {
            Ok(res) => break res,
            Err(e) => {
                println!("Retry {:#?} with {:?} times", idx, retries);
                retries += 1;
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            }
        };
    };
    let _ = img.save(_cap).await?;
    clt.log_api();
    Ok(idx)
}

async fn processing(
    images: Vec<modality::Image>,
    clients: Vec<client::RimClient>,
    limit: usize
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut tasks = futures::stream::FuturesUnordered::new();
    let mut num = 0;
    let total = clients.len();

    for chunk in images.chunks(limit) {
        for img in chunk {
            let clt = &clients[num % total];
            tasks.push(caption(img, clt, num));
            num += 1;
        }

        while let Some(handle) = tasks.next().await {
            match handle {
                Ok(i) => eprintln!("Success: {:?}", i),
                Err(e) => eprintln!("Task failed: {:?}", e),
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        tasks.clear();
    }
    Ok(())
}

pub fn _rt(pth: &str, keys: Vec<String>, prompt: String, limit: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
    let mut clients = Vec::new();

    for key in keys {
        let _prompt = prompt.clone();
        let _key = key.clone();
        let _client = client::RimClient::build(_prompt, _key);
        clients.push(_client);
    }

    let images: Vec<modality::Image> = std::fs::read_dir(pth)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path().display().to_string())
        .map(|f| modality::Image::from(&f).unwrap())
        .filter(|i| !i.existed())
        .collect();

    println!("Processing Media {:#?}", images.len());
    std::thread::sleep(std::time::Duration::from_secs(1));
    // println!("{:?}", images);
    // let mut images = Vec::new();
    // let i = modality::modality::Image::from(pth)?;
    // images.push(i);
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    match limit {
        Some(n) => rt.block_on(processing(images, clients, n)),
        None => rt.block_on(processing(images, clients, 1000))
    };
    Ok(())
}
