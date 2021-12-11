extern crate alloc;

use core::fmt::Write;

use alloc::vec::Vec;
use alloc::string::String;

use aoc21::utils::Hardware;
use aoc21::utils::tools::parse_with_err;


#[derive(PartialEq, Debug)]
pub enum Command {
    Forward(i8),
    Up(i8),
    Down(i8),
    NOOP
}


pub struct Course {
    pub commands: Vec<Command>
}

impl aoc21::solutions::ParsedData for Course {
    fn parse_file(hardware: &mut Hardware, in_data: String) -> Self {
        let commands = in_data.lines()
            .map(|v: &str| line_to_command(hardware, v)).collect();

        return Course { commands: commands };
    }
}

fn line_to_command(hardware: &mut Hardware, v: &str) -> Command {
    let result = if let Some((cmd, num)) = v.split_once(' ') {
        match cmd {
            "forward" => Command::Forward(parse_with_err(hardware, num)),
            "down" => Command::Down(parse_with_err(hardware, num)),
            "up" => Command::Up(parse_with_err(hardware, num)),
            _ => Command::NOOP
        }
    } else {
        Command::NOOP
    };

    if result == Command::NOOP {
        writeln!(hardware.writer, "ERR parsing {}", v).unwrap();
    }
    return result;
}
