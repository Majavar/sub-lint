#![feature(iter_map_windows)]

mod content;
mod lints;
mod message;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Subtitle file path
    path: PathBuf,
    /// Verbose mode
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.verbose {
        0 => std::env::set_var("RUST_LOG", "info"),
        1 => std::env::set_var("RUST_LOG", "debug"),
        _ => std::env::set_var("RUST_LOG", "trace"),
    }

    pretty_env_logger::init();

    content::Content::from_path(&args.path)?.check()
}
