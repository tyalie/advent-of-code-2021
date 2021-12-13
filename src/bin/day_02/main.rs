#![no_std]
#![no_main]

mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::utils::Hardware;
use aoc21::usbwriteln;

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
    fn part_a(&self, _: &mut Hardware, data: &mut Course) {
        let (mut pos, mut depth) = (0i32, 0i32);

        for cmd in &data.commands {
            match cmd {
                Command::Forward(n) => pos += *n as i32,
                Command::Down(n) => depth += *n as i32,
                Command::Up(n) => depth -= *n as i32,
                _ => {}
            };
        }

        usbwriteln!("Final num is {}", pos * depth);
    }

    fn part_b(&self, _: &mut Hardware, data: &mut Course) {
        let (mut pos, mut depth, mut aim) = (0i64, 0i64, 0i64);

        for cmd in &data.commands {
            match cmd {
                Command::Down(n) => aim += *n as i64,
                Command::Up(n) => aim -= *n as i64,
                Command::Forward(n) => {
                    pos += *n as i64;
                    depth += aim * *n as i64
                },
                _ => {}
            };
        }

        usbwriteln!("Final num is {}", pos * depth);
    }
}
