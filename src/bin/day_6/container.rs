extern crate alloc;

use core::convert::TryInto;
use core::fmt::{Write, Debug};

use alloc::vec::Vec;
use alloc::string::String;
use aoc21::usbwriteln;
use aoc21::utils::tools::parse_with_err;
use num_traits::int::PrimInt;
use aoc21::utils::Hardware;


pub struct School {
    pub fishs: Vec<u8>
}

impl aoc21::solutions::ParsedData for School {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let fishs: Vec<u8> = in_data.trim().split(",").map(parse_with_err).collect();
        return School {
            fishs: fishs
        }
    }
}

impl School {
    pub fn compress<T>(&self) -> CompressedSchool<T> where T: PrimInt + Debug {
        let fish_count = |c: u8| {
            T::from(
                self.fishs.iter().filter(|f| **f == c).count()
            ).expect("Conv failed")
        };

        // Count all fishs and put them into their corresponding incubation baskets
        let fishs: Vec<T> = (0..=8).rev().map(fish_count).collect(); 

        return CompressedSchool {
            new_fish: fishs[0..2].try_into().expect("Couldn't convert new fishs"),
            old_fish: fishs[2..].try_into().expect("Couldn't convert old fishs")
        }
    }
}

/// # Compressed Container for a school of fish
/// There are two buckets fish can fall into
///
///  * new_fish: which have an incubation time of 9 days
///  * old_fish: normal cycle with an incubation time of 7 days
///
///  After each incubation a new fish will be born
#[derive(Clone, Copy)]
pub struct CompressedSchool<T> where T: PrimInt {
    pub new_fish: [T; 2],
    pub old_fish: [T; 7]
}

impl<T> CompressedSchool<T> where T: PrimInt {
    pub fn do_step(&mut self) {
        self.new_fish.rotate_right(1);
        self.old_fish.rotate_right(1);

        let into_normal_cycle: T = self.new_fish[0];
        let new_born_fish: T = self.old_fish[0];

        self.old_fish[0] = self.old_fish[0] + into_normal_cycle;
        self.new_fish[0] = new_born_fish;
    }

    pub fn count_fish(&self) -> T {
        let mut sigma: T = T::zero();
        self.new_fish.iter().for_each(|v| sigma = sigma + *v);
        self.old_fish.iter().for_each(|v| sigma = sigma + *v);
        return sigma;
    }
}
