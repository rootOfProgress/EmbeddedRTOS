#![no_std]
#![no_main]
#![feature(asm)]
extern crate rt;
use core::*;
use rt::sched::scheduler;

#[repr(C)]
pub struct ProcessFrame {
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
    stuff: [u32; 32],
}

impl ProcessFrame {
    pub fn new(target: u32) -> ProcessFrame {
        ProcessFrame {
            r4: 0x66a,
            r5: 0x669,
            r6: 0x668,
            r7: 0x667,
            r8: 0x666,
            r9: 0x665,
            r10: 0x664,
            r11: 0x663,
            r0: 0x110,
            r1: 0,
            r2: 0,
            r3: 0,
            r12: 0x9978a,
            lr: deschedule as *const () as u32,
            pc: target,
            psr: 0x21000000,
            stuff: unsafe { mem::MaybeUninit::uninit().assume_init() },
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
        unsafe {
            let mut reg_content = 0x0000_0000;
            reg_content |= (0b1_u32) << 12;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
    }
}

fn context3() {
    loop {
        unsafe {
            let mut reg_content = 0x0000_0000;
            reg_content |= (0b1_u32) << 13;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
    }
}

fn context2() {
    loop {
        unsafe {
            let mut reg_content = 0x0000_0000;
            reg_content |= (0b1_u32) << 14;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
    }
}
fn context1() {
    unsafe {
        asm!(
            "
        mov r4, 0xAF4
        mov r5, 0xAF5
        mov r6, 0xAF6
        mov r7, 0xAF7 
        mov r8, 0xAF8
        mov r9, 0xAF9
        mov r10, 0xAFA 
        mov r11, 0xAFB
        "
        );
    }
    loop {
        unsafe {
            let mut reg_content = 0x0000_0000;
            reg_content |= (0b1_u32) << 11;
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

    // todo: pass addr!!!
    scheduler::insert_task(ptr::addr_of!(process_1.r4) as u32);
    scheduler::insert_task(ptr::addr_of!(process_2.r4) as u32);
    scheduler::insert_task(ptr::addr_of!(process_3.r4) as u32);
    scheduler::insert_task(ptr::addr_of!(process_4.r4) as u32);
    // unsafe {
    //     asm! {"bkpt"}
    // }
    unsafe {
        asm!(
            "
        mov r0, 0xF0
        mov r1, 0xF1
        mov r2, 0xF2
        mov r3, 0xF3 
        // 8 regs
        mov r4, 0xF4
        mov r5, 0xF5
        mov r6, 0xF6
        mov r7, 0xF7 
        mov r8, 0xF8
        mov r9, 0xF9
        mov r10, 0xFA 
        mov r11, 0xFB
        "
        );
    }
    // scheduler::run(0);
    loop {
        unsafe {
            let mut reg_content = 0x0000_0000;
            // for i in 0..31 {

            //     reg_content |= (0b1_u32) << i as u32;
            // }
            reg_content |= (0b1_u32) << 13;
            ptr::write_volatile(0x4800_1014 as *mut u32, reg_content);
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}
