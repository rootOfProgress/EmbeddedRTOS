#![no_std]
#![no_main]
#![feature(asm)]
extern crate rt;
use core::*;
use rt::sched::scheduler;
use rt::ctrl::control;

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
}

fn deschedule() {
    unsafe {
        asm! {"bkpt"}
    }
}

fn context1() {
    unsafe {
        asm! {"bkpt"}
    }
}

#[no_mangle]
pub fn main() -> ! {
    unsafe {
        let mut process_1 = ProcessFrame {
            // stuff: mem::MaybeUninit::uninit().assume_init(),
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
        };
        let xy = ptr::addr_of!(process_1.r4) as u32;
        // asm! {"bkpt"}
        scheduler::init(0, ptr::addr_of!(process_1.r4) as u32);
        // control::__write_psp(ptr::addr_of!(process_1.r4) as u32);
    }
    unsafe {
        asm! {"svc 0"}
        // control::__load_process_context();
        // control::__exec();
    }
    loop {
    
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}
