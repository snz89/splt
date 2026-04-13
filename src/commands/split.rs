use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, IsTerminal, Write},
    path::Path,
};

use anyhow::{Context, Ok, Result, bail};

use crate::{
    batching::{Batch, BatchesIterator},
    cli::SplitArgs,
    errors::TerminalInputNotSupportedError,
};

pub(crate) fn handle(args: SplitArgs) -> Result<()> {
    if args.input_path.is_none() && io::stdin().is_terminal() {
        bail!(TerminalInputNotSupportedError);
    }

    let reader: Box<dyn BufRead> = match args.input_path {
        Some(path) => {
            let file = File::open(path).context("Cannot open input file")?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin().lock())),
    };
    let lines = reader.lines().map_while(Result::ok);
    let batches = BatchesIterator::new(
        lines.into_iter(),
        args.max_line_length,
        args.weights.into_iter(),
    )?;

    write_batches(batches, &args.output_dir)?;
    Ok(())
}

fn write_batches(batches: impl Iterator<Item = Batch>, output_dir: &Path) -> io::Result<()> {
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
