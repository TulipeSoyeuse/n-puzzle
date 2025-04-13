use regex::Regex;
use std::fmt::Display;
use std::io;

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub puzzle: Vec<Vec<u16>>,
    pub mouv_count: usize,
    pub dim: usize,
    pub empty_cell: Point,
    init: bool,
    solved: bool,
}

#[derive(Debug, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

pub fn gen_solved_ref(dim: usize) -> Vec<Vec<u16>> {
    let (mut top, mut bottom, mut left, mut right) = (0, dim - 1, 0, dim - 1);
    let mut num = 1;
    let mut puzzle_ref = vec![vec![0u16; dim]; dim];
    while top < bottom && left < right {
        // left to right
        for i in left..=right {
            puzzle_ref[top][i] = num;
            num += 1;
        }
        top += 1;

        // right to bottom
        for i in top..=bottom {
            puzzle_ref[i][right] = num;
            num += 1;
        }
        right = right.saturating_sub(1);

        //bottom to left
        for i in (left..=right).rev() {
            println!("i:{}", i);
            puzzle_ref[bottom][i] = num;
            num += 1;
        }
        bottom = bottom.saturating_sub(1);

        // bottom to top
        for i in (top..=bottom).rev() {
            puzzle_ref[i][left] = num;
            num += 1;
        }
        left += 1;
    }
    puzzle_ref
}

impl Puzzle {
    pub fn new(dimension: usize) -> Self {
        Puzzle {
            puzzle: vec![vec![0u16; dimension]; dimension],
            mouv_count: 0,
            dim: dimension,
            empty_cell: Point { x: 0, y: 0 },

            init: false,
            solved: false,
        }
    }

    pub fn init<T: io::BufRead>(&mut self, mut f: T) -> io::Result<()> {
        if self.init {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Puzzle already initialized",
            ));
        }
        let mut buf = String::new();
        let re = Regex::new(r"^\s*(\d{1,3}(?:\s+\d{1,3})+)\s*$").unwrap();
        let mut n: usize = 0;
        while f.read_line(&mut buf)? != 0 {
            if buf.starts_with("#") {
                ()
            } else if re.is_match(&buf) {
                for (c, v) in buf.trim().split_whitespace().enumerate() {
                    if n < self.dim && c < self.dim {
                        let v = v.to_string().parse().unwrap_or(0);
                        if v == 0 {
                            self.empty_cell = Point { x: n, y: c }
                        }
                        self.puzzle[n][c] = v;
                    }
                }
                n += 1;
            }
            buf.clear();
        }
        self.init = true;
        Ok(())
    }

    pub fn init_from(&mut self, v: &Vec<Vec<u16>>) -> io::Result<()> {
        self.puzzle = v.clone();
        Ok(())
    }

    pub fn is_solved(&mut self, reference: Vec<Vec<u16>>) -> bool {
        if self.solved == true {
            return true;
        } else {
            self.solved = reference == self.puzzle;
            self.solved
        }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.puzzle.iter() {
            writeln!(f, "{}", "--".repeat(self.dim as usize * 4 + 1))?;
            for element in line {
                write!(f, "│\t{} ", element)?;
            }
            write!(f, "│\n")?;
        }
        writeln!(f, "{}", "--".repeat(self.dim as usize * 4 + 1))?;
        writeln!(
            f,
            "mouvement count: {}\ndimension: {}",
            self.mouv_count, self.dim
        )?;
        writeln!(
            f,
            "empty cell {{ x:{}, y: {} }}",
            self.empty_cell.x, self.empty_cell.y
        )
    }
}
pub trait Mouvement {
    fn up(&mut self) -> Result<(), ()>;
    fn down(&mut self) -> Result<(), ()>;
    fn left(&mut self) -> Result<(), ()>;
    fn right(&mut self) -> Result<(), ()>;

    fn clone_up(&self) -> Result<Puzzle, ()>;
    fn clone_down(&self) -> Result<Puzzle, ()>;
    fn clone_left(&self) -> Result<Puzzle, ()>;
    fn clone_right(&self) -> Result<Puzzle, ()>;
}

impl Mouvement for Puzzle {
    fn up(&mut self) -> Result<(), ()> {
        if self.empty_cell.x == 0 {
            Err(())
        } else {
            self.solved = false;
            self.puzzle[self.empty_cell.x][self.empty_cell.y] =
                self.puzzle[self.empty_cell.x - 1][self.empty_cell.y];
            self.puzzle[self.empty_cell.x - 1][self.empty_cell.y] = 0;
            self.empty_cell.x -= 1;
            self.mouv_count += 1;
            Ok(())
        }
    }
    fn down(&mut self) -> Result<(), ()> {
        if self.empty_cell.x == self.dim - 1 {
            Err(())
        } else {
            self.solved = false;
            self.puzzle[self.empty_cell.x][self.empty_cell.y] =
                self.puzzle[self.empty_cell.x + 1][self.empty_cell.y];
            self.puzzle[self.empty_cell.x + 1][self.empty_cell.y] = 0;
            self.empty_cell.x += 1;
            self.mouv_count += 1;
            Ok(())
        }
    }
    fn left(&mut self) -> Result<(), ()> {
        if self.empty_cell.y == 0 {
            Err(())
        } else {
            self.solved = false;
            self.puzzle[self.empty_cell.x][self.empty_cell.y] =
                self.puzzle[self.empty_cell.x][self.empty_cell.y - 1];
            self.puzzle[self.empty_cell.x][self.empty_cell.y - 1] = 0;
            self.empty_cell.y -= 1;
            self.mouv_count += 1;
            Ok(())
        }
    }
    fn right(&mut self) -> Result<(), ()> {
        if self.empty_cell.y == self.dim - 1 {
            Err(())
        } else {
            self.solved = false;
            self.puzzle[self.empty_cell.x][self.empty_cell.y] =
                self.puzzle[self.empty_cell.x][self.empty_cell.y + 1];
            self.puzzle[self.empty_cell.x][self.empty_cell.y + 1] = 0;
            self.empty_cell.y += 1;
            self.mouv_count += 1;
            Ok(())
        }
    }

    fn clone_up(&self) -> Result<Puzzle, ()> {
        let mut clone = self.clone();
        clone.up()?;
        Ok(clone)
    }

    fn clone_down(&self) -> Result<Puzzle, ()> {
        let mut clone = self.clone();
        clone.down()?;
        Ok(clone)
    }
    fn clone_left(&self) -> Result<Puzzle, ()> {
        let mut clone = self.clone();
        clone.left()?;
        Ok(clone)
    }
    fn clone_right(&self) -> Result<Puzzle, ()> {
        let mut clone = self.clone();
        clone.right()?;
        Ok(clone)
    }
}

use std::cmp::PartialEq;

impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        if self.dim == other.dim && self.init == other.init {
            for x in 0..self.dim {
                for y in 0..self.dim {
                    if self.puzzle[x][y] != other.puzzle[x][y] {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}
