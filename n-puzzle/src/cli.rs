use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // size of the puzzle
    #[arg(short, long)]
    pub size: usize,

    // file holding the puzzle (alternatively if not provided, read it from stdin)
    #[arg(short, long, default_value_t = String::from("stdin"))]
    pub file: String,

    //debug flag -> display every state the square will go through
    #[arg(long)]
    pub display_mode: bool,
}
