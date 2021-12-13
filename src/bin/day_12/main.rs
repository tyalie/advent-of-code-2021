#![no_std]
#![no_main]

mod container;
mod graph;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::{utils::Hardware, usbwriteln};

use container::*;


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

impl aoc21::solutions::Solution<Caves> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Caves) {
        usbwriteln!("{:?}", data.graph);
    }

    fn part_b(&self, _: &mut Hardware, data: &mut Caves) {
    }
}
