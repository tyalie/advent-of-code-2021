extern crate alloc;

use core::cmp::max;
use core::ops::Range;
use core::cmp::Ordering;
use core::fmt::{Write, Display, Debug};

use alloc::string::String;
use aoc21::usbwriteln;
use aoc21::{utils::Hardware, usbwrite};
use aoc21::utils::tools::parse_with_err;
use num_traits::{PrimInt, Signed};
use tuple::Map;

use crate::track::*;

#[derive(Clone)]
pub struct Target {
    range_x: Range<i16>,
    range_y: Range<i16>,
}

impl aoc21::solutions::ParsedData for Target {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let (range_x, range_y) = in_data.trim().strip_prefix("target area: ").unwrap()
            .split_once(", ").unwrap()
            .map(|v| {
                let v = v[2..].split_once("..").unwrap()
                    .map(|v| parse_with_err::<i16>(v));
                v.0..(v.1 + 1)
            });

        assert!(!range_y.is_empty(), "range y isn't in right order {:?}", range_y);
        assert!(!range_x.is_empty(), "range x isn't in right order {:?}", range_x);

        Target { range_x, range_y }
    }
}

impl Target {
    pub fn contains<T>(&self, p: &Point<T>) -> bool where T: PrimInt {
        self.range_x.contains(&p.x.to_i16().unwrap()) 
            && self.range_y.contains(&p.y.to_i16().unwrap())
    }

    pub fn compare<T>(&self, p: &Point<T>) -> Ordering where T: PrimInt {
        if p.x.to_i16().unwrap() > self.range_x.end || p.y.to_i16().unwrap() < self.range_y.start {
            Ordering::Greater
        } else if self.contains(p) { Ordering::Equal } 
        else { Ordering::Less }
    }

    pub fn verify_track<T>(&self, start: &Track<T>) -> Option<(Track<T>, T)> where T: PrimInt + Signed + Debug {
        let mut track = *start;
        let mut max_y = start.position.y;

        loop {
            match self.compare(&track.position) {
                Ordering::Equal => return Some((track, max_y)),
                Ordering::Greater => return None,
                _ => {}
            };

            max_y = max(max_y, track.position.y);
            track = track.do_step();
        }
    }
}
