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
    let pattern = std::env::args().nth(1).expect("missing pattern");
    let path = std::env::args().nth(2).expect("missing path");
}
