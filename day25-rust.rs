use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::{fmt, fs};

#[derive(Copy, Clone)]
enum SeaCucumber {
    E,
    S,
}

#[derive(Clone)]
struct Seafloor {
    rows: usize,
    cols: usize,
    data: Vec<Option<SeaCucumber>>,
}

impl Seafloor {
    fn new(rows: usize, cols: usize) -> Seafloor {
        let data = vec![None; rows * cols];
        Seafloor { rows, cols, data }
    }

    fn evolve(&mut self) -> bool {
        let a = self.move_right();
        let b = self.move_down();
        a | b
    }

    fn move_right(&mut self) -> bool {
        let mut moved = false;
        let mut new = Seafloor::new(self.rows, self.cols);
        for row in 0..self.rows as i32 {
            for col in 0..self.cols as i32 {
                match (self[(row, col)], self[(row, col + 1)]) {
                    (Some(SeaCucumber::E), None) => {
                        new[(row, col + 1)] = Some(SeaCucumber::E);
                        moved = true;
                    }
                    (other, _) => {
                        if new[(row, col)].is_none() {
                            new[(row, col)] = other;
                        }
                    }
                }
            }
        }
        self.data = new.data;
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved = false;
        let mut new = Seafloor::new(self.rows, self.cols);
        for row in 0..self.rows as i32 {
            for col in 0..self.cols as i32 {
                match (self[(row, col)], self[(row + 1, col)]) {
                    (Some(SeaCucumber::S), None) => {
                        new[(row + 1, col)] = Some(SeaCucumber::S);
                        moved = true;
                    }
                    (other, _) => {
                        if new[(row, col)].is_none() {
                            new[(row, col)] = other;
                        }
                    }
                }
            }
        }
        self.data = new.data;
        moved
    }
}

// 2-d index for Image
impl Index<(i32, i32)> for Seafloor {
    type Output = Option<SeaCucumber>;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        let r = ((index.0 % self.rows as i32) as usize + self.rows) % self.rows;
        let c = ((index.1 % self.cols as i32) as usize + self.cols) % self.cols;

        &self.data[r * self.cols + c]
    }
}

// 2-d index for Image (mutable)
impl IndexMut<(i32, i32)> for Seafloor {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        let r = ((index.0 % self.rows as i32) as usize + self.rows) % self.rows;
        let c = ((index.1 % self.cols as i32) as usize + self.cols) % self.cols;

        &mut self.data[r * self.cols + c]
    }
}

// "Parse" functionality for Image
impl FromStr for Seafloor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").filter(|row| !row.trim().is_empty()).collect();

        let result = Seafloor {
            cols: lines[0].len(),
            rows: lines.len(),
            data: lines
                .iter()
                .flat_map(|line| {
                    line.chars().map(|c| match c {
                        '>' => Some(SeaCucumber::E),
                        'v' => Some(SeaCucumber::S),
                        _ => None,
                    })
                })
                .collect(),
        };

        if result.data.len() == result.rows * result.cols {
            Ok(result)
        } else {
            Err(())
        }
    }
}

// "ToString" functionality for Image
impl fmt::Debug for Seafloor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                match self[(r as i32, c as i32)] {
                    Some(SeaCucumber::E) => write!(f, ">")?,
                    Some(SeaCucumber::S) => write!(f, "v")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("input-day-25.txt").unwrap();

    let mut seafloor: Seafloor = input.parse().unwrap();

    let mut i = 1;
    while seafloor.evolve() {
        i += 1;
    }

    println!("Solution: {:?}", i);
}
