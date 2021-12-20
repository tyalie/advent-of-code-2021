#![no_std]
#![no_main]
#![feature(map_first_last)]

mod container;

use aoc21::usbwriteln;
use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::utils::Hardware;
use aoc21::runtime::Memory;

use container::*;


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol, Memory::RAM1(300_000));
}

struct Solution {}

impl aoc21::solutions::Solution<ImageAlgo> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut ImageAlgo) {
        usbwriteln!("{}", data);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut ImageAlgo) {
    }
}
