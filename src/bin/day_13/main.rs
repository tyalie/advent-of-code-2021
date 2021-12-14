#![no_std]
#![no_main]

mod container;

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

impl aoc21::solutions::Solution<Code> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Code) {
        let mut paper = data.paper.clone();
        paper.apply_fold(&data.folds[0]);

        let count = paper.get_sorted_dots().count();
        usbwriteln!(" - {} dots are visible", count);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut Code) {
        let mut paper = data.paper.clone();

        for fold in &data.folds {
            paper.apply_fold(&fold);
        }

        usbwriteln!("{}", paper);
    }
}

impl Paper {
    pub fn apply_fold(&mut self, fold: &Fold) {
        match fold {  // check that fold is in the middle and set new size
            Fold::Vertical(e) => assert!(self.size().0 / 2 <= *e, "{:?} vs {}", self.size(), *e),
            Fold::Horizontal(e) => assert!(self.size().1 / 2 <= *e, "{:?} vs {}", self.size(), *e),
        }

        for point in self.dots.iter_mut() {
            match fold {
                Fold::Vertical(v) => if point.x > *v { 
                    *point = Point { x: 2 * v - point.x, y: point.y };
                },
                Fold::Horizontal(v) => if point.y > *v { 
                    *point = Point { x: point.x, y: 2 * v - point.y };
                }
            }
        }
    }
}
