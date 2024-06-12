use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use ffmpeg_next::{codec, format, frame, media, software};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io::Cursor;
use std::path::PathBuf;

pub trait MediaProcessor {
    fn path(&self) -> &PathBuf;
    async fn process(&self) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    fn get_save_path(&self, model: &str, prompt: &str) -> PathBuf {
        let mut local = self.path().clone();
        local.pop();
        local.push(model);
        local.push(prompt);
        let _ = std::fs::create_dir_all(&local);
        local.push(self.path().file_name().unwrap());
        local.set_extension("txt");
        local
    }
    fn is_processed(&self, model: &str, prompt: &str) -> bool {
        std::fs::metadata(self.get_save_path(model, prompt)).is_ok()
    }
    async fn save_result(&self, content: String, model: &str, prompt: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let save_path = self.get_save_path(model, prompt);
        let mut file = tokio::fs::OpenOptions::new().create(true).write(true).open(save_path).await?;
        file.write_all(content.as_bytes()).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Media {
    Image(Image),
    Video(Video),
    Unsupported,
}

impl Media {
    pub fn from(path: PathBuf) -> Option<Self> {
        match infer::get_from_path(path.clone()) {
            Ok(Some(kind)) => match kind.mime_type() {
                mime if mime.starts_with("image") => Some(Media::Image(Image::new(path))),
                mime if mime.starts_with("video") => Some(Media::Video(Video::new(path))),
                _ => None,
            },
            _ => None,
        }
    }
    
    pub fn path(&self) -> Option<PathBuf> {
        match self {
            Media::Image(image) => Some(image.path().into()),
            Media::Video(video) => Some(video.path().into()),
            Media::Unsupported => None
        }
    }

    pub fn is_processed(&self, model: &str, prompt: &str) -> bool {
        match self {
            Media::Image(image) => image.is_processed(model, prompt),
            Media::Video(video) => video.is_processed(model, prompt),
            Media::Unsupported => false
        }
    }

    pub async fn process(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        match self {
            Media::Image(image) => image.process().await,
            Media::Video(video) => video.process().await,
            Media::Unsupported => Err("Unsupported media type".into()),
        }
    }

    pub async fn save_result(&self, content: String, model: &str, prompt: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Media::Image(image) => image.save_result(content, model, &prompt).await,
            Media::Video(video) => video.save_result(content, model, &prompt).await,
            Media::Unsupported => Err("Unsupported media type".into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Video {
    path: PathBuf,
}

impl Image {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Video {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl MediaProcessor for Image {
    fn path(&self) -> &PathBuf {
        &self.path
    }
    async fn process(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut res = Vec::new();
        let img = image::open(&self.path)?;
        let rgb_img = img.to_rgb8();
        res.push(encode(rgb_img)?);
        Ok(res)
    }
}

impl MediaProcessor for Video {
    fn path(&self) -> &PathBuf {
        &self.path
    }
    async fn process(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        const FRAME_LIMIT: usize = 10;

        let mut res = Vec::new();
        let frames = self.seek(FRAME_LIMIT).await?;
        for frame in frames {
            let rgb_img = image::RgbImage::from_raw(
                frame.width(),
                frame.height(),
                frame.data(0).to_owned(),
            ).ok_or("Failed to create image buffer")?;

            res.push(encode(rgb_img)?);
        }
        Ok(res)
    }
}

impl Video {
    async fn seek(&self, frame_limit: usize) -> Result<Vec<frame::video::Video>, Box<dyn std::error::Error>> {
        let mut frames = Vec::with_capacity(frame_limit);

        ffmpeg_next::init()?;
        let mut ictx = format::input(&self.path)?;

        let input = ictx.streams().best(media::Type::Video).ok_or("Could not find video stream")?;
        let ctx = codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = ctx.decoder().video()?;

        let idx = input.index();

        let mut scaler = software::scaling::context::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            format::Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            software::scaling::flag::Flags::BILINEAR,
        )?;

        let time_base: f64 = input.time_base().into();
        let frame_rate: f64 = input.avg_frame_rate().into();
        let frame_num = input.frames();

        let frame_sample: Vec<usize> = (0..frame_num as usize).step_by(frame_num as usize / frame_limit).collect();

        for i in frame_sample {
            let seek_idx = (i as f64 / frame_rate) / time_base;
            ictx.seek((seek_idx * time_base) as i64, ..((seek_idx * time_base) as i64 + 1))?;
            for (stream, packet) in ictx.packets() {
                if stream.index() == idx {
                    decoder.send_packet(&packet)?;
                    let mut decoded_frame = frame::video::Video::empty();
                    while decoder.receive_frame(&mut decoded_frame).is_ok() {
                        let mut rgb_frame = frame::video::Video::empty();
                        scaler.run(&decoded_frame, &mut rgb_frame)?;
                        frames.push(rgb_frame);
                        break;
                    }
                    break;
                }
            }
        }
        decoder.send_eof()?;
        Ok(frames)
    }
}

fn encode(img: image::RgbImage) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    img.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Jpeg)?;
    let hash = BASE64.encode(buffer);
    Ok(hash)
}
