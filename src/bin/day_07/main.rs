#![no_std]
#![no_main]

mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::{utils::Hardware, usbwriteln};
use aoc21::runtime::Memory;

use container::*;


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol, Memory::RAM1(400_000));
}

struct Solution {
}

impl aoc21::solutions::Solution<Crabs> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Crabs) {
        position_finder(data, |v| v);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut Crabs) {
        position_finder(data, |v| (v * (v + 1)) / 2);
    }
}

impl Crabs {
    pub fn calc_cost<F>(&self, align_pos: u16, scalor: F) -> u32
        where F: Fn(u32) -> u32 {

        self.positions.iter()
            .map(|v| scalor((*v as i32 - align_pos as i32).abs() as u32))
            .sum()
    }
}

pub fn position_finder<F>(data: &Crabs, scalor: F) where F: Fn(u32) -> u32 {
    let min_pos = data.range().min_by_key(|pos| {
        data.calc_cost(*pos, &scalor)
    }).expect("Couldn't find a minimum");

    let fuel_spend = data.calc_cost(min_pos, &scalor);

    usbwriteln!(
        " - minimum fuel usage at position {} with {} fuel spend",
        min_pos, fuel_spend
    );
}
