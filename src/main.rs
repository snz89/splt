use anyhow::{Ok, Result};
use clap::Parser;

use crate::cli::{Cli, Commands};

mod batching;
mod cli;
mod commands;
mod errors;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Split(split_args) => commands::split::handle(split_args)?,
    };
    
    Ok(())
}
