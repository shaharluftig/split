mod models;

use clap::Parser;

fn main() {
    let args = models::args::Args::parse();
    println!("{:?}", args.path)
}
