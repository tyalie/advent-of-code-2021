extern crate alloc;

use core::fmt::{Display, Formatter};

use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;


pub struct CaveMap {
    pub floor: Vec<Vec<u8>>,
    pub local_minimas: Vec<(u16, u16)>
}

impl aoc21::solutions::ParsedData for CaveMap {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let floor: Vec<Vec<u8>> = in_data.lines()
            .map(|row: &str| { row.bytes().map(|v| v - ('0' as u8) ).collect() })
            .collect();

        assert!(floor.iter().all(|row| row.len() == floor[0].len()), "Map isn't rectangular");

        return CaveMap { floor: floor, local_minimas: Vec::new() }
    }
}

impl Display for CaveMap {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        for row in &self.floor {
            for &col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl CaveMap {
    pub fn get(&self, row: i32, col: i32) -> Option<&u8> {
        if row < 0 || col < 0 {
            return None
        }

        self.floor.get(row as usize)?.get(col as usize)
    }

    pub fn nrows(&self) -> usize {
        self.floor.len()
    }

    pub fn ncols(&self) -> usize {
        self.floor[0].len()  // map is rectengular so that works
    }
}
