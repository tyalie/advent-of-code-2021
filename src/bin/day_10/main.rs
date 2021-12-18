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
    aoc21::runtime::run(&mut sol)
}

struct Solution {
}

impl aoc21::solutions::Solution<NavigationSub> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut NavigationSub) {
        let eval_lines: Vec<(Option<Vec<u8>>, Option<u8>)> = data.lines.iter()
            .map(|line| get_corrupted_indicator(line))
            .collect();

        let score: u32 = eval_lines.iter()
            .filter_map(|&(_, v)| v)
            .map(|v| {
                match v {
                    b')' => 3,
                    b']' => 57,
                    b'}' => 1197,
                    b'>' => 25137,
                    _ => panic!("Unknown symbol {}", v)
                }
            }).sum();

        data.evaled_lines = eval_lines;

        usbwriteln!(" - final score is {}", score);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut NavigationSub) {
        let mut scores: Vec<u64> = data.evaled_lines.iter()
            .filter_map(|(v, _)| v.as_ref())
            .map(|line| {
                line.iter().rev().map(|v| 
                    match v {
                        b'(' => 1,
                        b'[' => 2,
                        b'{' => 3,
                        b'<' => 4,
                        _ => panic!("Unknown symbol {}", v)
                    }).fold(0u64, |a, v| a * 5 + v)
            }).collect();
        scores.sort();
        assert_eq!(scores.len() % 2, 1, "Number of scored lines shouldn't be even");

        usbwriteln!(" - mid score of all is {}", scores[scores.len() / 2]);
    }
}

fn get_corrupted_indicator(line: &String) -> (Option<Vec<u8>>, Option<u8>) {
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
                return (None, Some(b));
            }
        } else {
            panic!("Unknown character encoutered: {}", b);
        }
    }

    (Some(storage), None)
}

fn get_index<T>(arr: &mut Iter<T>, element: T) -> Option<usize> where T: Eq {
    arr.position(|e| *e == element)
}
