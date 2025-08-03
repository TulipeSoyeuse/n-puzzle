use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    name = "n-puzzle solver",
    about = "solve n-puzzle with the A* search algorithm",
    long_about = "try to solve puzzle of size N as fast as possible.\nThis program include multiple heuristics and some display option"
)]
pub struct Args {
    // size of the puzzle
    #[arg(short, long)]
    pub size: usize,

    // file holding the puzzle (alternatively if not provided, read it from stdin)
    #[arg(short, long, default_value_t = String::from("stdin"))]
    pub file: String,

    // heuristic flags
    #[arg(long, default_value_t = String::from("ed"))]
    pub heuristic: String,

    //debug flag -> display every state the square will go through
    #[arg(long)]
    pub display_mode: bool,

    #[arg(long, short, default_value_t = 1000)]
    pub display_step: usize,
}
