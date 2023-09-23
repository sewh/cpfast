use std::error::Error;
use std::path::PathBuf;

use clap::Parser;
use nix;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    src: PathBuf,
    #[arg(short, long)]
    dst: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let in_size = std::fs::metadata(&args.src)?.len();
    let in_file = std::fs::File::open(&args.src)?;
    let out_file = std::fs::File::create(&args.dst)?;

    nix::sys::sendfile::sendfile(out_file, in_file, None, in_size as usize)?;

    Ok(())
}
