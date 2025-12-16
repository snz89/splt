use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

use crate::{batching::BatchesIterator, cli::BatchConfig};

mod batching;
mod cli;

fn main() -> Result<()> {
    let config = BatchConfig::build()?;

    let file = File::open(config.input_path).context("Cannot open input file")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().filter_map(Result::ok);
    let batches = BatchesIterator::new(
        lines.into_iter(),
        config.max_line_length,
        config.weights.into_iter(),
    )?;

    batching::write_batches(batches, &config.output_dir)?;

    Ok(())
}
