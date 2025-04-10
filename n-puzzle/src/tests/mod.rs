#[cfg(test)]
mod tests {
    use crate::arena::Puzzle;

    #[test]
    fn capacity() {
        let puzzle = Puzzle::new(4);
        //assert_eq!(puzzle.puzzle.capacity(), 4);
        assert_eq!(puzzle.puzzle.len(), 4);
        println!("puzzle size: {}", puzzle.puzzle.capacity());
        for line in puzzle.puzzle {
            assert_eq!(line.capacity(), 4);
            println!("line size: {}", line.capacity());
        }
    }
}
