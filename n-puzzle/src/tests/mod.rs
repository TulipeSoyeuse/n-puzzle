//! test module
//!
//! What we should have done in Transcendance
#[cfg(test)]
mod puzzle_test {
    use crate::heuristics::EHeuristic;
    use crate::puzzle::{Mouvement, Point, Puzzle, gen_solved_ref};
    use crate::tree::Arena;
    use std::fs::File;
    use std::io::BufReader;
    use std::iter::zip;
    use std::rc::Rc;

    #[test]
    fn capacity() {
        let puzzle = Puzzle::new(4);
        //assert_eq!(puzzle.puzzle.capacity(), 4);
        assert_eq!(puzzle.puzzle.len(), 4);
        println!("puzzle size: {}", puzzle.puzzle.capacity());
        for line in puzzle.puzzle {
            assert_eq!(line.len(), 4);
            println!("line size: {}", line.capacity());
        }
    }

    #[test]
    fn dim_4_init() {
        let f = File::open("src/tests/test_dim4.puzzle").unwrap();
        let mut puzzle = Puzzle::new(4);
        let _ = puzzle.init(BufReader::new(f));
        assert_eq!(puzzle[0][0], 4);
        assert_eq!(puzzle[0][1], 9);
        assert_eq!(puzzle[0][2], 7);
        println!("{:?}", puzzle.empty_cell);
        assert!(puzzle.empty_cell == Point { x: 0, y: 3 });

        assert_eq!(puzzle[1][0], 3);
        assert_eq!(puzzle[1][1], 13);
        assert_eq!(puzzle[1][2], 1);
        assert_eq!(puzzle[1][3], 12);

        assert_eq!(puzzle[2][0], 15);
        assert_eq!(puzzle[2][1], 8);
        assert_eq!(puzzle[2][2], 2);
        assert_eq!(puzzle[2][3], 11);

        assert_eq!(puzzle[3][0], 5);
        assert_eq!(puzzle[3][1], 14);
        assert_eq!(puzzle[3][2], 10);
        assert_eq!(puzzle[3][3], 6);
    }

    #[test]
    fn mouvement_up() {
        // setup reference
        let f = File::open("src/tests/test_dim3_up.puzzle").unwrap();
        let mut p_ref = Puzzle::new(3);
        let _ = p_ref.init(BufReader::new(f));

        // setup 'other' and play mouvement
        let f = File::open("src/tests/test_dim3.puzzle").unwrap();
        let mut puzzle = Puzzle::new(3);
        let _ = puzzle.init(BufReader::new(f));
        assert!(puzzle.up() == Ok(()));

        assert_eq!(p_ref, puzzle);

        // impossible mouv test
        assert!(puzzle.up() == Err(()));
    }

    #[test]
    fn mouvement_down() {
        // setup reference
        let f = File::open("src/tests/test_dim3_down.puzzle").unwrap();
        let mut p_ref = Puzzle::new(3);
        let _ = p_ref.init(BufReader::new(f));

        // setup 'other' and play mouvement
        let f = File::open("src/tests/test_dim3.puzzle").unwrap();
        let mut puzzle = Puzzle::new(3);
        let _ = puzzle.init(BufReader::new(f));
        assert!(puzzle.down() == Ok(()));

        assert_eq!(p_ref, puzzle);

        // impossible mouv test
        assert!(puzzle.down() == Err(()));
    }

    #[test]
    fn mouvement_left() {
        // setup reference
        let f = File::open("src/tests/test_dim3_left.puzzle").unwrap();
        let mut p_ref = Puzzle::new(3);
        let _ = p_ref.init(BufReader::new(f));

        // setup 'other' and play mouvement
        let f = File::open("src/tests/test_dim3.puzzle").unwrap();
        let mut puzzle = Puzzle::new(3);
        let _ = puzzle.init(BufReader::new(f));
        assert!(puzzle.left() == Ok(()));

        assert_eq!(p_ref, puzzle);

        // impossible mouv test
        assert!(puzzle.left() == Err(()));
    }

