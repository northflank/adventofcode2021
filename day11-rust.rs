use std::{fs, fmt};
use std::str::FromStr;
use std::ops::{IndexMut, Index};

struct Octopuses {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}

impl Octopuses {
    pub fn simulate(&mut self) -> usize {
        // Charge
        for r in 0..self.rows {
            for c in 0..self.cols {
                self[(r, c)] += 1;
            }
        }

        // Simulate flashes
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.simulate_flashing(r, c);
            }
        }

        // Count flashes
        self.data.iter().filter(|a| **a == 0).count()
    }

    fn simulate_flashing(&mut self, r: usize, c: usize) {
        if self[(r, c)] <= 9 {
            return;
        }

        // trigger flash
        self[(r, c)] = 0;
        self.propagate_flash(r, c)
    }

    fn propagate_flash(&mut self, r0: usize, c0: usize) {
        // Charge neighbours that haven't flashed in this step yet
        for (r, c) in self.neighbour_indices(r0, c0) {
            let has_flashed = self[(r, c)] == 0;
            if !has_flashed {
                self[(r, c)] += 1;
                self.simulate_flashing(r, c);
            }
        }
    }

    fn neighbour_indices(&mut self, r0: usize, c0: usize) -> Vec<(usize, usize)> {
        let offsets = [-1, 0, 1];

        offsets.iter()
            .flat_map(|i| offsets.iter().map(move |j| (i, j)))
            .filter_map(|(i, j)| {
                let r = r0 as i32 + i;
                let c = c0 as i32 + j;

                if r < 0 || c < 0 { return None; }

                let r = r as usize;
                let c = c as usize;

                if r >= self.rows { return None; }
                if c >= self.cols { return None; }

                Some((r, c))
            })
            .collect()
    }

    pub fn count(&self) -> usize {
        return self.data.len();
    }
}

// 2-d index for Octopuses
impl Index<(usize, usize)> for Octopuses {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

// 2-d index for Octopuses (mutable)
impl IndexMut<(usize, usize)> for Octopuses {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.cols + index.1]
    }
}

// "Parse" functionality for Octopuses
impl FromStr for Octopuses {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n")
            .filter(|row| !row.trim().is_empty()).collect();

        let result = Octopuses {
            cols: lines[0].len(),
            rows: lines.len(),
            data: lines
                .iter()
                .flat_map(|line|
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                ).collect(),
        };

        if result.data.len() == result.rows * result.cols {
            Ok(result)
        } else {
            Err(())
        }
    }
}

// "ToString" functionality for Octopuses
impl fmt::Display for Octopuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                write!(f, "{:x}", self[(r, c)])?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

fn main() {
    let mut octopuses: Octopuses = fs::read_to_string("input-day-11.txt").unwrap().parse().unwrap();
    let mut flashes_total = 0;

    println!("{}", octopuses);

    let mut i = 0;
    loop {
        i += 1;
        let flashes = octopuses.simulate();

        flashes_total += flashes;

        if i == 100 {
            println!("Task 1: {}", flashes_total);
        }

        if flashes == octopuses.count() {
            println!("Task 2: {}", i);
            break;
        }
    }
}
