extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

use num_traits::{ToPrimitive, NumCast};
use num_traits::int::PrimInt;

use aoc21::utils::Hardware;
use aoc21::utils::tools::parse_with_err;
use tuple::Map;

pub struct Vents {
    pub lines: Vec<Line>
}

impl aoc21::solutions::ParsedData for Vents {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let lines: Vec<Line> = in_data.lines().map(|l| {
            l.split_once(" -> ").expect("Input is malformated")
                .map(|e| {
                    let xy = e.split_once(',')
                        .expect("Input is malformed")
                        .map(parse_with_err::<u16>);
                    Point::from_tuple(xy).expect("Tuple → Point wasn't successful")
                })
        }).map(|(start, end)| Line { start: start, stop: end } ).collect();

        return Vents {
            lines: lines
        }
    }
}


// Own Types

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point<T> where T: PrimInt {
    pub x: T,
    pub y: T 
}

impl<T: PrimInt + NumCast> Point<T> {
    fn from<U: PrimInt + ToPrimitive>(other: Point<U>) -> Option<Point<T>> {
        Some(Point { x: T::from(other.x)?, y: T::from(other.y)? })
    }

    fn from_tuple<U: PrimInt + ToPrimitive>(other: (U, U)) -> Option<Point<T>> {
        Some(Point { x: T::from(other.0)?, y: T::from(other.1)? })
    }
}


#[derive(Clone)]
pub struct Vector {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Point<u16>,
    pub stop: Point<u16>
}

impl Line {
    pub fn is_straight(&self) -> bool {
        return self.start.x == self.stop.x || self.start.y == self.stop.y;
    }

    pub fn step_size(&self) -> Vector {
        return Vector {
            x: (self.stop.x as i32 - self.start.x as i32).signum(),
            y: (self.stop.y as i32 - self.start.y as i32).signum()
        }
    }
}

impl core::iter::IntoIterator for &Line {
    type Item = <LineIntoIterator as Iterator>::Item;
    type IntoIter = LineIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIntoIterator {
            cur_pos: Point::from(self.start).unwrap(),
            end: self.stop.clone(),
            step: self.step_size()
        }
    }
}

pub struct LineIntoIterator {
    cur_pos: Point<i32>,
    end: Point<u16>,
    step: Vector,
}

impl core::iter::Iterator for LineIntoIterator {
    type Item = Point<u16>;

    fn next(&mut self) -> Option<Point<u16>> {
        if self.step.x == 0 && self.step.y == 0 {
            panic!("Line iterator step magnitude is 0");
        }

        if self.cur_pos.x == self.end.x as i32 + self.step.x 
            && self.cur_pos.y == self.end.y as i32 + self.step.y {
            return None;
        }

        let cur_pos = self.cur_pos;
        self.cur_pos.x += self.step.x;
        self.cur_pos.y += self.step.y;

        return Some(Point::from(cur_pos).expect("P Conversion failed (maybe neg?)"));
    }

}
