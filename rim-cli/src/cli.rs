mod conf;

use clap::{Args, Parser};
use librim;

#[derive(Args)]
#[group(required = false, multiple = true)]
struct Opts {
    #[arg(long, name = "LIMIT", help = "QPS limit num")]
    limit: Option<usize>,
}

#[derive(Parser)]
struct Cli {
    _in: String,

    #[arg(short = 'c', long, name = "CONFIG")]
    config: String,

    #[command(flatten)]
    opt: Opts,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let cli = Cli::parse();

    let (prompt, gemini_keys, _) = conf::load(&cli.config).expect("Failed to decode TOML config");
    let _ = librim::_rt(&cli._in, gemini_keys, prompt, cli.opt.limit);

    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}

