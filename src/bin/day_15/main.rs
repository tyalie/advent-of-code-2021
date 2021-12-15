#![no_std]
#![no_main]
#![feature(btree_drain_filter)]
#![feature(binary_heap_retain)]

mod container;
mod a_star;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::{utils::Hardware, usbwriteln, usbwrite};

use container::*;
use a_star::*;


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

impl aoc21::solutions::Solution<Cave> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Cave) {
        let goal = Position { x: data.cols() - 1, y: data.rows() - 1};
        let cost = calc_cost_a_star(data, &Position::from((0,0)), &goal)
            .expect("Couldn't find a path");

        usbwriteln!(" - cost for transversing the graph is: {}", cost);

    }
    fn part_b(&self, _: &mut Hardware, data: &mut Cave) {
        let data = data.expand();

        let goal = Position { x: data.cols() - 1, y: data.rows() - 1};
        let cost = calc_cost_a_star(&data, &Position::from((0,0)), &goal)
            .expect("Couldn't find a path");

        usbwriteln!(" - cost for transversing expanded graph is: {}", cost);
    }
}
