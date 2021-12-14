extern crate alloc;

use core::convert::TryInto;
use core::fmt::{Display, Write};

use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;
use aoc21::utils::Hardware;


pub struct Polymerization {
    pub polymer: Vec<u8>,
    pub insertion_rules: BTreeMap<[u8; 2], u8>
}

impl aoc21::solutions::ParsedData for Polymerization {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let (polymer, pairs) = in_data.split_once("\n\n").expect("Malformed input");
        let mut rules = BTreeMap::new(); 

        pairs.lines()
            .map(|line| line.split_once(" -> ").expect("Malformed input"))
            .for_each(|(from, to)| {
                let from =from.bytes().collect::<Vec<u8>>().try_into().unwrap();
                let to = to.bytes().next().unwrap();
                rules.insert(from, to); 
            });

        return Polymerization { 
            polymer: polymer.bytes().collect(),
            insertion_rules: rules
        }
    }
}

/// Polymer object
///
/// Instead of storing the polymer as a long string
/// of elements it makes sense to create a bucket system 
/// for them where each pair is in a map together with
/// how often it occurs in the polymer.
#[derive(Clone)]
pub struct Polymer {
    pub ends: [u8; 2],
    pub pairs: BTreeMap<[u8; 2], u64>
}

impl From<&Vec<u8>> for Polymer {
    /// Create a Polymer from an input vector of u8
    fn from(input: &Vec<u8>) -> Self {
        let mut buckets: BTreeMap<[u8; 2], u64> = BTreeMap::new();
        input.iter().zip(input.iter().skip(1))
            .map(|(a,b)| [*a, *b])
            .for_each(|pair| {
                *buckets.entry(pair).or_insert(0) += 1;
            });
        let ends = [input[0], input[input.len() - 1]];

        Polymer { ends: ends, pairs: buckets }
    }
}

impl Display for Polymer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Polymer")?;
        for (pair, amount) in &self.pairs {
            for c in pair {
                f.write_char(*c as char)?;
            }
            writeln!(f, " -> {}", amount)?;
        }
        Ok(())
    }
}

impl Polymer {
    pub fn new(ends: &[u8; 2]) -> Self {
        Polymer { ends: *ends, pairs: BTreeMap::new() }
    }

    pub fn count_elements(&self) -> BTreeMap<u8, u64> {
        let mut count: BTreeMap<u8, u64> = BTreeMap::new();

        self.ends.iter().for_each(|v| { count.insert(*v, 1); });

        for (pair, amount) in &self.pairs {
            pair.iter().for_each(|v| {
                *count.entry(*v).or_insert(0) += amount;
            });
        }

        for (_, amount) in &mut count {
            *amount /= 2;
        }

        return count;
    }
}
