use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::{fmt, fs};

#[derive(Clone)]
struct Image {
    rows: usize,
    cols: usize,
    data: Vec<bool>,
    default: bool,
}

impl Image {
    fn new(rows: usize, cols: usize, default: bool) -> Image {
        let data = vec![false; rows * cols];
        Image {
            rows,
            cols,
            data,
            default,
        }
    }

    fn lit_count(&self) -> usize {
        self.data.iter().filter(|a| **a == true).count()
    }
}

// 2-d index for Image
impl Index<(i32, i32)> for Image {
    type Output = bool;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        if index.0 < 0 || index.1 < 0 {
            return &self.default;
        }

        if (index.0 as usize >= self.rows || index.1 as usize >= self.cols) {
            return &self.default;
        }

        &self.data[index.0 as usize * self.cols + index.1 as usize]
    }
}

// 2-d index for Image (mutable)
impl IndexMut<(i32, i32)> for Image {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        if index.0 < 0 || index.1 < 0 {
            panic!()
        }

        if (index.0 as usize >= self.rows || index.1 as usize >= self.cols) {
            panic!()
        }

        &mut self.data[index.0 as usize * self.cols + index.1 as usize]
    }
}

// "Parse" functionality for Image
impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").filter(|row| !row.trim().is_empty()).collect();

        let result = Image {
            cols: lines[0].len(),
            rows: lines.len(),
            data: lines
                .iter()
                .flat_map(|line| line.chars().map(|c| c == '#'))
                .collect(),
            default: false,
        };

        if result.data.len() == result.rows * result.cols {
            Ok(result)
        } else {
            Err(())
        }
    }
}

// "ToString" functionality for Image
impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self[(r as i32, c as i32)] {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?
                }
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

struct ImageEnhancer {
    algorithm: Vec<bool>,
}

impl ImageEnhancer {
    fn enhance(&self, image: &Image, times: usize) -> Image {
        let mut image = image.clone();
        for _ in 0..times {
            image = self.enhance_iteration(image);
        }

        image
    }

    fn enhance_iteration(&self, image: Image) -> Image {
        let s = 1 as i32;
        let default = if image.default {
            *self.algorithm.last().unwrap()
        } else {
            *self.algorithm.first().unwrap()
        };

        let mut result = Image::new(
            image.rows + s as usize * 2,
            image.cols + s as usize * 2,
            default,
        );

        for r in -s..(image.rows as i32 + s) {
            for c in -s..(image.rows as i32 + s) {
                let mut index = 0;
                for r1 in -1..=1 {
                    for c1 in -1..=1 {
                        index <<= 1;
                        if image[(r + r1, c + c1)] {
                            index += 1;
                        }
                    }
                }

                assert!(index < 512);

                result[(r + s, c + s)] = self.algorithm[index];
            }
        }

        result
    }
}

// "Parse" functionality for Image
impl FromStr for ImageEnhancer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = ImageEnhancer {
            algorithm: s.trim().chars().map(|c| c == '#').collect(),
        };

        Ok(result)
    }
}

fn main() {
    let input = fs::read_to_string("input-day-20.txt").unwrap();
    let mut iter = input.split("\n");

    let a: Vec<&str> = iter.by_ref().take_while(|a| !a.is_empty()).collect();
    let b: Vec<&str> = iter.collect();

    let algorithm = a.join("");
    let image_data = b.join("\n");

    let image: Image = image_data.parse().unwrap();
    let enhancer: ImageEnhancer = algorithm.parse().unwrap();

    println!("{:?}", algorithm);
    println!("{}", image);
    println!("{}", enhancer.enhance(&image, 2).lit_count());
    println!("{}", enhancer.enhance(&image, 50).lit_count());

    // println!("{}", enhancer.enhance(&image, 0));
    // println!("{}", enhancer.enhance(&image, 1));
    // println!("{}", enhancer.enhance(&image, 2));
    // println!("{}", enhancer.enhance(&image, 3));
    // println!("{}", enhancer.enhance(&image, 4));
}
