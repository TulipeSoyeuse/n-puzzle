//! heuristcs
//!
//! emun type and logic for calculating the "distance to solved" of a Puzzle object with direrents approch a.k.a Heuristics

use std::collections::HashMap;

use crate::puzzle::{Point, Puzzle};

pub type PContainer = Vec<Vec<u16>>;
pub type MapReference = HashMap<u16, Point>;

#[derive(Clone, Debug)]
pub enum EHeuristic {
    /// this heuristics returns the number of tiles that are not in their final position.
    HammingDistance {
        reference: MapReference,
    },

    /// Manhattan distance of a tile is the distance or the number of slides/tiles away it is from it’s goal state.
    /// Thus, for a certain state the Manhattan distance will be the sum of the Manhattan distances of all the tiles except the blank tile.
    ManhattanDistance {
        reference: MapReference,
    },

    /// Manhattan distance enhanced by linear conflict, check row wise and column wise if two tiles are inverted, if so, 2 extra moves minimun are required
    /// to solve
    LinearConflict {
        reference: MapReference,
    },

    BruteForce {},
}

impl EHeuristic {
    pub fn execute(&self, p: &Puzzle) -> usize {
        match &self {
            Self::HammingDistance { reference } => hamming_distance(p, reference),
            Self::ManhattanDistance { reference } => manhattan_distance(p, reference),
            Self::LinearConflict { reference } => {
                linear_conflict(p, reference) + manhattan_distance(p, reference)
            }
            Self::BruteForce {} => 0,
        }
    }
}

/// generate a reference puzzle
pub fn gen_solved_ref_hashmap(dim: usize) -> HashMap<u16, Point> {
    let (mut top, mut bottom, mut left, mut right) = (0, dim - 1, 0, dim - 1);
    let mut num: u16 = 1;
    let mut puzzle_ref = HashMap::new();
    let max = (dim * dim - 1) as u16;

    while top < bottom && left < right {
        // left to right
        for i in left..=right {
            puzzle_ref.insert(num, Point { x: top, y: i });
            if num < max {
                num += 1;
            } else {
                return puzzle_ref;
            }
        }
        top += 1;

        // right to bottom
        for i in top..=bottom {
            puzzle_ref.insert(num, Point { x: i, y: right });
            if num < max {
                num += 1;
            } else {
                return puzzle_ref;
            }
        }
        right = right.saturating_sub(1);

        //bottom to left
        for i in (left..=right).rev() {
            puzzle_ref.insert(num, Point { x: bottom, y: i });
            if num < max {
                num += 1;
            } else {
                return puzzle_ref;
            }
        }
        bottom = bottom.saturating_sub(1);

        // bottom to top
        for i in (top..=bottom).rev() {
            puzzle_ref.insert(num, Point { x: i, y: left });
            if num < max {
                num += 1;
            } else {
                return puzzle_ref;
            }
        }
        left += 1;
    }
    puzzle_ref
}

fn linear_conflict(puzzle: &Puzzle, reference: &MapReference) -> usize {
    let dim = puzzle.dim;
    let mut conflicts = 0;

    // --- Row conflicts ---
    for row in 0..dim {
        let mut row_tiles = Vec::new();
        for col in 0..dim {
            let tile = puzzle.puzzle[row][col];
            if tile != 0 {
                let point = reference[&tile];
                if point.x == row {
                    row_tiles.push((col, point.y));
                }
            }
        }
        for i in 0..row_tiles.len() {
            for j in i + 1..row_tiles.len() {
                let conflict = if row % 2 == 0 {
                    row_tiles[i].1 > row_tiles[j].1 // left → right
                } else {
                    row_tiles[i].1 < row_tiles[j].1 // right → left
                };
                if conflict {
                    #[cfg(test)]
                    println!(
                        "find a linear conflict between ({}, {}) and ({}, {}) row wise",
                        row_tiles[i].0, row_tiles[i].1, row_tiles[j].0, row_tiles[j].1
                    );
                    conflicts += 1;
                }
            }
        }
    }

    // --- Column conflicts ---
    for col in 0..dim {
        let mut col_tiles = Vec::new();
        for row in 0..dim {
            let tile = puzzle.puzzle[row][col];
            if tile != 0 {
                let point = reference[&tile];
                if point.y == col {
                    col_tiles.push((row, point.x));
                }
            }
        }
        for i in 0..col_tiles.len() {
            for j in i + 1..col_tiles.len() {
                let conflict = if col % 2 == 0 {
                    col_tiles[i].1 > col_tiles[j].1 // top → bottom
                } else {
                    col_tiles[i].1 < col_tiles[j].1 // bottom → top
                };
                if conflict {
                    #[cfg(test)]
                    println!(
                        "find a linear conflict between {} and {} col wise",
                        col_tiles[i].1, col_tiles[j].1
                    );
                    conflicts += 1;
                }
            }
        }
    }
    conflicts * 2
}

fn hamming_distance(p: &Puzzle, reference: &MapReference) -> usize {
    let mut counter = 0;

    for i in 0..p.dim {
        for j in 0..p.dim {
            let value = p.puzzle[i][j];
            let position = Point { x: i, y: j };
            if value != 0 && position != reference[&value] {
                counter += 1;
            }
        }
    }
    counter
}

fn manhattan_distance(p: &Puzzle, reference: &MapReference) -> usize {
    let mut counter = 0;

    for i in 0..p.dim {
        for j in 0..p.dim {
            let value = p.puzzle[i][j];
            if value != 0 {
                // find where this tile should be in the reference (goal)
                let goal_pos = reference[&value];
                let mut local_counter = 0;
                local_counter += i.abs_diff(goal_pos.x);
                local_counter += j.abs_diff(goal_pos.y);

                #[cfg(test)]
                println!(
                    "For {} with reference {}, manhattan distance of {}",
                    Point { x: i, y: j },
                    goal_pos,
                    local_counter
                );

                counter += local_counter;
            }
        }
    }
    counter
}

#[cfg(test)]
mod heuristics {
    use std::{fs::File, io::BufReader};

    use crate::{
        heuristics::{
            gen_solved_ref_hashmap, hamming_distance, linear_conflict, manhattan_distance,
        },
        puzzle::Puzzle,
    };

    #[test]
    fn test_1() {
        let f = File::open("src/tests/test_dim4_heuristics.puzzle").unwrap();
        let reference = gen_solved_ref_hashmap(4);
        let mut p = Puzzle::new(4);
        p.init(BufReader::new(f)).unwrap();
        println!("{}", p);

        assert_eq!(hamming_distance(&p, &reference), 15);
        assert_eq!(manhattan_distance(&p, &reference), 35);
    }

    #[test]
    fn test_2() {
        let f = File::open("src/tests/test_dim3_linear_conflict.puzzle").unwrap();
        let reference = gen_solved_ref_hashmap(3);
        let mut p = Puzzle::new(3);
        p.init(BufReader::new(f)).unwrap();
        println!("{}", p);

        assert_eq!(linear_conflict(&p, &reference), 2);
    }
}
