extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;
use aoc21::utils::tools::parse_with_err;
use itertools::Itertools;
use tuple::Map;

use crate::cubes::*;

pub struct BootupSequence {
    pub commands: Vec<(bool, Cube)>
}

impl aoc21::solutions::ParsedData for BootupSequence {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let commands = in_data.lines().map(|line| {
            let (state, cube) = line.split_once(" ").unwrap();

            let (x, y, z) = cube.split(",").map(|coord| {
                let (start, end) = coord.split_once("=").unwrap().1
                    .split_once("..").unwrap()
                    .map(parse_with_err);

                start..=end
            }).collect_tuple().unwrap();

            (state == "on", Cube { x, y, z })
        }).collect_vec();

        BootupSequence { commands }
    }
}
