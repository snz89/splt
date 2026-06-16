use std::path::PathBuf;

use clap::{
    Args, Parser, Subcommand,
    builder::{Styles, styling::AnsiColor},
};

use crate::commands;

const fn get_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Cyan.on_default())
        .placeholder(AnsiColor::Blue.on_default())
}

#[derive(Parser)]
#[command(long_about = None, styles = get_styles(), version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Split(args) => commands::split::handle(args)?,
            Commands::Ipynb(args) => commands::ipynb::handle(args)?,
        }
        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Split(SplitArgs),
    Ipynb(IpynbConvertArgs),
}

#[derive(Args)]
#[command(about = "Split a file into multiple batches")]
pub struct SplitArgs {
    /// Input file to process
    pub input_path: Option<PathBuf>,

    /// Max length of line
    #[arg(short, long, default_value_t = 80)]
    pub max_line_length: usize,

    /// Maximum number of lines in a batch considering line wrapping.
    /// Multiple values can be specified; if there are more batches than values,
    /// the last value will be used for the remaining batches
    #[arg(short, long, default_values_t = [55, 61], verbatim_doc_comment)]
    pub weights: Vec<usize>,

    /// Output directory where generated batch files will be saved
    #[arg(short, long, default_value = "batches")]
    pub output_dir: PathBuf,
}

#[derive(Args)]
#[command(
    about = "Extracts code cells from a specified .ipynb file and generates a standard .py file"
)]
pub struct IpynbConvertArgs {
    /// Path to the source .ipynb notebook
    pub input_path: Option<PathBuf>,

    /// Path to the resulting .py file
    #[arg(short, long)]
    pub output_path: Option<PathBuf>,

    /// Add a comment header in the output file before each code cell
    #[arg(short, long = "cell_headers")]
    pub cell_headers_enabled: bool,

    /// Starting index number for the cell header comments
    #[arg(long, default_value_t = 1, requires = "cell_headers_enabled")]
    pub cell_start_index: u32,

    /// Prefix text placed before the index number in the cell header comments
    #[arg(long, default_value = "Cell ", requires = "cell_headers_enabled")]
    pub cell_prefix: String,
}
