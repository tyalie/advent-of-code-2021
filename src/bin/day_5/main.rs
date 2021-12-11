#![no_std]
#![no_main]

extern crate alloc;

mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Write;

use aoc21::{utils::Hardware, usbwriteln, usbwrite};

use container::*;


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution { };
    aoc21::runtime::run(&mut sol);
}

struct Solution {
}

impl aoc21::solutions::Solution<Vents> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Vents) {
        usbwriteln!("Working with {} lines", data.lines.len());
        // I know that field is that size
        let mut field = vec!(vec!(0u8; 1000); 1000);
        usbwriteln!("Init playing field");

//        let mut field: [[u8; 1000]; 1000] = [[0; 1000]; 1000];
        let mut count = 0;

        for line in data.lines.iter().filter(|l| l.is_straight()) {
            for pos in line.into_iter() {
                let pos_i = Point { x: pos.x as usize, y: pos.y as usize };
                if field[pos_i.x][pos_i.y] < 2 {
                    field[pos_i.x][pos_i.y] += 1;
                }
                if field[pos_i.x][pos_i.y] == 1 {
                    count += 1;
                }
            }
        }

        usbwriteln!("- Found {} positions with multiple lines across", count);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut Vents) {
    }
}
