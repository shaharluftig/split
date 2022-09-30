use std::time::Instant;

use clap::Parser;

use crate::models::args::Args;

mod models;
mod splitter;
mod utils;
mod tests;

fn main() {
    let now: Instant = Instant::now();
    let args: Args = models::args::Args::parse();
    splitter::generic_splitter::split_file(&args.path, args.lines, args.ignore_empty_lines);
    println!("Finished, time took: {} milliseconds", now.elapsed().as_millis());
}
