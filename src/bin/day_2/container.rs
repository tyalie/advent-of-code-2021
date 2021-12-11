extern crate alloc;

use core::fmt::Write;

use alloc::vec::Vec;
use alloc::string::String;

use aoc21::utils::Hardware;
use aoc21::utils::tools::parse_with_err;
use aoc21::usbwriteln;

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
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let commands = in_data.lines()
            .map(line_to_command).collect();

        return Course { commands: commands };
    }
}

fn line_to_command(v: &str) -> Command {
    let result = if let Some((cmd, num)) = v.split_once(' ') {
        match cmd {
            "forward" => Command::Forward(parse_with_err(num)),
            "down" => Command::Down(parse_with_err(num)),
            "up" => Command::Up(parse_with_err(num)),
            _ => Command::NOOP
        }
    } else {
        Command::NOOP
    };

    if result == Command::NOOP {
        usbwriteln!("ERR parsing {}", v);
    }
    return result;
}
