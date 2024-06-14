use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use futures::StreamExt;
use std::io::{self, Write};

pub mod client;
pub mod media;

pub use media::Media;
pub use client::{Service, Prompt};
use crate::client::base::API;
use crate::media::MediaProcessor;

async fn caption_n_shot(
    prompt: Prompt,
    media: Media,
    service: &Service,
    idx: usize,
    retry: usize
) -> Result<(usize, String), Box<dyn std::error::Error + Send + Sync>> {
    let images = media.process().await.unwrap();
    let delay = match retry {
        0 => (idx % 100) * 10,
        1 => 30 * 100,
        2 => 60 * 100,
        _ => 60 * 100,
    };
    tokio::time::sleep(tokio::time::Duration::from_millis(delay as u64)).await;

    match service.get_caption(&prompt.value, images.clone()).await {
        Ok(res) => {
            let (caption, consumption) = res;
            let _ = media.save_result(caption, service.current_model(), prompt.name).await?;
            Ok((idx, consumption))
        },
        Err(e) => {
            let failed = format!("{}", media.path().unwrap().display());
            // eprintln!("{}-shot {failed} failed: {:?}", retry, e);
            Err(failed.into())
        }
    }
}

async fn processing(
    prompts: Vec<Prompt>,
    media: Vec<Media>,
    services: Vec<Service>,
    limit: usize
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if services.is_empty() {
        eprintln!("No services configured");
        return Ok(());
    }

    let total_tasks = prompts.len() * media.len();
    let m = MultiProgress::new();
    let spinner_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {wide_bar:.cyan/blue} {pos}/{len} ({eta})")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let pb = m.add(ProgressBar::new(total_tasks as u64));
    let success_pb = m.add(ProgressBar::new(0));
    let failure_pb = m.add(ProgressBar::new(0));

    pb.set_style(spinner_style.clone());
    success_pb.set_style(spinner_style.clone());
    failure_pb.set_style(spinner_style.clone());

    let mut tasks = futures::stream::FuturesUnordered::new();
    let mut failed_tasks = Vec::new();
    let max_retries = 3;

    for prompt in &prompts {
        let mut num = 0;
        for _limited in media.chunks(limit) {
            for _media in _limited {
                let service = &services[num % services.len()];
                if !_media.is_processed(service.current_model(), &prompt.name) {
                    tasks.push(caption_n_shot(prompt.clone(), _media.clone(), service, num, 0));
                    num += 1;
                }
            }
            while let Some(handle) = tasks.next().await {
                pb.inc(1);
                match handle {
                    Ok((i, c)) => {
                        success_pb.set_message(format!("{} Consumption: {}", i, c));
                        success_pb.inc(1);
                    },
                    Err(e) => failed_tasks.push(e)
                };
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            tasks.clear();
        }
    }

    failure_pb.set_length(failed_tasks.len() as u64);
    failure_pb.inc(failed_tasks.len() as u64);

    for retry in 1..=max_retries {
        let mut current_failed_tasks = Vec::new();
        let mut num = 0;

        let media: Vec<_> = failed_tasks
            .into_iter()
            .map(|e| format!("{}", e))
            .filter_map(|f| Media::from(f.into()))
            .collect();

        for prompt in &prompts {
            for _media in &media {
                let service = &services[num % services.len()];
                if !_media.is_processed(service.current_model(), &prompt.name) {
                    tasks.push(caption_n_shot(prompt.clone(), _media.clone(), service, num, 0));
                    num += 1;
                }
            }

            while let Some(handle) = tasks.next().await {
                pb.inc(1);
                match handle {
                    Ok((i, c)) => {
                        success_pb.set_message(format!("{} Consumption: {}", i, c));
                        success_pb.inc(1);
                    },
                    Err(e) => {
                        eprintln!("{}-shot Task failed: {:?}", retry, e);
                        current_failed_tasks.push(e);
                    }
                };
            }
        }

        failed_tasks = current_failed_tasks;

        if failed_tasks.is_empty() {
            break;
        }
    }

    if !failed_tasks.is_empty() {
        let mut file = std::fs::OpenOptions::new().create(true).append(true).open("/tmp/failed.txt")?;
        for media_path in failed_tasks {
            writeln!(file, "{:?}", media_path)?;
            eprintln!("Media {:?} failed after {} retries:", media_path, max_retries);
        }
    }

    pb.finish_with_message("Processing completed");
    success_pb.finish_with_message("Successes completed");
    failure_pb.finish_with_message("Failures completed");

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

pub fn interface(pth: std::path::PathBuf, conf: String, limit: Option<usize>, qps: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let pth = match pth.is_file() {
        true => {
            let abs = std::fs::canonicalize(&pth)?;
            std::path::PathBuf::from(abs).parent().unwrap().to_path_buf()
        },
        false => pth,
    };

    println!("In {:?}", pth);

    let media: Vec<_> = std::fs::read_dir(pth)
        .unwrap()
        .filter_map(Result::ok)
        .filter_map(|f| Media::from(f.path()))
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
