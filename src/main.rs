use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Ok, Result};
use clap::Parser;

use crate::{batching::BatchesIterator, cli::BatchConfig};

mod batching;
mod cli;

fn handle(config: BatchConfig) -> Result<()> {
    let file = File::open(config.input_path).context("Cannot open input file")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map_while(Result::ok);
    let batches = BatchesIterator::new(
        lines.into_iter(),
        config.max_line_length,
        config.weights.into_iter(),
    )?;

    batching::write_batches(batches, &config.output_dir)?;
    Ok(())
}

fn main() -> Result<()> {
    let config = BatchConfig::parse();
    handle(config)?;
    Ok(())
}
