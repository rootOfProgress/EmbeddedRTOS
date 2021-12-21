#![no_std]
#![no_main]
#![feature(asm)]
extern crate rt;
extern crate userspace;

use core::ptr::addr_of;
use rt::interrupts;
use rt::sys::call_api::println;
use rt::sched::{process, scheduler, task_control};

#[no_mangle]
pub fn main() -> ! {
    let process_1 = process::ProcessFrame::new(userspace::user::context2 as *const () as u32);
    let process_2 = process::ProcessFrame::new(userspace::user::context3 as *const () as u32);
    let process_3 = process::ProcessFrame::new(userspace::user::context1 as *const () as u32);
    let process_4 = process::ProcessFrame::new(userspace::user::context0 as *const () as u32);
    task_control::insert(addr_of!(process_1.r4) as u32);
    task_control::insert(addr_of!(process_2.r4) as u32);
    task_control::insert(addr_of!(process_3.r4) as u32);
    task_control::insert(addr_of!(process_4.r4) as u32);

    scheduler::load();

    // TODO : make a syscall to enable on finishing setup
    interrupts::systick::STK::set_up_systick(4);

    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    println("---- !!!KERNEL PANIC!!! ---");
    loop {}
}
