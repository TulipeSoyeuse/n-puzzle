#[cfg(test)]
mod tests {
    use crate::algorithm::heuristics::EHeuristic;
    use crate::arena::{Mouvement, Puzzle, gen_solved_ref};
    use crate::tree::Arena;
    use std::fs::File;
    use std::io::BufReader;
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
        let reference = gen_solved_ref(3);
        let mut puzzle = Puzzle::new(3);
        puzzle.init_from(&reference).unwrap();
        assert!(puzzle.is_solved(reference))
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
}
