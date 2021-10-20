#![no_std]
#![no_main]
#![feature(asm)]
extern crate rt;
use core::*;
use rt::sched::scheduler;

#[repr(C)]
pub struct ProcessFrame {
    // stuff: [u32; 32],
    r0:  u32;
    r1:  u32;
    r2:  u32;
    r3:  u32;
    r12: u32;
    lr:  u32;
    pc:  u32;
    psr: u32;
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    lr: u32,
    // pc: u32
}

fn context1() {
    unsafe {
        asm! {"bkpt"}
    }
    loop {}
}

#[no_mangle]
pub fn main() -> ! {
    unsafe {
        let mut process_1 = ProcessFrame {
            // stuff: mem::MaybeUninit::uninit().assume_init(),
            r0:  0,
            r1:  0,
            r2:  0,
            r3:  0,
            r12: 0,
            lr:  0,
            pc:  0,
            psr: 0,
            r4:  0,
            r5:  0,
            r6:  0,
            r7:  0,
            r8:  0,
            r9:  0,
            r10: 0,
            r11: 0,
            lr: context1 as *const () as u32,
            // pc: context1 as *const () as u32,
        };
        scheduler::dispatch_task(ptr::addr_of!(process_1.r4) as *mut u32);
    }
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}
