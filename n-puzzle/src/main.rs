mod arena;
mod cli;
mod tests;

use std::io::{BufReader, stdin};

use crate::cli::Args;
use arena::Puzzle;
use clap::Parser;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut puzzle = Puzzle::new(args.size);

    // read and fill puzzle
    if args.file == "stdin" {
        let _ = puzzle.init(stdin().lock())?;
    } else {
        let f = File::open(args.file)?;
        let _ = puzzle.init(BufReader::new(f))?;
    }
    println!("{}", puzzle);
    Ok(())
}
