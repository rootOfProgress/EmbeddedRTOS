#![no_std]
#![no_main]

extern crate rt;

mod cp;
mod dp;
mod handler;
mod kernel;

use core::ptr::{read_volatile, write_volatile};

use cp::stk::SystemTimer;
use dp::bus::PERIPHERALS;
use kernel::{
    sched::{init_scheduler, Scheduler},
    svc::{sprint, syscall, SvcOrder, SvcRequest, SvcResult},
};

static mut TOGGLE_FLAG: bool = false;

extern "C" {
    pub fn __invoke(x: u32) -> u32;
    pub fn __syscall(order: *const SvcOrder) -> *mut SvcResult;
    pub fn __breakpoint();
    pub fn __save_psp() -> u32;
    pub fn __sprint(text: *const u8);
    pub fn __sreadc() -> u8;
    pub fn __sprintc(char: *const u8);
    pub fn __get_r0() -> *mut SvcOrder;
}

#[no_mangle]
fn user_task() {
    sprint("Task 1: Started\n");
    let serial = unsafe { PERIPHERALS.take_serial() };
    let mut ahb1 = serial.ahb1();
    ahb1.rcc(|rcc| rcc.iopeen());

    let gpioe = serial.ahb2().gpioe();
    let mut leds = handler::LED::new(gpioe);
    leds.on(13);

    sprint("Enter LED [8 or 9]:\n");
    let result = syscall(SvcRequest::SYS_READC);
    if let SvcResult::Char(number) = result {
        // Ascii table numbers start at 48
        leds.on(*&number - 48);
    }

    loop {
        unsafe {
            if TOGGLE_FLAG {
                sprint("TASK 1: Toggle LED\n");
                leds.toggle(13);
                TOGGLE_FLAG = false;
            }
        }
    }
}

#[no_mangle]
fn user_task_turn_off_led() {
    sprint("Task 2: Started\n");
    loop {
        unsafe {
            if !TOGGLE_FLAG {
                sprint("TASK 2: Set Toggle true\n");
                TOGGLE_FLAG = true;
            }
        }
    }
}

#[no_mangle]
fn load_scheduler() {
    sprint("\nScheduler loaded!\n");
    let mut scheduler = Scheduler::default();
    scheduler.add_user_task(user_task).unwrap();
    scheduler.add_user_task(user_task_turn_off_led).unwrap();
    scheduler.schedule_user_threads();
}

#[no_mangle]
fn main() -> ! {
    let st = SystemTimer::take();
    st.set_reload(0x3FFFF).enable();

    init_scheduler(load_scheduler);

    loop {}
}

#[no_mangle]
pub extern "C" fn SysTick() {
    trigger_PendSV();
}

// Set PendSV to pending
// Interrupt control and state register (ICSR)  0xE000ED04
#[no_mangle]
#[allow(non_snake_case)]
pub fn trigger_PendSV() {
    let icsr: u32 = 0xE000_ED04;
    unsafe {
        write_volatile(
            icsr as *mut u32,
            read_volatile(icsr as *mut u32) | (0b1 << 28),
        );
    }
}
