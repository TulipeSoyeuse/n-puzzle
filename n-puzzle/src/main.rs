mod algorithm;
mod arena;
mod cli;
mod tests;
mod tree;

use crate::cli::Args;
use algorithm::heuristics::{self, EHeuristic};
use arena::{Mouvement, Puzzle, gen_solved_ref};
use clap::{Error, Parser};
use std::fs::File;
use std::io::{BufReader, stdin};
use std::rc::Rc;
use tree::Arena;

fn match_heuristic(flag: String) -> Result<EHeuristic, ()> {
    match flag.as_str() {
        "hd" => Ok(EHeuristic::HammingDistance),
        _ => Err(()),
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut puzzle = Puzzle::new(args.size);
    let psref = gen_solved_ref(args.size);

    // read and fill puzzle
    if args.file == "stdin" {
        let _ = puzzle.init(stdin().lock())?;
    } else if !args.file.is_empty() {
        let f = File::open(args.file)?;
        let _ = puzzle.init(BufReader::new(f))?;
    } else {
        println!("No file provided");
        return Ok(());
    }

    // Tree setup
    let mut arena = Arena::new(heuristics::EHeuristic::HammingDistance, Rc::new(psref));
    arena.init(puzzle);
    let _res = arena.solve_puzzle();

    Ok(())
}
