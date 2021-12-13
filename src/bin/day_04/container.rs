extern crate alloc;

use core::convert::TryInto;

use alloc::vec::Vec;
use alloc::string::String;

use aoc21::utils::Hardware;
use aoc21::utils::tools::parse_with_err;

#[derive(Clone)]
pub struct Board {
    pub numbers: [[u8; 5]; 5],
}

pub struct Bingo {
    pub draws: Vec<u8>,
    pub boards: Vec<Board>
}

impl aoc21::solutions::ParsedData for Bingo {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let lines: Vec<&str> = in_data.lines().collect();

        // first line are the draws
        let draws: Vec<u8> = lines[0].split(',').map(parse_with_err).collect();

        // parsing boards
        let boards = lines[2..].chunks(6)
            .map(|ls| Board::parse(&ls[..5]) ).collect();

        return Bingo {
            draws: draws,
            boards: boards
        }
    }
}

impl Board {
    fn parse(lines: &[&str]) -> Self {
        let mut board = [[0u8; 5]; 5];

        if lines.len() != 5 {
            panic!("ERR: More than 5 lines provided\n{:?}", lines); 
        }

        for r in 0..5 {
            let row: Vec<u8> = lines[r].split_ascii_whitespace().map(parse_with_err).collect();
            board[r] = row.try_into().unwrap_or_else(|v| { 
                panic!("ERR: Couldn't fit {:?}", v); 
            });
        }

        return Board {
            numbers: board,
        }
    }
}

impl core::fmt::Debug for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for r in 0..5 {
            for c in 0..5 {
                write!(f, "{:2} ", self.numbers[r][c])?;
            }
            write!(f, "\n")?;
        }
        core::fmt::Result::Ok(())
    }
}
