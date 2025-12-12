use std::path::PathBuf;

use clap::{
    CommandFactory, Error, FromArgMatches, Parser, builder::{Styles, styling::AnsiColor}
};

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

    #[arg(
        short,
        long,
        default_value = "batches",
        help = "Output directory where generated batch files will be saved"
    )]
    pub output_dir: PathBuf,
}

impl BatchConfig {
    pub fn build() -> Result<Self, Error> {
        let mut command = BatchConfig::command();

        let styles = Styles::styled()
            .header(AnsiColor::Yellow.on_default())
            .usage(AnsiColor::Green.on_default())
            .literal(AnsiColor::Cyan.on_default())
            .placeholder(AnsiColor::Blue.on_default());

        command = command.styles(styles);

        let mut matches = command.get_matches();
        return BatchConfig::from_arg_matches_mut(&mut matches);
    }
}
