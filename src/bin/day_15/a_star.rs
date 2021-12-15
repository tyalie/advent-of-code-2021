extern crate alloc;

use crate::container::*;
use crate::CostField::*;

use alloc::collections::binary_heap::BinaryHeap;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use aoc21::usbwrite;
use core::fmt::Write;
use aoc21::runtime;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: u16,
    pub y: u16
}

#[derive(PartialEq, Eq)]
pub struct State {
    pub position: Position,
    pub cost: u16,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        /* Implement ordering manually for State 
         * in order to allow binary heap to sort 
         * by an actual minimum instead of maximum */
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Cave {
    fn real_rows(&self) -> u16 {
        self.chitons.len() as u16
    }

    fn real_cols(&self) -> u16 {
        self.chitons[0].len() as u16
    }

    pub fn rows(&self) -> u16 {
        self.real_rows() * if self.expanded { 5 } else { 1 }
    }

    pub fn cols(&self) -> u16 {
        self.real_cols() * if self.expanded { 5 } else { 1 }
    }

    pub fn adjacent(&'_ self, coord: &'_ Position) -> impl Iterator<Item = Position> + '_ {
        const ADJS: [(i32, i32); 4] = [
                      (0, -1),          
            (-1,  0),          (1,  0),
                      (0,  1)
        ];

        let coord = coord.clone();

        ADJS.iter()
            .map(move |(x, y)| (coord.x as i32 + x, coord.y as i32 + y))
            .filter(move |(x, y)| {
                    0 <= *x && *x < self.rows() as i32
                    && 0 <= *y && *y < self.cols() as i32
            }).map(|(x, y)| Position { x: x as u16, y: y as u16 })
            .into_iter()
    }

    pub fn cost(&self, coord: &Position) -> u8 {
        let add_cost = coord.x / self.real_cols() + coord.y / self.real_rows();
        let coord = Position { x: coord.x % self.real_cols(), y: coord.y % self.real_rows() };
        let cost = *coord.retrieve(&self.chitons).unwrap();

        return if self.expanded {
            (cost + add_cost as u8 - 1) % 9 + 1
        } else {
            cost
        }
    }
}

impl From<(u16, u16)> for Position {
    fn from(orig: (u16, u16)) -> Self {
        Position { x: orig.0, y: orig.1 }
    }
}

impl Into<(u16, u16)> for Position {
    fn into(self) -> (u16, u16) {
        return (self.x, self.y)
    }
}

impl Position {
    pub fn retrieve<'a, T>(&self, data: &'a Vec<Vec<T>>) -> Option<&'a T> {
        data.get(self.x as usize)?.get(self.y as usize)
    }

    pub fn retrieve_mut<'a, T>(&self, data: &'a mut Vec<Vec<T>>) -> &'a mut T {
        &mut data[self.x as usize][self.y as usize]
    }
}

fn abs_difference<T>(x: T, y: T) -> T where T: num_traits::PrimInt {
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn taxi_distance(a: &Position, b: &Position) -> u16 {
    abs_difference(a.x, b.x) as u16 + abs_difference(a.y, b.y) as u16
}

fn remove_all_distant_positions<T>(current: &Position, costs: &mut BTreeMap<Position, T>) {
    const CUT_OFF: u16 = 200;
    costs.drain_filter(|k, _| { taxi_distance(current, &k) > CUT_OFF });
}

fn debug_matrix(graph: &Cave, costs: &BTreeMap<Position, u16>) {
    usbwrite!("\n");
    for x in 0..graph.rows() {
        for y in 0..graph.cols() {
            if let Some(v) = costs.get(&Position::from((x, y))) { 
                if *v < u16::MAX {
                    usbwrite!("█");
                } else {
                    usbwrite!(" ");
                }
            } else {
                usbwrite!(" ");
            }
        }
        usbwrite!("\n");
    }
}

/// A* implementation
///
/// # Return
/// Lowest cost of moving through the graph
pub fn calc_cost_a_star(graph: &Cave, start: &Position, goal: &Position) -> Option<u16> {
    usbwrite!("\n{}\n", runtime::ALLOCATOR.free());
    let heuristic = |from: &Position| -> u16 { taxi_distance(from, goal) * 3 };

    let mut costs = CostField::new(&(graph.rows(), graph.cols()), &(300, 300));
    let mut heap = Vec::new();
    
    *costs.get_mut(Into::<(u16, u16)>::into(*start)).unwrap() = 0;
    heap.push(State { cost: heuristic(start), position: *start });

    let mut goaliest_point: (Position, u16) = (*start, taxi_distance(start, goal));
    let mut last_free_mem = runtime::ALLOCATOR.free();

    while let Some((idx, &State { cost, position })) = heap.iter().enumerate().max_by_key(|&(_, v)| v) {
        heap.remove(idx);

        let goal_dist = taxi_distance(&position, goal);
        let raw_cost = cost - heuristic(&position);  // more memory efficient

        if goal_dist < goaliest_point.1 {
            goaliest_point = (position, goal_dist);
//            remove_all_distant_positions(&goaliest_point.0, &mut costs);
            costs.move_field(&Into::<(u16, u16)>::into(position));
          /*  usbwrite!(
                "g_dist = {} | free_mem = {}b | #heap = {}\n", 
                goal_dist, runtime::ALLOCATOR.free(), heap.len()
            );*/
        }

        /*
        if abs_difference(last_free_mem, runtime::ALLOCATOR.free()) > 1000 {
            usbwrite!("- remaining: {}b | #heap: {} \n", runtime::ALLOCATOR.free(), heap.len());
        }*/

        if raw_cost > *costs.get(Into::<(u16, u16)>::into(position)).unwrap_or(&u16::MAX) { continue; }

        if position == *goal { 
           // debug_matrix(graph, &costs);
            return Some(cost) 
        };

        // current cost is better than the previous one
        // get all paths that are adjacent to the current
        // one, calculate their estimated costs and 
        // at the on the heap if better
        for ajds in graph.adjacent(&position) {
            let cost = raw_cost + graph.cost(&ajds) as u16;
            let next = State {
                cost: cost + heuristic(&ajds), position: ajds
            };

            
            if cost < *costs.get(Into::into(next.position)).unwrap_or(&u16::MAX) {
                if let Some(v) = costs.get_mut(Into::into(next.position)) {
                    *v = cost;
                }
                // clean up heap
                heap.retain(|v| v.position != position || v.cost < next.cost);
                heap.shrink_to_fit();
                heap.push(next);
            }
        }
    }
    
    None
}

