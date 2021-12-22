use core::ops::RangeInclusive;

#[derive(Debug)]
pub struct Cube {
    pub x: RangeInclusive<i32>,
    pub y: RangeInclusive<i32>,
    pub z: RangeInclusive<i32>,
}
