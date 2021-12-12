#![no_std]
#![no_main]

extern crate alloc;
mod container;

use itertools::Itertools;
use teensy4_panic as _;
use cortex_m_rt::entry;
use core::{fmt::Write, convert::TryFrom};

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
        let mut output = 0u32;

        for display in &data.displays {
            let valid_perm = (0..7).permutations(7)
                .map(|perm| <[u8; 7]>::try_from(perm.as_slice()).expect("Conversion failed"))
                .filter(|perm| {
                    display.combinations.iter()
                        .filter(|pattern| {
                            do_permute(pattern, &perm).to_num().is_some()
                        }).count() == 10
                })
                .next().expect("Couldn't find a solution");

            output += display.number.iter()
                .map(|pattern| do_permute(pattern, &valid_perm))
                .map(|digit| digit.to_num().unwrap())
                .fold(0u32, |acc, v| acc * 10 + (v as u32));
        }

        usbwriteln!(" - all numbers summed result in {}", output);
    }
}

pub fn do_permute(digit: &Digit, permutation: &[u8; 7]) -> Digit {
    let mut new_digit = Digit { on_segments: [false; 7] };

    for (from, &to) in permutation.iter().enumerate() {
        new_digit.on_segments[from] = digit.on_segments[to as usize];
    }
    return new_digit;
}
