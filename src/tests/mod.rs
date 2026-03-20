#[cfg(test)]
mod puzzle_test {
    use crate::heuristics::{EHeuristic, gen_solved_ref_hashmap};
    use crate::puzzle::{Mouvement, Point, Puzzle, gen_solved_ref};
    use crate::tree::Arena;
    use std::fs::File;
    use std::io::BufReader;
    use std::iter::zip;

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
            assert!(puzzle.is_solved(&reference))
        }
    }

    #[test]
    fn tree_setup() {
        let reference = gen_solved_ref_hashmap(3);
        let f = File::open("src/tests/test_dim3.puzzle").unwrap();
        let mut puzzle = Puzzle::new(3);
        let _ = puzzle.init(BufReader::new(f));

        let mut arena = Arena::new(EHeuristic::HammingDistance { reference });
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
        let f = File::open("src/tests/test_dim3_7mouv.puzzle").unwrap();
        let mut puzzle = Puzzle::new(3);
        let _ = puzzle.init(BufReader::new(f));

        let iter_ref = [8, 1, 3, 5, 6, 7, 2, 4];
        let puzzle_iter = puzzle.into_iter();

        for (a, b) in zip(iter_ref, puzzle_iter) {
            println!("{} : {}", a, b);
            assert_eq!(a, b);
        }

        // dim 4 test
        let f = File::open("src/tests/test_inversion_dim4.puzzle").unwrap();
        let mut puzzle = Puzzle::new(4);
        let _ = puzzle.init(BufReader::new(f));

        let iter_ref = [12, 5, 7, 2, 4, 10, 13, 15, 3, 11, 6, 1, 14, 9, 8];
        let puzzle_iter = puzzle.into_iter();

        for (a, b) in zip(iter_ref, puzzle_iter) {
            println!("{} : {}", a, b);
            assert_eq!(a, b);
        }

        // dim 5 test
        let f = File::open("src/tests/test_inversion_dim5.puzzle").unwrap();
        let mut puzzle = Puzzle::new(5);
        let _ = puzzle.init(BufReader::new(f));

        let iter_ref = [
            18, 11, 22, 15, 21, 20, 6, 4, 17, 24, 16, 14, 13, 12, 8, 5, 7, 3, 9, 1, 19, 10, 2, 23,
        ];
        let puzzle_iter = puzzle.into_iter();

        for (a, b) in zip(iter_ref, puzzle_iter) {
            println!("{} : {}", a, b);
            assert_eq!(a, b);
        }
    }
}

#[cfg(test)]
mod solving_test {
    use crate::heuristics::{EHeuristic, gen_solved_ref_hashmap};
    use crate::puzzle::Puzzle;
    use crate::tree::Arena;
    use crate::tree::Node;
    use std::fs::File;
    use std::io::BufReader;

    fn solve_puzzle(f: File, dim: usize) -> Option<Node> {
        let mut puzzle = Puzzle::new(dim);
        let _ = puzzle.init(BufReader::new(f)).unwrap();
        let mut arena = Arena::new(EHeuristic::ManhattanDistance {
            reference: gen_solved_ref_hashmap(dim),
        });
        arena.init(puzzle);
        arena.solve_puzzle(true).unwrap();

        arena.solved_node.map(|v| arena.nodes.remove(v))
    }

    #[test]
    fn solvable_dim3_1mouv() {
        let f = File::open("src/tests/test_dim3_1mouv.puzzle").unwrap();
        if let Some(v) = solve_puzzle(f, 3) {
            assert_eq!(v.state.mouv_count, 1);
        }
    }

    #[test]
    fn solvable_dim3_2mouv() {
        let f = File::open("src/tests/test_dim3_2mouv.puzzle").unwrap();
        if let Some(v) = solve_puzzle(f, 3) {
            assert_eq!(v.state.mouv_count, 2);
        }
    }

    #[test]
    fn solvable_dim3_5mouv() {
        let f = File::open("src/tests/test_dim3_5mouv.puzzle").unwrap();
        if let Some(v) = solve_puzzle(f, 3) {
            assert_eq!(v.state.mouv_count, 5);
        }
    }

    #[test]
    fn solvable_dim3_7mouv() {
        let f = File::open("src/tests/test_dim3_7mouv.puzzle").unwrap();
        if let Some(v) = solve_puzzle(f, 3) {
            assert_eq!(v.state.mouv_count, 7);
        }
    }
}

#[cfg(test)]
mod solvability {
    use crate::puzzle::Puzzle;
    use std::env;
    use std::io::BufReader;
    use std::process::{Command, Stdio};

    #[test]
    fn test_solvable_case_dim3() {
        for _ in 0..50 {
            // command setup and capture output ---------------------------------
            let dim = 3;
            let dimstr = dim.to_string();
            println!("{:?}", env::current_dir().unwrap());
            let mut child = Command::new("python")
                .args(["npuzzle-gen.py", "-s", &dimstr])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let output = child.stdout.take().unwrap();
            let reader = BufReader::new(output);

            // puzzle init and check solvability ---------------------------------
            let mut puzzle = Puzzle::new(dim);
            let _ = puzzle.init(reader);
            println!("{}", puzzle);
            assert!(puzzle.is_solvable());
        }
    }

