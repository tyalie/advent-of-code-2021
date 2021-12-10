//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![feature(alloc_error_handler)]

#![no_std]
#![no_main]

extern crate alloc;

use teensy4_panic as _;
use cortex_m_rt::entry;


#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    aoc21::runtime::main();
}
