use clap::{Args, Parser};
use librim::client::RimClient;
use librim::{single_cap, batch_cap};

mod conf;

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Opts {
    #[arg(short = 'f', long, name = "FILE")]
    file: Option<String>,

    #[arg(short = 'd', long, name = "DIR")]
    dir: Option<String>,
}

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    opt: Opts,

    #[arg(short = 'c', long, name = "CONFIG")]
    config: String,
}

fn main() {
    let cli = Cli::parse();

    let (prompt, gemini_keys, _) = conf::load(&cli.config).expect("Failed to decode TOML config");

    let client = RimClient::build("gemini", prompt, gemini_keys);

    let _ = client.log_prompt();

    let opt = &cli.opt;
    if let Some(file_path) = opt.file.as_deref() {
        let _ = single_cap(file_path);
    } else if let Some(dir_path) = opt.dir.as_deref() {
        let _ = batch_cap(dir_path);
    }
}
