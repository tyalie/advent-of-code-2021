#![no_std]
#![no_main]

mod container;

use aoc21::usbwriteln;
use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::utils::Hardware;
use aoc21::runtime::Memory;

use container::*;


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol, Memory::RAM1(300_000));
}

/** # Plan of attack
 * so I need to see whether for each pair of sensors
 * there's a matching pattern of 12 beacons.
 *
 * So I'll iterate over all sensor (#12) pairs 
 * (351 combinations) and determine whether there's
 * a match for that.
 *
 * How do I determine the match? Each beacon can be in
 * 24 positions
 */
struct Solution {}

impl aoc21::solutions::Solution<BeaconField> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut BeaconField) {
        usbwriteln!("{:?}", data);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut BeaconField) {
    }
}

