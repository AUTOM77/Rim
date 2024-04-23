use clap::{Args, Parser};

use librimc::{single_cap, batch_cap};

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

    let opt = &cli.opt;
    if let Some(file_path) = opt.file.as_deref() {
        let _ = single_cap(file_path, cli.config);
    } else if let Some(dir_path) = opt.dir.as_deref() {
        let _ = batch_cap(dir_path, cli.config);
    }
}
