use std::fs;
use std::path;
use std::io::prelude::*;
use md5::{Md5, Digest};

use image::RgbImage;
use ffmpeg_next::{codec, format, frame, media, software};

pub fn _md5(path: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = Md5::new();
    let _ = hasher.update(&buffer);
    let digest = hasher.finalize();

    Ok(format!("{:x}", digest))
}

pub fn processing(path: &str, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {

    ffmpeg_next::init()?;

    let root = path::Path::new(&prefix);    
    let md5_hash = _md5(&path)?;    
    let dir_path = root.join(&md5_hash);    
    let _ = fs::create_dir_all(&dir_path)?;

    let mut ictx = format::input(&path)?;

    let input = ictx
            .streams()
            .best(media::Type::Video)
            .ok_or(ffmpeg_next::Error::StreamNotFound)?;
    let idx = input.index();

    let ctx = codec::context::Context::from_parameters(input.parameters())?;
    let mut decoder = ctx.decoder().video()?;

    let mut scaler = software::scaling::context::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            format::Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            software::scaling::flag::Flags::BILINEAR,
        )?;

    let mut count = 0;

    for (stream, packet) in ictx.packets() {
        if stream.index() == idx {
            decoder.send_packet(&packet)?;
            let mut decoded_frame = frame::video::Video::empty();
            while decoder.receive_frame(&mut decoded_frame).is_ok() {
                let mut rgb_frame = frame::video::Video::empty();
                scaler.run(&decoded_frame, &mut rgb_frame)?;
                save_frame(&rgb_frame, count, &dir_path)?;
                count+=1;
            }
        }
    }
    decoder.send_eof()?;
    Ok(())
}

pub fn save_frame(frame: &frame::video::Video, idx: usize, dir_path: &path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let width = frame.width();
    let height = frame.height();
    let data = frame.data(0).to_vec();
    
    let img = RgbImage::from_raw(width, height, data.to_vec()).unwrap(); 
    img.save(dir_path.join(format!("{:04}.png", idx)))?; 
    Ok(())
}

// pub async fn async_md5(path: &str) -> Result<String, std::io::Error> {
//     let mut file = AsyncFile::open(path).await?;
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer).await?;

//     let mut hasher = Md5::new();
//     let _ = hasher.update(&buffer);
//     let digest = hasher.finalize();

//     Ok(format!("{:x}", digest))
// }



















