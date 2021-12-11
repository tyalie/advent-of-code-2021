#![no_std]
#![no_main]

extern crate alloc;
mod container;

use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;
use alloc::vec::Vec;

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

impl aoc21::solutions::Solution<Bingo> for Solution {
    fn part_a(&self, hardware: &mut Hardware, data: &mut Bingo) {
        
        let (idx, score, _) = data.boards.iter().filter_map(|b| evaluate_board(b, &data.draws))
            .min_by_key(|(idx, _, _)| *idx)
            .unwrap_or_else(|| {
                writeln!(hardware.writer, "ERR: No board was finished").unwrap();
                return (0,0,0);
        });
        writeln!(hardware.writer, " - finished board with score {} ({} turns)", score, idx).unwrap();
    }
    fn part_b(&self, hardware: &mut Hardware, data: &mut Bingo) {
    }
}

fn evaluate_board(board: &Board, draws: &Vec<u8>) -> Option<(usize, u16, u8)> {
    let mut selected: [bool; 25] = [false; 25];
    
    for (idx, n) in draws.iter().enumerate() {
        for (idx, _) in board.numbers.iter().flatten().enumerate().filter(|(_, v)| *v == n) {
            selected[idx] = true;
        }
        if is_winning(&selected) {
            let num_sum: u16 = board.numbers.iter().flatten().enumerate()
                .filter(|(idx,_)| !selected[*idx])
                .map(|(_,v)| *v as u16).sum();
            let score= num_sum * (*n as u16);
            return Some((idx, score, *n));
        }
    }
    None
}

fn is_winning(selected: &[bool; 25]) -> bool {
    let row_test = selected.chunks(5).any(|r| r.iter().all(|v| *v));
    let col_test = (0..5).any(|c| (0..5).all(|r| selected[c + 5*r]));
    return row_test || col_test;
}
