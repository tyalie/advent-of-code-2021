//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![feature(alloc_error_handler)]

#![no_std]
#![no_main]

extern crate alloc;

mod container;

use alloc::vec::Vec;
use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::utils::Hardware;
use aoc21::usbwriteln;

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
    fn part_a(&self, _: &mut Hardware, data: &mut Sonar) {
        let increases = data.depths.iter().zip(&data.depths[1..])
            .filter(|(v1, v2)| v1 < v2).count();
        usbwriteln!("- number is {}", increases);
    }

    fn part_b(&self, _: &mut Hardware, data: &mut Sonar) {
        let data_source: Vec<u16> = data.depths.iter()
            .zip(&data.depths[1..]).zip(&data.depths[2..])
            .map(|((v1, v2), v3)| v1 + v2 + v3).collect();
        let increases = data_source.iter().zip(&data_source[1..])
            .filter(|(v1, v2)| v1 < v2).count();
        usbwriteln!("- number is {}", increases);
    }
}
