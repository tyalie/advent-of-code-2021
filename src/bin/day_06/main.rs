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

impl aoc21::solutions::Solution<School> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut School) {
        self.do_solution(80, data);
    }
    

    fn part_b(&self, _: &mut Hardware, data: &mut School) {
        self.do_solution(256, data);
    }
}

impl Solution {
    pub fn do_solution(&self, days: usize, data: &School) {
        let mut compressed: CompressedSchool<u128> = data.compress();
        usbwriteln!(" - starting with {} fishs", compressed.count_fish());

        for _ in 0..days { compressed.do_step(); }

        usbwriteln!(" - there are {} fishs after {} days", compressed.count_fish(), days);
    }
}
