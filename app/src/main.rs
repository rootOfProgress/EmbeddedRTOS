#![no_std]
#![no_main]

extern crate rt;

mod cp;
mod dp;

use cp::stk::SystemTimer;
use dp::bus::{self};

#[no_mangle]
fn main() -> ! {
    
    let rcc = bus::AHB1::new().rcc();
    rcc.enable_gpioe();

    let gpioe = bus::AHB2::new().gpioe();
    gpioe.led_on(9);

    let st = SystemTimer::new();
    st.set_reload(0xFFFF).enable();

    loop {}
}

#[no_mangle]
pub extern "C" fn SysTick() {
    let gpioe = bus::AHB2::new().gpioe();
    gpioe.led_toggle(9);
}
