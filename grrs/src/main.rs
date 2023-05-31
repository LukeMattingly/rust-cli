#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Parser)]
struct Cli {
    ///The pattern to look for
    pattern: String,
    //the path to the file to read
    path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path).expect("could not open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
