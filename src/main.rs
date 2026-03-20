mod cli;
mod error;
mod heuristics;
mod puzzle;
mod tests;
mod tree;

use crate::cli::Args;
use crate::error::AppError;
use crate::heuristics::gen_solved_ref_hashmap;
use clap::Parser;
use heuristics::EHeuristic;
use puzzle::Puzzle;
use rand::prelude::*;
use std::fs::File;
use std::io::{BufReader, stdin};
use tree::Arena;

fn match_heuristic(flag: String, dim: usize) -> Result<EHeuristic, AppError> {
    let reference = gen_solved_ref_hashmap(dim);
    match flag.as_str() {
        "bf" => Ok(EHeuristic::BruteForce {}),
        "hd" => Ok(EHeuristic::HammingDistance { reference }),
        "md" => Ok(EHeuristic::ManhattanDistance { reference }),
        "lc" => Ok(EHeuristic::LinearConflict { reference }),
        _ => Err(AppError::new("unkown heuristic")),
    }
}

fn main() -> Result<(), AppError> {
    // read arg, create puzzle
    let args = Args::parse();
    let mut puzzle = Puzzle::new(args.size);

    let solvable_flag = if args.solvable {
        true
    } else if args.unsolvable {
        false
    } else {
        rand::rng().random()
    };

    // read and fill puzzle
    if let Some(file) = args.file {
        if file.to_str() == Some("stdin") {
            let _ = puzzle.init(stdin().lock())?;
        } else {
            let f = File::open(file)?;
            let _ = puzzle.init(BufReader::new(f))?;
        }
    } else {
        let _ = puzzle.generate(args.iterations, solvable_flag)?;
    }

    // is the puzzle solvable ?
    if !puzzle.is_solvable() {
        println!("puzzle not solvable");
        return Ok(());
    }

    // Tree setup
    let heuristic = match_heuristic(args.heuristic, args.size)?;
    let mut arena = Arena::new(heuristic);
    // solving with binary tree, using an arena system
    arena.init(puzzle);
    arena.solve_puzzle(args.debug)?;
    arena.display_solution();
    Ok(())
}
