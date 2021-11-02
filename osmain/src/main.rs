#![no_std]
#![no_main]
#![feature(asm)]
extern crate rt;
use core::*;
use rt::sched::scheduler;
use rt::interrupts;
use rt::{print_dec, print_str};    
use rt::dev::tim2;

#[repr(C)]
pub struct ProcessFrame {
    stuff: [u32; 256],
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
    pub fn new(target: u32, endpoint: u32) -> ProcessFrame {
        ProcessFrame {
            stuff: unsafe { mem::MaybeUninit::uninit().assume_init() },
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
            lr: endpoint as *const () as u32,
            pc: target,
            psr: 0x21000000,
        }
    }
}

fn deschedule() {
    unsafe {
        asm! {"bkpt"}
    }
}

fn context4() {
    loop {
        // print_k("running context -> 4...\n\r");
        unsafe {
            let mut reg_content = 0x0000_0000;
            reg_content |= (0b1_u32) << 12;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
    }
}

fn context3() {
    loop {
        // print_k("running context -> 3...\n\r");
        unsafe {
            let mut reg_content = 0x0000_0000;
            reg_content |= (0b1_u32) << 13;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
    }
}

fn context2() {
    loop {

        // print_k("running context -> 2...\n\r");
        unsafe {
            let mut reg_content = 0x0000_0000;
            reg_content |= (0b1_u32) << 14;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
    }
}
fn context1() {
    loop {
        // print_k("running context -> 1...\n\r");
        unsafe {
            let mut reg_content = 0x0000_0000;
            reg_content |= (0b1_u32) << 11;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
    }
}

fn init() {
    loop {}
}

#[no_mangle]
pub fn main() -> ! {
    let process_0 = ProcessFrame::new(init as *const () as u32, init as *const () as u32);
    let process_1 = ProcessFrame::new(context1 as *const () as u32, context1 as *const () as u32);
    let process_2 = ProcessFrame::new(context2 as *const () as u32, context1 as *const () as u32);
    let process_3 = ProcessFrame::new(context3 as *const () as u32, context1 as *const () as u32);
    let process_4 = ProcessFrame::new(context4 as *const () as u32, context1 as *const () as u32);

    scheduler::queue_task(ptr::addr_of!(process_0.r4) as u32, true);
    scheduler::queue_task(ptr::addr_of!(process_1.r4) as u32, true);
    scheduler::queue_task(ptr::addr_of!(process_2.r4) as u32, true);
    scheduler::queue_task(ptr::addr_of!(process_3.r4) as u32, true);
    scheduler::queue_task(ptr::addr_of!(process_4.r4) as u32, true);

    scheduler::immediate_start(ptr::addr_of!(process_0.r4));
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}
