#![no_std]
#![no_main]

mod container;

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

impl aoc21::solutions::Solution<Computer> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Computer) {
        let occurences = data.displays.iter()
            .flat_map(|display| display.number.iter())
            .filter(|digit| [2, 4, 3, 7].contains(&digit.count_on()))
            .count();

        usbwriteln!(" - found {} digitis that are either 1, 4, 7 or 8", occurences);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut Computer) {
    }
}
