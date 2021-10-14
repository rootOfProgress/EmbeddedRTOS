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
pub fn main() -> ! {
    let _x = 42;
    let mut usr_stack:  [*mut u32; 256];
    unsafe {
        usr_stack = core::mem::uninitialized();
        usr_stack[248] = context1 as *mut u32;
        // unsafe {
        //     asm! {"bkpt"}
        // }
    }
    rt::sched::scheduler::dispatch(usr_stack[240]);
    // let code_start: *const u32 = usr_stack.as_ptr();
    // context1();
    loop {}
}
