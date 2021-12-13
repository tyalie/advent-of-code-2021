extern crate alloc;


use core::convert::TryInto;
use core::fmt::{Formatter, Display};
use core::ops::{IndexMut, Index};

use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;


#[derive(Clone)]
pub struct Cave {
    pub octos: Vec<Vec<u8>>
}

impl aoc21::solutions::ParsedData for Cave {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let octos: Vec<Vec<u8>> = in_data.lines()
            .map(|v| v.bytes().map(|b| b - b'0').collect()).collect();

        assert!(octos.iter().all(|row| row.len() == octos[0].len()), "Array isn't rectangular");

        return Cave { octos: octos };
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for row in &self.octos {
            for col in row {
                write!(f, "{:x}", col)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Index<&(u8, u8)> for Cave {
    type Output = u8;
    fn index<'a>(&'a self, coord: &(u8, u8)) -> &'a u8 {
        &self.octos[coord.0 as usize][coord.1 as usize]
    }

}

impl IndexMut<&(u8, u8)> for Cave {
    fn index_mut<'a>(&'a mut self, coord: &(u8, u8)) -> &'a mut u8 {
        self.get_mut(&(coord.0 as i16, coord.1 as i16)).unwrap()
    }
}

impl Cave {
    pub fn get_mut<'a>(&'a mut self, coord: &(i16, i16)) -> Option<&'a mut u8> {
        if coord.0 < 0 || coord.1 < 0 {
            None
        } else {
            Some(self.octos.get_mut(coord.0 as usize)?.get_mut(coord.1 as usize)?)
        }
    }

    pub fn rows(&self) -> u8 {
        self.octos.len().try_into().expect("Too many rows")
    }

    pub fn cols(&self) -> u8 {
        self.octos[0].len().try_into().expect("Too many cols")
    }

    /// Find all octopussies that should light up 
    ///   => energy level over 9
    pub fn find_lighting(&self) -> Option<(u8, u8)> {
        (0..self.rows())
            .flat_map(|row| (0..self.cols()).map(move |col| (row, col)))
            .find(|pos| self[pos] > 9)
    }

    /// Get adjacent fields in an iterator
    ///
    /// This method is safe, that means it will not return fields that 
    /// are outside the cave
    pub fn get_adjacent(&self, coords: &(u8, u8)) -> Vec<(u8, u8)> {
        let adjs: [(i16, i16); 8] = [
            (-1, -1), (0, -1), (1, -1), 
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1)
        ];

        adjs.iter()
            .map(|(x, y)| (coords.0 as i16 + x, coords.1 as i16 + y))
            .filter(|(x, y)| 0 <= *x && *x < self.rows() as i16 && 0 <= *y && *y < self.cols() as i16)
            .map(|(x, y)| (x as u8, y as u8))
            .collect()
    }
}
