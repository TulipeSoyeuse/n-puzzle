use std::path::PathBuf;

use clap::Parser;
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
    #[arg(long, help = "size of the puzzle")]
    pub size: usize,

    // file holding the puzzle (alternatively if not provided, generated)
    #[arg(short, long, help = "optional: provide a path to a valid puzzle")]
    pub file: Option<PathBuf>,

    #[arg(long, conflicts_with = "file", help = "provide a puzzle through stdin")]
    pub stdin: bool,

    // heuristic flags
    #[arg(long, default_value_t = String::from("md"), help = "\"md\" -> manhattan distance, \"hd\" -> hamming distance, \"lc\" -> linear conflict + md")]
    pub heuristic: String,

    // ----  generation arguments ----

    // force solvability of the generated puzzle
    #[arg(short, long, conflicts_with_all = ["file", "stdin", "unsolvable"], help = "force generated puzzle solvability")]
    pub solvable: bool,

    #[arg(short, long, conflicts_with_all = ["file", "stdin", "solvable"], help = "force generated puzzle unsolvability")]
    pub unsolvable: bool,

    // if generated, number of iterations
    #[arg(short, long, conflicts_with_all = ["file", "stdin"], default_value_t = 1000)]
    pub iterations: u16,

    //debug flag -> display every state the square will go through
    #[arg(long, short)]
    pub debug: bool,
}
