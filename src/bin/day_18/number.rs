extern crate alloc;

use core::{ops::Add, fmt::Debug};
use alloc::vec::Vec;
use itertools::Itertools;


#[derive(Debug, Clone)]
pub struct SailfishNumber {
    numbers: Vec<DepthNumber>
}

#[derive(Clone, Copy)]
struct DepthNumber {
    depth: u8,
    value: u16
}

impl Debug for DepthNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}:{})", self.depth, self.value)
    }
}

impl Add<SailfishNumber> for SailfishNumber {
    type Output = SailfishNumber;
    fn add(self, rhs: SailfishNumber) -> Self::Output {
        /* Adding two sailfish numbers A and B has output [A, B]
         * as such I can just append all numbers from rhs
         * to my lhs and increase all their depths by one
         */
        let mut numbers = Vec::with_capacity(self.numbers.len() + rhs.numbers.len());
        for v in self.numbers.iter().chain(rhs.numbers.iter()) {
            numbers.push(DepthNumber { depth: v.depth + 1, value: v.value });
        }

        let mut sn = SailfishNumber { numbers };
        loop {
            if !(sn.explode() || sn.split()) {
                return sn;
            }
        }
    }
}

impl SailfishNumber {
    pub fn parse(input: &str) -> Self {
        let mut nums: Vec<DepthNumber> = Vec::new();
        let mut depth = 0;

        for b in input.bytes() {
            match b {
                91 => depth += 1,   // [
                93 => depth -= 1,   // ]
                44 => {},           // ,
                v => nums.push(DepthNumber { depth, value: (v - b'0') as u16 })
            }
        }

        SailfishNumber { numbers: nums }
    }

    pub fn explode(&mut self) -> bool {
        // there should be no depth > 5 => pair is only numbers
        let pos = self.numbers.iter().position(|v| v.depth >= 5);  
        if let Some(pos) = pos {  // pos is first number of pair
            assert!(pos + 1 < self.numbers.len());
            if pos > 0 {  // lhs
                self.numbers[pos - 1].value += self.numbers[pos].value;
            }
            if (pos + 1) < self.numbers.len() - 1 {  // rhs
                self.numbers[pos + 2].value += self.numbers[pos + 1].value;
            }
            self.numbers.remove(pos+1);
            self.numbers[pos] = DepthNumber {depth: self.numbers[pos].depth - 1, value: 0 };

            return true
        }
        return false
    }

    pub fn split(&mut self) -> bool {
        let pos = self.numbers.iter().position(|v| v.value > 9);
        if let Some(pos) = pos {
            let DepthNumber { depth, value } = self.numbers[pos];

            self.numbers.insert(pos + 1, DepthNumber {  // rhs
                value: value / 2 + (value % 2), depth: depth + 1 
            });  // insert value with ceil( value / 2 )

            self.numbers[pos] = DepthNumber {  // lhs
                value: value / 2, depth: depth + 1
            };

            return true
        }
        return false
    }

    pub fn magnitude(mut self) -> u16 {
        // this is the last element of a pair
        while self.numbers.len() > 1 {
            let rpos = self.numbers.iter().position_max_by_key(|v| v.depth).unwrap();
            let DepthNumber { depth, value } = self.numbers.remove(rpos);
            self.numbers[rpos - 1].depth = depth - 1;
            self.numbers[rpos - 1].value *= 3;
            self.numbers[rpos - 1].value += 2 * value;
        }
        self.numbers[0].value
    }
}
