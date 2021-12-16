//! The starter code slowly blinks the LED, and sets up
//! USB logging.

extern crate alloc;

use teensy4_bsp as bsp;
use teensy4_panic::sos;

use alloc_cortex_m::CortexMHeap;
use alloc::string::String;
use core::fmt::Write;
use core::alloc::Layout;

use crate::usbwrite;
use crate::usbwriteln;

use super::usb_io;
use super::utils::usb_input;
use super::solutions::{Solution, ParsedData};

pub use super::utils::container::Hardware;

#[global_allocator]
pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();


pub fn run<O, T>(solution: &mut T) -> ! where O:ParsedData, T : Solution<O> {
    // init allocator
    // let start = cortex_m_rt::heap_start() as usize;
    let start = 0x2020_5000 as usize;
    let size = 400_000; // in bytes
    unsafe { ALLOCATOR.init(start, size) }

    // do rest
    let mut p = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let (reader, writer) = usb_io::usb_init::split().unwrap();

    cortex_m::interrupt::free(|cs| {
        *usb_io::WRITER.borrow(cs).borrow_mut() = Some(writer);
    });
    // unsafe { WRITER = Some(writer) };

    usbwrite!("You might not see this message if the USB device isn't configured by the host");
    systick.delay(3000);

    let pins = bsp::t41::into_pins(p.iomuxc);
    let led = bsp::configure_led(pins.p13); 

    p.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc
    );

    usbwrite!("Hello");
    usbwriteln!("Initialized heap with {} bytes at {:x}", size, start);

    let mut hardware = Hardware {
        led: led, systick: systick,
        reader: reader,
    };

    // load input data
    loop {
        let in_file =  usb_input::load_input(&mut hardware);

        if let Some(data) = in_file {
            usbwriteln!("Initialized heap with {} bytes at {:x}", size, start);
            usbwriteln!("Loaded file with {:?} chars\n", data.len());
            hardware.systick.delay(1000);
            run_tests(&mut hardware, solution, data);
        } else {
        }
      
        let mut flag = 0u8;
        let mut buffer = [0;1];
        'waiting: loop {
            hardware.led.toggle();
            hardware.systick.delay( if (flag / 2) % 2 == 0 { 600 } else { 300 });
            flag = flag.wrapping_add(1);
           
            for _ in 0..256 {
                if hardware.reader.read(&mut buffer).unwrap() > 0 {
                    if buffer[0] == b'R' {
                        usbwriteln!("\n\n--------RESTARTING SOLVER----------\n");
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
    usbwriteln!("-----------------------------------");
    usbwriteln!("Parsing file with input length {:?}", data.len());
    let mut parsed = O::parse_file(hardware, data);
    usbwriteln!(" - Successfully parsed file");

    usbwriteln!("\nRunning solution part 1");
    solution.part_a(hardware, &mut parsed);

    usbwriteln!("\nRunning solution part 2");
    solution.part_b(hardware, &mut parsed);
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    panic!("Alloc error handler called. {}b remaining", ALLOCATOR.free());
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    usbwriteln!("\n==== panic occured ====");
    if let Some(message) = info.message() {
        usbwriteln!("- {}", message);
    } else {
        usbwriteln!("No reason provided.");
    }

    if let Some(location) = info.location() {
        usbwriteln!("at '{}':{}", location.file(), location.line());
    }

    let payload = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<dyn Any>",
        },
    };
    usbwriteln!("Payload: '{}'", payload);


    sos()
}
