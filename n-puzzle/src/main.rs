mod Tree;
mod algorithm;
mod algorithm;
mod arena;
mod cli;
mod tests;

use std::io::{BufReader, stdin};

use crate::cli::Args;
use algorithm::heuristics::EHeuristic;
use arena::{Mouvement, Puzzle, gen_solved_ref};
use clap::Parser;
use std::fs::File;

fn match_heuristic(flag: String) -> Result<EHeuristic, ()> {
    match flag.as_str() {
        "hd" => Ok(EHeuristic::Hamming_Distance),
        _ => Err(()),
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut puzzle = Puzzle::new(args.size);
    let psref = gen_solved_ref(args.size);

    //test
    let mut reference = Puzzle::new(args.size);
    reference.init_from(&psref)?;
    println!("{}", reference);

    // read and fill puzzle
    if args.file == "stdin" {
        let _ = puzzle.init(stdin().lock())?;
    } else {
        let f = File::open(args.file)?;
        let _ = puzzle.init(BufReader::new(f))?;
    }
    Ok(())
}
