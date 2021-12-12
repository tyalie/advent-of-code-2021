extern crate alloc;

use core::fmt::Write;
use core::ops::Range;

use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;
use aoc21::utils::tools::parse_with_err;


pub struct Crabs {
    pub positions: Vec<u16>
}

impl aoc21::solutions::ParsedData for Crabs {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let positions = in_data.trim().split(',').map(parse_with_err).collect();

        return Crabs {
            positions: positions
        }
    }
}

impl Crabs {
    pub fn range(&self) -> Range<u16> {
        assert_ne!(self.positions.len(), 0, "Cannot calculate range on empty positions list");

        return Range { 
            start: *self.positions.iter().min().unwrap(),
            end: *self.positions.iter().max().unwrap()
        }
    }
}
