pub mod heuristics {
    use crate::Tree::{Node, Tree};
    use crate::arena::Puzzle;
    type PContainer = Vec<Vec<u16>>;

    fn hamming_distance(p: Puzzle, reference: PContainer) -> i32 {
        let mut counter = 0;

        for i in 0..p.dim {
            for j in 0..p.dim {
                if p.puzzle[i][j] == 0 {
                    continue;
                } else if p.puzzle[i][j] != reference[i][j] {
                    counter += 1;
                }
            }
        }

        counter
    }
}
