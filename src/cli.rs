use std::path::PathBuf;

use clap::{
    Args, Parser, Subcommand,
    builder::{Styles, styling::AnsiColor},
};

fn get_styles() -> Styles {
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

#[derive(Subcommand)]
pub enum Commands {
    Split(SplitArgs),
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
#[command(about = "Converts an .ipynb file to a regular .py file")]
pub struct IpynbConvertArgs {
    /// Input file to process
    pub input_path: Option<PathBuf>,

    /// Path to the file where the result will be saved
    #[arg(short, long, default_value = "batches")]
    pub output_dir: Option<PathBuf>,
}
