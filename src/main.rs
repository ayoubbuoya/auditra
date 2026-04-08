use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, verbose } => {
            println!("🔍 Scanning path: {:?}", path);

            if verbose {
                println!("🔍 Starting scan in verbose mode...");
            }
        }
    }

    Ok(())
}
