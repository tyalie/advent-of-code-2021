//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![no_std]

pub mod usb_io;
pub mod utils;

pub mod runtime;
pub mod solutions;
