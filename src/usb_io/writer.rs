use teensy4_panic as _;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;


pub static WRITER: Mutex<RefCell<Option<teensy4_bsp::usb::Writer>>> = Mutex::new(RefCell::new(None));


#[macro_export]
macro_rules! usbwrite {
    ($($arg:tt)*) => {
        {
            cortex_m::interrupt::free(|cs| {
                if let Ok(mut container) = $crate::usb_io::writer::WRITER.borrow(cs).try_borrow_mut() {
                    if let Some(writer) = container.as_mut() {
                        write!(writer, $($arg)*).unwrap();
                    }
                }
            });
        }
    }
//    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))
}

#[macro_export]
macro_rules! usbwriteln {
    ($($arg:tt)*) => {
        $crate::usbwrite!("\n");
        $crate::usbwrite!($($arg)*);
    }
}
