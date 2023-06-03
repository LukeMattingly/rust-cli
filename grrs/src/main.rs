#![allow(unused)]

use clap::Parser;
use log::{info, warn};
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
    env_logger::init();
    info!("starting up");

    let args: Cli = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    //let content = std::fs::read_to_string(&args.path)
    //.with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
