#![no_std]
#![no_main]

mod container;
mod game;

use aoc21::{usbwriteln, usbwrite};
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
    aoc21::runtime::run(&mut sol, Memory::RAM2);
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
        let game = DiracGame::new(data.clone(), Dice::Deterministic);
        let res = game.get_paths_brut_force();
        usbwriteln!("{:?}", res);
    }
}

#[allow(dead_code)]
fn generate_solutions_map(score: u64) {
    let mut win_map = [[0u64; 10]; 10];

    for pos_a in 1..=10 {
        for pos_b in 1..=10 {
            let game = DiracGame::new(
                DiracDice { player_pos: [pos_a, pos_b] }, 
                Dice::Quantum
            );

            win_map[pos_a as usize - 1][pos_b as usize - 1] = 
                *game.get_paths_cached(score).iter().max().unwrap();
        }
    }


    for pos_a in 0..=10 {
        usbwrite!("{:2}: ", pos_a);
        for pos_b in 1..=10 {
            let value = if pos_a == 0 { pos_b as u64 }
                else { win_map[pos_a - 1][pos_b - 1] };

            usbwrite!("{:6}, ", value);
        }
        usbwriteln!("");
    }
    usbwriteln!("");
}
