use num_traits::Num;
use num_traits::bounds::Bounded;

pub fn parse_with_err<T>(v: &str) -> T where T: Num + Bounded {
    return parse_with_err_radix(v, 10);
}

pub fn parse_with_err_radix<T>(v: &str, radix: u32) -> T where T: Num + Bounded {
    match T::from_str_radix(v, radix) {
        Ok(obj) => obj,
        Err(_) => {
            panic!("ERR: parsing string into num '{}'", v);
        }
    }

}
