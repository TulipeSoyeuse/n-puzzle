//! heuristcs
//!
//! emun type and logic for calculating the "distance to solved" of a Puzzle object with direrents approch a.k.a Heuristics

use crate::puzzle::Puzzle;

pub type PContainer = Vec<Vec<u16>>;

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum EHeuristic {
    /// this heuristics returns the number of tiles that are not in their final position.
    HammingDistance,

    /// Manhattan distance of a tile is the distance or the number of slides/tiles away it is from it’s goal state.
    /// Thus, for a certain state the Manhattan distance will be the sum of the Manhattan distances of all the tiles except the blank tile.
    ManhattanDistance,
}

fn hamming_distance(p: &Puzzle, reference: &PContainer) -> usize {
    let mut counter = 0;

    for i in 0..p.dim {
        for j in 0..p.dim {
            let value = p.puzzle[i][j];
            if value != 0 && value != reference[i][j] {
                counter += 1;
            }
        }
    }
    counter
}

fn manhattan_distance(p: &Puzzle, reference: &PContainer) -> usize {
    let mut counter = 0;
    let mut reference_p = Puzzle::new(p.dim);
    let _ = reference_p.init_from(reference);

    for i in 0..p.dim {
        for j in 0..p.dim {
            let value = p.puzzle[i][j];
            if value != 0 {
                // find where this tile should be in the reference (goal)
                let goal_pos = reference_p.find(value);
                counter += i.abs_diff(goal_pos.x);
                counter += j.abs_diff(goal_pos.y);
            }
        }
    }
    counter
}

pub fn set_heuristics(heuristic: &EHeuristic, p: &Puzzle, reference: &PContainer) -> usize {
    if heuristic == &EHeuristic::HammingDistance {
        hamming_distance(p, &reference)
    } else if heuristic == &EHeuristic::ManhattanDistance {
        manhattan_distance(p, &reference)
    } else {
        0
    }
}
