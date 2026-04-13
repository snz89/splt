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
        None => Box::new(BufReader::new(io::stdin().lock())),
    };

    let notebook: Notebook = serde_json::from_reader(reader)?;
    let code_sources = notebook
        .cells
        .into_iter()
        .filter(|c| c.cell_type == "code")
        .map(|c| c.source.join(""));

    let mut writer: Box<dyn Write> = match args.output_path {
        Some(path) => {
            let file = File::create(path).context("Cannot open input file")?;
            Box::new(BufWriter::new(file))
        }
        None => Box::new(BufWriter::new(io::stdout().lock())),
    };

    for code in code_sources {
        writeln!(writer, "{}", code)?;
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
