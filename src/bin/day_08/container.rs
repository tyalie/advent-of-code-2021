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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Digit {
    pub on_segments: [bool; 7]
}

impl Digit {
    pub fn from_str(value: &str) -> Self {
        let mut on_segs = [false; 7];
        value.bytes().for_each(|b| on_segs[(b - b'a') as usize] = true );
        return Digit { on_segments: on_segs };
    }

    pub fn count_on(&self) -> u8 {
        self.on_segments.iter().filter(|&&v| v).count() as u8
    }

    pub fn to_num(&self) -> Option<u8> {
        let permutations: [u8; 10] = [  // all digits in the order 0..9
            0b1110111, 0b0100100,
            0b1011101, 0b1101101,
            0b0101110, 0b1101011,
            0b1111011, 0b0100101,
            0b1111111, 0b1101111
        ];

        let digit: u8 = self.clone().into();
        permutations.iter().position(|&v| digit == v).map(|v| v as u8)
    }
}

impl Into<u8> for Digit {
    fn into(self) -> u8 {
        let mut out: u8 = 0;
        self.on_segments.iter().rev().for_each(|&e| { out <<= 1; out |= e as u8; });
        return out;
    }
}
