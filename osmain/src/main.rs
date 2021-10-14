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
        let mut quax:  [u32; 256] = core::mem::uninitialized();
        let ptr = &quax as *const u32;
        // for i in 0..240 {
        //     quax[i] = 123;
        // }
        // for i in 240..256 {
        //     quax[i] = i as u32;
        // }
        let usertask_stack_start: *const u32 = ptr.offset(256 - 16);


        // let thread_psp = usertask_stack_start.offset(8)  as *mut u32;
        // quax[i] = i as u32;
        // for i in 7..9 {
            let fn_start = usertask_stack_start.offset(9)  as *mut u32;
            let sdf = context1 as *const ();
            ptr::write_volatile(fn_start, sdf as u32);
        // }

        bar(usertask_stack_start as *mut u32);
    }
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}