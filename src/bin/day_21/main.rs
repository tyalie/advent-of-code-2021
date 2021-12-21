#![no_std]
#![no_main]

mod container;
mod game;

use aoc21::usbwriteln;
use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use aoc21::utils::Hardware;
use aoc21::runtime::Memory;

use container::*;
use game::*;


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol, Memory::RAM1(300_000));
}

struct Solution {}

impl aoc21::solutions::Solution<DiracDice> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut DiracDice) {
        let game = DiracGame::new(data.clone(), Dice::Deterministic);
        let final_state = game.get_winner_a(1000);
        let score = *final_state.scores.iter().min().unwrap() as u32 
            * final_state.dice_rolls as u32;
        usbwriteln!(" - the final score is: {}", score);
    }
    fn part_b(&self, _: &mut Hardware, data: &mut DiracDice) {
    }
}
