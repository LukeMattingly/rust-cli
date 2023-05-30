#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
struct Cli {
    ///The pattern to look for
    pattern: String,
    //the path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    print!("{}", args.pattern);
    print!("{:?}", args.path);
}
