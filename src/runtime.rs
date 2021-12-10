//! The starter code slowly blinks the LED, and sets up
//! USB logging.

extern crate alloc;

use teensy4_bsp as bsp;
use teensy4_panic as _;

use alloc_cortex_m::CortexMHeap;
use core::fmt::Write;
use core::alloc::Layout;

use super::usb_io;
use super::utils::usb_input;
use super::solutions::{Solution, ParsedData};

pub use super::utils::container::Hardware;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

pub fn run<O, T>(solution: &mut T) -> ! where O:ParsedData, T : Solution<O> {
    // init allocator
    let start = cortex_m_rt::heap_start() as usize;
    let size = 1_000_000; // in bytes
    unsafe { ALLOCATOR.init(start, size) }

    // do rest
    let mut p = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let (reader, writer) = usb_io::split().unwrap();

    log::error!("You might not see this message if the USB device isn't configured by the host");
    systick.delay(1000);

    let pins = bsp::t41::into_pins(p.iomuxc);
    let led = bsp::configure_led(pins.p13); 

    p.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc
    );

    let mut hardware = Hardware {
        led: led, systick: systick,
        reader: reader, writer: writer,
    };

    // load input data
    loop {
        let in_file =  usb_input::load_input(&mut hardware);

        if let Some(data) = in_file {
            writeln!(hardware.writer, "Loaded file with {:?} chars\n", data.len()).unwrap();
            run_tests(&mut hardware, solution, data);
        } else {
        }
      
        let mut flag = 0u8;
        let mut buffer = [0;1];
        'waiting: loop {
            hardware.led.toggle();
            hardware.systick.delay( if (flag / 2) % 2 == 0 { 600 } else { 300 });
            flag += 1;
           
            for _ in 0..256 {
                if hardware.reader.read(&mut buffer).unwrap() > 0 {
                    if buffer[0] == b'R' {
                        writeln!(hardware.writer, "\n\n--------RESTARTING SOLVER----------\n").unwrap();
                        break 'waiting;
                    }
                } else {
                    break;
                }
            }
        }
    }

}

fn run_tests<O, T>(hardware: &mut Hardware, solution: &mut T, data: alloc::string::String) where O: ParsedData, T: Solution<O> {
    writeln!(hardware.writer, "-----------------------------------").unwrap();
    writeln!(hardware.writer, "Parsing file with input length {:?}", data.len()).unwrap();
    let parsed = O::parse_file(hardware, data);
    writeln!(hardware.writer, " - Successfully parsed file").unwrap();

    writeln!(hardware.writer, "Running solution part 1").unwrap();
    solution.part_a(hardware, &parsed);

    writeln!(hardware.writer, "Running solution part 2").unwrap();
    solution.part_b(hardware, &parsed);
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    panic!();
}
