extern crate alloc;


use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;


pub struct NavigationSub {
    pub lines: Vec<String>,
    pub evaled_lines: Vec<(Option<Vec<u8>>, Option<u8>)> 
}

impl aoc21::solutions::ParsedData for NavigationSub {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        return NavigationSub {
            lines: in_data.lines().map(|s: &str| String::from(s)).collect(),
            evaled_lines: Vec::new()
        };
    }
}
