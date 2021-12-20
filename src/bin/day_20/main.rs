#![no_std]
#![no_main]
#![feature(map_first_last)]
#![feature(step_trait)]

extern crate alloc;

mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;
use alloc::collections::BTreeSet;

use aoc21::usbwriteln;
use aoc21::utils::Hardware;
use aoc21::runtime::Memory;
use aoc21::runtime;

use container::*;


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol, Memory::RAM2);
}

struct Solution {}

impl aoc21::solutions::Solution<ImageAlgo> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut ImageAlgo) {
        let score = execute_algorithm(data, 2);
        usbwriteln!("{}", score);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut ImageAlgo) {
        let score = execute_algorithm(data, 50);
        usbwriteln!("{}", score);
    }
}

fn execute_algorithm(data: &ImageAlgo, steps: usize) -> usize {
    let mut image = data.image.clone();

    for _ in 0..steps {
        let mut new_data = BTreeSet::new();

        for point in image.iter_pixels() {
            let value = image.get_box(&point);
            if data.algo[value as usize] {
                new_data.insert(point);
            }
        }

        image = Image::extend(new_data, &image, data.algo[0]);
    }

    assert!(!image.infity_val, "Return is ∞");
    usbwriteln!("[{} bytes memory remaining]", runtime::ALLOCATOR.free());
    image.data.len()
    
}
