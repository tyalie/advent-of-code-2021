extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;
use aoc21::utils::tools::parse_with_err;
use tuple::Map;


#[derive(Debug, Clone)]
pub struct DiracDice {
    pub player_pos: [u16; 2],
}

impl aoc21::solutions::ParsedData for DiracDice {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let (p1, p2) = in_data.trim().split_once("\n").unwrap()
            .map(|line| line.split_once(": ").unwrap().1)
            .map(parse_with_err);

        DiracDice { player_pos: [p1, p2] }
    }
}
