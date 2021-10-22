#![no_std]
#![no_main]
#![feature(asm)]
extern crate rt;
use core::*;
use rt::sched::scheduler;
use rt::ctrl::control;
use rt::gpio;
#[repr(C)]
pub struct ProcessFrame {
    // stuff: [u32; 32],
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r0:  u32,
    r1:  u32,
    r2:  u32,
    r3:  u32,
    r12: u32,
    lr:  u32,
    pc:  u32,
    psr: u32,
    stuff: [u32; 32]
}

fn deschedule() {
    unsafe {
        asm! {"bkpt"}
    }
}



fn context2() {
    // unsafe {
    //     asm! {"bkpt"}
    // }
    loop {
        unsafe {
            let mut foo = ptr::read_volatile(0x4800_1014 as *const u32);
            foo = 0x0000_0000;
            foo |= (0b1 as u32) << 14;
            ptr::write_volatile(0x4800_1014 as *mut u32, foo);
        }

    }
}
fn context1() {
    // unsafe {
    //     asm! {"bkpt"}
    // }
    loop {
        unsafe {
            let mut foo = ptr::read_volatile(0x4800_1014 as *const u32);
            foo = 0x0000_0000;
            foo |= (0b1 as u32) << 11;
            ptr::write_volatile(0x4800_1014 as *mut u32, foo);
        }

    }
}

#[no_mangle]
pub fn main() -> ! {
    unsafe {
        let mut process_1 = ProcessFrame {
            r4:  0x66a,
            r5:  0x669,
            r6:  0x668,
            r7:  0x667,
            r8:  0x666,
            r9:  0x665,
            r10: 0x664,
            r11: 0x663,
            r0:  0x110,
            r1:  0,
            r2:  0,
            r3:  0,
            r12: 0x9978a,
            lr:  deschedule as *const () as u32,
            pc:  context1 as *const () as u32,
            psr: 0x21000000,
            stuff: mem::MaybeUninit::uninit().assume_init(),
        };
        let xy = ptr::addr_of!(process_1.r4) as u32;
        scheduler::init(0, ptr::addr_of!(process_1.r4) as u32);

        let mut process_2 = ProcessFrame {
            r4:  0x66a,
            r5:  0x669,
            r6:  0x668,
            r7:  0x667,
            r8:  0x666,
            r9:  0x665,
            r10: 0x664,
            r11: 0x663,
            r0:  0x110,
            r1:  0,
            r2:  0,
            r3:  0,
            r12: 0x9978a,
            lr:  deschedule as *const () as u32,
            pc:  context2 as *const () as u32,
            psr: 0x21000000,
            stuff: mem::MaybeUninit::uninit().assume_init(),

        };
        let xy = ptr::addr_of!(process_2.r4) as u32;
        scheduler::init(1, ptr::addr_of!(process_2.r4) as u32);
    }
    unsafe {
        asm! {"svc 0"}
    }
    loop {
    
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}
