pub mod model;
pub mod client;

pub fn single_cap(f: &str, conf: String) {
    println!("Processing file: {} with {}", f, conf);
}

pub fn batch_cap(d: &str, conf: String) {
    println!("Processing directory: {} with {}", d, conf);
}