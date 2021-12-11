use core::fmt::Write;
use super::container::Hardware;

use num_traits::Num;
use num_traits::bounds::Bounded;

pub fn parse_with_err<T>(hardware: &mut Hardware, v: &str) -> T where T: Num + Bounded {
    return parse_with_err_radix(hardware, v, 10);
}

pub fn parse_with_err_radix<T>(hardware: &mut Hardware, v: &str, radix: u32) -> T where T: Num + Bounded {
    match T::from_str_radix(v, radix) {
        Ok(obj) => obj,
        Err(_) => {
            writeln!(hardware.writer, "ERR: parsing string into num '{}'", v).unwrap();
            T::max_value()
        }
    }

}
