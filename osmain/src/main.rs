#![no_std]
#![no_main]
#![feature(asm)]
extern crate rt;
use core::*;
use rt::dev::tim2;
use rt::dev::uart::{print_dec, print_str};
use rt::interrupts;
use rt::sched::{scheduler, task_control};

use rt::sys::call_api;

#[repr(C)]
pub struct ProcessFrame {
    free_space: [u32; 256],
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    psr: u32,
}

impl ProcessFrame {
    pub fn new(target: u32) -> ProcessFrame {
        ProcessFrame {
            free_space: unsafe { mem::zeroed() },
            r4: 0x66a,
            r5: 0x669,
            r6: 0x668,
            r7: 0x667,
            r8: 0x666,
            r9: 0x665,
            r10: 0x664,
            r11: 0x663,
            r0: 0x110,
            r1: 0xAA,
            r2: 0xBB,
            r3: 0xCC,
            r12: 0x9978a,
            lr: call_api::terminate as *const () as u32,
            pc: target,
            psr: 0x21000000,
        }
    }
}


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
            let mut reg_content = 0x0000_0000;
            reg_content |= (0b1_u32) << 12;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
    }
}

fn context3() {
    loop {
        call_api::yield_task();
        // unsafe {
        // let mut reg_content = 0x0000_0000;
        // reg_content |= (0b1_u32) << 13;
        // ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        // }
    }
}

fn context2() {
    //call_api::sleep();
    loop {
        unsafe {
            let mut reg_content = 0x0000_0000;
            reg_content |= (0b1_u32) << 14;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
    }
}
fn context1() {
    loop {
        tim2::start_measurement();
        call_api::enable_rt_mode();
        fibonacci(20);
        call_api::disable_rt_mode();
        tim2::stop_measurement();
        let t = tim2::read_value() / 1_000_000;
        print_str("fibonacci 20th digit calc took -> ");
        print_dec(t);
        print_str(" ms\n\r");
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
    let process_1 = ProcessFrame::new(context1 as *const () as u32);
    let process_2 = ProcessFrame::new(context2 as *const () as u32);
    let process_3 = ProcessFrame::new(context3 as *const () as u32);
    let process_4 = ProcessFrame::new(context4 as *const () as u32);
    task_control::insert(ptr::addr_of!(process_1.r4) as u32);
    task_control::insert(ptr::addr_of!(process_2.r4) as u32);
    task_control::insert(ptr::addr_of!(process_3.r4) as u32);
    task_control::insert(ptr::addr_of!(process_4.r4) as u32);

    scheduler::immediate_start(ptr::addr_of!(process_1.r4));
    unsafe {
        asm!("bkpt");
    }

    // TODO : make a syscall to enable on finishing setup
    interrupts::systick::STK::set_up_systick(5);

    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}
