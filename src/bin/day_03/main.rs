#![no_std]
#![no_main]

mod container;

use num_traits::pow;
use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::utils::Hardware;
use aoc21::usbwriteln;

use container::*;



#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol)
}

struct Solution {
}

impl aoc21::solutions::Solution<Diagnostic> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut Diagnostic) {
        let mut gamma_rate = 0u64;
        for i in (0..data.num_length).rev() {
            let c = data.report.iter().filter(|v| *v & (1 << i) != 0).count();
            let mcb = c >= data.report.len() / 2;

            gamma_rate = (gamma_rate << 1) | (mcb as u64);
        }

        let epsilon_rate = (pow(2u64, data.num_length) - 1) ^ gamma_rate;

        usbwriteln!(
            "- e:{} * g:{} = {}", 
            epsilon_rate, gamma_rate, epsilon_rate * gamma_rate
        );
    }

    fn part_b(&self, _: &mut Hardware, data: &mut Diagnostic) {
        let o2_gen = find_value(data, false).expect("Wasn't able to get single O2 value");
        let co2_scrub = find_value(data, true).expect("Wasn't able to get single CO2 value");

        usbwriteln!(
            "- o:{} * co2:{} = {}", 
            o2_gen, co2_scrub, o2_gen as u64 * co2_scrub as u64
        );
    }
}


fn find_value(data: &mut Diagnostic, use_lcb: bool) -> Option<u16> {
    let mut arr = data.report.clone();

    for p in (0..data.num_length).rev() {
        let c = arr.iter().filter(|v| *v & (1 << p) != 0).count();
        let mcb = (c * 2 >= arr.len()) != use_lcb;
        arr.retain(|v| (v >> p) & 1 == mcb as u16);

        if arr.len() == 1 {
            return Some(arr[0]);
        } else if arr.len() == 0 {
            return None;
        }
    }
    return None;
}
