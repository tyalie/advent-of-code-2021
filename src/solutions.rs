extern crate alloc;

use alloc::string::String;
use super::utils::container::Hardware;

pub trait ParsedData {
    fn parse_file(hardware: &mut Hardware, in_data: String) -> Self;
}

pub trait Solution<T: ParsedData> {
    fn part_a(&self, hardware: &mut Hardware, data: &mut T);
    fn part_b(&self, hardware: &mut Hardware, data: &mut T);
}
