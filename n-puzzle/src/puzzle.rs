//! Arena
//!
//! regroup structure and logic for the puzzle itself, including mouvement and helpers

use crate::error::AppError;
use crate::heuristics::PContainer;
use colored::*;
use rand::prelude::*;
use regex::Regex;
use std::fmt::Debug;
use std::fmt::Display;
use std::io;
#[derive(Debug, Clone)]
pub struct Puzzle {
    pub puzzle: PContainer,
    pub mouv_count: usize,
    pub dim: usize,
    pub empty_cell: Point,
    init: bool,
    solved: bool,
}

pub struct PuzzleIntoIter {
    puzzle: Puzzle,
    reference: PContainer,
    index: u16,
}

impl PuzzleIntoIter {
    fn get_coordonates(&self, index: u16) -> Point {
        let mut reference = Puzzle::new(self.puzzle.dim);
        let _ = reference.init_from(&self.reference);

        if index as usize == self.puzzle.dim * self.puzzle.dim {
            reference.find(0)
        } else {
            reference.find(index.into())
        }
    }
}

impl Iterator for PuzzleIntoIter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let dim = self.puzzle.dim;
        if (self.index as usize) > dim * dim {
            None
        } else {
            let p = self.get_coordonates(self.index);
            self.index += 1;
            if self.puzzle.puzzle[p.x][p.y] == 0 {
                self.next()
            } else {
                Some(self.puzzle.puzzle[p.x][p.y])
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// generate a reference puzzle
pub fn gen_solved_ref(dim: usize) -> PContainer {
    let (mut top, mut bottom, mut left, mut right) = (0, dim - 1, 0, dim - 1);
    let mut num = 1;
    let mut puzzle_ref = vec![vec![0u16; dim]; dim];
    let max = (dim * dim - 1) as u16;

    while top < bottom && left < right {
        // left to right
        for i in left..=right {
            puzzle_ref[top][i] = num;
            if num < max {
                num += 1;
            } else {
                return puzzle_ref;
            }
        }
        top += 1;

        // right to bottom
        for i in top..=bottom {
            puzzle_ref[i][right] = num;
            if num < max {
                num += 1;
            } else {
                return puzzle_ref;
            }
        }
        right = right.saturating_sub(1);

        //bottom to left
        for i in (left..=right).rev() {
            puzzle_ref[bottom][i] = num;
            if num < max {
                num += 1;
            } else {
                return puzzle_ref;
            }
        }
        bottom = bottom.saturating_sub(1);

        // bottom to top
        for i in (top..=bottom).rev() {
            puzzle_ref[i][left] = num;
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

//Todo: implement copy trait
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

    pub fn generate(
        &mut self,
        mut iterations: u16,
        solvable: bool,
        reference: &PContainer,
    ) -> Result<(), AppError> {
        let mut rng = rand::rng();

        self.init_from(reference)?;

        let mut retry = false;
        if !solvable {
            println!("generate unsolvable puzzle");
            self.puzzle[0][0] = 2;
            self.puzzle[0][1] = 1;
        } else {
            println!("generate solvable puzzle");
        }

        while iterations != 0 {
            let a = rng.random::<u8>() % 4;
            //println!("a: {}", a);
            match a {
                0 => self.up().unwrap_or_else(|_| retry = true),
                1 => self.down().unwrap_or_else(|_| retry = true),
                2 => self.left().unwrap_or_else(|_| retry = true),
                _ => self.right().unwrap_or_else(|_| retry = true),
            };
            // if the mouvement couldn't be acheved
            if retry {
                let mut b = a;
                while retry {
                    b = (b + 1) % 4;
                    retry = false;
                    match b {
                        0 => self.up().unwrap_or_else(|_| retry = true),
                        1 => self.down().unwrap_or_else(|_| retry = true),
                        2 => self.left().unwrap_or_else(|_| retry = true),
                        _ => self.right().unwrap_or_else(|_| retry = true),
                    };
                }
            }
            iterations -= 1;
            //println!("{}", self);
        }
        self.mouv_count = 0;
        Ok(())
    }

    pub fn find(&self, val: u16) -> Point {
        let mut x = 0;
        let mut y = 0;
        for (count, line) in self.puzzle.iter().enumerate() {
            match line.iter().position(|&x| x == val) {
                Some(v) => {
                    x = count;
                    y = v;
                    break;
                }
                None => continue,
            };
        }

        Point { x, y }
    }

    /// initialise a puzzle element from a T object implementing `io::BufRead`
    /// Todo: break on wrong input
    pub fn init<T: io::BufRead>(&mut self, mut f: T) -> io::Result<()> {
        if self.init {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Puzzle already initialized",
            ));
        }
        let mut buf = String::new();
        let re = Regex::new(r"\b\d+\b").unwrap();
        let mut i = 0;
        let mut first_line = true;

        while f.read_line(&mut buf)? != 0 {
            if buf.starts_with("#") {
            } else if first_line {
                first_line = false;
                ()
            } else {
                let mut j = 0;
                for c in re
                    .find_iter(&buf)
                    .map(|f| f.as_str().parse::<u16>().unwrap())
                {
                    if c == 0 {
                        self.empty_cell = Point { x: i, y: j };
                    }
                    self.puzzle[i][j] = c;
                    j += 1;
                }
                i += 1;
            }
            buf.clear();
        }
        self.init = true;
        Ok(())
    }

    pub fn init_from(&mut self, v: &PContainer) -> Result<(), AppError> {
        if self.init {
            return Err(AppError::new("Puzzle already initialized"));
        }
        self.puzzle = v.clone();
        self.empty_cell = self.find(0);
        Ok(())
    }

    /// check if self if solved by comparing it to a PContainer reference
    /// comparing is done by the `PartialEq` trait
    pub fn is_solved(&mut self, reference: &PContainer) -> bool {
        if self.solved == true {
            return true;
        } else {
            self.solved = *reference == self.puzzle;
            self.solved
        }
    }

    fn set_neutral(&mut self) {
        let mut reference = Puzzle::new(self.dim);
        let _ = reference.init_from(&gen_solved_ref(self.dim));

        // reset in x
        let mouvs = reference.empty_cell.x as i32 - self.empty_cell.x as i32;
        if mouvs < 0 {
            for _ in mouvs..0 {
                let _ = self.up();
            }
        } else {
            for _ in 0..mouvs {
                let _ = self.down();
            }
        }

        // reset in y
        let mouvs = reference.empty_cell.y as i32 - self.empty_cell.y as i32;
        if mouvs < 0 {
            for _ in mouvs..0 {
                let _ = self.left();
            }
        } else {
            for _ in 0..mouvs {
                let _ = self.right();
            }
        };
    }

    fn inversion_counter(&self) -> usize {
        let mut inversion = 0;
        let tiles: Vec<u16> = self.clone().into_iter().collect();
        for i in 0..tiles.len() {
            for j in (i + 1)..tiles.len() {
                if tiles[i] > tiles[j] {
                    inversion += 1;
                }
            }
        }
        inversion
    }

    /// check the solvability of a puzzle
    pub fn is_solvable(&self) -> bool {
        let mut clone_ = self.clone();
        clone_.set_neutral();
        let inversion = clone_.inversion_counter();
        inversion % 2 == 0
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reference = gen_solved_ref(self.dim);
        for (row_idx, row) in self.puzzle.iter().enumerate() {
            for _ in 0..self.dim {
                write!(f, "+---")?;
            }
            writeln!(f, "+")?;

            // Row content
            for (col_idx, &element) in row.iter().enumerate() {
                let is_done = element == reference[row_idx][col_idx];
                let cell = if element == 0 {
                    format!(" {} ", "_".blue().bold())
                } else if is_done {
                    format!(" {:<2}", element.to_string().green().bold())
                } else {
                    format!(" {:<2}", element.to_string().red())
                };
                write!(f, "|{}", cell)?;
            }
            writeln!(f, "|")?;
        }

        // Final bottom border
        for _ in 0..self.dim {
            write!(f, "+---")?;
        }
        writeln!(f, "+")
    }
}

impl Index<usize> for Puzzle {
    type Output = Vec<u16>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.puzzle[index]
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
use std::ops::Index;
use std::u8;

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

impl IntoIterator for Puzzle {
    type Item = u16;
    type IntoIter = PuzzleIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let dim = self.dim;
        PuzzleIntoIter {
            puzzle: self,
            reference: gen_solved_ref(dim),
            index: 1,
        }
    }
}
