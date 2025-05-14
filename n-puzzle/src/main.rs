mod algorithm;
mod arena;
mod cli;
mod tests;
mod tree;

use std::io::{BufReader, stdin};

use crate::cli::Args;
use algorithm::heuristics::{self, EHeuristic};
use arena::{Mouvement, Puzzle, gen_solved_ref};
use clap::Parser;
use std::fs::File;
use tree::Tree;

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
    } else {
        let f = File::open(args.file)?;
        let _ = puzzle.init(BufReader::new(f))?;
    }

    // Tree setup
    let mut tree = Tree::new(puzzle, heuristics::EHeuristic::HammingDistance, &psref);
    tree.solve_puzzle();
    println!("{}", tree);

    Ok(())
}
