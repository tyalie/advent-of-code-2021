extern crate alloc;

use core::fmt::Display;
use alloc::vec::Vec;
use alloc::string::String;

use aoc21::utils::Hardware;


#[derive(Clone)]
pub struct Cave {
    pub chitons: Vec<Vec<u8>>,
    pub expanded: bool
}

impl aoc21::solutions::ParsedData for Cave {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let chitons: Vec<Vec<u8>> = in_data.lines()
            .map(|line| line.bytes().map(|v| v - b'0').collect())
            .collect();

        assert!(chitons.iter().all(|l| l.len() == chitons[0].len()), "Field isn't rectengular");

        Cave {
            chitons: chitons,
            expanded: false
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for row in &self.chitons {
            for v in row {
                write!(f, "{}", v)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


impl Cave {
    pub fn expand(&self) -> Self {
        Cave { expanded: true, ..self.clone() }
    }
}
