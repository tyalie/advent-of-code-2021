extern crate alloc;

use crate::container::*;
use crate::cost_field::*;

use alloc::collections::binary_heap::BinaryHeap;
use alloc::vec::Vec;
use core::fmt::Write;
use aoc21::runtime;
use aoc21::usbwrite;
use aoc21::utils::Hardware;


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
}

impl core::ops::Index<Position> for CostField {
    type Output = u16;
    fn index(&self, index: Position) -> &Self::Output {
        self.get(index.x, index.y)
    }
}

impl core::ops::IndexMut<Position> for CostField {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        self.get_mut(index.x, index.y)
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

#[allow(dead_code)]
fn debug_matrix(graph: &Cave, costs: &CostField) {
    usbwrite!("GRAPH_START\n");
    for y in 0..graph.rows() {
        for x in 0..graph.cols() {
            usbwrite!("{},", costs.get(x, y));
        }
        usbwrite!("\n");
    }
    usbwrite!("GRAPH_END\n");
}

/// A* implementation
///
/// # Return
/// Lowest cost of moving through the graph
pub fn calc_cost_a_star(hwd: &mut Hardware, graph: &Cave, start: &Position, goal: &Position) -> Option<u16> {
    usbwrite!("{}\n", runtime::ALLOCATOR.free());
    let heuristic = |from: &Position| -> u16 { taxi_distance(from, goal) };

    let mut costs = CostField::new(&(graph.rows(), graph.cols()));
    let mut heap = BinaryHeap::new();
  
    costs[*start] = 0;
    heap.push(State { cost: heuristic(start), position: *start });

    let mut counter = 0u64;

    while let Some(State { cost, position }) = heap.pop() {
        counter += 1;

        if counter % 1000 == 0 {
            hwd.led.toggle();
        }

        let raw_cost = cost - heuristic(&position);  // more memory efficient

        if raw_cost > costs[position]{ continue; }

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

           
            if cost < costs[next.position] {
                costs[next.position] = cost;

                // clean up heap
                heap.retain(|v| v.position != position || v.cost <= next.cost);
                heap.push(next);
            }
        }
    }
    
    None
}

