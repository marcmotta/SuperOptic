// src/main.rs
/*
 * Main executable for SuperOptic
 */

use clap::Parser;
use superoptic::{Result, run};

#[derive(Parser)]
#[command(version, about = "SuperOptic - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
