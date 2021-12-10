extern crate alloc; 

use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;
use core::convert::TryInto;
use core::fmt::Write;
use core::mem as mem;

use teensy4_bsp as bsp;

const START_CODE: &'static [u8] = b"\xC0\xDEGO";

/// Blocked read of specified bytes from serial
///
/// # Arguments
/// 
/// * `reader` - The serial reader
/// * `num_bytes` - The amount of bytes to load
///
/// # Return
/// a byte vector of length `num_bytes`
pub fn read_bytes(reader: &mut bsp::usb::Reader, num_bytes: usize) -> Vec<u8> {
    let mut values: vec::Vec<u8>  = Vec::with_capacity(num_bytes);
    let mut buffer = [0; 1];

    while values.len() < values.capacity() {
        let bytes_read = reader.read(&mut buffer).unwrap();
        if bytes_read > 0 {
            values.extend(&buffer[..bytes_read]);
        }
    }
    return values;
}

/// load input file from serial terminal
///
/// # Arguments
///
/// * `reader` - The serial reader
/// * `writer` - The serial writer 
/// * `systick` - systick instance to so that delay is possible
///
/// # Return
/// the loaded input file
pub fn load_input(reader: &mut bsp::usb::Reader, writer: &mut bsp::usb::Writer, systick: &mut bsp::SysTick) -> Option<String> {
    let length = get_input_size(reader, writer, systick);

    writeln!(writer, "Waiting for {:?} bytes", length).unwrap();

    let in_file = read_bytes(reader, length);

    match alloc::string::String::from_utf8(in_file) {
        Ok(obj) => return Some(obj),
        Err(e) => {
            writeln!(
                writer, "Error parsing input file with len {} ({:?})",
                length, e
            ).unwrap();
            return None;
        }
    }
}

/// Retrive file input size from serial terminal by waiting for 
/// the start code and the following length.
///
/// Prints out repeatately `ready` until the START_CODE has been send.
/// 
/// # Arguments
///
/// * `reader` - The serial reader
/// * `writer` - The serial writer 
/// * `systick` - systick instance to so that delay is possible
///
/// # Return
/// the input file size
fn get_input_size(reader: &mut bsp::usb::Reader, writer: &mut bsp::usb::Writer, systick: &mut bsp::SysTick) -> usize {
    // Wait for START_CODE so that file can be parsed
    let mut position = 0;
    let mut counter = 0_u8;
    loop {
        let mut single_byte_buffer = [0;1];
        let bytes_read = reader.read(&mut single_byte_buffer).unwrap();

        if bytes_read > 0 {
            if single_byte_buffer[0] == START_CODE[position] {
                position += 1;

                if position >= START_CODE.len() {
                    break;
                }
            } else {
                position = 0;
            }
        }

        counter += 1;
        systick.delay(10);

        if counter % 32 == 0 {
            writeln!(writer, "ready").unwrap();
        }
    }

    usize::from_be_bytes(
        read_bytes(reader, mem::size_of::<usize>()).try_into().unwrap()
    )
}
