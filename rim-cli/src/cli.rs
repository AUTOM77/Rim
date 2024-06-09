use clap::{Args, Parser};

#[derive(Args)]
#[group(required = false, multiple = true)]
struct Opts {
    #[arg(long, name = "LIMIT", help = "NUM of FILE limit")]
    limit: Option<usize>,

    #[arg(long, name = "QPS", help = "QPS limit")]
    qps: Option<usize>,
}

#[derive(Parser)]
struct Cli {
    pth: String,

    #[arg(short = 'c', long, name = "CONFIG")]
    config: String,

    #[command(flatten)]
    opt: Opts,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();

    {
        let cli = Cli::parse();
        let conf_str = std::fs::read_to_string(&cli.config)?;
        let _ = librim::runtime(
            cli.pth.into(),
            conf_str, 
            cli.opt.limit, 
            cli.opt.qps
        );
    }

    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}

