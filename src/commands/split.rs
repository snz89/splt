use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, IsTerminal, Write},
    path::Path,
};

use anyhow::{Context, bail};

use crate::{
    batching::{Batch, BatchesIterator},
    cli::SplitArgs,
    errors::TerminalInputNotSupportedError,
};

pub fn handle(args: SplitArgs) -> anyhow::Result<()> {
    if args.input_path.is_none() && io::stdin().is_terminal() {
        bail!(TerminalInputNotSupportedError);
    }

    let max_line_length = args.max_line_length;
    let weights_iter = args.weights.into_iter();
    let output_dir = &args.output_dir;

    if let Some(path) = args.input_path {
        let file = File::open(path).context("Cannot open input file")?;
        let reader = BufReader::new(file);
        split(reader, max_line_length, weights_iter, output_dir)
    } else {
        let reader = io::stdin().lock();
        split(reader, max_line_length, weights_iter, output_dir)
    }
}

fn split<R>(
    reader: R,
    max_line_length: usize,
    weights_iter: impl Iterator<Item = usize>,
    output_dir: &Path,
) -> anyhow::Result<()>
where
    R: BufRead,
{
    let lines = reader.lines().map_while(Result::ok);
    let batches = BatchesIterator::new(lines, max_line_length, weights_iter)?;

    write_batches(batches, output_dir)?;
    Ok(())
}

fn write_batches<I>(batches: I, output_dir: &Path) -> io::Result<()>
where
    I: Iterator<Item = Batch>,
{
    fs::create_dir_all(output_dir)?;

    for (batch_id, batch) in batches.enumerate() {
        let batch_path = output_dir.join(format!("batch_{batch_id}.txt"));
        let file = File::create(batch_path)?;
        let mut writer = BufWriter::new(file);

        for (i, line) in batch.lines().iter().enumerate() {
            if i > 0 {
                writer.write_all(b"\n")?;
            }
            writer.write_all(line.as_bytes())?;
        }

        writer.flush()?;
    }

    std::result::Result::Ok(())
}
