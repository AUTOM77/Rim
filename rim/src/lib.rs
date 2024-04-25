pub mod model;
pub mod utils;
pub mod client;

use tokio;

pub fn single_cap(f: &str) {
    println!("Processing file: {:?} ", utils::image::_base64(f));
}

pub fn async_single_cap(f: &str) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        match utils::image::async_base64(f).await {
            Ok(encoded_string) => {
                println!("Processing file: {}", encoded_string);
            }
            Err(error) => {
                eprintln!("Error encoding file: {}", error);
            }
        }
    });
}

pub fn batch_cap(d: &str) {
    println!("Processing directory: {:?}", d);
}