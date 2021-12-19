extern crate alloc;

use core::fmt::Debug;

use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;
use aoc21::utils::tools::parse_with_err;
use itertools::Itertools;

use crate::beaconmap::*;
use crate::point::*;


pub struct BeaconField {
    pub sensors: Vec<Sensor>,
    pub sensor_positions: Option<Vec<Point>>
}

impl aoc21::solutions::ParsedData for BeaconField {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let mut iter = in_data.lines();
        let mut sensors = Vec::new();
        while let Some(_) = iter.next() {
            let beacons = iter.by_ref().take_while(|v| v.len() > 1)
                .map(|line| {
                    let (x, y, z) = line.split(",")
                        .map(parse_with_err).collect_tuple().unwrap();
                    Point { x, y, z }
                }).collect();
            sensors.push(Sensor { beacons });
        }

        BeaconField { sensors, sensor_positions: None }
    }
}

impl Debug for BeaconField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for sensor in &self.sensors {
            writeln!(f, "{:?}", sensor)?;
        }
        Ok(())
    }
}
