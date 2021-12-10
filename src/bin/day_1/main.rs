//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![feature(alloc_error_handler)]

#![no_std]
#![no_main]

extern crate alloc;

mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;

use core::fmt::Write;

use aoc21::utils::Hardware;

use container::Sonar;

#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol);
}

struct Solution {
}

impl aoc21::solutions::Solution<Sonar> for Solution {
    fn part_a(&self, hardware: &mut Hardware, data: &Sonar) {
        writeln!(hardware.writer, "Parsed {:?} sonar points", data.depths.len()).unwrap();
        let increases = data.depths.iter().zip(&data.depths[1..]).filter(|(v1, v2)| v1 < v2).count();
        writeln!(hardware.writer, "- number is {}", increases).unwrap();
    }

    fn part_b(&self, hardware: &mut Hardware, data: &Sonar) {
    }
}
