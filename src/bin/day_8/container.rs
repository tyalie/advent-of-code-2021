extern crate alloc;

use core::convert::TryInto;

use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;


pub struct Computer {
    pub displays: Vec<Display>
}

impl aoc21::solutions::ParsedData for Computer {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let process_digits = |s: &str| {
            s.split_whitespace().map(Digit::from_str).collect::<Vec<Digit>>()
        };

        let displays = in_data.lines().map(|line: &str| {
            let (combs, number) = line.split_once("|").unwrap();
            Display {
                combinations: process_digits(combs).try_into().expect("Input string comb doesn't fit"),
                number: process_digits(number).try_into().expect("Input string number doesn't fit")
            }
        }).collect();

        return Computer {
            displays: displays
        }
    }
}

#[derive(Debug)]
pub struct Display {
    pub combinations: [Digit; 10],
    pub number: [Digit; 4]
}

#[derive(Debug, PartialEq, Eq)]
pub struct Digit {
    pub on_segments: u8
}

impl Digit {
    pub fn from_str(value: &str) -> Self {
        let mut on_segs = 0;
        value.bytes().for_each(|b| on_segs |= 1 << (b - b'a') );
        return Digit { on_segments: on_segs };
    }

    pub fn count_on(&self) -> u8 {
        self.on_segments.count_ones() as u8
    }
}
