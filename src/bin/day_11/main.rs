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

impl aoc21::solutions::Solution<Cave> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Cave) {
        let mut cave = data.clone();

        let flashes: u32 = (0..100).map(|_| do_step(&mut cave)).sum();
        usbwriteln!(" - number of flashes {}", flashes);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut Cave) {
        let mut cave = data.clone();

        let max_flashes = data.rows() as u32 * data.cols() as u32;
        let mut steps = 0;

        while do_step(&mut cave) != max_flashes {
            steps += 1;
        }

        usbwriteln!(" - all flash simulatnously step {}" , steps + 1);

    }
}

fn do_step(data: &mut Cave) -> u32 {
    // increase all octo energy values by 1
    data.octos.iter_mut().for_each(|row| row.iter_mut().for_each(|v| *v += 1));
    let mut flashes = 0;

    while let Some(pos) = data.find_lighting() {
        data.get_adjacent(&pos).iter().for_each(|apos| {
            if data[&apos] > 0 {
                // increase energy level of adjacent fishies if they 
                // haven't lighted up before (energy is 0)
                data[&apos] += 1;  
            }
        });
        data[&pos] = 0;
        flashes += 1;
    }

    return flashes;
}
