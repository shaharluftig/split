use clap::Parser;

mod models;
mod utils;

fn main() {
    let args = models::args::Args::parse();
    utils::splitter::split_file(&args.path, args.lines, args.ignore_empty_lines);
}
