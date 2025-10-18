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

fn match_heuristic(flag: String) -> Result<EHeuristic, AppError> {
    match flag.as_str() {
        "hd" => Ok(EHeuristic::HammingDistance),
        "md" => Ok(EHeuristic::ManhattanDistance),
        _ => Err(AppError::new("unkown heuristic")),
    }
}

fn main() -> Result<(), AppError> {
    // read arg, create puzzle
    let args = Args::parse();
    let mut puzzle = Puzzle::new(args.size);

    // generate reference
    let psref = gen_solved_ref(args.size);
    let mut ref_ = Puzzle::new(args.size);
    ref_.init_from(&psref)?;

    // read and fill puzzle
    if let Some(file) = args.file {
        if file.to_str() == Some("stdin") {
            let _ = puzzle.init(stdin().lock())?;
        } else {
            let f = File::open(file)?;
            let _ = puzzle.init(BufReader::new(f))?;
        }
    } else {
        let _ = puzzle.generate(args.iterations, args.solvable, &psref)?;
    }

    // is the puzzle solvable ?
    if !puzzle.is_solvable() {
        println!("puzzle not salvable");
        return Ok(());
    }

    // Tree setup
    let heuristic = match_heuristic(args.heuristic)?;
    let mut arena = Arena::new(heuristic, Rc::new(psref));
    println!("solving with heuristic: {:?}", heuristic);
    // solving with binary tree, using an arena system
    arena.init(puzzle);
    arena.solve_puzzle(args.debug)?;
    arena.display_solution();
    Ok(())
}
