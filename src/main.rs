use clap::Parser;

mod models;
mod file_utils;

fn main() {
    let args = models::args::Args::parse();
    file_utils::splitter::split_file(args.path, args.lines, args.files);
}
