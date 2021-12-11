extern crate alloc;

use core::fmt::Write;

use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;


pub struct Diagnostic {
    pub report: Vec<Vec<bool>>
}

impl aoc21::solutions::ParsedData for Diagnostic {
    fn parse_file(hardware: &mut Hardware, in_data: String) -> Self {
        let data: Vec<Vec<bool>> = in_data.lines().map(|v: &str| v.chars().map(|c| c == '1').collect()).collect();
        if !data.iter().all(|v| v.len() == data[0].len()) {
            writeln!(hardware.writer, "ERR: not all lines have same length\n{:?}", data).unwrap();
            panic!();
        }
        if data.len() % 2 == 0 {
            writeln!(hardware.writer, "WARN: even number of lines read. Undefined behaviour ({})", data.len()).unwrap();
        }

        return Diagnostic {
            report: data
        };
    }
}
