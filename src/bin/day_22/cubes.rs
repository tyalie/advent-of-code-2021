extern crate alloc;

use core::cmp::{min, max};
use core::fmt::{Write, Debug};
use alloc::vec::Vec;
use itertools::{Itertools};

use aoc21::usbwriteln;
use tuple::TupleElements;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cube {
    pub x: Range,
    pub y: Range,
    pub z: Range,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub start: i32,
    pub end: i32
}

impl Cube {
    pub fn new(
        x: core::ops::Range<i32>, y: core::ops::Range<i32>, z: core::ops::Range<i32>
    ) -> Self {
        Cube { x: x.into(), y: y.into(), z: z.into() }
    }

    /** # Intersecting cubes
     * This is the main logic of this code, where all 
     * the magic happens. Given two cubes, this function will
     * intersect them and return a list of new cubes derived 
     * from that action.
     */
    pub fn intersect(&self, other: &Cube) -> Option<Cube> {
        let xs = max(self.x.start, other.x.start);
        let xe = min(self.x.end, other.x.end);
        let ys = max(self.y.start, other.y.start);
        let ye = min(self.y.end, other.y.end);
        let zs = max(self.z.start, other.z.start);
        let ze = min(self.z.end, other.z.end);

        let c = Cube::new(xs..xe, ys..ye, zs..ze);

        if !c.is_empty() { Some(c) } else { None }
   }

    pub fn area(&self) -> u64 {
        self.x.len() as u64 * self.y.len() as u64 * self.z.len() as u64
    }

    pub fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }

    pub fn restrict(&self, range: core::ops::Range<i32>) -> Option<Self> {
        let c = Cube::new(
            max(range.start, self.x.start)..min(range.end, self.x.end),
            max(range.start, self.y.start)..min(range.end, self.y.end),
            max(range.start, self.z.start)..min(range.end, self.z.end),
        );
        if c.is_empty() { None } else { Some(c) }
    }
}

impl Range {
    pub fn new(start: i32, end: i32) -> Self {
        Range { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }

    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }
}

impl From<core::ops::Range<i32>> for Range {
    fn from(o: core::ops::Range<i32>) -> Self {
        Range::new(o.start, o.end)
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
