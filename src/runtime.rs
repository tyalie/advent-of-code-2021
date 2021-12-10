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
use super::solutions::Solution;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

pub fn main() -> ! {
    // init allocator
    let start = cortex_m_rt::heap_start() as usize;
    let size = 1_000_000; // in bytes
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

    // load input data
    let in_file =  usb_input::load_input(&mut reader, &mut writer, &mut systick, &mut led);

    if let Some(data) = in_file {
        write!(writer, "{}", data).unwrap();
    }
  
    let mut flag = 0u8;
    loop {
        led.toggle();
        systick.delay( if (flag / 2) % 2 == 0 { 600 } else { 300 });
        flag += 1;
    }

}

fn test<T, O>(input: O) where O: Solution<T> {
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    panic!();
}
