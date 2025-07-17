// src/main.rs
/*
 * Main executable for NftMarketplaceEngineChainX
 */

use clap::Parser;
use nftmarketplaceenginechainx::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceEngineChainX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
