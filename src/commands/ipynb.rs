use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, IsTerminal, Write},
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

struct IpynbConverter<'a> {
    cell_index: u32,
    cell_headers_enabled: bool,
    cell_prefix: &'a str,
}

impl<'a> IpynbConverter<'a> {
    fn new(cell_start_index: u32, cell_headers_enabled: bool, cell_prefix: &'a str) -> Self {
        Self {
            cell_index: cell_start_index,
            cell_headers_enabled,
            cell_prefix,
        }
    }

    fn run<R, W>(&mut self, reader: R, writer: &mut W) -> anyhow::Result<()>
    where
        R: BufRead,
        W: Write,
    {
        let notebook: Notebook = serde_json::from_reader(reader)?;
        let code_cells = notebook.cells.into_iter().filter(|c| c.cell_type == "code");

        for cell in code_cells {
            if self.cell_headers_enabled {
                writeln!(writer, "# {}{}", self.cell_prefix, self.cell_index)?;
                self.cell_index += 1;
            }

            for line in cell.source {
                write!(writer, "{line}")?;
            }
            writeln!(writer)?;
        }

        writer.flush()?;

        Ok(())
    }
}

pub fn handle(args: IpynbConvertArgs) -> anyhow::Result<()> {
    if args.input_path.is_none() && io::stdin().is_terminal() {
        bail!(TerminalInputNotSupportedError);
    }

    let mut converter = IpynbConverter::new(
        args.cell_start_index,
        args.cell_headers_enabled,
        &args.cell_prefix,
    );

    if let Some(path) = &args.input_path {
        let file = File::open(path).context("Cannot open input file")?;
        let reader = BufReader::new(file);
        handle_writing(&args, &mut converter, reader)
    } else {
        let reader = io::stdin().lock();
        handle_writing(&args, &mut converter, reader)
    }
}

fn handle_writing<R>(
    args: &IpynbConvertArgs,
    converter: &mut IpynbConverter,
    reader: R,
) -> anyhow::Result<()>
where
    R: BufRead,
{
    if let Some(path) = &args.output_path {
        let file = File::create(path).context("Cannot create output file")?;
        let mut writer = BufWriter::new(file);
        converter.run(reader, &mut writer)
    } else {
        let mut writer = io::stdout().lock();
        converter.run(reader, &mut writer)
    }
}
