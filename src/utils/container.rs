use teensy4_bsp as bsp;


pub struct Hardware {
    pub led: bsp::LED,
    pub systick: bsp::SysTick,
    pub reader: bsp::usb::Reader,
}
