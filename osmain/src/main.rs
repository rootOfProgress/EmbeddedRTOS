#![no_std]
#![no_main]
#![feature(asm)]
extern crate rt;
use core::*;
use rt::dev::uart::print_dec;
use rt::dev::{tim2, tim3};
use rt::interrupts;
use rt::sched::{scheduler, task_control, process};

use rt::sys::call_api;


fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn context4() {
    loop {
        unsafe {
            let mut reg_content = ptr::read_volatile(0x4800_1014 as *mut u32);
            reg_content |= (0b1_u32) << 12;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
        call_api::sleep(500);
        unsafe {
            let mut reg_content = ptr::read_volatile(0x4800_1014 as *mut u32);
            reg_content &= !((0b1_u32) << 12);
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
        call_api::sleep(500);
    }
}

fn context3() {
    loop {
        call_api::yield_task();
    }
}

fn context2() {
    loop {
        fibonacci(22);
    }
}
fn context1() {
    loop {
        tim2::start_measurement();
        call_api::enable_rt_mode();
        fibonacci(22);
        call_api::disable_rt_mode();
        tim2::stop_measurement();
        let t = tim2::read_value() / 1_000_000;
        call_api::println("fibonacci 20th digit calc took -> \0");
        print_dec(t);
        call_api::println(" ms\n\r\0");
        tim2::reset_timer();
    }
}

fn _init() {
    loop {
        unsafe {
            let mut reg_content = 0x0000_0000;
            reg_content |= (0b1_u32) << 10;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
    }
}

#[no_mangle]
pub fn main() -> ! {
    let process_1 = process::ProcessFrame::new(context1 as *const () as u32);
    let process_2 = process::ProcessFrame::new(context2 as *const () as u32);
    let process_3 = process::ProcessFrame::new(context3 as *const () as u32);
    let process_4 = process::ProcessFrame::new(context4 as *const () as u32);
    task_control::insert(ptr::addr_of!(process_1.r4) as u32);
    task_control::insert(ptr::addr_of!(process_2.r4) as u32);
    task_control::insert(ptr::addr_of!(process_3.r4) as u32);
    task_control::insert(ptr::addr_of!(process_4.r4) as u32);

    scheduler::load();

    // unsafe {

    // asm!("bkpt");
    // }
    // tim3::reset_timer();
    unsafe {
        asm!("bkpt");
    }

    // TODO : make a syscall to enable on finishing setup
    interrupts::systick::STK::set_up_systick(7);

    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}
