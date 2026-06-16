use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, IsTerminal, Write},
};

use serde::Deserialize;

use crate::cli::IpynbConvertArgs;
use anyhow::{Context, Result, bail};

use crate::errors::TerminalInputNotSupportedError;

pub(crate) fn handle(args: IpynbConvertArgs) -> Result<()> {
    if args.input_path.is_none() && io::stdin().is_terminal() {
        bail!(TerminalInputNotSupportedError);
    }

    let reader: Box<dyn BufRead> = match args.input_path {
        Some(path) => {
            let file = File::open(path).context("Cannot open input file")?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(io::stdin().lock()),
    };

    let notebook: Notebook = serde_json::from_reader(reader)?;

    let mut writer: Box<dyn Write> = match args.output_path {
        Some(path) => {
            let file = File::create(path).context("Cannot create output file")?;
            Box::new(BufWriter::new(file))
        }
        None => Box::new(io::stdout().lock()),
    };

    let code_cells = notebook
        .cells
        .into_iter()
        .filter(|c| c.cell_type == "code");

    for cell in code_cells {
        for line in cell.source {
            write!(writer, "{}", line)?;
        }
        writeln!(writer)?;
    }

    writer.flush()?;

    Ok(())
}

#[derive(Deserialize)]
struct Notebook {
    cells: Vec<Cell>,
}

#[derive(Deserialize, Debug)]
struct Cell {
    cell_type: String,
    source: Vec<String>,
}
