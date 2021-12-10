extern crate alloc; 

use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::fmt::Write;

use teensy4_bsp as bsp;

const START_CODE: &'static [u8] = b"\xC0\xDEGO";


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

pub fn load_input(reader: &mut bsp::usb::Reader, writer: &mut bsp::usb::Writer, systick: &mut bsp::SysTick) {
    let length = get_input_size(reader, writer, systick);

    writeln!(writer, "length {:?}", length).unwrap();
}

fn get_input_size(reader: &mut bsp::usb::Reader, writer: &mut bsp::usb::Writer, systick: &mut bsp::SysTick) -> u32 {
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

    u32::from_be_bytes(read_bytes(reader, 4).try_into().unwrap())
}
