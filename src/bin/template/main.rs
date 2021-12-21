#![no_std]
#![no_main]

mod container;

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

impl aoc21::solutions::Solution<> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut ) {
    }
    fn part_b(&self, _: &mut Hardware, data: &mut ) {
    }
}
