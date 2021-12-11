extern crate alloc;

use core::fmt::Write;

use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;
use aoc21::usbwriteln;

use aoc21::utils::tools::parse_with_err_radix;

pub struct Diagnostic {
    pub report: Vec<u16>,
    pub num_length: usize,
}

impl aoc21::solutions::ParsedData for Diagnostic {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let lines: Vec<&str> = in_data.lines().collect();

        if !lines.iter().all(|v| v.len() == lines[0].len()) {
            panic!("ERR: not all lines have same length\n{:?}", lines);
        }
        if lines.len() % 2 == 0 {
            usbwriteln!("WARN: even number of lines read. Undefined behaviour ({})", lines.len());
        }

        let data: Vec<u16> = lines.iter().map(|l| parse_with_err_radix(l, 2)).collect();

        return Diagnostic {
            report: data,
            num_length: lines[0].len(),
        };
    }
}
