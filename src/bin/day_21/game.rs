use aoc21::usbwriteln;
use core::fmt::Write;

use crate::container::DiracDice;

#[derive(Debug)]
pub enum Dice {
    Deterministic
}

#[derive(Debug)]
pub struct DiracGame {
    pub position: DiracDice,
    pub scores: [u16; 2],

    pub dice_rolls: u16,
    pub dice: Dice
}

impl DiracGame {
    pub fn new(start: DiracDice, dice: Dice) -> Self {
        DiracGame { dice, position: start, scores: [0; 2], dice_rolls: 0 }
    }

    pub fn get_winner_a(mut self, max_score: u16) -> Self {
        let mut cur_player = 0;
        while self.scores.iter().all(|&v| v < max_score) {
            let moves = self.dice.roll_three(&mut self.dice_rolls);

            let position = (self.position.player_pos[cur_player] + moves - 1) % 10 + 1;
            self.position.player_pos[cur_player] = position;
            self.scores[cur_player] += position;

            cur_player = (cur_player + 1) % self.scores.len()
        }

        self
    }
}

impl Dice {
    pub fn roll_three(&self, already_rolls: &mut u16) -> u16 {
        let v = match self {
            Dice::Deterministic => (*already_rolls..(*already_rolls + 3))
                .map(|v| v % 100 + 1).sum()
        };

        *already_rolls += 3;
        return v;
    }
}
