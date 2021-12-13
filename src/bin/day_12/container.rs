extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;
use tuple::Map;

use crate::graph::Graph;


pub struct Caves {
    pub graph: Graph<u8>
}

impl aoc21::solutions::ParsedData for Caves {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let edges: Vec<(String, String)> = in_data.lines()
            .map(|line| {
                line.split_once("-").expect("Malformated input!").map(|v| String::from(v))
            }).collect();

        return Caves {
            graph: Graph::from_edges(edges)
        };
    }
}
