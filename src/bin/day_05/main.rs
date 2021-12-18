#![no_std]
#![no_main]

extern crate alloc;

mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use core::fmt::Write;

use aoc21::{utils::Hardware, usbwriteln};
use aoc21::runtime::ALLOCATOR;

use container::*;


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution { };
    aoc21::runtime::run(&mut sol)
}

struct Solution {
}

impl aoc21::solutions::Solution<Vents> for Solution {
    fn part_a(&self, hardware: &mut Hardware, data: &mut Vents) {
        let lines: Vec<Line> = data.lines.iter().filter(|l| l.is_straight()).cloned().collect();
        let (intersections, hits) = self.determine_intersections(hardware, &lines);
        usbwriteln!("\n- Found {} positions with multiple lines across ({} hits)", intersections, hits);
    }

    fn part_b(&self, hardware: &mut Hardware, data: &mut Vents) {
        let (intersections, hits) = self.determine_intersections(hardware, &data.lines);
        usbwriteln!("\n- Found {} positions with multiple lines across ({} hits)", intersections, hits);
    }
}

impl Solution {
    fn determine_intersections(&self, hardware: &mut Hardware, lines: &Vec<Line>) -> (usize, u32) {
        let num = lines.iter().flat_map(|l| l.into_iter()).count();

        usbwriteln!("Memory: free: {} | used: {}", ALLOCATOR.free(), ALLOCATOR.used());
        usbwriteln!("Iterating over {} points from {} lines", num, lines.len());

        let mut count = 0;
        let mut locations: BTreeSet<Point<u16>> = BTreeSet::new();
        let steps = core::cmp::max(lines.len() / 20, 1);

        for (idx, line) in lines[..lines.len() - 1].iter().enumerate() {
            hardware.led.toggle();
            if idx % steps == 0 {
                usbwriteln!(" - {} / {} lines ({})", idx, lines.len(), locations.len());
            }

            for point_a in line.into_iter() {
                for point_b in lines[(idx+1)..].iter().flat_map(|l| l.into_iter()) {
                    if point_a == point_b {
                        if !locations.contains(&point_a) {
                            locations.insert(point_a);
                        }
                        count += 1
                    }
                }
            }
        }

        return (locations.len(), count);

    }
}


