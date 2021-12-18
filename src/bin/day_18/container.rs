extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use core::fmt::Debug;

use aoc21::utils::Hardware;

use crate::number::*;

pub struct Homework {
    pub numbers: Vec<SailfishNumber>
}

impl aoc21::solutions::ParsedData for Homework {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        Homework {
            numbers: in_data.trim().lines()
                .map(|line| SailfishNumber::parse(line)).collect()
        }
    }
}

impl Debug for Homework {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for number in &self.numbers {
            writeln!(f, "{:?}", number)?;
        }
        Ok(())
    }
}
