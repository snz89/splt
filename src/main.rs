use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

use crate::cli::BatchConfig;

mod batching;
mod cli;

fn main() -> Result<()> {
    let config = BatchConfig::build()?;

    let file = File::open(config.input_path).context("Cannot open input file")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let batches = batching::collect_lines_to_batches(
        &lines,
        config.max_line_length,
        &mut config.weigths.into_iter(),
    )?;
    batching::write_batches(&batches, &config.output_dir)?;

    Ok(())
}
