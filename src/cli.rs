use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = "Split a file into multiple batches", long_about = None)]
pub struct BatchConfig {
    #[arg(help = "Input file to process")]
    pub input_path: PathBuf,

    #[arg(short, long, default_value_t = 80, help = "Max length of line")]
    pub max_line_length: usize,

    #[arg(short, long, default_values_t = [55, 61], help = "Maximum number of lines in a batch considering line wrapping.
Multiple values can be specified; if there are more batches than values,
the last value will be used for the remaining batches")]
    pub weigths: Vec<usize>,

    #[arg(short, long, default_value = "batches", help = "Output directory where generated batch files will be saved")]
    pub output_dir: PathBuf,
}
