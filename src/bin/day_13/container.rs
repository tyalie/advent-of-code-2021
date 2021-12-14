extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use itertools::Itertools;
use core::fmt::Display;
use tuple::Map;

use aoc21::utils::tools::parse_with_err;
use aoc21::utils::Hardware;


pub struct Code {
    pub paper: Paper,
    pub folds: Vec<Fold>,
}

#[derive(Clone)]
pub struct Paper {
    pub dots: Vec<Point<u16>>,
}

impl aoc21::solutions::ParsedData for Code {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let (dots, folds) = in_data.split_once("\n\n").expect("Malformed input");
        let dots: Vec<Point<u16>> = dots.lines()
            .map(|line| 
                 Point::from(line.split_once(",").expect("Malformed in").map(parse_with_err))
            ).collect();

        let folds: Vec<Fold> = folds.lines().map(|line| {
            let (fold_type, value) = line.split_whitespace()
                .nth(2).expect("Malformed in")
                .split_once("=").expect("Malformed in");
            let value = parse_with_err(value);
            match fold_type {
                "x" => Fold::Vertical(value),
                "y" => Fold::Horizontal(value),
                &_ => panic!("Unknown fold type")
            }
        }).collect();

        return Code {
            paper: Paper { dots: dots },
            folds: folds,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

#[derive(Debug, Clone)]
pub enum Fold {
    Vertical(u16),
    Horizontal(u16)
}

impl<T> From<(T, T)> for Point<T> {
    fn from(orig: (T, T)) -> Self {
        Point { x: orig.0, y: orig.1 }
    }

}

impl Paper {
    pub fn size(&self) -> (u16, u16) {
        let max_x = self.dots.iter().map(|p| p.x).max().unwrap();
        let max_y = self.dots.iter().map(|p| p.y).max().unwrap();

        // force size to have uneven values
        (max_x, max_y).map(|v| (v + 1) / 2 * 2 + 1)
    }

    pub fn get_sorted_dots(&self) -> impl Iterator<Item = &Point<u16>> {
        let (_, my) = self.size();

        (0..my).flat_map(move |row_i| { 
            self.dots.iter()
                .filter(|p| p.y == row_i)
                .sorted_by_key(|v| v.x) 
        }).dedup()
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "---------------")?;

        let mut current_position = Point {x: 0, y: 0};
        for el in self.get_sorted_dots() {
            if el.y > current_position.y {
                current_position.x = 0;
                current_position.y = el.y;
                writeln!(f)?;
            }

            for _ in 0..(el.x - current_position.x) {
                write!(f, " ")?;
            }
            write!(f, "█")?;
            current_position.x = el.x + 1;
        }
        writeln!(f, "\n---------------")?;

        Ok(())
    }
}

