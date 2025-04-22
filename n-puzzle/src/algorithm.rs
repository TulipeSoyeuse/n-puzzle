pub mod heuristics {
    use crate::arena::Puzzle;

    pub type PContainer = Vec<Vec<u16>>;

    #[derive(PartialEq, Eq, Clone)]
    pub enum EHeuristic {
        HammingDistance,
    }

    pub fn hamming_distance(p: &Puzzle, reference: &PContainer) -> u32 {
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

    pub fn set_heuristics(heuristic: &EHeuristic, p: &Puzzle, reference: &PContainer) -> u32 {
        if *heuristic == EHeuristic::HammingDistance {
            hamming_distance(p, &reference)
        } else {
            0
        }
    }
}
