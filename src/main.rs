use clap::Parser;

use crate::models::args::Args;

mod models;
mod utils;

fn main() {
    let args: Args = models::args::Args::parse();
    utils::splitter::split_file(&args.path, args.lines, args.ignore_empty_lines);
}
