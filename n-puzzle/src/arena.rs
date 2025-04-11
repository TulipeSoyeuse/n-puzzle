use regex::Regex;
use std::error::Error;
use std::fmt::Display;
use std::io;

pub struct Puzzle {
    pub puzzle: Vec<Vec<u16>>,
    pub mouv_count: usize,
    pub dim: usize,
    empty_cell: Point,
    init: bool,
}

struct Point {
    x: u8,
    y: u8,
}

impl Puzzle {
    pub fn new(dimension: usize) -> Self {
        Puzzle {
            puzzle: vec![vec![0u16; dimension]; dimension],
            mouv_count: 0,
            dim: dimension,
            init: false,
            empty_cell: Point { x: 0, y: 0 },
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
                            self.empty_cell = Point {
                                x: n as u8,
                                y: c as u8,
                            }
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
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.puzzle.iter() {
            writeln!(f, "{}", "-".repeat(self.dim as usize * 4 + 1))?;
            for element in line {
                write!(f, "│ {} ", element)?;
            }
            write!(f, "│\n")?;
        }
        writeln!(f, "{}", "-".repeat(self.dim as usize * 4 + 1))?;
        write!(
            f,
            "mouvement count: {}\ndimension: {}\n",
            self.mouv_count, self.dim
        )
    }
}
trait Mouvement {
    fn up(&self) -> Result<(), ()>;
    fn down(&self) -> Result<(), ()>;
    fn left(&self) -> Result<(), ()>;
    fn right(&self) -> Result<(), ()>;
}

impl Mouvement for Puzzle {
    fn up(&self) -> Result<(), ()> {
        let
    }
}
