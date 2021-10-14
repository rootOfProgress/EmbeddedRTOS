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
    fn bar(stack_ptr: *mut u32);
}

#[no_mangle]
pub fn main() -> ! {
    let _x = 42;
    let mut usr_stack:  [*mut u32; 256];
    unsafe {
        usr_stack = core::mem::uninitialized();
        usr_stack[248] = context1 as *mut u32;
    }
    rt::sched::scheduler::dispatch(usr_stack[240]);
    unsafe {
        asm! {"bkpt"}
    }
    // unsafe {
    //     bar(usr_stack[240]);
    // }
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}