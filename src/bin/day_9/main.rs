#![no_std]
#![no_main]

extern crate alloc;
mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;
use alloc::collections::BTreeSet;
use alloc::vec;
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

        let mut loc_mins: Vec<(u16, u16)> = Vec::new();
        let mut score = 0u32;

        for row in 0..data.nrows() as i32 {
            for col in 0..data.ncols() as i32 {
                let cur_val = data.get(row, col).unwrap();
                let is_min = data.get_adjacents(row, col).iter()
                    .filter_map(|&pos| pos)
                    .all(|pos| data.get(pos.0 as i32, pos.1 as i32).unwrap() > cur_val);

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
        let mut basins: Vec<u16> = data.local_minimas.iter()
            .map(|mini| find_basin(data, mini))
            .collect();
        basins.sort_unstable();
        let score = basins.iter().rev().take(3).fold(1u32, |a, &v| v as u32 * a);
        usbwriteln!(" - three largests basins score is {}", score);
    }
}

fn find_basin(data: &CaveMap, minima: &(u16, u16)) -> u16 {
    let mut searched: BTreeSet<(u16, u16)> = BTreeSet::new();
    let mut to_search: Vec<(u16, u16)> = vec!(*minima);
    let mut size = 0u16;

    while let Some(next) = to_search.pop() {
        if searched.contains(&next) {
            continue;
        }
        searched.insert(next);

        if *data.get(next.0 as i32, next.1 as i32).unwrap() == 9 {
            continue;  // end of basin
        }

        size += 1;

        data.get_adjacents(next.0 as i32, next.1 as i32).iter()
            .filter_map(|&v| v)
            .filter(|v| !searched.contains(v))
            .for_each(|v| to_search.push(v));
    }

    return size;
}
