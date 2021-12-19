#![no_std]
#![no_main]

extern crate alloc;

mod container;
mod beaconmap;
mod point;
mod math;

use aoc21::usbwriteln;
use itertools::Itertools;
use teensy4_panic as _;
use cortex_m_rt::entry;
use core::fmt::Write;

use alloc::vec::Vec;
use alloc::vec;
use alloc::collections::BTreeSet;
use tuple::TupleElements;

use aoc21::utils::Hardware;
use aoc21::runtime::Memory;

use container::*;
use beaconmap::*;
use point::*;
use math::*;

#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    let mut sol = Solution {};
    aoc21::runtime::run(&mut sol, Memory::RAM1(300_000));
}

/** # Plan of attack
 * so I need to see whether for each pair of sensors
 * there's a matching pattern of 12 beacons.
 *
 * So I'll iterate over all sensor (#12) pairs 
 * (351 combinations) and determine whether there's
 * a match for that.
 *
 * How do I determine the match? Each beacon can be in
 * 24 positions
 */
struct Solution {}

impl aoc21::solutions::Solution<BeaconField> for Solution {
    fn part_a(&self, _: &mut Hardware, data: &mut BeaconField) {
        let (points,sensor_pos) = solve_task(data);
        data.sensor_positions = Some(sensor_pos);

        usbwriteln!(" - found {} points", points.len());
    }
    fn part_b(&self, _: &mut Hardware, data: &mut BeaconField) {
        let max_val = data.sensor_positions.as_ref().unwrap()
            .iter().combinations(2)
            .map(|comb| manhatten_distance(comb[0], comb[1]))
            .max().unwrap();
        usbwriteln!(" - max distance is {}", max_val);
    }
}


fn solve_task(data: &BeaconField) -> (BTreeSet<Point>, Vec<Point>) {
    let mut trans_sens: Vec<Sensor> = data.sensors.clone();

    let mut diffs: Vec<Option<Vector>> = vec![None; data.sensors.len()];
    diffs[0] = Some(Vector { x: 0, y: 0, z: 0 });

    let mappings = data.sensors.iter().map(|s| s.calc_distmap()).collect_vec();

    while diffs.iter().any(|v| v.is_none()) {
'inner: 
       for combis in mappings.iter().enumerate().combinations(2) {
            let (s1, s2) = (combis[0].0, combis[1].0);
            if (diffs[s1].is_none() && diffs[s2].is_none()) 
                || (diffs[s1].is_some() && diffs[s2].is_some()) {
                continue'inner;
            }

            let ((s1, (m1, d1)), (s2, (m2, d2))) = if diffs[s2].is_some() {
                (combis[1], combis[0])
            } else { (combis[0], combis[1]) };

            let intersections = d1.intersection(d2).collect_vec();
            if intersections.len() >= count_combinations(12, 2) as usize {
                let pairs = 
                    intersections.iter().flat_map(|d| {
                        (0..1).cartesian_product(0..1)
                            .map(move |(i1,i2)| {
                                (*m1[d].get(i1).unwrap(), *m2[d].get(i2).unwrap())
                            })
                    }).collect_vec();

                let fitted = fit(pairs, &trans_sens[s1], &trans_sens[s2]);

                if let Some((diff, rot)) = fitted {
                    diffs[s2] = Some(diff);
                    trans_sens[s2].beacons.iter_mut()
                        .for_each(|p| *p = p.rotate(rot) + diff);
                }
            }
        }
    }

    let mut points = BTreeSet::new();
    trans_sens.iter().flat_map(|sensor| sensor.beacons.iter())
        .for_each(|p| { points.insert(*p); });
    return (points, diffs.iter().map(|v| v.unwrap().to_point()).collect_vec());
}

fn fit(point_pairs: Vec<(usize, usize)>, sensor_1: &Sensor, sensor_2: &Sensor) -> Option<(Vector, (i8, i8, i8))> {
    for rot in Point::rotation_iter() {
        for (point_idx_a, point_idx_b) in &point_pairs {
            let (point_a, point_b) = (sensor_1.beacons[*point_idx_a], sensor_2.beacons[*point_idx_b]);
            let diff = point_a - point_b.rotate(rot);
            let mut trans_points = BTreeSet::new();
            sensor_2.beacons.iter().for_each(|p| { trans_points.insert(p.rotate(rot) + diff); });

            let c = sensor_1.beacons.iter().filter(|p| trans_points.contains(p)).count();
            if c >= 12 {
                return Some((diff, rot));
            }
        }
    }

    None
}
