pub mod heuristics {
    use crate::arena::Puzzle;

    pub type PContainer = Vec<Vec<u16>>;

    #[derive(PartialEq, Eq, Clone)]
    pub enum EHeuristic {
        HammingDistance,
        EuclidienDistance,
    }

    pub fn hamming_distance(p: &Puzzle, reference: &PContainer) -> u32 {
        let mut counter = 0;

        for i in 0..p.dim {
            for j in 0..p.dim {
                if p.puzzle[i][j] != reference[i][j] {
                    counter += 1;
                }
            }
        }

        counter
    }

    pub fn euclidian_distance(p: &Puzzle, reference: &PContainer) -> u32 {
        let mut counter = 0;

        for i in 0..p.dim {
            for j in 0..p.dim {
                if p.puzzle[i][j] != reference[i][j] {
                    let reference_position = p.find(reference[i][j]);
                    counter += i.abs_diff(reference_position.x) as u32;
                    counter += j.abs_diff(reference_position.y) as u32;
                }
            }
        }
        counter
    }

    pub fn set_heuristics(heuristic: &EHeuristic, p: &Puzzle, reference: &PContainer) -> u32 {
        if heuristic == &EHeuristic::HammingDistance {
            hamming_distance(p, &reference)
        } else if heuristic == &EHeuristic::EuclidienDistance {
            euclidian_distance(p, &reference)
        } else {
            0
        }
    }
}
