extern crate alloc;

use core::convert::TryInto;
use core::fmt::{Display, Formatter};

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

    pub fn get_adjacents(&self, row: i32, col: i32) -> [Option<(u16, u16)>; 4] {
        let adjacent_fields = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        return adjacent_fields.iter()
            .map(|a| (row + a.0, col + a.1))
            .map(|pos| {
                if pos.0 < 0 || pos.0 as usize >= self.floor.len() 
                    || pos.1 < 0 || pos.1 as usize >= self.floor[0].len() {
                    None
                } else {
                    Some((pos.0 as u16, pos.1 as u16))
                }
            }).collect::<Vec<Option<(u16, u16)>>>().try_into().unwrap();
    }

    pub fn nrows(&self) -> usize {
        self.floor.len()
    }

    pub fn ncols(&self) -> usize {
        self.floor[0].len()  // map is rectengular so that works
    }
}
