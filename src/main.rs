use clap::Parser;

use crate::cli::Cli;

mod batching;
mod cli;
mod commands;
mod errors;

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
