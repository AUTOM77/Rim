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

    let (prompt, vertex_project, vertex_key, gemini_keys, _) = rim_cli::parse(&cli.config).expect("Failed to decode TOML config");
    // println!("{}, {}", vertex_project, vertex_key);
    // let _ = librim::_rt(&cli._in, gemini_keys, prompt, cli.opt.limit);
    let _ = librim::_rt(vertex_project, vertex_key, prompt);
    
    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}

