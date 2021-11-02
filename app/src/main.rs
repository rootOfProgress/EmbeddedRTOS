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
    svc::{sprint, SVC},
};

static mut TOGGLE_FLAG: bool = false;

extern "C" {
    pub fn __invoke(x: u32) -> u32;
    pub fn __syscall(svc: &SVC);
    pub fn __breakpoint();
    pub fn __save_psp() -> u32;
    pub fn __sprint(text: *const u8);
    pub fn __sreadc();
    pub fn __sprintc(char: *const u8);
    pub fn __get_r0() -> u32;
    pub fn __get_r5() -> u32;
}

#[no_mangle]
fn user_task() {
    sprint("Task 1 started\n");
    let serial = unsafe { PERIPHERALS.take_serial() };
    let mut ahb1 = serial.ahb1();
    ahb1.rcc(|rcc| rcc.iopeen());

    let gpioe = serial.ahb2().gpioe();
    let mut leds = handler::LED::new(gpioe);
    leds.on(9);
    leds.on(8);
    loop {
        unsafe {
            if TOGGLE_FLAG {
                sprint("TASK 1: Toggle LED\n");
                leds.toggle(9);
                TOGGLE_FLAG = false;
            }
        }
    }
}

#[no_mangle]
fn user_task_turn_off_led() {
    sprint("Task 2 started\n");
    loop {
        unsafe {
            if !TOGGLE_FLAG {
                sprint("TASK 2: Set Toggle true\n");
            }
            TOGGLE_FLAG = true;
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

    unsafe {
        __syscall(&SVC::SYS_READC);
        let input_text = [__get_r5() as u8];
        __syscall(&SVC::SYS_WRITEC(input_text.as_ptr()));
    }

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
