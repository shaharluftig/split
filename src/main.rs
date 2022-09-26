use clap::Parser;

use crate::models::args::Args;

mod models;
mod utils;
mod splitter;

fn main() {
    let args: Args = models::args::Args::parse();
    splitter::generic_splitter::split_file(&args.path, args.lines, args.ignore_empty_lines);
}
