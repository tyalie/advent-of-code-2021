#![no_std]
#![no_main]

mod container;
mod number;

use aoc21::usbwriteln;
use itertools::Itertools;
use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::utils::Hardware;
use aoc21::runtime::Memory;

use container::*;

use crate::number::SailfishNumber;


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol, Memory::RAM1(300_000));
}

struct Solution {}

impl aoc21::solutions::Solution<Homework> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Homework) {
        let mut res = data.numbers[0].clone();
        for num in &data.numbers[1..] {
            res = res + num.clone();
        }
        let mag = res.magnitude();
        usbwriteln!(" - sailfish number magnitude is {}", mag);
    }

    fn part_b(&self, _: &mut Hardware, data: &mut Homework) {
        let max_mag = data.numbers.iter().permutations(2)
            .map(|v| (v[0].clone() + v[1].clone()).magnitude())
            .max().unwrap();
        usbwriteln!(" - max archivable magnitude is: {}", max_mag);
    }
}
