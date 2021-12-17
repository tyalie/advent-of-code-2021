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
        let (track, heighest_y) = find_heighest_y(data).unwrap();
        usbwriteln!(" - track with heighest y position ({}): {:?}", heighest_y, track);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut Target) {
        let amount = get_all_possible_velocities(data.clone()).count();
        usbwriteln!(" - found {} possible velocities", amount);
    }
}

fn find_heighest_y(target: &Target) -> Option<(Vector<i32>, i32)> {
    get_all_possible_velocities(target.clone())
        .max_by_key(|(_, height)| *height)
}

fn get_all_possible_velocities(target: Target) -> impl Iterator<Item = (Vector<i32>, i32)> {
    (0..200).flat_map(|dx| (-200..200).map(move |dy| Vector { dx, dy }))
        .filter_map(move |start| {
            let track = Track::<i32>::start(start);
            let result = target.verify_track(&track)?;
            Some((start, result.1))
        })
}
