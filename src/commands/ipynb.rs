use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, IsTerminal, Write},
    path::Path,
};

use serde::Deserialize;

use crate::cli::IpynbConvertArgs;
use anyhow::{Context, bail};

use crate::errors::TerminalInputNotSupportedError;

#[derive(Deserialize)]
struct Notebook {
    cells: Vec<Cell>,
}

#[derive(Deserialize, Debug)]
struct Cell {
    cell_type: String,
    source: Vec<String>,
}

pub fn handle(args: IpynbConvertArgs) -> anyhow::Result<()> {
    if args.input_path.is_none() && io::stdin().is_terminal() {
        bail!(TerminalInputNotSupportedError);
    }

    let output_path = args.output_path.as_deref();

    if let Some(path) = args.input_path {
        let file = File::open(path).context("Cannot open input file")?;
        let reader = BufReader::new(file);
        handle_output(
            output_path,
            args.cell_start_index,
            args.cell_headers_enabled,
            &args.cell_prefix,
            reader,
        )
    } else {
        let reader = io::stdin().lock();
        handle_output(
            output_path,
            args.cell_start_index,
            args.cell_headers_enabled,
            &args.cell_prefix,
            reader,
        )
    }
}

fn handle_output<R>(
    output_path: Option<&Path>,
    cell_start_index: u32,
    cell_headers_enabled: bool,
    cell_prefix: &str,
    reader: R,
) -> anyhow::Result<()>
where
    R: BufRead,
{
    if let Some(path) = output_path {
        let file = File::create(path).context("Cannot create output file")?;
        let mut writer = BufWriter::new(file);
        handle_streams(
            cell_start_index,
            cell_headers_enabled,
            cell_prefix,
            reader,
            &mut writer,
        )
    } else {
        let mut writer = io::stdout().lock();
        handle_streams(
            cell_start_index,
            cell_headers_enabled,
            cell_prefix,
            reader,
            &mut writer,
        )
    }
}

fn handle_streams<R, W>(
    cell_start_index: u32,
    cell_headers_enabled: bool,
    cell_prefix: &str,
    reader: R,
    writer: &mut W,
) -> anyhow::Result<()>
where
    R: BufRead,
    W: Write,
{
    let notebook: Notebook = serde_json::from_reader(reader)?;
    let code_cells = notebook.cells.into_iter().filter(|c| c.cell_type == "code");

    let mut cell_index = cell_start_index;
    for cell in code_cells {
        if cell_headers_enabled {
            writeln!(writer, "# {cell_prefix}{cell_index}")?;
            cell_index += 1;
        }

        for line in cell.source {
            write!(writer, "{line}")?;
        }
        writeln!(writer)?;
    }

    writer.flush()?;

    Ok(())
}
