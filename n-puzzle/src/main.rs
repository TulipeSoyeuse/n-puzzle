mod cli;
mod error;
mod heuristics;
mod puzzle;
mod tests;
mod tree;

use crate::cli::Args;
use crate::error::AppError;
use clap::Parser;
use heuristics::EHeuristic;
use puzzle::{Puzzle, gen_solved_ref};
use std::fs::File;
use std::io::{BufReader, stdin};
use std::rc::Rc;
use tree::Arena;

fn match_heuristic(flag: String) -> Result<EHeuristic, ()> {
    match flag.as_str() {
        "hd" => Ok(EHeuristic::HammingDistance),
        "ed" => Ok(EHeuristic::EuclidienDistance),
        _ => Err(()),
    }
}

fn main() -> Result<(), AppError> {
    // read arg, init puzzle
    let args = Args::parse();
    let mut puzzle = Puzzle::new(args.size);

    // generate reference
    let psref = gen_solved_ref(args.size);
    let mut ref_ = Puzzle::new(args.size);
    ref_.init_from(&psref)?;

    // read and fill puzzle
    if args.file == "stdin" {
        let _ = puzzle.init(stdin().lock())?;
    } else if !args.file.is_empty() {
        let f = File::open(args.file)?;
        let _ = puzzle.init(BufReader::new(f))?;
    } else {
        return Err(AppError::new("no file provided"));
    }
    println!("{}", puzzle);

    // is the puzzle solvable ?
    if !puzzle.is_solvable() {
        return Err(AppError::new("Puzzle not solvable.."));
    }

    // Tree setup
    let heuristic = match_heuristic(args.heuristic).unwrap_or(EHeuristic::EuclidienDistance);
    let mut arena = Arena::new(heuristic, Rc::new(psref));
    println!("solving with heuristic: {:?}", heuristic);
    // solving with binary tree, using an arena system
    arena.init(puzzle);
    arena.solve_puzzle()?;
    arena.display_solution();
    Ok(())
}
