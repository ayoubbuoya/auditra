use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "auditra")]
#[command(version = "0.1.0")]
#[command(about = "AI-powered smart contract security scanner")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan a Solidity project
    Scan {
        /// Path to the project (Foundry project)
        #[arg(value_name = "PATH")]
        path: PathBuf,

        /// Enable verbose logging
        #[arg(short, long, default_value_t = false)]
        verbose: bool,
    },
}
