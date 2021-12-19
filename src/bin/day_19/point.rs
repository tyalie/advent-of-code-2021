use core::ops::{Add, Sub};
use core::fmt::Debug;

use itertools::Itertools;
use tuple::{Map, TupleElements};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i16,
    pub y: i16,
    pub z: i16
}

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: i16,
    pub y: i16,
    pub z: i16
}

impl Debug for Point {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:4}, {:4}, {:4})", self.x, self.y, self.z)
    }
}

impl Point {
    pub fn rotate(&self, mapping: (i8, i8, i8)) -> Point {
        let self_tuple = (self.x, self.y, self.z);
        let (x, y, z) = mapping.map(|v| {
            v.signum() as i16 * self_tuple.get(v.abs() as usize - 1).unwrap()
        });
        Point { x, y, z }
    }

    pub fn rotation_iter() -> impl Iterator<Item = (i8, i8, i8)> {
        (1..=3).permutations(3).flat_map(|combi| {
            (0..=7).map(move |n| {
                combi.iter().enumerate()
                    .map(|(i, v2)| ((n >> i & 1) * 2 - 1) * v2)
                    .collect_tuple().unwrap()
            })
        })
    }
}

impl Vector {
    pub fn to_point(&self) -> Point {
        Point { x: self.x, y: self.y, z: self.z }
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Point { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }
    }
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector { 
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }
    }
}
