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
        let score = calc_version_sum(&data.root);
        usbwriteln!("- BITS transmission has version sum {}", score);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut Transmission) {
        let evaluated = evaluate_expression(&data.root);
        usbwriteln!("- BITS transmission calcualtes to {}", evaluated);
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

fn evaluate_expression(package: &Package) -> u128 {
    match package {
        Package::Literal { value, version: _} => *value as u128,
        Package::Operator { version: _, type_id, subpackages } => {
            let mut evaled_subs = subpackages.iter().map(|p| evaluate_expression(p));
            match type_id {
                0 => Some(evaled_subs.sum()),
                1 => Some(evaled_subs.product()),
                2 => evaled_subs.min(),
                3 => evaled_subs.max(),
                5 => Some(evaled_subs.next().gt(&evaled_subs.next()) as u128),
                6 => Some(evaled_subs.next().lt(&evaled_subs.next()) as u128),
                7 => Some(evaled_subs.next().eq(&evaled_subs.next()) as u128),
                v => panic!("Unknown type_id {}", v)
            }.unwrap()
        },
    }
}
