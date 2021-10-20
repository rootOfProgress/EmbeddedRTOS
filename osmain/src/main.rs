#![no_std]
#![no_main]
#![feature(asm)]
extern crate rt;
use core::*;
use rt::sched::scheduler;

fn context1() {
    unsafe {
        asm! {"svc 0"}
    }
    loop {}
}


#[no_mangle]
pub fn main() -> ! {
    unsafe {
        let process_stack:  [u32; 32] = mem::MaybeUninit::uninit().assume_init();
        let stack_ptr = &process_stack as *const u32;

        let proc_stack_start: *const u32 = stack_ptr.offset(32 - 16);

        let fn_start = proc_stack_start.offset(9)  as *mut u32;
        ptr::write_volatile(fn_start, context1 as *const () as u32);

        scheduler::dispatch_task(proc_stack_start as *mut u32);

    }
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}