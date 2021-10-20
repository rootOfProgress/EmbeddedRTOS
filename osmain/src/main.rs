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
    ip: u32,
    lr: u32,
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    pc: u32,
    psr: u32,
}

impl ProcessFrame {
    // pub fn new() -> ProcessFrame {
    //     ProcessFrame {
    //         p: unsafe { &mut *(0x2000_0)FF as *mut ProcessFrame) }
    //     }
    // }
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
        let process_stack: [u32; 16] = mem::MaybeUninit::uninit().assume_init();
        let mut frame = ProcessFrame {
            r4: 0x123C,
            r5: 0x123F,
            r6: 0xFFFF_AAAA,
            r7: 0x78F3,
            r8: 0x78F2,
            r9: 0x78F1,
            r10: 0x666,
            r11: 0x66,
            ip: 0,
            lr: context1 as *const () as u32,
            r0: 0xFFFF_AAAB,
            r1: 0x123,
            r2: 0x123,
            r3: 0x123,
            r12:0x123D,
            pc: 0x123B,
            psr:0x123,
        };
        // let stack_ptr = &process_stack as *const u32;
// 
        // let proc_stack_start: *const u32 = stack_ptr.offset(16 - 16);
        // let fn_start = proc_stack_start.offset(9) as *mut u32;
        // ptr::write_volatile(fn_start, context1 as *const () as u32);
        
        // frame.lr = context1 as *const () as u32;
        // let raw = ptr::addr_of!(frame);
        // // asm! {"bkpt"};
        // let baz = raw.offset(9) as *mut u32;
        scheduler::dispatch_task(ptr::addr_of!(frame) as *mut u32);
    }
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}
