extern crate alloc;

use core::fmt::{Display, Debug};
use core::convert::TryInto;
use core::iter::Step;

use alloc::collections::BTreeSet;
use alloc::string::String;
use itertools::Itertools;
use num_traits::{PrimInt, Signed};
use aoc21::utils::Hardware;


pub struct ImageAlgo {
    pub algo: [bool; 512],
    pub image: Image<i16>
}

#[derive(Clone)]
pub struct Image<T> where T: PrimInt + Signed {
    pub data: BTreeSet<Pixel<T>>,
    pub infity_val: bool,
    min: Pixel<T>,
    max: Pixel<T>
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pixel<T> where T: PrimInt + Signed {
    pub y: T,
    pub x: T
}

impl aoc21::solutions::ParsedData for ImageAlgo {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let (algo_str, image_str) = in_data.split_once("\n\n").unwrap();
        let algo = algo_str.trim().chars().map(|b| b == '#').collect_vec();
        let mut image = BTreeSet::new();

        assert!(!algo[0] || !algo[511], "algorithm would output infinite value");

        image_str.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().filter(|(_, b)| *b == '#').for_each(|(x, _)| {
                image.insert(Pixel { x: x as i16, y: y as i16 });
            });
        });

        ImageAlgo { 
            algo: algo.try_into().unwrap(), 
            image: Image::new(image)
        }
    }
}

impl<T> Display for Image<T> where T: PrimInt + Signed + Step {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Pixel { x: mut last_x, y: mut last_y } = self.min;
        let min_x = last_x;

        for Pixel { x, y } in self.data.iter() {
            for _ in last_y..*y {
                writeln!(f)?;
                last_x = min_x;
            }
            for _ in last_x..*x {
                write!(f, " ")?;
            }
            write!(f, "█")?;
            last_x = *x + T::one();
            last_y = *y;
        }
        Ok(())
    }
}

impl<T> Image<T> where T: PrimInt + Signed + Step {
    pub fn new(data: BTreeSet<Pixel<T>>) -> Self {
        let min = Self::min(&data);
        let max = Self::max(&data);

        Image {
            data, infity_val: false, min, max
        }
    }

    pub fn extend(data: BTreeSet<Pixel<T>>, previous: &Image<T>, change_infty: bool) -> Self {
        Image {
            data: data,
            infity_val: !previous.infity_val && change_infty,
            min: Pixel { x: previous.min.x - T::one(), y: previous.min.y - T::one() },
            max: Pixel { x: previous.max.x + T::one(), y: previous.max.y + T::one() },
        }
    }

    fn min(data: &BTreeSet<Pixel<T>>) -> Pixel<T> {
        let min_y = data.first().unwrap().y;
        let min_x = data.iter().min_by_key(|e| e.x).unwrap().x;
        return Pixel { y: min_y, x: min_x }
    }

    fn max(data: &BTreeSet<Pixel<T>>) -> Pixel<T> {
        let max_y = data.last().unwrap().y;
        let max_x = data.iter().max_by_key(|e| e.x).unwrap().x;
        return Pixel { y: max_y, x: max_x }
    }

    pub fn iter_pixels(&self) -> impl Iterator<Item = Pixel<T>> {
        let min = self.min;
        let max = self.max;

        ((min.y - T::one())..=(max.y + T::one()))
            .flat_map(move |y| {
                ((min.x - T::one())..=(max.x + T::one()))
                    .map(move |x| Pixel { x, y })
            })
    }

    pub fn is_on(&self, pixel: &Pixel<T>) -> bool {
        if pixel.y < self.min.y || pixel.y > self.max.y
            || pixel.x < self.min.x || pixel.x > self.max.x {
                return self.infity_val;
        }

        return self.data.contains(pixel);
    }

    pub fn get_box(&self, pixel: &Pixel<T>) -> u16 {
        ((pixel.y - T::one())..=(pixel.y + T::one()))
            .cartesian_product((pixel.x - T::one())..=(pixel.x + T::one()))
            .map(|(y, x)| self.is_on(&Pixel { y, x }) as u16)
            .fold(0u16, |acc, v| (acc << 1) | v)
    }
}

