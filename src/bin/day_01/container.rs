extern crate alloc;

use core::fmt::Write;

use alloc::vec::Vec;
use alloc::string::String;
use aoc21::utils::Hardware;
use aoc21::usbwriteln;


pub struct Sonar {
    pub depths: Vec<u16>
}

impl aoc21::solutions::ParsedData for Sonar {
    fn parse_file(_: &mut Hardware, in_data: String) -> Self {
        let num_parse_with_err = |v: &str| -> u16 {
            match v.parse() {
                Ok(obj) => obj,
                Err(e) => {
                    usbwriteln!("ERR: parsing string {} ({:?})", v, e);
                    u16::MAX
                }
            }
        };

        let out = Sonar { 
            depths: in_data.lines().map(num_parse_with_err).collect() 
        };
        usbwriteln!("Parsed {:?} sonar points", out.depths.len());
        return out;
    }
}
