#![no_std]
#![no_main]

mod container;

use num_traits::pow;
use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::utils::Hardware;

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

impl aoc21::solutions::Solution<Diagnostic> for Solution {
    fn part_a(&self, hardware: &mut Hardware, data: &Diagnostic) {
        let mut gamma_rate = 0u64;
        for i in 0..data.num_length {
            let c = data.report.iter().filter(|v| v.clone() & (1 << i) != 0).count();
            let mcb = c >= data.report.len() / 2;

            gamma_rate = (gamma_rate << 1) | (mcb as u64);
        }

        let epsilon_rate = (pow(2u64, data.num_length) - 1) ^ gamma_rate;

        writeln!(
            hardware.writer, "- e:{} * g:{} = {}", 
            epsilon_rate, gamma_rate, epsilon_rate * gamma_rate
        ).unwrap();
    }

    fn part_b(&self, hardware: &mut Hardware, data: &Diagnostic) {
    }
}
