extern crate alloc;

use core::fmt::Debug;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;
use aoc21::usbwrite;
use num_traits::{int::PrimInt, Unsigned, ToPrimitive};
use tuple::Map;
use core::fmt::Write;


pub struct Graph<T> where T: PrimInt + Unsigned + ToPrimitive {
    nodes: Vec<String>,
    adjacent_nodes: BTreeMap<T, Vec<T>>
}

impl<T> Graph<T> where T: PrimInt + Unsigned + ToPrimitive + Copy {
    fn insert_adjacent(&mut self, from: &T, to: &T) {
        if let Some(adj) = self.adjacent_nodes.get_mut(&from) {
            adj
        } else {
            self.adjacent_nodes.insert(*from, Vec::new());
            self.adjacent_nodes.get_mut(&from).unwrap()
        }.push(*to);
    }

    pub fn from_edges(edges: Vec<(String, String)>) -> Self {
        let mut graph = Graph::new();
        
        for edge in edges {
            let edge = edge.map_mut(|v| graph.index_or_add(&v));

            graph.insert_adjacent(&edge.0, &edge.1);
            graph.insert_adjacent(&edge.1, &edge.0);
        }

        return graph;
    }

    pub fn new() -> Self {
        Graph { 
            nodes: Vec::new(),
            adjacent_nodes: BTreeMap::new() 
        }
    }

    fn index_or_add(&mut self, node: &String) -> T {
        self.node_to_index(node).unwrap_or_else(|| {
            self.nodes.push(node.clone());
            T::from(self.nodes.len() - 1).expect("Node list to long")
        })
    }

    pub fn node_to_index(&self, node: &String) -> Option<T> {
        T::from(self.nodes.iter().position(|v| v == node)?)
    }
}

impl<T> Debug for Graph<T> where T: PrimInt + Unsigned + ToPrimitive + Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for k in self.adjacent_nodes.keys() {
            write!(f, "{} → ", self.nodes[k.to_usize().unwrap()])?;
            for v in &self.adjacent_nodes[k] {
                write!(f, "{} ", self.nodes[v.to_usize().unwrap()])?;
            }
            writeln!(f)?;
        } 
        Ok(())
    }
}
