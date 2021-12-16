extern crate alloc;

use core::fmt::Display;
use alloc::vec::Vec;
use alloc::string::String;
use num_traits::{PrimInt, Unsigned, CheckedShl};

use aoc21::utils::Hardware;

fn bits2u<T>(bits: impl Iterator<Item = u8>) -> T where T: PrimInt + Unsigned + CheckedShl {
    bits.fold(T::zero(), |acc, v| CheckedShl::checked_shl(&acc, 1).unwrap() | T::from(v).unwrap())
}

pub struct Transmission {
    pub data: Vec<u8>,
    pub root: Package,
}

impl aoc21::solutions::ParsedData for Transmission {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let data: Vec<u8> = in_data.trim().chars().flat_map(|v| { 
            let v = v.to_digit(16).unwrap_or_else(|| panic!("Unknown symbol '{}'", v));
            (0..4).rev().map(move |i| ((v >> i) & 0b1) as u8)
        }).collect();

        let root = Package::build_tree(data.iter());

        Transmission { data, root }
    }
}

impl Display for Transmission {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.data.iter().try_for_each(|v| write!(f, "{}", v))?;
        Ok(())
    }
}


#[derive(Debug)]
pub enum Package {
    Literal {
        version: u8,
        value: u64
    },
    Operator {
        version: u8,
        type_id: u8,
        subpackages: Vec<Package>
    },
}

/// Parsing
impl Package {
    pub fn build_tree<'a>(input: impl Iterator<Item = &'a u8>) -> Self {
        return Self::parse(&mut input.cloned());
    }

    fn parse(input: &mut impl Iterator<Item = u8>) -> Self {
        let version: u8 = bits2u(input.by_ref().take(3));
        let type_id: u8 = bits2u(input.by_ref().take(3));

        match type_id {
            4 => Self::parse_remaining_literal(input, version),
            _ => Self::parse_remaining_operator(input, version, type_id)
        }
    }

    fn parse_remaining_literal(input: &mut impl Iterator<Item = u8>, version: u8) -> Self {
        let mut value = 0u64;
        loop {
            let flag_bit = input.next().unwrap();
            let v: u64 = bits2u(input.by_ref().take(4));
            value = value.checked_shl(4).unwrap() | v;

            if flag_bit == 0 { break; }
        }

        Package::Literal { version, value }
    }

    fn parse_remaining_operator(input: &mut impl Iterator<Item = u8>, version: u8, type_id: u8) -> Self {
        let subpackages = match input.next().unwrap() {  // parse length-type-id
            0 => Self::parse_operator_subpackages_ltype_0(input),
            1 => Self::parse_operator_subpackages_ltype_1(input),
            v => panic!("Invalid bit returned {}", v)
        };

        Package::Operator { version, type_id, subpackages }
    }

    fn parse_operator_subpackages_ltype_0(input: &mut impl Iterator<Item = u8>) -> Vec<Package> {
        let total_length: usize = bits2u(input.by_ref().take(15));

        let sub_bits = input.take(total_length).collect::<Vec<_>>();
        let mut sub_iter = sub_bits.iter().cloned().peekable();
        let mut packages = Vec::new();

        while sub_iter.peek().is_some() {
            packages.push(Self::parse(&mut sub_iter));
        }

        packages
    }

    fn parse_operator_subpackages_ltype_1(input: &mut impl Iterator<Item = u8>) -> Vec<Package> {
        let total_packages: usize = bits2u(input.by_ref().take(11));

        (0..total_packages).map(|_| Self::parse(input)).collect()
    }
}
