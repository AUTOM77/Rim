pub mod llm;
pub mod client;
pub mod modality;

use futures::StreamExt;

async fn caption(img: modality::Image, clt: &client::RimClient) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _b64 = img._base64().await?;
    let _cap = clt.generate_caption(_b64).await?;
    let _ = img.save(_cap).await?;
    Ok(())
}

async fn processing(images: Vec<modality::Image>, clients: Vec<client::RimClient>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut tasks = futures::stream::FuturesUnordered::new();
    let total = clients.len();

    for (i, img) in images.into_iter().enumerate() {
        let clt = &clients[i % total];
        tasks.push(caption(img, clt));
    }

    while let Some(handle) = tasks.next().await {
        let res = match handle {
            Ok(socket) => socket,
            Err(e) => {
                println!("Error {:#?}", e);
                std::thread::sleep(tokio::time::Duration::from_millis(10));
                continue;
            }
        };
    }

    Ok(())
}

pub fn _rt(pth: &str, keys: Vec<String>, prompt: String) -> Result<(), Box<dyn std::error::Error>> {

    let mut clients = Vec::new();

    for key in keys{
        let _prompt = prompt.clone();
        let _key = key.clone();
        let _client= client::RimClient::build(_prompt, _key);
        clients.push(_client);
    }

    let images: Vec<modality::Image> = std::fs::read_dir(pth)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path().display().to_string())
        .map(|f| modality::Image::from(&f).unwrap())
        .filter(|i| !i.existed())
        .collect();

    // println!("{:?}", images);
    // let mut images = Vec::new();
    // let i = modality::modality::Image::from(pth)?;
    // images.push(i);

    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let _ = rt.block_on(processing(images, clients));
    Ok(())
}