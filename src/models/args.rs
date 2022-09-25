use clap::Parser;

/// Split a file to multiply files
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// lines per output file
    #[clap(short, long, value_parser, default_value_t = 1000)]
    pub lines: usize,
    /// number of output files
    #[clap(short, long, value_parser, default_value_t = 1)]
    pub files: usize,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    pub path: std::path::PathBuf,
}