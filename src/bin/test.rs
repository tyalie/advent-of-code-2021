//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![feature(alloc_error_handler)]

#![no_std]
#![no_main]

extern crate alloc;

use teensy4_bsp as bsp;
use teensy4_panic as _;
use cortex_m_rt::entry;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;

use aoc21::usb_io;
use aoc21::utils::usb_input;

const LED_PERIOD_MS: u32 = 500;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[entry]
fn wrapper() -> ! {
    main();
}

fn main() -> ! {
    // init allocator
    let start = cortex_m_rt::heap_start() as usize;
    let size = 1024; // in bytes
    unsafe { ALLOCATOR.init(start, size) }

    // do rest
    let mut p = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let (mut reader, mut writer) = usb_io::split().unwrap();

    log::error!("You might not see this message if the USB device isn't configured by the host");
    systick.delay(1000);

    let pins = bsp::t41::into_pins(p.iomuxc);
    let mut led = bsp::configure_led(pins.p13); 

    p.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc
    );
    // See the `logging` module docs for more info.
    
    usb_input::load_input(&mut reader, &mut writer, &mut systick);

    let mut buffer = [0; 256];

    // load input file
    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();
        if bytes_read > 0 {
            let bytes = &buffer[..bytes_read];
            match core::str::from_utf8(bytes) {
                Ok(msg) => log::info!("Received message: {} ({:?})", msg, bytes),
                Err(e) => log::warn!(
                    "Read {} bytes, but could not interpret message {:?}: {:?}",
                    bytes_read, bytes, e
                ),
            }
        }


        led.toggle();
        systick.delay(500);
        log::info!("Hello world");
    }

}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    panic!();
}
