extern crate alloc;

use alloc::collections::BTreeMap;
use aoc21::{usbwriteln, usbwrite};
use core::fmt::Write;

use crate::container::DiracDice;

#[derive(Debug, PartialEq, Eq)]
pub enum Dice {
    Deterministic,
    Quantum
}

#[derive(Debug)]
pub struct DiracGame {
    pub position: DiracDice,
    pub scores: [u16; 2],

    pub dice_rolls: u16,
    pub dice: Dice
}

impl Dice {
    pub fn roll_three(&self, already_rolls: &mut u16) -> u16 {
        let v = match self {
            Dice::Deterministic => (*already_rolls..(*already_rolls + 3))
                .map(|v| v % 100 + 1).sum(),
            Dice::Quantum => panic!("Not implemented")
        };

        *already_rolls += 3;
        return v;
    }
}

impl DiracGame {
    const DICE_PATHS: [u8; 7] = [  // every access to it should be with offset 3
        1,  //3: 111
        3,  //4: 112 121 211
        6,  //5: 113 131 311 | 122 212 221 | 
        7,  //6: 123 132, 213 231, 312, 321 | 222
        6,  //7: 133 313 331 | 223 232 322
        3,  //8: 332 323 233
        1   //9: 333
    ];

    pub fn new(start: DiracDice, dice: Dice) -> Self {
        DiracGame { dice, position: start, scores: [0; 2], dice_rolls: 0 }
    }

    pub fn get_winner_a(mut self, max_score: u16) -> Self {
        assert!(self.dice == Dice::Deterministic);

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

    pub fn get_paths_brut_force(&self) -> [u64; 2] {
        let (mut win_a, mut win_b) = (0u64, 0u64);
        Self::brut_force(
            self.position.player_pos[0] as u8, 0, &mut win_a,
            self.position.player_pos[1] as u8, 0, &mut win_b,
            1
        );
        return [win_a, win_b];
    }

    pub fn brut_force(
        pos_a: u8, score_a: u8, win_a: &mut u64,
        pos_b: u8, score_b: u8, win_b: &mut u64,
        score_mult: u64
    ) {
        if score_b >= 21 {
            *win_b += score_mult;
        } else {
            for dice_roll in 3..=9 {
                let new_pos_a = (pos_a + dice_roll - 1) % 10 + 1;
                Self::brut_force(
                    pos_b, score_b, win_b, 
                    new_pos_a, score_a + new_pos_a, win_a,
                    score_mult * Self::DICE_PATHS[dice_roll as usize - 3] as u64
                );
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_paths_cached(&self, max_score: u64) -> [u64; 2] {
        let mut cache = BTreeMap::new();

        return Self::roll_dice_cached(
            self.position.player_pos[0] as u8, 
            self.position.player_pos[1] as u8,
            0, 0, &mut cache, max_score
        );
    }

    fn roll_dice_cached(
        pos_a: u8, pos_b: u8, score_a: u64, score_b: u64, 
        cache: &mut BTreeMap<(u8, u8, u64, u64), [u64; 2]>, max_score: u64
    ) -> [u64; 2] {
        let input = (pos_a, pos_b, score_a, score_b);
        if let Some(result) = cache.get(&input) {
            return *result;
        }

        if score_a >= max_score {
            return [1, 0];
        } else if score_b >= max_score {
            return [0, 1];
        }

        let mut score = [0u64; 2];

        for dice_roll in 3..=9 {
            let new_pos_a = (pos_a + dice_roll - 1) % 10 + 1;
            let new_score_a = score_a + new_pos_a as u64;
            let sub_score = Self::roll_dice_cached(
                pos_b, new_pos_a, score_b, new_score_a, cache, max_score
            );

            score.iter_mut().enumerate().for_each(|(idx, v)| {
                *v += Self::DICE_PATHS[dice_roll as usize - 3] as u64 * sub_score[1 - idx];
            });
        }

        cache.insert(input, score);
        return score;
    }
    
    #[allow(dead_code)]
    pub fn get_win_count_combinatoric(self, winning_score: usize) -> [u64; 2] {
        assert!(self.dice == Dice::Quantum, "Wrong dice");
        assert!(winning_score <= 21, "Maximum score cannot be larger than 21");

        // this is max_score * positions * max_moves
        let mut path_map = [[[u64::MAX; 33]; 10]; 11];
        let (max_moves, max_score) = (path_map.len(), path_map[0][0].len());

        for score in 1..=max_score {
            for position in 1..=10 {
                for moves in 1..=max_moves {

                    let mut new_value = 0;
                    for move_by in 3..=9 {

                        let n_pos = (position + move_by - 1) % 10 + 1;

                        if score == n_pos && moves == 1 {  // perfect hit
                            new_value += Self::DICE_PATHS[move_by - 3] as u64;
                        } else if n_pos < score && moves >= 2 {  // evaluate from rest
                            if score - n_pos < winning_score {  // if 21, we've already won, so don't count those
                                let value = path_map[moves - 2][n_pos - 1][score - n_pos - 1];
                                assert!(value != u64::MAX, "using path that wasn't evaluated yet");

                                new_value += Self::DICE_PATHS[move_by - 3] as u64 * value;
                            }
                        }
                    }

                    path_map[moves - 1][position - 1][score - 1] = new_value;
                }
            }
        }

        // for start position player A * player B and whether player A has the first move
        let mut win_map = [[[u64::MAX; 10]; 10]; 2];

        for is_starting in 0..=1 {
            for pos_a in 1..=10 {
                for pos_b in 1..=10 {
                    let mut tmp_value = 0;

                    for moves in 1..=max_moves {
                        let mut games_lost = 0;
                        for score_b in 1..winning_score {
                            if is_starting == 1 && moves > 1 {
                                // if I'm starting, then player B is still on last move
                                games_lost += path_map[moves - 2][pos_b - 1][score_b - 1];
                            } else if is_starting == 0 {
                                games_lost += path_map[moves - 1][pos_b - 1][score_b - 1];
                            }
                        }

                        for score_a in winning_score..=max_score {
                            let value = path_map[moves - 1][pos_a - 1][score_a - 1];

                            // verify that map is complete
                            assert!(score_a != max_score || value == 0, "Map isn't large enough - score");
                            assert!(moves != max_moves || value == 0, "Map isn't large enough - moves");

                            tmp_value += games_lost * value;
                        }
                    }

                    win_map[is_starting][pos_a - 1][pos_b - 1] = tmp_value;
                }
            }
        }

        /*for m in 1..=6 {
            usbwriteln!("--- move {}", m);
            for p in 1..=10 {
                usbwriteln!("{:2}: {:?}", p, path_map[m - 1][p - 1]);
            }
        }*/

        for pos_a in 0..=10 {
            usbwrite!("{:2}: ", pos_a);
            for pos_b in 1..=10 {
                let value = if pos_a == 0 { pos_b as u64 }
                    else { win_map[0][pos_a - 1][pos_b - 1] };

                usbwrite!("{:6}, ", value);
            }
            usbwriteln!("");
        }

        unimplemented!("Doesn't work");
    }
}

