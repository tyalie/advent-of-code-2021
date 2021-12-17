#![no_std]
#![no_main]

mod container;
mod track;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::{utils::Hardware, usbwriteln};
use aoc21::runtime::Memory;

use container::*;
use track::{Track, Vector};


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol, Memory::RAM1(300_000));
}

struct Solution {}

impl aoc21::solutions::Solution<Target> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Target) {
        let heighest_y = find_heighest_y(data);
        usbwriteln!(" - track with heighest y position: {}", heighest_y.unwrap());
    }
    fn part_b(&self, _: &mut Hardware, data: &mut Target) {
    }
}

fn find_heighest_y(target: &Target) -> Option<i32> {
    (0..200).zip(-200..200).filter_map(|(dx, dy)| {
        let track = Track::<i32>::start(Vector { dx, dy });
        Some(target.verify_track(&track)?.1)
    }).max()
}
