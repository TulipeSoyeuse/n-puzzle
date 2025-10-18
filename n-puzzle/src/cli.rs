use std::path::PathBuf;

use clap::Parser;
use rand::prelude::*;
// args parser

#[derive(Parser)]
#[command(
    version = "1.0",
    name = "n-puzzle solver",
    about = "solve n-puzzle with the A* search algorithm",
    long_about = "try to solve puzzle of size N as fast as possible.\nThis program include multiple heuristics and some display option"
)]

pub struct Args {
    // size of the puzzle
    #[arg(long)]
    pub size: usize,

    // file holding the puzzle (alternatively if not provided, generated)
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    #[arg(long, conflicts_with = "file")]
    pub stdin: Option<bool>,

    // heuristic flags
    #[arg(long, default_value_t = String::from("md"))]
    pub heuristic: String,

    // ----  generation arguments ----

    // force solvability of the generated puzzle
    #[arg(short, long, default_value_t = rand::rng().random(), conflicts_with_all = ["file", "stdin"])]
    pub solvable: bool,

    // if generated, number of iterations
    #[arg(short, long, conflicts_with_all = ["file", "stdin"], default_value_t = rand::rng().random())]
    pub iterations: u16,

    //debug flag -> display every state the square will go through
    #[arg(long, short)]
    pub debug: bool,
}
