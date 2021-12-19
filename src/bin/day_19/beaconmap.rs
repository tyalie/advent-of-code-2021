extern crate alloc;

use alloc::vec::Vec;
use alloc::collections::{BTreeMap, BTreeSet};
use itertools::Itertools;
use core::fmt::Debug;

use crate::point::*;


#[derive(Clone)]
pub struct Sensor {
    pub beacons: Vec<Point>
}

impl Debug for Sensor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "-- Sensor --")?;
        for point in &self.beacons {
            writeln!(f, "{:?}", point)?;
        }
        Ok(())
    }
}

fn absdiff<T>(x: T, y: T) -> T where T: num_traits::PrimInt {
    if x < y { y - x } else { x - y }
}

pub fn manhatten_distance(a: &Point, b: &Point) -> u16 {
    absdiff(a.x, b.x) as u16 + absdiff(a.y, b.y) as u16 + absdiff(a.z, b.z) as u16
}


impl Sensor {
    pub fn calc_distmap<'a>(&'a self) -> (BTreeMap<u16, (usize, usize)>, BTreeSet<u16>) {
        let mut map: BTreeMap<u16, (usize, usize)> = BTreeMap::new();
        let mut set: BTreeSet<u16> = BTreeSet::new();
        for beacons in self.beacons.iter().enumerate().combinations(2) {
            let dist = manhatten_distance(beacons[0].1, beacons[1].1);
            map.insert(dist, (beacons[0].0, beacons[1].0));
            set.insert(dist);
        }
        (map , set)
    }
}

