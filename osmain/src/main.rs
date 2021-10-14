#![no_std]
#![no_main]
#![feature(asm)]
extern crate rt;
use core::*;
// mod rt::sched;
fn context1() {
    // let mut usr_stack: &[u32] = &[0; 256];
    unsafe {
        asm! {"bkpt"}
    }
    loop {}
}

#[no_mangle]
extern "C" {
    // fn bar(stack_ptr: *mut u32);
    fn bar(stack_ptr: *mut u32);
}

#[no_mangle]
pub fn main() -> ! {
    // context1();
    unsafe {
        let mut process_stack:  [u32; 256] = core::mem::uninitialized();
        let stack_ptr = &process_stack as *const u32;
        let usertask_stack_start: *const u32 = stack_ptr.offset(256 - 16);

        let fn_start = usertask_stack_start.offset(9)  as *mut u32;
        ptr::write_volatile(fn_start, context1 as *const () as u32);


        bar(usertask_stack_start as *mut u32);
    }
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}