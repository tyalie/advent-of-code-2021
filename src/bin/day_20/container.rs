extern crate alloc;

use core::fmt::{Write, Display};

use alloc::vec::Vec;
use alloc::collections::BTreeSet;
use alloc::string::String;
use num_traits::{PrimInt, Signed};
use aoc21::{utils::Hardware, usbwriteln};


pub struct ImageAlgo {
    pub algo: Vec<u8>,
    pub image: BTreeSet<Pixel<i16>>
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Pixel<T> where T: PrimInt + Signed {
    pub y: T,
    pub x: T
}

impl aoc21::solutions::ParsedData for ImageAlgo {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let (algo_str, image_str) = in_data.split_once("\n\n").unwrap();
        let algo = algo_str.trim().chars().map(|b| (b == '#') as u8).collect();
        let mut image = BTreeSet::new();

        image_str.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().filter(|(_, b)| *b == '#').for_each(|(x, _)| {
                image.insert(Pixel { x: x as i16, y: y as i16 });
            });
        });

        ImageAlgo { algo, image }
    }
}

impl Display for ImageAlgo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Pixel { x: mut last_x, y: mut last_y } = self.image.first().unwrap();
        for Pixel { x, y } in self.image.iter() {
            for _ in last_y..*y {
                writeln!(f)?;
                last_x = 0;
            }
            for _ in last_x..*x {
                write!(f, " ")?;
            }
            write!(f, "█")?;
            last_x = *x + 1;
            last_y = *y;
        }
        Ok(())
    }
}
