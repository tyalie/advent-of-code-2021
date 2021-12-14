#![no_std]
#![no_main]

mod container;
mod graph;

use num_traits::ToPrimitive;
use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

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

impl aoc21::solutions::Solution<Caves> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Caves) {
        let num_paths = data.graph.find_simple_paths("start", "end", None);
        usbwriteln!(" - found {} paths through cave", num_paths);
    }

    fn part_b(&self, _: &mut Hardware, data: &mut Caves) {
        let mut num_paths: u32 = data.graph.nodes.iter()
            .filter(|v| v != &"start" && v != &"end" && data.graph.is_node_small(v))
            .map(|v| data.graph.find_simple_paths("start", "end", Some(v)))
            .sum();
        num_paths += data.graph.find_simple_paths("start", "end", None);
        usbwriteln!(" - found {} paths through cave", num_paths);
    }
}
