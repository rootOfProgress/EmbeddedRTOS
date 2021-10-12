#![no_std]
#![no_main]

extern crate rt;

mod cp;
mod dp;
mod handler;

use cp::stk::SystemTimer;
use dp::bus::{self};

#[no_mangle]
fn main() -> ! {
    let rcc = bus::AHB1::take().rcc();
    rcc.enable_gpioe();

    let gpioe = bus::AHB2::take().gpioe();
    let mut leds = handler::LED::new(gpioe);
    leds.on(9);
    leds.on(8);
    leds.on(15);

    let st = SystemTimer::take();
    st.set_reload(0x3FFFF).enable();

    loop {}
}

#[no_mangle]
pub extern "C" fn SysTick() {
    let gpioe = bus::AHB2::take().gpioe();
    let mut leds = handler::LED::new(gpioe);
    leds.toggle(9);
}
