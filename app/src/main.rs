#![no_std]
#![no_main]

extern crate rt;

mod cp;
mod dp;
mod handler;
mod kernel;

use cp::stk::SystemTimer;
use dp::bus::PERIPHERALS;
use kernel::sched::{init_scheduler, Scheduler};

static mut TOGGLE_FLAG: bool = false;

extern "C" {
    pub fn __invoke(x: u32) -> u32;
    pub fn __start_kernel(x: u32);
    pub fn syscall();
    pub fn __schedule();
}

#[no_mangle]
fn user_task() {
    let mut x = 0;
    loop {
        unsafe {
            TOGGLE_FLAG = true;
            x += 1;
            __schedule();
        }
    }
}

#[no_mangle]
fn load_scheduler() {
    unsafe { __schedule() };
    let mut scheduler = Scheduler::default();
    scheduler.add_user_task(user_task).unwrap();
    scheduler.schedule_user_threads();
}

#[no_mangle]
fn main() -> ! {
    let serial = unsafe { PERIPHERALS.take_serial() };
    let mut ahb1 = serial.ahb1();
    ahb1.rcc(|rcc| rcc.iopeen());

    let gpioe = serial.ahb2().gpioe();
    let mut leds = handler::LED::new(gpioe);
    leds.on(9);
    leds.on(8);
    // leds.on(15);
    let st = SystemTimer::take();
    st.set_reload(0x3FFFF).enable();

    init_scheduler(load_scheduler);

    loop {
        unsafe {
            if TOGGLE_FLAG {
                leds.toggle(9);
                TOGGLE_FLAG = false;
            }
        }
    }
}

pub fn kernel_thread() {}

#[no_mangle]
pub extern "C" fn SysTick() {
    unsafe {
        __schedule();
    }
}

#[no_mangle]
pub extern "C" fn PendSV() {}

// #[no_mangle]
// pub extern "C" fn SVCall() {
//     unsafe { __schedule(); }
// }
