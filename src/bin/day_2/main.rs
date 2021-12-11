#![no_std]
#![no_main]

mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::utils::Hardware;

use container::{Course, Command};


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

impl aoc21::solutions::Solution<Course> for Solution {
    fn part_a(&self, hardware: &mut Hardware, data: &Course) {
        let (mut pos, mut depth) = (0i32, 0i32);

        for cmd in &data.commands {
            match cmd {
                Command::Forward(n) => pos += n.clone() as i32,
                Command::Down(n) => depth += n.clone() as i32,
                Command::Up(n) => depth -= n.clone() as i32,
                _ => {}
            };
        }

        writeln!(hardware.writer, "Final num is {}", pos * depth).unwrap();
    }

    fn part_b(&self, hardware: &mut Hardware, data: &Course) {
    }
}
