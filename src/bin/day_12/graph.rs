extern crate alloc;

use core::fmt::Debug;
use alloc::{collections::BTreeMap, vec};
use alloc::vec::Vec;
use alloc::string::String;
use num_traits::{int::PrimInt, Unsigned, ToPrimitive};
use tuple::Map;


pub struct Graph<T> where T: PrimInt + Unsigned + ToPrimitive {
    pub nodes: Vec<String>,
    is_small: Vec<bool>,
    adjacent_nodes: BTreeMap<T, Vec<T>>
}

impl<T> Graph<T> where T: PrimInt + Unsigned + ToPrimitive + Copy + Debug {
    fn insert_adjacent(&mut self, from: &T, to: &T) {
        self.adjacent_nodes
            .entry(*from)
            .or_insert(Vec::new())
            .push(*to);
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
            is_small: Vec::new(),
            adjacent_nodes: BTreeMap::new() 
        }
    }

    fn index_or_add(&mut self, node: &String) -> T {
        self.node_to_index(node).unwrap_or_else(|| {
            self.nodes.push(node.clone());
            self.is_small.push(node.bytes().all(|b| b'a' <= b && b <= b'z'));
            T::from(self.nodes.len() - 1).expect("Node list to long")
        })
    }

    pub fn node_to_index(&self, node: &str) -> Option<T> {
        T::from(self.nodes.iter().position(|v| v == node)?)
    }

    pub fn is_node_small(&self, node: &str) -> bool {
        self.is_small[self.node_to_index(node).unwrap().to_usize().unwrap()]
    }

    fn neighbors(&self, node: &T) -> impl Iterator<Item = &T> {
        self.adjacent_nodes.get(node)
            .map(|neighs| neighs as &[T])
            .unwrap_or(&[])
            .into_iter()
    }

    /// # find all simple paths algorithm
    /// Adopted from https://www.baeldung.com/cs/simple-paths-between-two-vertices
    pub fn find_simple_paths(&self, start: &str, end: &str, visiting_twice: Option<&str>) -> u32 {
        let start = self.node_to_index(start).expect("Invalid start");
        let end = self.node_to_index(end).expect("Invalid stop");
        let vis_2: Option<T> = visiting_twice
            .map(|v| self.node_to_index(v).expect("Invalid visit twice"));

        let mut visited = vec!(0u8; self.nodes.len());
        let mut children_stack = vec!((start, self.neighbors(&start)));
        let mut num_paths = 0;

        visited[start.to_usize().unwrap()] = 1;

        while let Some((root, children)) = children_stack.last_mut() {
            assert!(visited.iter().filter(|v| **v == 2).count() <= 1, "More than one node with two visits");

            if let Some(child) = children.next() {
                let child_idx = child.to_usize().unwrap();
                if visited[child_idx] > 0 && (vis_2 != Some(*child) || visited[child_idx] >= 2) {
                    // already visited node
                    // only for small caves and one node can be visited twice if set
                    continue;
                }

                if *child == end {
                    if vis_2.map_or(true, |idx| visited[idx.to_usize().unwrap()] == 2) {
                        num_paths += 1;
                    }
                } else {
                    children_stack.push((*child, self.neighbors(child)));
                    if self.is_small[child_idx] {
                        visited[child_idx] += 1;
                    }
                }
            } else {
                let root = root.to_usize().unwrap();
                if self.is_small[root] {
                    visited[root] -= 1;
                }
                children_stack.pop();
            }
        }

        return num_paths;
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
