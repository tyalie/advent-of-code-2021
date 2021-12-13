#![no_std]
#![no_main]

extern crate alloc;
mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::{fmt::Write, slice::Iter};
use alloc::vec::Vec;
use alloc::string::String;

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

impl aoc21::solutions::Solution<NavigationSub> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut NavigationSub) {
        let score: u32 = data.lines.iter()
            .filter_map(get_corrupted_indicator)
            .map(|v| {
                match v {
                    b')' => 3,
                    b']' => 57,
                    b'}' => 1197,
                    b'>' => 25137,
                    _ => panic!("Unknown symbol {}", v)
                }
            }).sum();

        usbwriteln!(" - final score is {}", score);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut NavigationSub) {
    }
}

fn get_corrupted_indicator(line: &String) -> Option<u8> {
    let mut storage: Vec<u8> = Vec::with_capacity(line.len());
    let (opening, closing) = (b"([{<", b")]}>");

    for b in line.bytes() {
        if opening.contains(&b) {
            storage.push(b);
        } else if closing.contains(&b) {
            if get_index(&mut opening.iter(), storage[storage.len()-1]) 
                == get_index(&mut closing.iter(), b) {
                storage.pop();
            } else {
                return Some(b);
            }
        } else {
            panic!("Unknown character encoutered: {}", b);
        }
    }

    None
}

fn get_index<T>(arr: &mut Iter<T>, element: T) -> Option<usize> where T: Eq {
    arr.position(|e| *e == element)
}
