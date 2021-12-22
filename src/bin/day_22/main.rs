#![no_std]
#![no_main]

mod container;
mod cubes;

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

/** # Idea
 * Using any kind of __store stuff in memory__ would
 * kill my RAM. Even task 1 would need a highly 
 * efficient bitvector storage, that would consume 
 * about 125kb RAM (or ¼ of it). Looking at my input
 * this is very very far from enough for task b.
 * 
 * Soooo? What's the solution?
 * 
 * Well instead of storing each and every cuboid that
 * will be turned on or off, I'll store a cube with 
 * maybe thousands or more cuboids in it which will 
 * help me will represent that.
 *
 * ## Generating the cubes
 * Here comes the tricky part. Let's say we've two cubes 
 * that describe the on/off state. The first represents on,
 * the second has the commando to turn them off.
 *
 * Soo? Intersecting them and splitting the result into
 * cubes would bring me in a fairly essy way 26 new cubes
 * (looking at rubics cube here).
 *
 * Which isn't really ideal either as you can do it with
 * 6 cubes at most. Although now is the question wether that
 * would need special cases in order to do that.
 *
 * So I'll start with the 26 new cubes (at most if Y is 
 * completely contained by X) and go on from there if I
 * hit a memory limit.
 */
struct Solution {}

impl aoc21::solutions::Solution<BootupSequence> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut BootupSequence) {
        for (on, cube) in &data.commands {
            usbwriteln!("{}: {:?}", on, cube);
        }
    }
    fn part_b(&self, _: &mut Hardware, data: &mut BootupSequence) {
    }
}
