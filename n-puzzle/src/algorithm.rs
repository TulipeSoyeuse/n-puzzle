use crate::Tree::{Node, Tree};
use crate::arena::Puzzle;

pub mod heuristics {
    enum Heuristic {
        AStarAlgorithm(&mut Node),
        IDA_Algorithm(&mut Node)
    }
    set_heuristic();
}
