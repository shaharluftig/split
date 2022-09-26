use clap::Parser;

/// A simple and very fast implementation of Unix Split tool using Rust
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Number of lines per output file
    #[clap(short, long, value_parser, default_value_t = 1000)]
    pub lines: usize,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    pub path: std::path::PathBuf,
    /// Ignore empty lines,
    #[clap(long, value_parser, default_value_t = false)]
    pub ignore_empty_lines: bool,
}