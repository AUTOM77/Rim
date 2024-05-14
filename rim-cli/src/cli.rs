mod conf;

use clap::{Args, Parser};
use librim;

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Opts {
    _in: Option<String>,
}

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    opt: Opts,

    #[arg(short = 'c', long, name = "CONFIG")]
    config: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let cli = Cli::parse();

    let (prompt, gemini_keys, _) = conf::load(&cli.config).expect("Failed to decode TOML config");

    let opt = &cli.opt;

    if let Some(path) = opt._in.as_deref() {
        let _ = librim::_rt(path);
    }
    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}

