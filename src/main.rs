use std::{
    fs::File,
    io::{self, BufRead, BufReader, IsTerminal},
};

use anyhow::{Context, Ok, Result, bail};
use clap::Parser;

use crate::{batching::BatchesIterator, cli::BatchConfig, errors::TerminalInputNotSupportedError};

mod batching;
mod cli;
mod errors;

fn handle(config: BatchConfig) -> Result<()> {
    if config.input_path.is_none() && io::stdin().is_terminal() {
        bail!(TerminalInputNotSupportedError);
    }

    let reader: Box<dyn BufRead> = match config.input_path {
        Some(path) => {
            let file = File::open(path).context("Cannot open input file")?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin().lock())),
    };
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
