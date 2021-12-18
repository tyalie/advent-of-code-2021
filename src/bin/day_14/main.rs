#![no_std]
#![no_main]

extern crate alloc;
mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;
use alloc::collections::BTreeMap;

use aoc21::{utils::Hardware, usbwriteln};

use container::*;


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol)
}

struct Solution {
}

impl aoc21::solutions::Solution<Polymerization> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Polymerization) {
        run_test(data, 10);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut Polymerization) {
        run_test(data, 40);
    }
}

impl Polymer {
    fn do_step(&self, rules: &BTreeMap<[u8;2], u8>) -> Polymer {
        let mut new_poly = Polymer::new(&self.ends);

        for (from, to) in rules {
            if let Some(pair_count) = self.pairs.get(from) {
                let new_pairs = [[from[0], *to], [*to, from[1]]];
                new_pairs.iter().for_each(|entry| {
                    *new_poly.pairs.entry(*entry).or_insert(0) += pair_count;
                });
            }
        }

        return new_poly;
    }
}

fn run_test(data: &Polymerization, steps: usize) {
    let mut poly = Polymer::from(&data.polymer);
    for _ in 0..steps {
        poly = poly.do_step(&data.insertion_rules);
    }

    let counted = poly.count_elements();
    let min = counted.values().min().unwrap();
    let max = counted.values().max().unwrap();

    usbwriteln!(" - after {} steps: {} - {} = {}", steps, max, min, max - min);
}
