#![no_std]
#![no_main]

mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::utils::Hardware;
use aoc21::runtime::Memory;
use aoc21::usbwriteln;

use container::*;


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol, Memory::RAM1(400_000));
}

struct Solution {
}

impl aoc21::solutions::Solution<Transmission> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Transmission) {
        let tree = Package::build_tree(data.data.iter());
        let score = calc_version_sum(&tree);
        usbwriteln!("- BITS transmission has version sum {}", score);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut Transmission) {
    }
}

fn calc_version_sum(package: &Package) -> usize {
    match package {
        Package::Literal { version, value: _} => *version as usize,
        Package::Operator { version, type_id: _, subpackages } => {
            *version as usize + subpackages.iter().map(|p| calc_version_sum(p)).sum::<usize>()
        }
    }
}