    #[test]
    fn test_solvable_case_dim4() {
        for _ in 0..50 {
            // command setup and capture output ---------------------------------
            let dim = 4;
            let dimstr = dim.to_string();
            let mut child = Command::new("python")
                .args(["npuzzle-gen.py", "-s", &dimstr])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let output = child.stdout.take().unwrap();
            let reader = BufReader::new(output);

            // puzzle init and check solvability ---------------------------------
            let mut puzzle = Puzzle::new(dim);
            let _ = puzzle.init(reader);
            println!("{}", puzzle);
            assert!(puzzle.is_solvable());
        }
    }

    #[test]
    fn test_solvable_case_dim5() {
        for _ in 0..50 {
            // command setup and capture output ---------------------------------
            let dim = 5;
            let dimstr = dim.to_string();
            let mut child = Command::new("python")
                .args(["npuzzle-gen.py", "-s", &dimstr])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let output = child.stdout.take().unwrap();
            let reader = BufReader::new(output);

            // puzzle init and check solvability ---------------------------------
            let mut puzzle = Puzzle::new(dim);
            let _ = puzzle.init(reader);
            println!("{}", puzzle);
            assert!(puzzle.is_solvable());
        }
    }

    #[test]
    fn test_solvable_case_dim6() {
        for _ in 0..50 {
            // command setup and capture output ---------------------------------
            let dim = 6;
            let dimstr = dim.to_string();
            let mut child = Command::new("python")
                .args(["npuzzle-gen.py", "-s", &dimstr])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let output = child.stdout.take().unwrap();
            let reader = BufReader::new(output);

            // puzzle init and check solvability ---------------------------------
            let mut puzzle = Puzzle::new(dim);
            let _ = puzzle.init(reader);
            println!("{}", puzzle);
            assert!(puzzle.is_solvable());
        }
    }

    #[test]
    fn test_unsolvable_case_dim3() {
        for _ in 0..50 {
            // command setup and capture output ---------------------------------
            let dim = 3;
            let dimstr = dim.to_string();
            let mut child = Command::new("python")
                .args(["npuzzle-gen.py", "-u", &dimstr])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let output = child.stdout.take().unwrap();
            let reader = BufReader::new(output);

            // puzzle init and check solvability ---------------------------------
            let mut puzzle = Puzzle::new(dim);
            let _ = puzzle.init(reader);
            println!("{}", puzzle);
            assert!(!puzzle.is_solvable());
        }
    }

    #[test]
    fn test_unsolvable_case_dim4() {
        for _ in 0..50 {
            // command setup and capture output ---------------------------------
            let dim = 4;
            let dimstr = dim.to_string();
            let mut child = Command::new("python")
                .args(["npuzzle-gen.py", "-u", &dimstr])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let output = child.stdout.take().unwrap();
            let reader = BufReader::new(output);

            // puzzle init and check solvability ---------------------------------
            let mut puzzle = Puzzle::new(dim);
            let _ = puzzle.init(reader);
            println!("{}", puzzle);
            assert!(!puzzle.is_solvable());
        }
    }

    #[test]
    fn test_unsolvable_case_dim5() {
        for _ in 0..50 {
            // command setup and capture output ---------------------------------
            let dim = 5;
            let dimstr = dim.to_string();
            let mut child = Command::new("python")
                .args(["npuzzle-gen.py", "-u", &dimstr])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let output = child.stdout.take().unwrap();
            let reader = BufReader::new(output);

            // puzzle init and check solvability ---------------------------------
            let mut puzzle = Puzzle::new(dim);
            let _ = puzzle.init(reader);
            println!("{}", puzzle);
            assert!(!puzzle.is_solvable());
        }
    }

    #[test]
    fn test_unsolvable_case_dim6() {
        for _ in 0..50 {
            // command setup and capture output ---------------------------------
            let dim = 6;
            let dimstr = dim.to_string();
            let mut child = Command::new("python")
                .args(["npuzzle-gen.py", "-u", &dimstr])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let output = child.stdout.take().unwrap();
            let reader = BufReader::new(output);

            // puzzle init and check solvability ---------------------------------
            let mut puzzle = Puzzle::new(dim);
            let _ = puzzle.init(reader);
            println!("{}", puzzle);
            assert!(!puzzle.is_solvable());
        }
    }
}

#[cfg(test)]
mod generate_test {
    use crate::puzzle::{Puzzle, gen_solved_ref};

    #[test]
    fn base_generation_dim3_solvable() {
        let mut puzzle = Puzzle::new(3);
        match puzzle.generate(0, true) {
            Ok(()) => {
                assert!(puzzle.is_solvable());
                assert!(puzzle.is_solved(&gen_solved_ref(3)));
            }
            Err(_) => panic!(),
        }
    }

    #[test]
    fn base_generation_dim3_unsolvable() {
        let mut puzzle = Puzzle::new(3);
        match puzzle.generate(0, false) {
            Ok(()) => {
                assert!(!puzzle.is_solvable());
            }
            Err(_) => panic!(),
        }
    }
}