    #[test]
    fn mouvement_right() {
        // setup reference
        let f = File::open("src/tests/test_dim3_right.puzzle").unwrap();
        let mut p_ref = Puzzle::new(3);
        let _ = p_ref.init(BufReader::new(f));

        // setup 'other' and play mouvement
        let f = File::open("src/tests/test_dim3.puzzle").unwrap();
        let mut puzzle = Puzzle::new(3);
        let _ = puzzle.init(BufReader::new(f));
        assert!(puzzle.right() == Ok(()));

        assert_eq!(p_ref, puzzle);

        // impossible mouv test
        assert!(puzzle.right() == Err(()));
    }

    #[test]
    fn is_solved_test() {
        for dim in 3..8 {
            let reference = gen_solved_ref(dim);
            let mut puzzle = Puzzle::new(dim);
            puzzle.init_from(&reference).unwrap();
            assert!(puzzle.is_solved(reference))
        }
    }

    #[test]
    fn tree_setup() {
        let reference = gen_solved_ref(3);
        let f = File::open("src/tests/test_dim3.puzzle").unwrap();
        let mut puzzle = Puzzle::new(3);
        let _ = puzzle.init(BufReader::new(f));

        let mut arena = Arena::new(EHeuristic::HammingDistance, Rc::new(reference));
        arena.init(puzzle);
        arena.generate_children(0);
        assert_eq!(arena.len(), 5);
    }

    #[test]
    fn equivalence_test() {
        // puzzle 1
        let f = File::open("src/tests/test_dim3.puzzle").unwrap();
        let mut puzzle1 = Puzzle::new(3);
        let _ = puzzle1.init(BufReader::new(f));

        // puzzle 2
        let f = File::open("src/tests/test_dim3.puzzle").unwrap();
        let mut puzzle2 = Puzzle::new(3);
        let _ = puzzle2.init(BufReader::new(f));
        let _ = puzzle2.up();
        let _ = puzzle2.down();

        assert_eq!(puzzle1, puzzle2);
    }

    #[test]
    fn iter_test() {
        let f = File::open("src/tests/test_dim3_5mouv.puzzle").unwrap();
        let mut puzzle = Puzzle::new(3);
        let _ = puzzle.init(BufReader::new(f));

        let iter_ref: [u16; 8] = [1, 2, 3, 4, 5, 7, 8, 6];
        let puzzle_iter = puzzle.into_iter();

        for (a, b) in zip(iter_ref, puzzle_iter) {
            println!("{} : {}", a, b);
            assert_eq!(a, b);
        }
    }
}

#[cfg(test)]
mod solving_test {
    use crate::heuristics::EHeuristic;
    use crate::puzzle::{Puzzle, gen_solved_ref};
    use crate::tree::Arena;
    use crate::tree::Node;
    use std::fs::File;
    use std::io::BufReader;
    use std::rc::Rc;

    fn solve_puzzle(f: File, dim: usize, step: usize) -> Option<Node> {
        let mut puzzle = Puzzle::new(dim);
        let _ = puzzle.init(BufReader::new(f)).unwrap();
        let psref = gen_solved_ref(dim);
        let mut arena = Arena::new(EHeuristic::EuclidienDistance, Rc::new(psref));
        arena.init(puzzle);
        arena.solve_puzzle(step);

        arena.solved_node.map(|v| arena.nodes.remove(v))
    }

    #[test]
    fn solvable_dim3_1mouv() {
        let f = File::open("src/tests/test_dim3_1mouv.puzzle").unwrap();
        if let Some(v) = solve_puzzle(f, 3, 1) {
            assert_eq!(v.state.mouv_count, 1);
        }
    }

    #[test]
    fn solvable_dim3_2mouv() {
        let f = File::open("src/tests/test_dim3_2mouv.puzzle").unwrap();
        if let Some(v) = solve_puzzle(f, 3, 1) {
            assert_eq!(v.state.mouv_count, 2);
        }
    }

    #[test]
    fn solvable_dim3_5mouv() {
        let f = File::open("src/tests/test_dim3_5mouv.puzzle").unwrap();
        if let Some(v) = solve_puzzle(f, 3, 1) {
            assert_eq!(v.state.mouv_count, 5);
        }
    }

    #[test]
    fn solvable_dim3_7mouv() {
        let f = File::open("src/tests/test_dim3_7mouv.puzzle").unwrap();
        if let Some(v) = solve_puzzle(f, 3, 1) {
            assert_eq!(v.state.mouv_count, 7);
        }
    }
}
