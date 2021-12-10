extern crate alloc;

use alloc::string::String;
use super::utils::container::Hardware;

pub trait Solution<T> {
    fn parse_file(&self, hardware: &Hardware, in_data: String) -> T;
    fn part_a(&self, hardware: &Hardware, data: &T);
    fn part_b(&self, hardware: &Hardware, data: &T);
}
