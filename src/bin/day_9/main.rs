#![no_std]
#![no_main]

extern crate alloc;
mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;
use alloc::vec::Vec;

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

impl aoc21::solutions::Solution<CaveMap> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut CaveMap) {
        let adjacent_fields = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        let mut loc_mins: Vec<(u16, u16)> = Vec::new();
        let mut score = 0u32;

        for row in 0..data.nrows() as i32 {
            for col in 0..data.ncols() as i32 {
                let cur_val = data.get(row, col).unwrap();
                let is_min = adjacent_fields.iter()
                    .all(|v| {
                        data.get(row+v.0, col+v.1).unwrap_or(&u8::MAX) > cur_val
                    });

                if is_min {
                    loc_mins.push((row as u16, col as u16));
                    score += *cur_val as u32 + 1;
                }
            }
        }

        usbwriteln!(" - found {} local minima = {} risk", loc_mins.len(), score);
        data.local_minimas = loc_mins;
    }
    fn part_b(&self, _: &mut Hardware, data: &mut CaveMap) {
    }
}
