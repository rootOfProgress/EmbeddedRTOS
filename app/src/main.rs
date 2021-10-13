#![no_std]
#![no_main]

extern crate rt;

mod cp;
mod dp;
mod handler;

use core::sync::atomic::{AtomicBool, Ordering};

use cp::stk::SystemTimer;
use dp::bus::PERIPHERALS;

static TOGGLE_FLAG: AtomicBool = AtomicBool::new(false);

#[no_mangle]
fn main() -> ! {
    let serial = unsafe { PERIPHERALS.take_serial() };
    let mut ahb1 = serial.ahb1();
    ahb1.rcc(|rcc| rcc.iopeen());

    let gpioe = serial.ahb2().gpioe();
    let mut leds = handler::LED::new(gpioe);
    leds.on(9);
    leds.on(8);
    leds.on(15);

    let st = SystemTimer::take();
    st.set_reload(0x3FFFF).enable();

    loop {
        if TOGGLE_FLAG.fetch_and(false, Ordering::Relaxed) {
            leds.toggle(9);
        }
    }
}

#[no_mangle]
pub extern "C" fn SysTick() {
    TOGGLE_FLAG.store(true, Ordering::Relaxed)
}
