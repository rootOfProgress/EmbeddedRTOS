use core::{
    panic,
    ptr::{self, addr_of},
};

use crate::{__invoke, __save_psp, __schedule};
use ProcessState::*;
/// # Scheduler

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum ProcessState {
    Created,
    Waiting,
    Running,
    _Blocked,
    _Terminated,
}

#[repr(C, align(8))]
pub struct AutoStackFrame {
    auto_r0: u32,
    auto_r1: u32,
    auto_r2: u32,
    auto_r3: u32,
    auto_r12: u32,
    auto_lr: u32,
    auto_pc: u32,
    auto_x_psr: u32,
}

#[repr(C)]
pub struct InitialStackFrame {
    r4_control: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    // r9_tr: u32,
    r10: u32,
    r11: u32,
    lr: u32,
}

#[repr(C)]
pub struct ProcessFrame {
    p_stack: [u32; 64],
    isf: InitialStackFrame,
    asf: AutoStackFrame,
    psp: u32,
    id: usize,
    state: ProcessState,
}

#[repr(C)]
struct KernelFrame {
    p_stack: [u32; 512],
    isf: InitialStackFrame,
    asf: AutoStackFrame,
    id: usize,
    state: ProcessState,
    p_buffer: [u32; 512],
}

pub struct Scheduler {
    pub threads: [Option<ProcessFrame>; 4],
}

impl Scheduler {
    pub fn default() -> Scheduler {
        Scheduler {
            threads: [None, None, None, None],
        }
    }

    pub fn add_user_task(&mut self, user_task: fn()) -> Result<(), ()> {
        let user_task_routine = ptr::addr_of!(user_task);
        match self
            .threads
            .iter_mut()
            .enumerate()
            .find(|(_, t)| t.is_none())
        {
            Some((n, empty_slot)) => {
                let mut p = ProcessFrame {
                    p_stack: [0; 64],
                    isf: InitialStackFrame {
                        r4_control: 0x3,
                        r5: 0xFF5,
                        r6: 0xFF6,
                        r7: 0xFF7,
                        r8: 0xFF8,
                        // r9_tr: 0xFF9,
                        r10: 0xFFA,
                        r11: 0xFFB,
                        lr: 0xfffffffd,
                    },
                    asf: AutoStackFrame {
                        auto_r0: 0xaa,
                        auto_r1: 0xbb,
                        auto_r2: 0xcc,
                        auto_r3: 0xdd,
                        auto_r12: 0xabcdef,
                        auto_lr: 0xaabbcc,
                        auto_pc: unsafe { user_task_routine.read_unaligned() as *const () as u32 },
                        auto_x_psr: 0x1000000,
                    },
                    psp: 0x0,
                    id: n,
                    state: Created,
                };
                p.psp = addr_of!(p.asf.auto_r0) as *const () as u32;
                // p.isf.r9_tr = addr_of!(p.asf.auto_r0) as *const () as u32;
                *empty_slot = Some(p);
                Ok(())
            }
            None => Err(()),
        }
    }

    pub fn schedule_user_threads(&mut self) {
        loop {
            for thread in self.threads.iter_mut() {
                if let Some(t) = thread {
                    match t.state {
                        Created => Scheduler::handle_created(t),
                        Waiting => Scheduler::handle_waiting(t),
                        Running => Scheduler::handle_running(t),
                        _Blocked => panic!(),
                        _Terminated => panic!(),
                    }
                }
            }
        }
    }

    fn handle_created(t: &mut ProcessFrame) {
        unsafe {
            // __schedule();
            // needs to call handler!!!
            t.state = Running;
            __invoke(t.psp);
            t.psp = __save_psp();
            t.state = Waiting;
        }
    }

    fn handle_waiting(t: &mut ProcessFrame) {
        unsafe {
            t.state = Running;
            __invoke(t.psp);
            t.psp = __save_psp();
            t.state = Waiting;
        }
    }

    fn handle_running(_t: &mut ProcessFrame) {}
}

// #[no_mangle]
// pub extern "C" fn SVCall(state: ProcessState) {

// }

pub fn init_scheduler(init_fn: fn()) {
    let init_fn = ptr::addr_of!(init_fn);
    let kernel_thread = KernelFrame {
        p_stack: [0; 512],
        isf: InitialStackFrame {
            r4_control: 0x2,
            r5: 0xFF5,
            r6: 0xFF6,
            r7: 0xFF7,
            r8: 0xFF8,
            // r9_tr: 0xFF9,
            r10: 0xFFA,
            r11: 0xFFB,
            lr: 0xfffffffd,
        },
        asf: AutoStackFrame {
            auto_r0: 0xaaa,
            auto_r1: 0xbbb,
            auto_r2: 0xccc,
            auto_r3: 0xddd,
            auto_r12: 0xabcdef,
            auto_lr: 0xffaabbcc,
            auto_pc: unsafe { init_fn.read_unaligned() as *const () as u32 },
            auto_x_psr: 0x1000000,
        },
        id: 1,
        state: Running,
        p_buffer: [0; 512],
    };
    // kernel_thread.isf.r9_tr = ptr::addr_of!(kernel_thread.asf.auto_r0) as *const () as u32;
    unsafe {
        __invoke(addr_of!(kernel_thread.asf.auto_r0) as *const () as u32);
    }
}
