//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![feature(alloc_error_handler)]

#![no_std]
#![no_main]

extern crate alloc;

use teensy4_panic as _;
use cortex_m_rt::entry;

use alloc::string::String;

use aoc21::utils::Hardware;


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

impl aoc21::solutions::Solution<u32> for Solution {
    fn parse_file(&self, hardware: &Hardware, in_data: String) -> u32 {
        return 5;
    }

    fn part_a(&self, hardware: &Hardware, data: &u32) {
        panic!("Test");
    }

    fn part_b(&self, hardware: &Hardware, data: &u32) {
    }
}
